use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem};

use crate::app::App;
use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(Span::styled(" Commit Log ", Style::default().fg(theme::ACCENT_GREEN).bold()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::BG));

    if app.state.commit_log.is_empty() {
        let empty = ratatui::widgets::Paragraph::new(
            Span::styled("  No commits yet...", Style::default().fg(theme::DIM))
        ).block(block);
        frame.render_widget(empty, area);
        return;
    }

    let max_items = (area.height.saturating_sub(2)) as usize;
    let items: Vec<ListItem> = app.state.commit_log.iter()
        .take(max_items)
        .map(|commit| {
            let prefix_color = match commit.message.split('(').next().unwrap_or("") {
                "feat" => theme::ACCENT_GREEN,
                "fix" => theme::ACCENT_YELLOW,
                "refactor" => theme::ACCENT_CYAN,
                "perf" => theme::ACCENT_PURPLE,
                _ => theme::DIM,
            };

            let msg_width = area.width.saturating_sub(6) as usize; // borders + "* " prefix
            let truncated: String = commit.message.chars().take(msg_width).collect();

            let line = Line::from(vec![
                Span::styled("* ", Style::default().fg(prefix_color)),
                Span::styled(truncated, Style::default().fg(theme::FG)),
            ]);

            let detail = Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(format!("[{}] ", commit.agent_name), Style::default().fg(theme::ACCENT_CYAN)),
                Span::styled(format!("+{}", commit.additions), Style::default().fg(theme::ACCENT_GREEN)),
                Span::styled("/", Style::default().fg(theme::DIM)),
                Span::styled(format!("-{}", commit.deletions), Style::default().fg(theme::ACCENT_RED)),
            ]);

            ListItem::new(vec![line, detail])
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}
