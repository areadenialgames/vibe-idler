use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Tabs};

use crate::app::App;
use crate::game::formulas;
use crate::game::state::*;
use crate::ui::{centered_rect, theme};

pub fn render(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 70, frame.area());
    frame.render_widget(Clear, area);

    let accent = theme::phase_accent(app.state.phase);
    let block = Block::default()
        .title(Span::styled(" Shop ", Style::default().fg(accent).bold()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent))
        .style(Style::default().bg(theme::BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 5 {
        return;
    }

    // Dynamic tab bar based on phase
    let tab_titles = visible_tab_titles(&app.state);
    let tabs = Tabs::new(
        tab_titles
            .iter()
            .map(|t| Line::from(*t))
            .collect::<Vec<_>>(),
    )
    .select(app.ui.shop_tab)
    .style(Style::default().fg(theme::DIM))
    .highlight_style(Style::default().fg(accent).bold())
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
        3 => render_experimental_tab(frame, sections[1], app),
        4 => render_robotics_tab(frame, sections[1], app),
        5 => render_space_tab(frame, sections[1], app),
        _ => {}
    }

    let help = Paragraph::new(Line::from(vec![
        Span::styled(" Tab", Style::default().fg(accent)),
        Span::styled(":switch  ", Style::default().fg(theme::DIM)),
        Span::styled("j/k", Style::default().fg(accent)),
        Span::styled(":select  ", Style::default().fg(theme::DIM)),
        Span::styled("Enter", Style::default().fg(accent)),
        Span::styled(":buy  ", Style::default().fg(theme::DIM)),
        Span::styled("Esc", Style::default().fg(theme::ACCENT_YELLOW)),
        Span::styled(":close", Style::default().fg(theme::DIM)),
    ]));
    frame.render_widget(help, sections[2]);
}

fn visible_tab_titles(state: &GameState) -> Vec<&'static str> {
    let mut tabs = vec!["Hardware", "LLM", "Agents", "Perks"];
    if state.phase >= GamePhase::PostHuman {
        tabs.push("Robotics");
    }
    if state.phase >= GamePhase::SpaceAge {
        tabs.push("Space");
    }
    tabs
}

fn render_hardware_tab(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();

    // Only show unlocked hardware (endgame tiers completely hidden until unlocked)
    let visible: Vec<&HardwareKind> = HardwareKind::all()
        .iter()
        .filter(|k| {
            app.state
                .unlocked_upgrades
                .contains(&k.unlock_id().to_string())
        })
        .collect();

    for (i, kind) in visible.iter().enumerate() {
        let owned = app
            .state
            .hardware
            .iter()
            .find(|h| h.kind == **kind)
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

        let marker = if selected { "> " } else { "  " };
        let name_style = if selected {
            Style::default().fg(theme::ACCENT_GREEN).bold()
        } else {
            Style::default().fg(theme::FG)
        };
        let cost_color = if can_afford {
            theme::ACCENT_GREEN
        } else {
            theme::ACCENT_RED
        };

        lines.push(Line::from(vec![
            Span::styled(marker, Style::default().fg(theme::ACCENT_GREEN)),
            Span::styled(format!("{:<20}", kind.name()), name_style),
            Span::styled(
                format!("{:<14}", formulas::format_cash(cost)),
                Style::default().fg(cost_color),
            ),
            Span::styled(format!("x{} ", owned), Style::default().fg(theme::DIM)),
            Span::styled(
                format!("+{} compute", format_compute(kind.compute())),
                Style::default().fg(theme::ACCENT_CYAN),
            ),
        ]));
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, area);
}

