use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::App;
use crate::game::state::GamePhase;
use crate::game::tech_tree;
use crate::ui::{centered_rect, theme};

pub fn render(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 70, frame.area());
    frame.render_widget(Clear, area);

    let accent = theme::phase_accent(app.state.phase);
    let block = Block::default()
        .title(Span::styled(
            " Tech Tree ",
            Style::default().fg(accent).bold(),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent))
        .style(Style::default().bg(theme::BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let completed = app.state.completed_project_count;
    let milestones = tech_tree::all_milestones();

    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled(
                " Projects Completed: ",
                Style::default().fg(theme::ACCENT_CYAN).bold(),
            ),
            Span::styled(
                format!("{}", completed),
                Style::default().fg(theme::ACCENT_GREEN),
            ),
        ]),
        Line::from(""),
    ];

    // Group milestones by phase, only show phases we've reached or are near
    let mut current_phase_name = "";
    let horizon = completed + 15; // Show milestones up to 15 ahead

    for milestone in &milestones {
        // Don't show milestones way beyond our reach
        if milestone.projects_needed > horizon && milestone.projects_needed > completed {
            // But still show the next locked phase header
            if milestone.phase_name != current_phase_name {
                let phase_color = phase_name_color(milestone.phase_name);
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!(" {} [LOCKED]", milestone.phase_name.to_uppercase()),
                    Style::default().fg(phase_color),
                )));
                current_phase_name = milestone.phase_name;
            }
            continue;
        }

        // Phase section header
        if milestone.phase_name != current_phase_name {
            lines.push(Line::from(""));
            let phase_color = phase_name_color(milestone.phase_name);
            lines.push(Line::from(Span::styled(
                format!(" {}", milestone.phase_name.to_uppercase()),
                Style::default().fg(phase_color).bold(),
            )));
            current_phase_name = milestone.phase_name;
        }

        let unlocked = completed >= milestone.projects_needed;
        let (marker, color) = if unlocked {
            ("[x]", theme::ACCENT_GREEN)
        } else {
            ("[ ]", theme::DIM)
        };

        let req = if milestone.projects_needed == 0 {
            "Start".to_string()
        } else {
            format!("{} proj", milestone.projects_needed)
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", marker), Style::default().fg(color)),
            Span::styled(
                format!("{:<8} ", req),
                Style::default().fg(if unlocked {
                    theme::ACCENT_CYAN
                } else {
                    theme::DIM
                }),
            ),
            Span::styled(
                milestone.label,
                Style::default().fg(if unlocked { theme::FG } else { theme::DIM }),
            ),
        ]));
    }

    // Mega-project status
    let mp = &app.state.mega_projects;
    if app.state.phase >= GamePhase::Kardashev || mp.dyson_segments_completed > 0 {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            " MEGA-PROJECTS",
            Style::default().fg(Color::Rgb(220, 220, 255)).bold(),
        )));
        lines.push(Line::from(vec![
            Span::styled("  Dyson Segments:    ", Style::default().fg(theme::DIM)),
            Span::styled(
                format!("{}/10", mp.dyson_segments_completed),
                Style::default().fg(if mp.dyson_segments_completed >= 10 {
                    theme::ACCENT_GREEN
                } else {
                    theme::FG
                }),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Dyson Sphere:      ", Style::default().fg(theme::DIM)),
            Span::styled(
                if mp.dyson_sphere_complete {
                    "COMPLETE"
                } else {
                    "In Progress"
                },
                Style::default().fg(if mp.dyson_sphere_complete {
                    theme::ACCENT_GREEN
                } else {
                    theme::ACCENT_YELLOW
                }),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Planets Converted: ", Style::default().fg(theme::DIM)),
            Span::styled(
                format!("{}/8", mp.planets_converted),
                Style::default().fg(if mp.planets_converted >= 8 {
                    theme::ACCENT_GREEN
                } else {
                    theme::FG
                }),
            ),
        ]));
    }

    // Scroll: only render lines that fit
    let max_lines = inner.height as usize;
    if lines.len() > max_lines {
        lines.truncate(max_lines);
    }

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, inner);
}

fn phase_name_color(name: &str) -> Color {
    match name {
        "Consultancy" => theme::ACCENT_GREEN,
        "Industry" => theme::ACCENT_CYAN,
        "Post-Human" => theme::ACCENT_PURPLE,
        "Space Age" => theme::ACCENT_YELLOW,
        "Kardashev" => Color::Rgb(220, 220, 255),
        _ => theme::DIM,
    }
}
