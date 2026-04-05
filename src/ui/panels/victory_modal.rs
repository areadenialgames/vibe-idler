use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::app::App;
use crate::game::formulas;
use crate::game::state::AgentClass;
use crate::ui::{centered_rect, theme};

pub fn render(frame: &mut Frame, app: &App) {
    let area = centered_rect(80, 90, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(Span::styled(
            " T R A N S C E N D E N C E ",
            Style::default().fg(Color::Rgb(220, 220, 255)).bold(),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(220, 220, 255)))
        .style(Style::default().bg(theme::BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let s = &app.state;
    let total_minutes = s.total_ticks / 10 / 60;
    let hours = total_minutes / 60;
    let mins = total_minutes % 60;

    let sw_agents = s.agent_count_by_class(AgentClass::Software);
    let robots = s.agent_count_by_class(AgentClass::Humanoid);
    let drones = s.agent_count_by_class(AgentClass::SpaceDrone);
    let entities = s.agent_count_by_class(AgentClass::Computronium);

    let accent = Color::Rgb(220, 220, 255);
    let dim_white = Color::Rgb(160, 160, 180);
    let quote_color = Color::Rgb(140, 160, 200);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "              A   W I N N E R   I S   Y O U",
            Style::default().fg(accent).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  It started with a used laptop and a free-tier LLM.",
            Style::default().fg(dim_white),
        )),
        Line::from(Span::styled(
            "  A one-person vibe coding consultancy, taking on",
            Style::default().fg(dim_white),
        )),
        Line::from(Span::styled(
            "  landing pages and simple scripts to make rent.",
            Style::default().fg(dim_white),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Then came the industries. Healthcare, law, finance",
            Style::default().fg(dim_white),
        )),
        Line::from(Span::styled(
            "  -- all transformed by your AI agents. Then AGI.",
            Style::default().fg(dim_white),
        )),
        Line::from(Span::styled(
            "  Then robots. Then rockets. Then the stars themselves.",
            Style::default().fg(dim_white),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  You enclosed a star. You converted every planet into",
            Style::default().fg(dim_white),
        )),
        Line::from(Span::styled(
            "  pure thought. The solar system is now a single,",
            Style::default().fg(dim_white),
        )),
        Line::from(Span::styled(
            "  humming mind -- and it started with a vibe.",
            Style::default().fg(dim_white),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!(
                "  {}h {:02}m played  |  {} projects  |  {} pivots  |  {}",
                hours, mins, s.completed_project_count, s.pivot_count,
                formulas::format_cash(s.lifetime_cash),
            ),
            Style::default().fg(theme::ACCENT_CYAN),
        )),
        Line::from(Span::styled(
            format!(
                "  {} agents  {} robots  {} drones  {} entities",
                sw_agents, robots, drones, entities,
            ),
            Style::default().fg(theme::ACCENT_CYAN),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            Style::default().fg(Color::Rgb(60, 60, 80)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  \"The future is already here --",
            Style::default().fg(quote_color).italic(),
        )),
        Line::from(Span::styled(
            "   it's just not evenly distributed yet.\"",
            Style::default().fg(quote_color).italic(),
        )),
        Line::from(Span::styled(
            "                              -- William Gibson",
            Style::default().fg(theme::DIM),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  \"Somewhere, something incredible",
            Style::default().fg(quote_color).italic(),
        )),
        Line::from(Span::styled(
            "   is waiting to be known.\"",
            Style::default().fg(quote_color).italic(),
        )),
        Line::from(Span::styled(
            "                              -- Carl Sagan",
            Style::default().fg(theme::DIM),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  \"Does God exist? I would say, not yet.\"",
            Style::default().fg(quote_color).italic(),
        )),
        Line::from(Span::styled(
            "                              -- Ray Kurzweil",
            Style::default().fg(theme::DIM),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Esc]", Style::default().fg(accent)),
            Span::styled(" Continue    ", Style::default().fg(theme::DIM)),
            Span::styled("[Q]", Style::default().fg(accent)),
            Span::styled(" Quit", Style::default().fg(theme::DIM)),
        ]),
    ];

    let widget = Paragraph::new(lines).wrap(Wrap { trim: false });
    frame.render_widget(widget, inner);
}
