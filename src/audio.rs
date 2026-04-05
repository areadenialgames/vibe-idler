#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum SfxKind {
    MenuOpen,
    MenuClose,
    MenuNav,
    TabSwitch,
    Purchase,
    CantAfford,
    ProjectComplete,
    AgentHired,
    Unlock,
    BugFound,
    RandomEvent,
    ClientMessage,
    Pivot,
    Reset,
    Toggle,
    StationChange,
}

impl SfxKind {
    #[allow(dead_code)]
    fn filename(&self) -> &'static str {
        match self {
            Self::MenuOpen => "menu_open",
            Self::MenuClose => "menu_close",
            Self::MenuNav => "menu_nav",
            Self::TabSwitch => "tab_switch",
            Self::Purchase => "purchase",
            Self::CantAfford => "cant_afford",
            Self::ProjectComplete => "project_complete",
            Self::AgentHired => "agent_hired",
            Self::Unlock => "unlock",
            Self::BugFound => "bug_found",
            Self::RandomEvent => "random_event",
            Self::ClientMessage => "client_message",
            Self::Pivot => "pivot",
            Self::Reset => "reset",
            Self::Toggle => "toggle",
            Self::StationChange => "station_change",
        }
    }

    #[allow(dead_code)]
    fn all() -> &'static [SfxKind] {
        &[
            Self::MenuOpen,
            Self::MenuClose,
            Self::MenuNav,
            Self::TabSwitch,
            Self::Purchase,
            Self::CantAfford,
            Self::ProjectComplete,
            Self::AgentHired,
            Self::Unlock,
            Self::BugFound,
            Self::RandomEvent,
            Self::ClientMessage,
            Self::Pivot,
            Self::Reset,
            Self::Toggle,
            Self::StationChange,
        ]
    }
}

/// Ambient tier: 0 = office, 1 = server_rack, 2 = data_center
#[allow(dead_code)]
pub enum AudioCommand {
    PlayAmbient(u8),
    StopAmbient,
    ChangeAmbientTier(u8),
    PlaySfx(SfxKind),
    PlayRadio,
    StopRadio,
    ChangeStation(usize),
    NextTrack,
    Shutdown,
}

#[cfg(feature = "audio")]
mod inner {
    use super::{AudioCommand, SfxKind};
    use rodio::{Decoder, OutputStream, Sink};
    use std::collections::HashMap;
    use std::io::Cursor;
    use std::path::PathBuf;
    use std::sync::mpsc;

    pub struct AudioHandle {
        tx: mpsc::Sender<AudioCommand>,
    }

    impl AudioHandle {
        pub fn new() -> Self {
            let (tx, rx) = mpsc::channel();
            std::thread::spawn(move || run_audio_thread(rx));
            AudioHandle { tx }
        }

        pub fn send(&self, cmd: AudioCommand) {
            let _ = self.tx.send(cmd);
        }
    }

    struct RadioStation {
        tracks: Vec<PathBuf>,
        track_index: usize,
    }

    fn run_audio_thread(rx: mpsc::Receiver<AudioCommand>) {
        let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
            // No audio device — drain commands silently
            for cmd in rx {
                if matches!(cmd, AudioCommand::Shutdown) {
                    break;
                }
            }
            return;
        };

        let mut ambient_sink = Sink::try_new(&stream_handle).ok();
        // Start ambient paused — reconcile() will send PlayAmbient when appropriate
        if let Some(ref sink) = ambient_sink {
            sink.pause();
        }
        let radio_sink = Sink::try_new(&stream_handle).ok();
        // Start radio paused — reconcile() will send PlayRadio when appropriate
        if let Some(ref sink) = radio_sink {
            sink.pause();
        }

        // Load ambient tiers into memory
        let ambient_tiers = load_ambient_tiers();
        let mut current_ambient_tier: u8 = 0;

        // Load SFX files into memory
        let sfx_cache = load_sfx_cache();

        // Discover radio stations
        let mut stations = discover_radio_stations();
        let mut current_station: usize = 0;

