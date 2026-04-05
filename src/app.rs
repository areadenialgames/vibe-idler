use crossterm::event::KeyEvent;

use crate::audio::{AudioCommand, AudioHandle, AudioPlayback, SfxKind};
use crate::game::state::{EventKind, GameState};
use crate::input::Action;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Modal {
    None,
    Shop,
    Projects,
    Agents,
    TechTree,
    Help,
    ConfirmPivot,
    ConfirmReset,
}

pub struct UiState {
    pub modal: Modal,
    pub shop_tab: usize,
    pub selected_item: usize,
    pub pivot_story: String,
}

impl UiState {
    fn new() -> Self {
        Self {
            modal: Modal::None,
            shop_tab: 0,
            selected_item: 0,
            pivot_story: String::new(),
        }
    }
}

pub struct App {
    pub state: GameState,
    pub ui: UiState,
    pub running: bool,
    pub ticks_per_frame: u32,
    pub audio: AudioHandle,
    pub audio_playback: AudioPlayback,
}

impl App {
    pub fn with_state(state: GameState, audio: AudioHandle) -> Self {
        Self {
            state,
            ui: UiState::new(),
            running: true,
            ticks_per_frame: 1,
            audio,
            audio_playback: AudioPlayback::new(),
        }
    }

    pub fn tick(&mut self) {
        let events_before = self.state.event_log.len();
        for _ in 0..self.ticks_per_frame {
            crate::game::tick::tick(&mut self.state);
        }
        self.play_event_sfx(events_before);
        self.audio_playback.reconcile(&self.state, &self.audio);
    }

    fn sfx(&self, kind: SfxKind) {
        if self.state.audio_enabled
            && self.state.unlocked_upgrades.contains(&"perk_ambient_audio_owned".to_string())
        {
            self.audio.send(AudioCommand::PlaySfx(kind));
        }
    }

    fn play_event_sfx(&self, events_before: usize) {
        if !self.state.audio_enabled
            || !self.state.unlocked_upgrades.contains(&"perk_ambient_audio_owned".to_string())
        {
            return;
        }
        let start = events_before.min(self.state.event_log.len());
        for event in self.state.event_log[start..].iter() {
            let sfx = match event.kind {
                EventKind::ProjectCompleted => Some(SfxKind::ProjectComplete),
                EventKind::Achievement => Some(SfxKind::Unlock),
                EventKind::Upgrade => Some(SfxKind::Unlock),
                EventKind::AgentHired => Some(SfxKind::AgentHired),
                EventKind::BugFound => Some(SfxKind::BugFound),
                EventKind::RandomEvent => Some(SfxKind::RandomEvent),
                EventKind::ClientMessage => Some(SfxKind::ClientMessage),
                EventKind::Income | EventKind::Expense => None,
            };
            if let Some(sfx) = sfx {
                self.audio.send(AudioCommand::PlaySfx(sfx));
            }
        }
    }

