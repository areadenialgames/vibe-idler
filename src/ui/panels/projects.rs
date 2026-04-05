use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::game::formulas;
use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(Span::styled(
            " Active Projects ",
            Style::default().fg(theme::ACCENT_YELLOW).bold(),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 3 {
        return;
    }

    // Split inner area: active projects top, contracts & passive bottom
    // Reserve space for separator (1) + contracts (up to 3) + passive (2)
    let bottom_reserve: u16 = 1 + app.state.available_contracts.len().min(3) as u16
        + if app.state.passive_income_sources.is_empty() { 0 } else { 2 };
    let max_active_lines = inner.height.saturating_sub(bottom_reserve);
    let max_active_projects = (max_active_lines / 2) as usize; // 2 lines per project
    let active_count = app.state.active_projects.len().min(max_active_projects).max(1);
    let active_height = (active_count as u16 * 2).max(2);
    let remaining = inner.height.saturating_sub(active_height + 1);

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(active_height),
            Constraint::Length(1), // separator
            Constraint::Length(remaining),
        ])
        .split(inner);

    // Active projects with progress bars
    let mut lines: Vec<Line> = Vec::new();
    for proj in app.state.active_projects.iter().take(max_active_projects) {
        let pct = (proj.progress * 100.0) as u16;
        let bar_width = (inner.width.saturating_sub(25)) as usize;
        let filled = (bar_width as f64 * proj.progress) as usize;
        let empty = bar_width.saturating_sub(filled);

        let payment_str = match &proj.payment {
            crate::game::state::ProjectPayment::OneTime(a) => formulas::format_cash(*a),
            crate::game::state::ProjectPayment::Recurring { monthly } => {
                format!("{}/mo", formulas::format_cash(*monthly))
            }
        };

        let name_width = inner.width.saturating_sub(7) as usize; // space + pct + borders
        let name: String = proj.name.chars().take(name_width).collect();
        lines.push(Line::from(vec![
            Span::styled(
                format!(" {:<width$}", name, width = name_width),
                Style::default().fg(theme::FG),
            ),
            Span::styled(
                format!("{:>3}%", pct),
                Style::default().fg(theme::ACCENT_GREEN),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("█".repeat(filled), Style::default().fg(theme::ACCENT_GREEN)),
            Span::styled("░".repeat(empty), Style::default().fg(theme::DIM)),
            Span::styled(
                format!(" {}", payment_str),
                Style::default().fg(theme::ACCENT_YELLOW),
            ),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No active projects",
            Style::default().fg(theme::DIM),
        )));
    }

    let active_widget = Paragraph::new(lines);
    frame.render_widget(active_widget, sections[0]);

    // Separator
    let sep = Paragraph::new(Line::from(Span::styled(
        " ─── Available Contracts ───",
        Style::default().fg(theme::BORDER),
    )));
    frame.render_widget(sep, sections[1]);

    // Available contracts + passive income
    let mut bottom_lines: Vec<Line> = Vec::new();
    for contract in app.state.available_contracts.iter().take(3) {
        let payment_str = match &contract.payment {
            crate::game::state::ProjectPayment::OneTime(a) => formulas::format_cash(*a),
            crate::game::state::ProjectPayment::Recurring { monthly } => {
                format!("{}/mo", formulas::format_cash(*monthly))
            }
        };
        let contract_name_width = inner.width.saturating_sub(18) as usize; // " > " + payment + difficulty
        let name: String = contract.name.chars().take(contract_name_width).collect();
        bottom_lines.push(Line::from(vec![
            Span::styled(
                format!(" > {:<width$}", name, width = contract_name_width),
                Style::default().fg(theme::FG),
            ),
            Span::styled(payment_str, Style::default().fg(theme::ACCENT_GREEN)),
            Span::styled(
                format!(" d:{}", contract.difficulty),
                Style::default().fg(theme::DIM),
            ),
        ]));
    }

    if !app.state.passive_income_sources.is_empty() {
        let total_passive: f64 = app
            .state
            .passive_income_sources
            .iter()
            .map(|p| p.monthly_income)
            .sum();
        bottom_lines.push(Line::from(""));
        bottom_lines.push(Line::from(vec![Span::styled(
            format!(
                " Passive: {}/mo ({} sources)",
                formulas::format_cash(total_passive),
                app.state.passive_income_sources.len()
            ),
            Style::default().fg(theme::ACCENT_GREEN),
        )]));
    }

    let bottom_widget = Paragraph::new(bottom_lines);
    frame.render_widget(bottom_widget, sections[2]);
}
