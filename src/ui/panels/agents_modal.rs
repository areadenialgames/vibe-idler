use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::App;
use crate::game::state::*;
use crate::ui::{centered_rect, theme};

pub fn render(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 70, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(Span::styled(
            " Agents ",
            Style::default().fg(theme::ACCENT_CYAN).bold(),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACCENT_CYAN))
        .style(Style::default().bg(theme::BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let max = app.state.max_agents + app.state.prestige_bonuses.extra_agent_slots;
    let mut lines: Vec<Line> = vec![
        Line::from(vec![Span::styled(
            format!(" Agents: {} / {}", app.state.agents.len(), max),
            Style::default().fg(theme::ACCENT_CYAN).bold(),
        )]),
        Line::from(""),
    ];

    for agent in &app.state.agents {
        let spec_color = match agent.specialization {
            AgentSpec::Frontend => theme::ACCENT_YELLOW,
            AgentSpec::Backend => theme::ACCENT_GREEN,
            AgentSpec::Mobile => theme::ACCENT_PURPLE,
            AgentSpec::DevOps => theme::ACCENT_CYAN,
            _ => theme::FG,
        };

        lines.push(Line::from(vec![
            Span::styled(
                format!("  {}", agent.name),
                Style::default().fg(theme::ACCENT_CYAN).bold(),
            ),
            Span::styled(
                format!("  [{}]", agent.specialization.name()),
                Style::default().fg(spec_color),
            ),
        ]));

        // Status
        let (status_text, status_color) = match &agent.status {
            AgentStatus::Idle => ("Idle".to_string(), theme::DIM),
            AgentStatus::Working => {
                let proj_name = agent
                    .current_project
                    .and_then(|i| app.state.active_projects.get(i))
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| "...".into());
                (format!("Working: {}", proj_name), theme::ACCENT_GREEN)
            }
            AgentStatus::Debugging => ("Debugging...".to_string(), theme::ACCENT_RED),
        };
        lines.push(Line::from(Span::styled(
            format!("    {}", status_text),
            Style::default().fg(status_color),
        )));

        // Stats
        let skill_pct = ((agent.skill_level - 1.0) / 2.0).min(1.0);
        let bar_len = 15;
        let filled = (bar_len as f64 * skill_pct) as usize;
        let empty = bar_len - filled;
        lines.push(Line::from(vec![
            Span::styled(
                format!("    Skill: {:.2} ", agent.skill_level),
                Style::default().fg(theme::DIM),
            ),
            Span::styled("█".repeat(filled), Style::default().fg(theme::ACCENT_CYAN)),
            Span::styled("░".repeat(empty), Style::default().fg(theme::DIM)),
        ]));

        lines.push(Line::from(Span::styled(
            format!(
                "    Lines: {}  Bugs: {}",
                agent.lines_written, agent.bugs_introduced
            ),
            Style::default().fg(theme::DIM),
        )));
        lines.push(Line::from(""));
    }

    if app.state.agents.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No agents spun up yet",
            Style::default().fg(theme::DIM),
        )));
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, inner);
}
