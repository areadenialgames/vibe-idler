use ratatui::style::Color;

use crate::game::state::GamePhase;

pub const BG: Color = Color::Rgb(15, 15, 25);
pub const FG: Color = Color::Rgb(180, 210, 180);
pub const ACCENT_GREEN: Color = Color::Rgb(0, 255, 128);
pub const ACCENT_RED: Color = Color::Rgb(255, 80, 80);
pub const ACCENT_CYAN: Color = Color::Rgb(0, 200, 255);
pub const ACCENT_YELLOW: Color = Color::Rgb(255, 200, 0);
pub const ACCENT_PURPLE: Color = Color::Rgb(180, 100, 255);
pub const DIM: Color = Color::Rgb(80, 100, 80);
pub const BORDER: Color = Color::Rgb(50, 70, 50);

pub fn phase_accent(phase: GamePhase) -> Color {
    match phase {
        GamePhase::Consultancy => ACCENT_GREEN,
        GamePhase::Industry => ACCENT_CYAN,
        GamePhase::PostHuman => ACCENT_PURPLE,
        GamePhase::SpaceAge => ACCENT_YELLOW,
        GamePhase::Kardashev | GamePhase::Victory => Color::Rgb(220, 220, 255),
    }
}

pub fn phase_border(phase: GamePhase) -> Color {
    match phase {
        GamePhase::Consultancy => Color::Rgb(50, 70, 50),
        GamePhase::Industry => Color::Rgb(50, 50, 80),
        GamePhase::PostHuman => Color::Rgb(80, 50, 80),
        GamePhase::SpaceAge => Color::Rgb(80, 70, 40),
        GamePhase::Kardashev => Color::Rgb(80, 80, 90),
        GamePhase::Victory => Color::Rgb(100, 100, 120),
    }
}
