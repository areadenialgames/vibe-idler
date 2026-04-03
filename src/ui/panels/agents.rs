use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(Span::styled(" Agent Status ", Style::default().fg(theme::ACCENT_CYAN).bold()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let max_agents = (inner.height / 3) as usize;
    let mut lines: Vec<Line> = Vec::new();

    for agent in app.state.agents.iter().take(max_agents) {
        // Agent name and specialization
        let spec_color = match agent.specialization {
            crate::game::state::AgentSpec::Frontend => theme::ACCENT_YELLOW,
            crate::game::state::AgentSpec::Backend => theme::ACCENT_GREEN,
            crate::game::state::AgentSpec::Mobile => theme::ACCENT_PURPLE,
            crate::game::state::AgentSpec::DevOps => theme::ACCENT_CYAN,
            _ => theme::FG,
        };

        lines.push(Line::from(vec![
            Span::styled(format!(" {}", agent.name), Style::default().fg(theme::ACCENT_CYAN).bold()),
            Span::styled(format!(" [{}]", agent.specialization.name()), Style::default().fg(spec_color)),
        ]));

        // Status line
        let (status_text, status_color) = match &agent.status {
            crate::game::state::AgentStatus::Idle => ("Idle".to_string(), theme::DIM),
            crate::game::state::AgentStatus::Working => {
                let proj_name = agent.current_project
                    .and_then(|i| app.state.active_projects.get(i))
                    .map(|p| {
                        let n: String = p.name.chars().take(18).collect();
                        n
                    })
                    .unwrap_or_else(|| "...".into());
                (format!("Working: {}", proj_name), theme::ACCENT_GREEN)
            }
            crate::game::state::AgentStatus::Debugging => ("Debugging...".to_string(), theme::ACCENT_RED),
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {}", status_text), Style::default().fg(status_color)),
        ]));

        // Skill bar
        let skill_pct = ((agent.skill_level - 1.0) / 2.0).min(1.0); // 1.0-3.0 mapped to 0-1
        let bar_len = 10;
        let filled = (bar_len as f64 * skill_pct) as usize;
        let empty = bar_len - filled;
        lines.push(Line::from(vec![
            Span::styled(format!("  Skill:{:.2} ", agent.skill_level), Style::default().fg(theme::DIM)),
            Span::styled("█".repeat(filled), Style::default().fg(theme::ACCENT_CYAN)),
            Span::styled("░".repeat(empty), Style::default().fg(theme::DIM)),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled("  No agents", Style::default().fg(theme::DIM))));
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, inner);
}
