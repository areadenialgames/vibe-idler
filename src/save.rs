use std::io;
use std::path::PathBuf;

use crate::game::state::GameState;

fn save_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("vibe-idler");
    let _ = std::fs::create_dir_all(&path);
    path.push("save.json");
    path
}

pub fn save_game(state: &GameState) -> io::Result<()> {
    let path = save_path();
    let json = serde_json::to_string(state)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let tmp_path = path.with_extension("json.tmp");
    std::fs::write(&tmp_path, &json)?;
    std::fs::rename(&tmp_path, &path)?;
    Ok(())
}

pub fn delete_save() -> io::Result<()> {
    let path = save_path();
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    Ok(())
}

pub fn load_game() -> io::Result<Option<GameState>> {
    let path = save_path();
    if !path.exists() {
        return Ok(None);
    }
    let json = std::fs::read_to_string(&path)?;
    let state: GameState = serde_json::from_str(&json)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(Some(state))
}
