use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Sparkline};

use crate::app::App;
use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area);

    // Income sparkline
    let income_data: Vec<u64> = app
        .state
        .income_history
        .iter()
        .map(|v| (v * 100.0).max(0.0) as u64)
        .collect();
    let income_spark = Sparkline::default()
        .block(
            Block::default()
                .title(Span::styled(
                    " Income ",
                    Style::default().fg(theme::ACCENT_GREEN),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::BORDER))
                .style(Style::default().bg(theme::BG)),
        )
        .data(&income_data)
        .style(Style::default().fg(theme::ACCENT_GREEN));
    frame.render_widget(income_spark, cols[0]);

    // Expense sparkline
    let expense_data: Vec<u64> = app
        .state
        .expense_history
        .iter()
        .map(|v| (v * 100.0).max(0.0) as u64)
        .collect();
    let expense_spark = Sparkline::default()
        .block(
            Block::default()
                .title(Span::styled(
                    " Expenses ",
                    Style::default().fg(theme::ACCENT_RED),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::BORDER))
                .style(Style::default().bg(theme::BG)),
        )
        .data(&expense_data)
        .style(Style::default().fg(theme::ACCENT_RED));
    frame.render_widget(expense_spark, cols[1]);

    // Net profit sparkline
    let net_data: Vec<u64> = app
        .state
        .income_history
        .iter()
        .zip(app.state.expense_history.iter())
        .map(|(i, e)| ((i - e) * 100.0).max(0.0) as u64)
        .collect();
    let net_spark = Sparkline::default()
        .block(
            Block::default()
                .title(Span::styled(
                    " Net Profit ",
                    Style::default().fg(theme::ACCENT_CYAN),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::BORDER))
                .style(Style::default().bg(theme::BG)),
        )
        .data(&net_data)
        .style(Style::default().fg(theme::ACCENT_CYAN));
    frame.render_widget(net_spark, cols[2]);
}