        loop {
            // Poll for commands with a timeout so we can refill sinks
            let cmd = match rx.recv_timeout(std::time::Duration::from_millis(500)) {
                Ok(cmd) => Some(cmd),
                Err(mpsc::RecvTimeoutError::Timeout) => None,
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            };

            if let Some(cmd) = cmd {
                match cmd {
                    AudioCommand::PlayAmbient(tier) => {
                        current_ambient_tier = tier;
                        if let Some(ref sink) = ambient_sink {
                            if sink.empty() {
                                append_ambient_track(sink, &ambient_tiers, current_ambient_tier);
                            }
                            sink.play();
                        }
                    }
                    AudioCommand::StopAmbient => {
                        if let Some(ref sink) = ambient_sink {
                            sink.pause();
                        }
                    }
                    AudioCommand::ChangeAmbientTier(tier) => {
                        if tier != current_ambient_tier {
                            current_ambient_tier = tier;
                            if let Some(ref sink) = ambient_sink {
                                if !sink.is_paused() {
                                    sink.stop();
                                    ambient_sink = Sink::try_new(&stream_handle).ok();
                                    if let Some(ref sink) = ambient_sink {
                                        append_ambient_track(
                                            sink,
                                            &ambient_tiers,
                                            current_ambient_tier,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    AudioCommand::PlaySfx(kind) => {
                        play_sfx(&kind, &stream_handle, &sfx_cache);
                    }
                    AudioCommand::PlayRadio => {
                        if let Some(ref sink) = radio_sink {
                            if sink.empty() {
                                play_next_station_track(sink, &mut stations, current_station);
                            }
                            sink.play();
                        }
                    }
                    AudioCommand::StopRadio => {
                        if let Some(ref sink) = radio_sink {
                            sink.pause();
                        }
                    }
                    AudioCommand::ChangeStation(idx) => {
                        current_station = idx;
                        if let Some(ref sink) = radio_sink {
                            sink.stop();
                        }
                    }
                    AudioCommand::NextTrack => {
                        if let Some(ref sink) = radio_sink {
                            sink.stop();
                        }
                    }
                    AudioCommand::Shutdown => break,
                }
            }

            // Keep ambient looping — refill when the queue runs low
            if let Some(ref sink) = ambient_sink {
                if !sink.is_paused() && sink.len() < 2 {
                    append_ambient_track(sink, &ambient_tiers, current_ambient_tier);
                }
            }

            // Auto-advance radio tracks when current one finishes
            if let Some(ref sink) = radio_sink {
                if sink.empty() && !sink.is_paused() {
                    play_next_station_track(sink, &mut stations, current_station);
                }
            }
        }
    }

    const AMBIENT_FILENAMES: &[&str] = &["office", "server_rack", "data_center"];

    fn load_ambient_tiers() -> Vec<Option<Vec<u8>>> {
        AMBIENT_FILENAMES
            .iter()
            .map(|name| {
                for ext in &["mp3", "ogg", "wav"] {
                    let path = format!("assets/ambient/{}.{}", name, ext);
                    if let Ok(data) = std::fs::read(&path) {
                        return Some(data);
                    }
                }
                if let Some(data_dir) = dirs::data_dir() {
                    for ext in &["mp3", "ogg", "wav"] {
                        let path = data_dir
                            .join("vibe-idler")
                            .join("ambient")
                            .join(format!("{}.{}", name, ext));
                        if let Ok(data) = std::fs::read(path) {
                            return Some(data);
                        }
                    }
                }
                None
            })
            .collect()
    }

    fn append_ambient_track(sink: &Sink, tiers: &[Option<Vec<u8>>], tier: u8) {
        let idx = tier as usize;
        // Fall back to lower tiers if the requested one isn't available
        let data = (0..=idx)
            .rev()
            .find_map(|i| tiers.get(i).and_then(|t| t.as_ref()));

        if let Some(data) = data {
            let cursor = Cursor::new(data.clone());
            if let Ok(source) = Decoder::new(cursor) {
                sink.append(source);
            }
        }
    }

    fn load_sfx_cache() -> HashMap<&'static str, Vec<u8>> {
        let mut cache = HashMap::new();

        for kind in SfxKind::all() {
            let filename = kind.filename();
            // Check multiple directories and formats
            for dir in &["assets/sfx"] {
                for ext in &["ogg", "wav", "mp3"] {
                    let path = format!("{}/{}.{}", dir, filename, ext);
                    if let Ok(data) = std::fs::read(&path) {
                        cache.insert(filename, data);
                        break;
                    }
                }
                if cache.contains_key(filename) {
                    break;
                }
            }

            // Also check OS data dir
            if !cache.contains_key(filename) {
                if let Some(data_dir) = dirs::data_dir() {
                    let sfx_dir = data_dir.join("vibe-idler").join("sfx");
                    for ext in &["ogg", "wav", "mp3"] {
                        let path = sfx_dir.join(format!("{}.{}", filename, ext));
                        if let Ok(data) = std::fs::read(&path) {
                            cache.insert(filename, data);
                            break;
                        }
                    }
                }
            }
        }

        cache
    }

    fn play_sfx(
        kind: &SfxKind,
        stream_handle: &rodio::OutputStreamHandle,
        sfx_cache: &HashMap<&str, Vec<u8>>,
    ) {
        if let Some(data) = sfx_cache.get(kind.filename()) {
            let cursor = Cursor::new(data.clone());
            if let Ok(source) = Decoder::new(cursor) {
                if let Ok(sink) = Sink::try_new(stream_handle) {
                    sink.append(source);
                    sink.detach(); // Fire and forget
                }
            }
        }
    }

    fn discover_radio_stations() -> Vec<RadioStation> {
        let mut stations = Vec::new();
        let mut seen_names = Vec::new();

        for base_dir in radio_base_dirs() {
            if let Ok(entries) = std::fs::read_dir(&base_dir) {
                let mut dirs: Vec<_> = entries.flatten().filter(|e| e.path().is_dir()).collect();
                dirs.sort_by_key(|e| e.file_name());

                for entry in dirs {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if seen_names.contains(&name) {
                        continue;
                    }
                    let mut tracks = Vec::new();
                    scan_dir_for_tracks(&entry.path(), &mut tracks);
                    if !tracks.is_empty() {
                        seen_names.push(name);
                        stations.push(RadioStation {
                            tracks,
                            track_index: 0,
                        });
                    }
                }
            }
        }

        stations
    }

    fn radio_base_dirs() -> Vec<PathBuf> {
        let mut dirs = vec![PathBuf::from("assets/radio")];
        if let Some(data_dir) = dirs::data_dir() {
            dirs.push(data_dir.join("vibe-idler").join("radio"));
        }
        dirs
    }

    fn scan_dir_for_tracks(dir: &std::path::Path, tracks: &mut Vec<PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        match ext.to_lowercase().as_str() {
                            "mp3" | "ogg" | "wav" | "flac" => tracks.push(path),
                            _ => {}
                        }
                    }
                }
            }
        }
        tracks.sort();
    }

