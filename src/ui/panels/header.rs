use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::game::formulas;
use crate::game::state::{AgentClass, GamePhase};
use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let s = &app.state;
    let phase = s.phase;
    let accent = theme::phase_accent(phase);
    let border = theme::phase_border(phase);

    let income_per_sec = s.income_per_tick() * 10.0;

    let sep = Span::styled("  |  ", Style::default().fg(border));

    let mut spans = vec![
        Span::styled(
            format!("  {}", phase.name()),
            Style::default().fg(accent).bold(),
        ),
        sep.clone(),
        Span::styled(
            formulas::format_cash(s.cash),
            Style::default().fg(theme::ACCENT_GREEN).bold(),
        ),
        sep.clone(),
        Span::styled(
            format!("{}/s", formulas::format_cash(income_per_sec)),
            Style::default().fg(if income_per_sec > 0.0 {
                theme::ACCENT_GREEN
            } else {
                theme::DIM
            }),
        ),
        sep.clone(),
        Span::styled(
            format_compute_short(s.total_compute),
            Style::default().fg(theme::ACCENT_CYAN),
        ),
        sep.clone(),
    ];

    // Phase-adaptive stats
    match phase {
        GamePhase::Consultancy | GamePhase::Industry => {
            spans.push(Span::styled(
                format!(
                    "{}/{} agents",
                    s.agents.len(),
                    s.max_for_class(AgentClass::Software)
                ),
                Style::default().fg(theme::ACCENT_CYAN),
            ));
            spans.push(sep.clone());
            spans.push(Span::styled(
                format!("LLM: {}", s.active_llm.name()),
                Style::default().fg(theme::ACCENT_YELLOW),
            ));
            spans.push(sep.clone());
            spans.push(Span::styled(
                format!("Rep: {}", s.reputation as u64),
                Style::default().fg(theme::ACCENT_PURPLE),
            ));
        }
        GamePhase::PostHuman => {
            let robots = s.agent_count_by_class(AgentClass::Humanoid);
            spans.push(Span::styled(
                format!(
                    "{} agents + {} robots",
                    s.agent_count_by_class(AgentClass::Software),
                    robots
                ),
                Style::default().fg(theme::ACCENT_CYAN),
            ));
            spans.push(sep.clone());
            spans.push(Span::styled(
                s.active_llm.name(),
                Style::default().fg(theme::ACCENT_YELLOW),
            ));
        }
        GamePhase::SpaceAge => {
            let total_units: u32 = s.agents.len() as u32;
            let drones = s.agent_count_by_class(AgentClass::SpaceDrone);
            spans.push(Span::styled(
                format!("{} units ({} drones)", total_units, drones),
                Style::default().fg(theme::ACCENT_CYAN),
            ));
            spans.push(sep.clone());
            spans.push(Span::styled(
                s.active_llm.name(),
                Style::default().fg(theme::ACCENT_YELLOW),
            ));
        }
        GamePhase::Kardashev => {
            spans.push(Span::styled(
                format!("{} workers", s.agents.len()),
                Style::default().fg(theme::ACCENT_CYAN),
            ));
            spans.push(sep.clone());
            spans.push(Span::styled(
                format!("Dyson: {}/10", s.mega_projects.dyson_segments_completed),
                Style::default().fg(if s.mega_projects.dyson_sphere_complete {
                    theme::ACCENT_GREEN
                } else {
                    theme::ACCENT_YELLOW
                }),
            ));
            spans.push(sep.clone());
            spans.push(Span::styled(
                format!("Sol: {}/8", s.mega_projects.planets_converted),
                Style::default().fg(theme::ACCENT_PURPLE),
            ));
        }
        GamePhase::Victory => {
            spans.push(Span::styled(
                "SOLAR SYSTEM: 100% COMPUTRONIUM",
                Style::default().fg(Color::Rgb(220, 220, 255)).bold(),
            ));
        }
    }

    if cfg!(debug_assertions) && app.ticks_per_frame > 1 {
        spans.push(Span::styled("  |  ", Style::default().fg(border)));
        spans.push(Span::styled(
            format!("{}x", app.ticks_per_frame),
            Style::default().fg(theme::ACCENT_RED).bold(),
        ));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border))
        .style(Style::default().bg(theme::BG));

    let paragraph = Paragraph::new(Line::from(spans))
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

fn format_compute_short(val: f64) -> String {
    if val >= 1_000_000_000.0 {
        format!("{:.1}B compute", val / 1_000_000_000.0)
    } else if val >= 1_000_000.0 {
        format!("{:.1}M compute", val / 1_000_000.0)
    } else if val >= 1_000.0 {
        format!("{:.0}K compute", val / 1_000.0)
    } else {
        format!("{:.0} compute", val)
    }
}