fn render_llm_tab(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();

    // Only show unlocked LLM tiers
    let visible: Vec<&LlmTier> = LlmTier::all()
        .iter()
        .filter(|t| {
            app.state
                .unlocked_upgrades
                .contains(&t.unlock_id().to_string())
        })
        .collect();

    for (i, tier) in visible.iter().enumerate() {
        let is_active = app.state.active_llm == **tier;
        let cost = tier.unlock_cost();
        let can_afford = app.state.cash >= cost;
        let selected = i == app.ui.selected_item;

        let marker = if selected { "> " } else { "  " };
        let name_style = if is_active {
            Style::default().fg(theme::ACCENT_PURPLE).bold()
        } else if selected {
            Style::default().fg(theme::ACCENT_GREEN).bold()
        } else {
            Style::default().fg(theme::FG)
        };

        let monthly = tier.monthly_cost();
        let monthly_str = if monthly > 0.0 {
            format!("{}/mo", formulas::format_cash(monthly))
        } else if **tier == LlmTier::MatrioshkaBrain {
            "Dyson-powered".into()
        } else {
            "free".into()
        };

        let is_below_active = tier.quality() < app.state.active_llm.quality();
        let (status_str, status_color) = if is_active {
            ("[ACTIVE]".to_string(), theme::ACCENT_PURPLE)
        } else if is_below_active {
            ("[OWNED]".to_string(), theme::DIM)
        } else {
            let color = if can_afford {
                theme::ACCENT_GREEN
            } else {
                theme::ACCENT_RED
            };
            (formulas::format_cash(cost), color)
        };

        lines.push(Line::from(vec![
            Span::styled(marker, Style::default().fg(theme::ACCENT_GREEN)),
            Span::styled(format!("{:<18}", tier.name()), name_style),
            Span::styled(
                format!("{:<10}", status_str),
                Style::default().fg(status_color),
            ),
            Span::styled(
                format!("{:<12}", monthly_str),
                Style::default().fg(theme::ACCENT_YELLOW),
            ),
            Span::styled(
                format!("q:{:.0}", tier.quality()),
                Style::default().fg(theme::ACCENT_CYAN),
            ),
        ]));
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, area);
}

fn render_agent_tab(frame: &mut Frame, area: Rect, app: &App) {
    let sw_count = app.state.agent_count_by_class(AgentClass::Software);
    let max = app.state.max_for_class(AgentClass::Software);
    let cost = formulas::agent_hire_cost(AgentClass::Software, sw_count);
    let can_afford = app.state.cash >= cost;
    let at_cap = sw_count >= max;

    let mut lines = vec![
        Line::from(vec![Span::styled(
            format!(" Agents: {} / {}", sw_count, max),
            Style::default().fg(theme::ACCENT_CYAN).bold(),
        )]),
        Line::from(""),
    ];

    if at_cap {
        lines.push(Line::from(Span::styled(
            "  Agent cap reached! Complete more projects to unlock slots.",
            Style::default().fg(theme::ACCENT_YELLOW),
        )));
    } else {
        let cost_color = if can_afford {
            theme::ACCENT_GREEN
        } else {
            theme::ACCENT_RED
        };
        lines.push(Line::from(vec![
            Span::styled(
                "> Spin Up Agent  ",
                Style::default().fg(theme::ACCENT_GREEN).bold(),
            ),
            Span::styled(formulas::format_cash(cost), Style::default().fg(cost_color)),
        ]));
        lines.push(Line::from(Span::styled(
            "  Press Enter to spin up",
            Style::default().fg(theme::DIM),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Current Agents:",
        Style::default().fg(theme::ACCENT_CYAN),
    )));

    for agent in app
        .state
        .agents
        .iter()
        .filter(|a| a.agent_class == AgentClass::Software)
    {
        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", agent.name), Style::default().fg(theme::FG)),
            Span::styled(
                format!("[{}] ", agent.specialization.name()),
                Style::default().fg(theme::DIM),
            ),
            Span::styled(
                format!("skill:{:.2}", agent.skill_level),
                Style::default().fg(theme::ACCENT_CYAN),
            ),
        ]));
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, area);
}