    pub fn shutdown_audio(&self) {
        self.audio.send(AudioCommand::Shutdown);
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        if let Some(action) = crate::input::map_key(key, &self.ui) {
            match action {
                Action::Quit => self.running = false,
                Action::OpenShop => {
                    self.ui.modal = Modal::Shop;
                    self.sfx(SfxKind::MenuOpen);
                }
                Action::OpenProjects => {
                    self.ui.modal = Modal::Projects;
                    self.sfx(SfxKind::MenuOpen);
                }
                Action::OpenAgents => {
                    self.ui.modal = Modal::Agents;
                    self.sfx(SfxKind::MenuOpen);
                }
                Action::OpenTechTree => {
                    self.ui.modal = Modal::TechTree;
                    self.sfx(SfxKind::MenuOpen);
                }
                Action::OpenHelp => {
                    self.ui.modal = Modal::Help;
                    self.sfx(SfxKind::MenuOpen);
                }
                Action::CloseModal => {
                    self.ui.modal = Modal::None;
                    self.sfx(SfxKind::MenuClose);
                }
                Action::SelectNext => {
                    self.ui.selected_item = self.ui.selected_item.saturating_add(1);
                    self.sfx(SfxKind::MenuNav);
                }
                Action::SelectPrev => {
                    self.ui.selected_item = self.ui.selected_item.saturating_sub(1);
                    self.sfx(SfxKind::MenuNav);
                }
                Action::TabNext => {
                    self.ui.shop_tab = (self.ui.shop_tab + 1) % 5;
                    self.ui.selected_item = 0;
                    self.sfx(SfxKind::TabSwitch);
                }
                Action::TabPrev => {
                    self.ui.shop_tab = if self.ui.shop_tab == 0 { 4 } else { self.ui.shop_tab - 1 };
                    self.ui.selected_item = 0;
                    self.sfx(SfxKind::TabSwitch);
                }
                Action::Confirm => {
                    if self.ui.modal == Modal::Shop {
                        let cash_before = self.state.cash;
                        crate::game::economy::try_purchase(&mut self.state, self.ui.shop_tab, self.ui.selected_item);
                        if self.state.cash < cash_before {
                            self.sfx(SfxKind::Purchase);
                        } else {
                            self.sfx(SfxKind::CantAfford);
                        }
                    } else if self.ui.modal == Modal::ConfirmPivot {
                        // Stop audio before pivot (perks are lost)
                        self.audio.send(AudioCommand::StopAmbient);
                        self.audio.send(AudioCommand::StopRadio);
                        self.audio_playback.ambient_playing = false;
                        self.audio_playback.radio_playing = false;
                        crate::game::prestige::perform_pivot(&mut self.state);
                        self.ui.modal = Modal::None;
                        self.sfx(SfxKind::Pivot);
                    } else if self.ui.modal == Modal::ConfirmReset {
                        // Stop all audio before resetting state
                        self.audio.send(AudioCommand::StopAmbient);
                        self.audio.send(AudioCommand::StopRadio);
                        self.audio_playback.ambient_playing = false;
                        self.audio_playback.radio_playing = false;
                        self.state = GameState::new();
                        let _ = crate::save::delete_save();
                        self.ui.modal = Modal::None;
                    }
                }
                Action::SpeedUp => {
                    self.ticks_per_frame = (self.ticks_per_frame * 2).min(1024);
                }
                Action::SpeedDown => {
                    self.ticks_per_frame = (self.ticks_per_frame / 2).max(1);
                }
                Action::ResetGame => {
                    self.ui.modal = Modal::ConfirmReset;
                }
                Action::ToggleAmbientAudio => {
                    if self.state.unlocked_upgrades.contains(&"perk_ambient_audio_owned".to_string()) {
                        self.state.audio_enabled = !self.state.audio_enabled;
                        // Play toggle SFX directly — bypass the sfx() gate since
                        // we might be toggling audio back on
                        self.audio.send(AudioCommand::PlaySfx(SfxKind::Toggle));
                    }
                }
                Action::ToggleRadio => {
                    if self.state.unlocked_upgrades.contains(&"perk_radio_owned".to_string()) {
                        self.state.radio_enabled = !self.state.radio_enabled;
                        self.sfx(SfxKind::Toggle);
                    }
                }
                Action::NextStation => {
                    if self.state.unlocked_upgrades.contains(&"perk_radio_owned".to_string())
                        && !self.audio_playback.station_names.is_empty()
                    {
                        self.state.radio_station =
                            (self.state.radio_station + 1) % self.audio_playback.station_names.len();
                        self.sfx(SfxKind::StationChange);
                    }
                }
                Action::PrevStation => {
                    if self.state.unlocked_upgrades.contains(&"perk_radio_owned".to_string())
                        && !self.audio_playback.station_names.is_empty()
                    {
                        let len = self.audio_playback.station_names.len();
                        self.state.radio_station =
                            if self.state.radio_station == 0 { len - 1 } else { self.state.radio_station - 1 };
                        self.sfx(SfxKind::StationChange);
                    }
                }
                Action::Pivot => {
                    let rep = crate::game::prestige::calculate_pivot_reputation(&self.state);
                    if rep > 0.0 {
                        self.ui.pivot_story = crate::data::pivot_stories::random_story(&mut rand::thread_rng());
                        self.ui.modal = Modal::ConfirmPivot;
                    }
                }
            }
        }
    }
}
