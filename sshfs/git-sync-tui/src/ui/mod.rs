mod detail;
mod list;
mod modal;
mod mount_bar;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::model::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    app.clear_clicks();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(frame.area());

    mount_bar::draw(frame, app, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    list::draw(frame, app, main_chunks[0]);
    detail::draw(frame, app, main_chunks[1]);

    draw_footer(frame, chunks[2]);

    match app.modal {
        crate::model::Modal::MountPicker => modal::draw_mount_picker(frame, app),
        crate::model::Modal::SyncConfirm => modal::draw_sync_confirm(frame, app),
        crate::model::Modal::Result => modal::draw_result(frame, app),
        crate::model::Modal::None => {}
    }
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    use ratatui::style::{Color, Style};
    use ratatui::widgets::Paragraph;

    let text = "Click repo to select · Enter: sync · r: refresh · q: quit · Mount: click buttons only";
    let p = Paragraph::new(text).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(p, area);
}