fn render_experimental_tab(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            " Office Perks",
            Style::default().fg(theme::ACCENT_PURPLE).bold(),
        )),
        Line::from(""),
    ];

    for (i, perk) in PerkKind::all().iter().enumerate() {
        let visible = app
            .state
            .unlocked_upgrades
            .contains(&perk.unlock_id().to_string());
        let owned = app.state.unlocked_upgrades.contains(&perk.owned_id());
        let selected = i == app.ui.selected_item;

        if !visible {
            lines.push(Line::from(Span::styled(
                format!("  {} [LOCKED]", perk.name()),
                Style::default().fg(theme::DIM),
            )));
            continue;
        }

        if owned {
            let marker = if selected { "> " } else { "  " };
            lines.push(Line::from(vec![
                Span::styled(marker, Style::default().fg(theme::ACCENT_PURPLE)),
                Span::styled(perk.name(), Style::default().fg(theme::ACCENT_PURPLE)),
                Span::styled(" [OWNED]", Style::default().fg(theme::ACCENT_PURPLE)),
            ]));
            continue;
        }

        let cost = perk.cost();
        let can_afford = app.state.cash >= cost;
        let marker = if selected { "> " } else { "  " };
        let name_style = if selected {
            Style::default().fg(theme::ACCENT_GREEN).bold()
        } else {
            Style::default().fg(theme::FG)
        };
        let cost_color = if can_afford {
            theme::ACCENT_GREEN
        } else {
            theme::ACCENT_RED
        };

        lines.push(Line::from(vec![
            Span::styled(marker, Style::default().fg(theme::ACCENT_GREEN)),
            Span::styled(format!("{:<24}", perk.name()), name_style),
            Span::styled(formulas::format_cash(cost), Style::default().fg(cost_color)),
        ]));

        if selected {
            lines.push(Line::from(Span::styled(
                format!("    {}", perk.description()),
                Style::default().fg(theme::DIM),
            )));
        }
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, area);
}

fn render_robotics_tab(frame: &mut Frame, area: Rect, app: &App) {
    let hum_count = app.state.agent_count_by_class(AgentClass::Humanoid);
    let hum_max = app.state.max_for_class(AgentClass::Humanoid);

    let mut lines = vec![
        Line::from(Span::styled(
            " Robotics Division",
            Style::default().fg(theme::ACCENT_PURPLE).bold(),
        )),
        Line::from(Span::styled(
            format!(" Humanoid Robots: {} / {}", hum_count, hum_max),
            Style::default().fg(theme::ACCENT_CYAN),
        )),
        Line::from(""),
    ];

    let robot_types: &[(&str, &str, AgentClass)] = &[
        (
            "Humanoid Worker",
            "1.5x bonus on physical projects",
            AgentClass::Humanoid,
        ),
        (
            "Humanoid Engineer",
            "2.5x bonus on space/manufacturing",
            AgentClass::Humanoid,
        ),
    ];

    for (i, (name, desc, class)) in robot_types.iter().enumerate() {
        let count = app.state.agent_count_by_class(*class);
        let max = app.state.max_for_class(*class);
        let at_cap = count >= max;
        let cost = formulas::agent_hire_cost(*class, count);
        let can_afford = app.state.cash >= cost;
        let selected = i == app.ui.selected_item;

        let marker = if selected { "> " } else { "  " };

        if at_cap {
            lines.push(Line::from(vec![
                Span::styled(marker, Style::default().fg(theme::DIM)),
                Span::styled(format!("{} [CAP]", name), Style::default().fg(theme::DIM)),
            ]));
        } else {
            let name_style = if selected {
                Style::default().fg(theme::ACCENT_GREEN).bold()
            } else {
                Style::default().fg(theme::FG)
            };
            let cost_color = if can_afford {
                theme::ACCENT_GREEN
            } else {
                theme::ACCENT_RED
            };
            lines.push(Line::from(vec![
                Span::styled(marker, Style::default().fg(theme::ACCENT_GREEN)),
                Span::styled(format!("{:<22}", name), name_style),
                Span::styled(formulas::format_cash(cost), Style::default().fg(cost_color)),
            ]));
        }
        if selected {
            lines.push(Line::from(Span::styled(
                format!("    {}", desc),
                Style::default().fg(theme::DIM),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Active Robots:",
        Style::default().fg(theme::ACCENT_CYAN),
    )));
    for agent in app
        .state
        .agents
        .iter()
        .filter(|a| a.agent_class == AgentClass::Humanoid)
    {
        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", agent.name), Style::default().fg(theme::FG)),
            Span::styled(
                format!("[{}] ", agent.specialization.name()),
                Style::default().fg(theme::DIM),
            ),
            Span::styled(
                format!("skill:{:.2}", agent.skill_level),
                Style::default().fg(theme::ACCENT_CYAN),
            ),
        ]));
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, area);
}

