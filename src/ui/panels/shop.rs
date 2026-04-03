use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Tabs};

use crate::app::App;
use crate::game::formulas;
use crate::game::state::*;
use crate::ui::{centered_rect, theme};

pub fn render(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 70, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(Span::styled(" Shop ", Style::default().fg(theme::ACCENT_GREEN).bold()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACCENT_GREEN))
        .style(Style::default().bg(theme::BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 5 {
        return;
    }

    // Tab bar
    let tab_titles = vec!["Hardware", "LLM", "Agents", "Automation"];
    let tabs = Tabs::new(tab_titles.iter().map(|t| Line::from(*t)).collect::<Vec<_>>())
        .select(app.ui.shop_tab)
        .style(Style::default().fg(theme::DIM))
        .highlight_style(Style::default().fg(theme::ACCENT_GREEN).bold())
        .divider(Span::styled(" | ", Style::default().fg(theme::BORDER)));

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(inner);

    frame.render_widget(tabs, sections[0]);

    match app.ui.shop_tab {
        0 => render_hardware_tab(frame, sections[1], app),
        1 => render_llm_tab(frame, sections[1], app),
        2 => render_agent_tab(frame, sections[1], app),
        3 => render_automation_tab(frame, sections[1], app),
        _ => {}
    }

    let help = Paragraph::new(Line::from(vec![
        Span::styled(" Tab", Style::default().fg(theme::ACCENT_GREEN)),
        Span::styled(":switch  ", Style::default().fg(theme::DIM)),
        Span::styled("j/k", Style::default().fg(theme::ACCENT_GREEN)),
        Span::styled(":select  ", Style::default().fg(theme::DIM)),
        Span::styled("Enter", Style::default().fg(theme::ACCENT_GREEN)),
        Span::styled(":buy  ", Style::default().fg(theme::DIM)),
        Span::styled("Esc", Style::default().fg(theme::ACCENT_YELLOW)),
        Span::styled(":close", Style::default().fg(theme::DIM)),
    ]));
    frame.render_widget(help, sections[2]);
}

fn render_hardware_tab(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();

    for (i, kind) in HardwareKind::all().iter().enumerate() {
        let unlocked = app.state.unlocked_upgrades.contains(&kind.unlock_id().to_string());
        let owned = app.state.hardware.iter()
            .find(|h| h.kind == *kind)
            .map(|h| h.count)
            .unwrap_or(0);

        let cost = formulas::hardware_cost(
            kind.base_cost(),
            kind.growth_rate(),
            owned,
            app.state.prestige_bonuses.cost_reduction,
        );

        let can_afford = app.state.cash >= cost;
        let selected = i == app.ui.selected_item;

        if !unlocked {
            lines.push(Line::from(Span::styled(
                format!("  {} [LOCKED]", kind.name()),
                Style::default().fg(theme::DIM),
            )));
            continue;
        }

        let marker = if selected { "> " } else { "  " };
        let name_style = if selected {
            Style::default().fg(theme::ACCENT_GREEN).bold()
        } else {
            Style::default().fg(theme::FG)
        };
        let cost_color = if can_afford { theme::ACCENT_GREEN } else { theme::ACCENT_RED };

        lines.push(Line::from(vec![
            Span::styled(marker, Style::default().fg(theme::ACCENT_GREEN)),
            Span::styled(format!("{:<18}", kind.name()), name_style),
            Span::styled(format!("{:<12}", formulas::format_cash(cost)), Style::default().fg(cost_color)),
            Span::styled(format!("owned:{} ", owned), Style::default().fg(theme::DIM)),
            Span::styled(format!("+{:.0} compute", kind.compute()), Style::default().fg(theme::ACCENT_CYAN)),
        ]));
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, area);
}

fn render_llm_tab(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();

    for (i, tier) in LlmTier::all().iter().enumerate() {
        let unlocked = app.state.unlocked_upgrades.contains(&tier.unlock_id().to_string());
        let is_active = app.state.active_llm == *tier;
        let cost = tier.unlock_cost();
        let can_afford = app.state.cash >= cost;
        let selected = i == app.ui.selected_item;

        if !unlocked {
            lines.push(Line::from(Span::styled(
                format!("  {} [LOCKED]", tier.name()),
                Style::default().fg(theme::DIM),
            )));
            continue;
        }

        let marker = if selected { "> " } else { "  " };
        let name_style = if is_active {
            Style::default().fg(theme::ACCENT_PURPLE).bold()
        } else if selected {
            Style::default().fg(theme::ACCENT_GREEN).bold()
        } else {
            Style::default().fg(theme::FG)
        };

        let status = if is_active {
            Span::styled(" [ACTIVE]", Style::default().fg(theme::ACCENT_PURPLE))
        } else {
            let cost_color = if can_afford { theme::ACCENT_GREEN } else { theme::ACCENT_RED };
            Span::styled(format!(" {}", formulas::format_cash(cost)), Style::default().fg(cost_color))
        };

        lines.push(Line::from(vec![
            Span::styled(marker, Style::default().fg(theme::ACCENT_GREEN)),
            Span::styled(format!("{:<22}", tier.name()), name_style),
            status,
            Span::styled(format!("  quality:{:.1}", tier.quality()), Style::default().fg(theme::ACCENT_CYAN)),
        ]));
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, area);
}

fn render_agent_tab(frame: &mut Frame, area: Rect, app: &App) {
    let agent_count = app.state.agents.len() as u32;
    let max = app.state.max_agents + app.state.prestige_bonuses.extra_agent_slots;
    let cost = formulas::agent_hire_cost(agent_count);
    let can_afford = app.state.cash >= cost;
    let at_cap = agent_count >= max;

    let mut lines = vec![
        Line::from(vec![
            Span::styled(
                format!(" Agents: {} / {}", agent_count, max),
                Style::default().fg(theme::ACCENT_CYAN).bold(),
            ),
        ]),
        Line::from(""),
    ];

    if at_cap {
        lines.push(Line::from(Span::styled(
            "  Agent cap reached! Unlock more in tech tree.",
            Style::default().fg(theme::ACCENT_YELLOW),
        )));
    } else {
        let cost_color = if can_afford { theme::ACCENT_GREEN } else { theme::ACCENT_RED };
        lines.push(Line::from(vec![
            Span::styled("> Hire Agent  ", Style::default().fg(theme::ACCENT_GREEN).bold()),
            Span::styled(formulas::format_cash(cost), Style::default().fg(cost_color)),
        ]));
        lines.push(Line::from(Span::styled(
            "  Press Enter to hire",
            Style::default().fg(theme::DIM),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" Current Agents:", Style::default().fg(theme::ACCENT_CYAN))));

    for agent in &app.state.agents {
        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", agent.name), Style::default().fg(theme::FG)),
            Span::styled(format!("[{}] ", agent.specialization.name()), Style::default().fg(theme::DIM)),
            Span::styled(format!("skill:{:.2}", agent.skill_level), Style::default().fg(theme::ACCENT_CYAN)),
        ]));
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, area);
}

fn render_automation_tab(frame: &mut Frame, area: Rect, _app: &App) {
    let lines = vec![
        Line::from(Span::styled(
            " Automation upgrades coming soon...",
            Style::default().fg(theme::DIM),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " Complete more projects to unlock!",
            Style::default().fg(theme::DIM),
        )),
    ];

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, area);
}
