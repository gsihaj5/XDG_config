use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

use crate::model::{App, ClickAction};

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Repositories ")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let visible_height = inner.height as usize;
    let items: Vec<ListItem> = app
        .entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let icon = app.list_icon(i);
            let line = format!(" {} {}", icon, entry.relative_path);
            let style = if i == app.selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(line).style(style)
        })
        .collect();

    let list = List::new(items);
    let mut state = ListState::default();
    state.select(Some(app.selected.saturating_sub(app.list_scroll)));

    let list_area = inner;
    frame.render_stateful_widget(list, list_area, &mut state);

    // Register click zones for visible rows
    for (vis, repo_idx) in (app.list_scroll..app.entries.len())
        .take(visible_height)
        .enumerate()
    {
        let row_rect = Rect {
            x: inner.x,
            y: inner.y + vis as u16,
            width: inner.width,
            height: 1,
        };
        app.register_click(row_rect, ClickAction::RepoRow(repo_idx));
    }
}