fn render_space_tab(frame: &mut Frame, area: Rect, app: &App) {
    let drone_count = app.state.agent_count_by_class(AgentClass::SpaceDrone);
    let drone_max = app.state.max_for_class(AgentClass::SpaceDrone);
    let comp_count = app.state.agent_count_by_class(AgentClass::Computronium);
    let comp_max = app.state.max_for_class(AgentClass::Computronium);

    let mut lines = vec![
        Line::from(Span::styled(
            " Space Division",
            Style::default().fg(theme::ACCENT_YELLOW).bold(),
        )),
        Line::from(Span::styled(
            format!(
                " Space Drones: {} / {}  |  Computronium: {} / {}",
                drone_count, drone_max, comp_count, comp_max
            ),
            Style::default().fg(theme::ACCENT_CYAN),
        )),
        Line::from(""),
    ];

    let unit_types: &[(&str, &str, AgentClass)] = &[
        (
            "Orbital Drone",
            "3.0x bonus on space projects",
            AgentClass::SpaceDrone,
        ),
        (
            "Deep Space Unit",
            "4.0x bonus on deep space projects",
            AgentClass::SpaceDrone,
        ),
        (
            "Computronium Entity",
            "10.0x bonus on everything",
            AgentClass::Computronium,
        ),
    ];

    for (i, (name, desc, class)) in unit_types.iter().enumerate() {
        let count = app.state.agent_count_by_class(*class);
        let max = app.state.max_for_class(*class);
        let at_cap = count >= max;
        let cost = formulas::agent_hire_cost(*class, count);
        let can_afford = app.state.cash >= cost;
        let selected = i == app.ui.selected_item;

        let marker = if selected { "> " } else { "  " };

        if at_cap {
            lines.push(Line::from(vec![
                Span::styled(marker, Style::default().fg(theme::DIM)),
                Span::styled(format!("{} [CAP]", name), Style::default().fg(theme::DIM)),
            ]));
        } else {
            let name_style = if selected {
                Style::default().fg(theme::ACCENT_GREEN).bold()
            } else {
                Style::default().fg(theme::FG)
            };
            let cost_color = if can_afford {
                theme::ACCENT_GREEN
            } else {
                theme::ACCENT_RED
            };
            lines.push(Line::from(vec![
                Span::styled(marker, Style::default().fg(theme::ACCENT_GREEN)),
                Span::styled(format!("{:<24}", name), name_style),
                Span::styled(formulas::format_cash(cost), Style::default().fg(cost_color)),
            ]));
        }
        if selected {
            lines.push(Line::from(Span::styled(
                format!("    {}", desc),
                Style::default().fg(theme::DIM),
            )));
        }
    }

    // Show mega-project progress if applicable
    let mp = &app.state.mega_projects;
    if mp.dyson_segments_completed > 0 || mp.planets_converted > 0 {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            " Mega-Projects:",
            Style::default().fg(theme::ACCENT_YELLOW).bold(),
        )));

        // Dyson sphere progress
        let dyson_check = if mp.dyson_sphere_complete {
            "[x]"
        } else {
            "[ ]"
        };
        lines.push(Line::from(vec![
            Span::styled(
                format!("   {} Dyson Sphere  ", dyson_check),
                Style::default().fg(if mp.dyson_sphere_complete {
                    theme::ACCENT_GREEN
                } else {
                    theme::FG
                }),
            ),
            Span::styled(
                format!("{}/10 segments", mp.dyson_segments_completed),
                Style::default().fg(theme::DIM),
            ),
        ]));

        // Planetary conversion checklist
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            " Planetary Conversion:",
            Style::default().fg(theme::ACCENT_YELLOW).bold(),
        )));

        let planets = [
            "Mercury",
            "The Moon",
            "Mars",
            "Venus",
            "Ceres & Asteroid Belt",
            "Saturn's Rings",
            "Neptune",
            "Jupiter",
        ];
        for (i, planet) in planets.iter().enumerate() {
            let done = mp.planets_converted > i as u32;
            let (check, color) = if done {
                ("[x]", theme::ACCENT_GREEN)
            } else {
                ("[ ]", theme::DIM)
            };
            lines.push(Line::from(Span::styled(
                format!("   {} {}", check, planet),
                Style::default().fg(color),
            )));
        }
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, area);
}

fn format_compute(val: f64) -> String {
    if val >= 1_000_000_000.0 {
        format!("{:.1}B", val / 1_000_000_000.0)
    } else if val >= 1_000_000.0 {
        format!("{:.1}M", val / 1_000_000.0)
    } else if val >= 1_000.0 {
        format!("{:.0}K", val / 1_000.0)
    } else {
        format!("{:.0}", val)
    }
}
