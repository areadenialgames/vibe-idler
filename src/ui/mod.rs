pub mod panels;
pub mod theme;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, Modal};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Top-level vertical layout
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
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
        Modal::Projects => panels::projects_modal::render(frame, app),
        Modal::Agents => panels::agents_modal::render(frame, app),
        Modal::TechTree => panels::tech_tree_modal::render(frame, app),
        Modal::Help => render_help_modal(frame, app),
        Modal::ConfirmPivot => render_confirm_pivot_modal(frame, app),
        Modal::ConfirmReset => render_confirm_reset_modal(frame),
        Modal::Victory => panels::victory_modal::render(frame, app),
        Modal::None => {}
    }
}

fn render_help_modal(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 60, frame.area());
    frame.render_widget(ratatui::widgets::Clear, area);

    let mut help_text = vec![
        Line::from(Span::styled(
            "VIBE IDLER - Help",
            Style::default().fg(theme::ACCENT_CYAN).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "You run a vibe coding consultancy.",
            Style::default().fg(theme::FG),
        )),
        Line::from(Span::styled(
            "AI agents build software that earns money.",
            Style::default().fg(theme::FG),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Controls:",
            Style::default().fg(theme::ACCENT_YELLOW).bold(),
        )),
        Line::from(Span::styled(
            "  S - Open Shop (buy hardware, LLMs, agents)",
            Style::default().fg(theme::FG),
        )),
        Line::from(Span::styled(
            "  ? - This help screen",
            Style::default().fg(theme::FG),
        )),
        Line::from(Span::styled(
            "  Q - Quit game",
            Style::default().fg(theme::FG),
        )),
        Line::from(Span::styled(
            "  R - Reset game (delete save)",
            Style::default().fg(theme::ACCENT_RED),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "In Shop:",
            Style::default().fg(theme::ACCENT_YELLOW).bold(),
        )),
        Line::from(Span::styled(
            "  Tab/Arrow - Switch tabs",
            Style::default().fg(theme::FG),
        )),
        Line::from(Span::styled(
            "  j/k       - Navigate items",
            Style::default().fg(theme::FG),
        )),
        Line::from(Span::styled(
            "  Enter     - Buy selected item",
            Style::default().fg(theme::FG),
        )),
        Line::from(Span::styled(
            "  Esc       - Close",
            Style::default().fg(theme::FG),
        )),
    ];

    let has_ambient = app
        .state
        .unlocked_upgrades
        .contains(&"perk_ambient_audio_owned".to_string());
    let has_radio = app
        .state
        .unlocked_upgrades
        .contains(&"perk_radio_owned".to_string());

    if has_ambient || has_radio {
        help_text.push(Line::from(""));
        help_text.push(Line::from(Span::styled(
            "Audio:",
            Style::default().fg(theme::ACCENT_YELLOW).bold(),
        )));
        if has_ambient {
            let status = if app.state.audio_enabled { "ON" } else { "OFF" };
            help_text.push(Line::from(Span::styled(
                format!("  M - Toggle Ambient Sound [{}]", status),
                Style::default().fg(theme::ACCENT_PURPLE),
            )));
        }
        if has_radio {
            let status = if app.state.radio_enabled { "ON" } else { "OFF" };
            help_text.push(Line::from(Span::styled(
                format!("  N - Toggle Streaming Sub [{}]", status),
                Style::default().fg(theme::ACCENT_PURPLE),
            )));
            if !app.audio_playback.station_names.is_empty() {
                let station_name = app
                    .audio_playback
                    .station_names
                    .get(app.state.radio_station)
                    .map(String::as_str)
                    .unwrap_or("???");
                help_text.push(Line::from(Span::styled(
                    format!(
                        "  ,/. - Station: {} ({}/{})",
                        station_name,
                        app.state.radio_station + 1,
                        app.audio_playback.station_names.len()
                    ),
                    Style::default().fg(theme::ACCENT_PURPLE),
                )));
            }
        }
    }

    help_text.push(Line::from(""));
    help_text.push(Line::from(Span::styled(
        "Design (c) 2026 - Area Denial LLC",
        Style::default().fg(theme::DIM),
    )));
    help_text.push(Line::from(""));
    help_text.push(Line::from(Span::styled(
        "Press Esc to close",
        Style::default().fg(theme::DIM),
    )));

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACCENT_YELLOW))
        .style(Style::default().bg(theme::BG));

    let paragraph = Paragraph::new(help_text).block(block);
    frame.render_widget(paragraph, area);
}

fn render_confirm_pivot_modal(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 40, frame.area());
    frame.render_widget(ratatui::widgets::Clear, area);

    let rep = crate::game::prestige::calculate_pivot_reputation(&app.state);
    let new_total = app.state.reputation + rep;

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Why are you pivoting?",
            Style::default().fg(theme::ACCENT_PURPLE).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  \"{}\"", app.ui.pivot_story),
            Style::default().fg(theme::ACCENT_YELLOW),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  You will lose all progress but gain:",
            Style::default().fg(theme::FG),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("    Reputation: ", Style::default().fg(theme::DIM)),
            Span::styled(
                format!("+{:.0}", rep),
                Style::default().fg(theme::ACCENT_GREEN),
            ),
            Span::styled(
                format!("  (total: {:.0})", new_total),
                Style::default().fg(theme::DIM),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Enter", Style::default().fg(theme::ACCENT_PURPLE).bold()),
            Span::styled(" to pivot    ", Style::default().fg(theme::FG)),
            Span::styled("Esc", Style::default().fg(theme::ACCENT_GREEN).bold()),
            Span::styled(" to cancel", Style::default().fg(theme::FG)),
        ]),
    ];

    let block = Block::default()
        .title(Span::styled(
            " Pivot Consultancy ",
            Style::default().fg(theme::ACCENT_PURPLE).bold(),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACCENT_PURPLE))
        .style(Style::default().bg(theme::BG));

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn render_confirm_reset_modal(frame: &mut Frame) {
    let area = centered_rect(40, 20, frame.area());
    frame.render_widget(ratatui::widgets::Clear, area);

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Are you sure you want to reset?",
            Style::default().fg(theme::ACCENT_RED).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  All progress will be lost.",
            Style::default().fg(theme::FG),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Enter", Style::default().fg(theme::ACCENT_RED).bold()),
            Span::styled(" to confirm    ", Style::default().fg(theme::FG)),
            Span::styled("Esc", Style::default().fg(theme::ACCENT_GREEN).bold()),
            Span::styled(" to cancel", Style::default().fg(theme::FG)),
        ]),
    ];

    let block = Block::default()
        .title(Span::styled(
            " Reset Game ",
            Style::default().fg(theme::ACCENT_RED).bold(),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACCENT_RED))
        .style(Style::default().bg(theme::BG));

    let paragraph = Paragraph::new(text).block(block);
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
