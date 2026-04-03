use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::game::formulas;
use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let s = &app.state;

    let income_per_sec = s.income_per_tick() * 10.0; // 10 ticks/sec

    let spans = vec![
        Span::styled("  VIBE IDLER", Style::default().fg(theme::ACCENT_CYAN).bold()),
        Span::styled("  |  ", Style::default().fg(theme::BORDER)),
        Span::styled(formulas::format_cash(s.cash), Style::default().fg(theme::ACCENT_GREEN).bold()),
        Span::styled("  |  ", Style::default().fg(theme::BORDER)),
        Span::styled(
            format!("{}/s", formulas::format_cash(income_per_sec)),
            Style::default().fg(if income_per_sec > 0.0 { theme::ACCENT_GREEN } else { theme::DIM }),
        ),
        Span::styled("  |  ", Style::default().fg(theme::BORDER)),
        Span::styled(
            format!("{:.0} compute", s.total_compute),
            Style::default().fg(theme::ACCENT_CYAN),
        ),
        Span::styled("  |  ", Style::default().fg(theme::BORDER)),
        Span::styled(
            format!("{}/{} agents", s.agents.len(), s.max_agents + s.prestige_bonuses.extra_agent_slots),
            Style::default().fg(theme::ACCENT_CYAN),
        ),
        Span::styled("  |  ", Style::default().fg(theme::BORDER)),
        Span::styled(
            format!("LLM: {}", s.active_llm.name()),
            Style::default().fg(theme::ACCENT_YELLOW),
        ),
        Span::styled("  |  ", Style::default().fg(theme::BORDER)),
        Span::styled(
            format!("Rep: {}", s.reputation as u64),
            Style::default().fg(theme::ACCENT_PURPLE),
        ),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::BG));

    let paragraph = Paragraph::new(Line::from(spans))
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}
