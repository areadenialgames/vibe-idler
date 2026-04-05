use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(Span::styled(
            " Event Log ",
            Style::default().fg(theme::ACCENT_YELLOW).bold(),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let max_events = inner.height as usize;
    let events: Vec<Line> = app
        .state
        .event_log
        .iter()
        .rev()
        .take(max_events)
        .map(|event| {
            let icon_color = match event.kind {
                crate::game::state::EventKind::ProjectCompleted => theme::ACCENT_GREEN,
                crate::game::state::EventKind::BugFound => theme::ACCENT_RED,
                crate::game::state::EventKind::ClientMessage => theme::ACCENT_CYAN,
                crate::game::state::EventKind::Achievement => theme::ACCENT_PURPLE,
                crate::game::state::EventKind::Upgrade => theme::ACCENT_CYAN,
                crate::game::state::EventKind::AgentHired => theme::ACCENT_CYAN,
                crate::game::state::EventKind::Income => theme::ACCENT_GREEN,
                crate::game::state::EventKind::Expense => theme::ACCENT_RED,
                crate::game::state::EventKind::RandomEvent => theme::ACCENT_YELLOW,
                crate::game::state::EventKind::PhaseTransition => theme::ACCENT_PURPLE,
                crate::game::state::EventKind::MegaProjectUpdate => theme::ACCENT_YELLOW,
                crate::game::state::EventKind::Victory => Color::Rgb(220, 220, 255),
            };

            let game_minutes = event.tick / 10 / 60;
            let game_seconds = (event.tick / 10) % 60;

            Line::from(vec![
                Span::styled(
                    format!(" [{:02}:{:02}] ", game_minutes, game_seconds),
                    Style::default().fg(theme::DIM),
                ),
                Span::styled(
                    format!("{} ", event.kind.icon()),
                    Style::default().fg(icon_color),
                ),
                Span::styled(&event.message, Style::default().fg(theme::FG)),
            ])
        })
        .collect();

    let widget = Paragraph::new(events);
    frame.render_widget(widget, inner);
}
