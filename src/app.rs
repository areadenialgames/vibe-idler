use crossterm::event::KeyEvent;

use crate::game::state::GameState;
use crate::input::Action;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Modal {
    None,
    Shop,
    Help,
}

pub struct UiState {
    pub modal: Modal,
    pub shop_tab: usize,
    pub selected_item: usize,
}

impl UiState {
    fn new() -> Self {
        Self {
            modal: Modal::None,
            shop_tab: 0,
            selected_item: 0,
        }
    }
}

pub struct App {
    pub state: GameState,
    pub ui: UiState,
    pub running: bool,
}

impl App {
    pub fn with_state(state: GameState) -> Self {
        Self {
            state,
            ui: UiState::new(),
            running: true,
        }
    }

    pub fn tick(&mut self) {
        crate::game::tick::tick(&mut self.state);
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        if let Some(action) = crate::input::map_key(key, &self.ui) {
            match action {
                Action::Quit => self.running = false,
                Action::OpenShop => self.ui.modal = Modal::Shop,
                Action::OpenHelp => self.ui.modal = Modal::Help,
                Action::CloseModal => self.ui.modal = Modal::None,
                Action::SelectNext => {
                    self.ui.selected_item = self.ui.selected_item.saturating_add(1);
                }
                Action::SelectPrev => {
                    self.ui.selected_item = self.ui.selected_item.saturating_sub(1);
                }
                Action::TabNext => {
                    self.ui.shop_tab = (self.ui.shop_tab + 1) % 4;
                    self.ui.selected_item = 0;
                }
                Action::TabPrev => {
                    self.ui.shop_tab = if self.ui.shop_tab == 0 { 3 } else { self.ui.shop_tab - 1 };
                    self.ui.selected_item = 0;
                }
                Action::Confirm => {
                    if self.ui.modal == Modal::Shop {
                        crate::game::economy::try_purchase(&mut self.state, self.ui.shop_tab, self.ui.selected_item);
                    }
                }
                Action::Pivot => {
                    let rep = crate::game::prestige::calculate_pivot_reputation(&self.state);
                    if rep > 0.0 {
                        crate::game::prestige::perform_pivot(&mut self.state);
                    }
                }
            }
        }
    }
}
