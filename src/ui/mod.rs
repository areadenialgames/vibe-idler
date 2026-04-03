pub mod theme;
pub mod panels;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, Modal};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Top-level vertical layout
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Sparklines
            Constraint::Length(6), // Event log
            Constraint::Length(1), // Hotkey bar
        ])
        .split(area);

    // Header
    panels::header::render(frame, rows[0], app);

    // Main content: 3 columns
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Commit log
            Constraint::Percentage(40), // Projects
            Constraint::Percentage(30), // Agents
        ])
        .split(rows[1]);

    panels::commit_log::render(frame, cols[0], app);
    panels::projects::render(frame, cols[1], app);
    panels::agents::render(frame, cols[2], app);

    // Sparklines
    panels::finances::render(frame, rows[2], app);

    // Event log
    panels::event_log::render(frame, rows[3], app);

    // Hotkey bar
    let hotkeys = Paragraph::new(Line::from(vec![
        Span::styled(" [S]", Style::default().fg(theme::ACCENT_GREEN).bold()),
        Span::styled("hop  ", Style::default().fg(theme::FG)),
        Span::styled("[P]", Style::default().fg(theme::ACCENT_GREEN).bold()),
        Span::styled("rojects  ", Style::default().fg(theme::FG)),
        Span::styled("[A]", Style::default().fg(theme::ACCENT_GREEN).bold()),
        Span::styled("gents  ", Style::default().fg(theme::FG)),
        Span::styled("[T]", Style::default().fg(theme::ACCENT_GREEN).bold()),
        Span::styled("ech Tree  ", Style::default().fg(theme::FG)),
        Span::styled("[V]", Style::default().fg(theme::ACCENT_PURPLE).bold()),
        Span::styled("Pivot  ", Style::default().fg(theme::FG)),
        Span::styled("[?]", Style::default().fg(theme::ACCENT_YELLOW).bold()),
        Span::styled("Help  ", Style::default().fg(theme::FG)),
        Span::styled("[Q]", Style::default().fg(theme::ACCENT_RED).bold()),
        Span::styled("uit", Style::default().fg(theme::FG)),
    ]))
    .style(Style::default().bg(theme::BG));
    frame.render_widget(hotkeys, rows[4]);

    // Render modal overlay if active
    match app.ui.modal {
        Modal::Shop => panels::shop::render(frame, app),
        Modal::Help => render_help_modal(frame),
        Modal::None => {}
    }
}

fn render_help_modal(frame: &mut Frame) {
    let area = centered_rect(60, 60, frame.area());
    frame.render_widget(ratatui::widgets::Clear, area);

    let help_text = vec![
        Line::from(Span::styled("VIBE IDLER - Help", Style::default().fg(theme::ACCENT_CYAN).bold())),
        Line::from(""),
        Line::from(Span::styled("You run a vibe coding consultancy.", Style::default().fg(theme::FG))),
        Line::from(Span::styled("AI agents build software that earns money.", Style::default().fg(theme::FG))),
        Line::from(""),
        Line::from(Span::styled("Controls:", Style::default().fg(theme::ACCENT_YELLOW).bold())),
        Line::from(Span::styled("  S - Open Shop (buy hardware, LLMs, agents)", Style::default().fg(theme::FG))),
        Line::from(Span::styled("  ? - This help screen", Style::default().fg(theme::FG))),
        Line::from(Span::styled("  Q - Quit game", Style::default().fg(theme::FG))),
        Line::from(""),
        Line::from(Span::styled("In Shop:", Style::default().fg(theme::ACCENT_YELLOW).bold())),
        Line::from(Span::styled("  Tab/Arrow - Switch tabs", Style::default().fg(theme::FG))),
        Line::from(Span::styled("  j/k       - Navigate items", Style::default().fg(theme::FG))),
        Line::from(Span::styled("  Enter     - Buy selected item", Style::default().fg(theme::FG))),
        Line::from(Span::styled("  Esc       - Close", Style::default().fg(theme::FG))),
        Line::from(""),
        Line::from(Span::styled("Press Esc to close", Style::default().fg(theme::DIM))),
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACCENT_YELLOW))
        .style(Style::default().bg(theme::BG));

    let paragraph = Paragraph::new(help_text).block(block);
    frame.render_widget(paragraph, area);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
