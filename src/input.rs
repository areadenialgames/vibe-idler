use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{Modal, UiState};

pub enum Action {
    Quit,
    OpenShop,
    OpenHelp,
    CloseModal,
    SelectNext,
    SelectPrev,
    TabNext,
    TabPrev,
    Confirm,
    Pivot,
}

pub fn map_key(key: KeyEvent, ui: &UiState) -> Option<Action> {
    // Ctrl+C always quits
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Some(Action::Quit);
    }

    if ui.modal != Modal::None {
        return match key.code {
            KeyCode::Esc => Some(Action::CloseModal),
            KeyCode::Up | KeyCode::Char('k') => Some(Action::SelectPrev),
            KeyCode::Down | KeyCode::Char('j') => Some(Action::SelectNext),
            KeyCode::Tab | KeyCode::Right => Some(Action::TabNext),
            KeyCode::BackTab | KeyCode::Left => Some(Action::TabPrev),
            KeyCode::Enter => Some(Action::Confirm),
            _ => None,
        };
    }

    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('s') => Some(Action::OpenShop),
        KeyCode::Char('?') => Some(Action::OpenHelp),
        KeyCode::Char('v') => Some(Action::Pivot),
        _ => None,
    }
}