    fn play_next_station_track(sink: &Sink, stations: &mut [RadioStation], station_idx: usize) {
        if station_idx >= stations.len() {
            return;
        }
        let station = &mut stations[station_idx];
        if station.tracks.is_empty() {
            return;
        }

        station.track_index %= station.tracks.len();
        let path = &station.tracks[station.track_index];
        station.track_index += 1;

        if let Ok(file) = std::fs::File::open(path) {
            let reader = std::io::BufReader::new(file);
            if let Ok(source) = Decoder::new(reader) {
                sink.append(source);
            }
        }
    }
}

#[cfg(not(feature = "audio"))]
mod inner {
    use super::AudioCommand;

    pub struct AudioHandle;

    impl AudioHandle {
        pub fn new() -> Self {
            AudioHandle
        }

        pub fn send(&self, _cmd: AudioCommand) {}
    }
}

pub use inner::AudioHandle;

/// Tracks audio playback state to avoid redundant commands each tick.
pub struct AudioPlayback {
    pub ambient_playing: bool,
    pub ambient_tier: u8,
    pub radio_playing: bool,
    pub current_station: usize,
    pub station_names: Vec<String>,
}

impl AudioPlayback {
    pub fn new() -> Self {
        Self {
            ambient_playing: false,
            ambient_tier: 0,
            radio_playing: false,
            current_station: 0,
            station_names: discover_station_names(),
        }
    }

    pub fn reconcile(&mut self, state: &crate::game::state::GameState, audio: &AudioHandle) {
        let should_ambient = state
            .unlocked_upgrades
            .contains(&"perk_ambient_audio_owned".to_string())
            && state.audio_enabled;
        let should_radio = state
            .unlocked_upgrades
            .contains(&"perk_radio_owned".to_string())
            && state.radio_enabled;

        let tier = ambient_tier_for_state(state);

        if should_ambient && !self.ambient_playing {
            self.ambient_tier = tier;
            audio.send(AudioCommand::PlayAmbient(tier));
            self.ambient_playing = true;
        } else if !should_ambient && self.ambient_playing {
            audio.send(AudioCommand::StopAmbient);
            self.ambient_playing = false;
        } else if self.ambient_playing && tier != self.ambient_tier {
            self.ambient_tier = tier;
            audio.send(AudioCommand::ChangeAmbientTier(tier));
        }

        if should_radio && !self.radio_playing {
            audio.send(AudioCommand::ChangeStation(state.radio_station));
            audio.send(AudioCommand::PlayRadio);
            self.radio_playing = true;
            self.current_station = state.radio_station;
        } else if !should_radio && self.radio_playing {
            audio.send(AudioCommand::StopRadio);
            self.radio_playing = false;
        } else if self.radio_playing && state.radio_station != self.current_station {
            audio.send(AudioCommand::ChangeStation(state.radio_station));
            audio.send(AudioCommand::PlayRadio);
            self.current_station = state.radio_station;
        }
    }
}

/// Determine ambient tier based on hardware ownership.
/// 0 = office (default), 1 = server_rack, 2 = data_center
fn ambient_tier_for_state(state: &crate::game::state::GameState) -> u8 {
    use crate::game::state::HardwareKind;
    let has = |kind: HardwareKind| state.hardware.iter().any(|h| h.kind == kind && h.count > 0);

    if has(HardwareKind::DataCenter) {
        2
    } else if has(HardwareKind::ServerRack) {
        1
    } else {
        0
    }
}

/// Discover station names by scanning radio directories for subdirectories.
/// Used by the UI to display station names without needing the audio feature.
fn discover_station_names() -> Vec<String> {
    let mut names = Vec::new();

    for base in &["assets/radio"] {
        if let Ok(entries) = std::fs::read_dir(base) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        if !names.contains(&name.to_string()) {
                            names.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    if let Some(data_dir) = dirs::data_dir() {
        let radio_dir = data_dir.join("vibe-idler").join("radio");
        if let Ok(entries) = std::fs::read_dir(radio_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !names.contains(&name) {
                        names.push(name);
                    }
                }
            }
        }
    }

    names.sort();
    names
}
