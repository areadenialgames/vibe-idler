use crossterm::event::KeyEvent;

use crate::game::state::GameState;
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
}

impl App {
    pub fn with_state(state: GameState) -> Self {
        Self {
            state,
            ui: UiState::new(),
            running: true,
            ticks_per_frame: 1,
        }
    }

    pub fn tick(&mut self) {
        for _ in 0..self.ticks_per_frame {
            crate::game::tick::tick(&mut self.state);
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        if let Some(action) = crate::input::map_key(key, &self.ui) {
            match action {
                Action::Quit => self.running = false,
                Action::OpenShop => self.ui.modal = Modal::Shop,
                Action::OpenProjects => self.ui.modal = Modal::Projects,
                Action::OpenAgents => self.ui.modal = Modal::Agents,
                Action::OpenTechTree => self.ui.modal = Modal::TechTree,
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
                    } else if self.ui.modal == Modal::ConfirmPivot {
                        crate::game::prestige::perform_pivot(&mut self.state);
                        self.ui.modal = Modal::None;
                    } else if self.ui.modal == Modal::ConfirmReset {
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
