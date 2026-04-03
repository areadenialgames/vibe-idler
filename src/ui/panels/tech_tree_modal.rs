use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::App;
use crate::ui::{centered_rect, theme};

struct Milestone {
    projects_needed: u32,
    label: &'static str,
    unlock_ids: &'static [&'static str],
}

const MILESTONES: &[Milestone] = &[
    Milestone { projects_needed: 0, label: "Landing Page, Personal Site, Script", unlock_ids: &["proj_landing", "proj_personal_site", "proj_simple_script"] },
    Milestone { projects_needed: 3, label: "CRUD App, REST API", unlock_ids: &["proj_crud_app", "proj_rest_api"] },
    Milestone { projects_needed: 5, label: "Mobile App", unlock_ids: &["proj_mobile_app"] },
    Milestone { projects_needed: 8, label: "E-commerce", unlock_ids: &["proj_ecommerce"] },
    Milestone { projects_needed: 12, label: "SaaS Product", unlock_ids: &["proj_saas"] },
    Milestone { projects_needed: 15, label: "Data Pipeline, Open Source", unlock_ids: &["proj_data_pipeline", "proj_open_source"] },
    Milestone { projects_needed: 20, label: "ML Model, Enterprise Software", unlock_ids: &["proj_ml_model", "proj_enterprise"] },
    Milestone { projects_needed: 30, label: "Crypto Protocol, Game Dev", unlock_ids: &["proj_crypto", "proj_gamedev"] },
];

pub fn render(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 70, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(Span::styled(" Tech Tree ", Style::default().fg(theme::ACCENT_PURPLE).bold()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACCENT_PURPLE))
        .style(Style::default().bg(theme::BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let completed = app.state.completed_project_count;

    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled(" Projects Completed: ", Style::default().fg(theme::ACCENT_CYAN).bold()),
            Span::styled(format!("{}", completed), Style::default().fg(theme::ACCENT_GREEN)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            " Project Unlocks",
            Style::default().fg(theme::ACCENT_YELLOW).bold(),
        )),
        Line::from(""),
    ];

    for milestone in MILESTONES {
        let unlocked = milestone.unlock_ids.iter()
            .all(|id| app.state.unlocked_upgrades.contains(&id.to_string()));

        let (marker, color) = if unlocked {
            ("[x]", theme::ACCENT_GREEN)
        } else {
            ("[ ]", theme::DIM)
        };

        let req = if milestone.projects_needed == 0 {
            "Start".to_string()
        } else {
            format!("{} projects", milestone.projects_needed)
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", marker), Style::default().fg(color)),
            Span::styled(format!("{:<8} ", req), Style::default().fg(if unlocked { theme::ACCENT_CYAN } else { theme::DIM })),
            Span::styled(milestone.label, Style::default().fg(if unlocked { theme::FG } else { theme::DIM })),
        ]));
    }

    // Hardware unlocks
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Hardware & LLM Unlocks",
        Style::default().fg(theme::ACCENT_YELLOW).bold(),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Purchasing hardware/LLM tiers unlocks the next tier",
        Style::default().fg(theme::DIM),
    )));

    let widget = Paragraph::new(lines);
    frame.render_widget(widget, inner);
}
