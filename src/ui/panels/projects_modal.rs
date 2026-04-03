use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::App;
use crate::game::formulas;
use crate::game::state::*;
use crate::ui::{centered_rect, theme};

pub fn render(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 70, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(Span::styled(" Projects ", Style::default().fg(theme::ACCENT_YELLOW).bold()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACCENT_YELLOW))
        .style(Style::default().bg(theme::BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    // Active projects
    lines.push(Line::from(Span::styled(
        " Active Projects",
        Style::default().fg(theme::ACCENT_CYAN).bold(),
    )));
    lines.push(Line::from(""));

    if app.state.active_projects.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No active projects",
            Style::default().fg(theme::DIM),
        )));
    } else {
        for proj in &app.state.active_projects {
            let pct = (proj.progress * 100.0) as u16;
            let bar_width = (inner.width.saturating_sub(30)) as usize;
            let filled = (bar_width as f64 * proj.progress) as usize;
            let empty = bar_width.saturating_sub(filled);

            let payment_str = match &proj.payment {
                ProjectPayment::OneTime(a) => formulas::format_cash(*a),
                ProjectPayment::Recurring { monthly } => format!("{}/mo", formulas::format_cash(*monthly)),
            };

            let assigned: Vec<String> = proj.assigned_agents.iter()
                .filter_map(|id| app.state.agents.iter().find(|a| a.id == *id))
                .map(|a| a.name.clone())
                .collect();

            let name_width = inner.width.saturating_sub(10) as usize;
            let name: String = proj.name.chars().take(name_width).collect();
            lines.push(Line::from(vec![
                Span::styled(format!("  {}", name), Style::default().fg(theme::FG)),
                Span::styled(format!("  {}", payment_str), Style::default().fg(theme::ACCENT_YELLOW)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled("█".repeat(filled), Style::default().fg(theme::ACCENT_GREEN)),
                Span::styled("░".repeat(empty), Style::default().fg(theme::DIM)),
                Span::styled(format!(" {:>3}%", pct), Style::default().fg(theme::ACCENT_GREEN)),
            ]));
            if !assigned.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("    Assigned: {}", assigned.join(", ")),
                    Style::default().fg(theme::DIM),
                )));
            }
            lines.push(Line::from(""));
        }
    }

    // Completed stats
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(" Completed: ", Style::default().fg(theme::ACCENT_CYAN).bold()),
        Span::styled(
            format!("{}", app.state.completed_project_count),
            Style::default().fg(theme::ACCENT_GREEN),
        ),
    ]));

    // Passive income
    if !app.state.passive_income_sources.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            " Passive Income",
            Style::default().fg(theme::ACCENT_CYAN).bold(),
        )));
        lines.push(Line::from(""));
        for src in &app.state.passive_income_sources {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}", src.source_name), Style::default().fg(theme::FG)),
                Span::styled(
                    format!("  {}/mo", formulas::format_cash(src.monthly_income)),
                    Style::default().fg(theme::ACCENT_GREEN),
                ),
            ]));
        }
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, inner);
}
