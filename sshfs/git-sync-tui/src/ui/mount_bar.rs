use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::model::{App, ClickAction, ConnectState};

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Mount ")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(inner);

    draw_mount_row(frame, app, rows[0]);
    draw_projects_row(frame, app, rows[1]);
}

fn draw_mount_row(frame: &mut Frame, app: &mut App, inner: Rect) {
    let mounted = app.mounted;
    let label = &app.active_mount.label;
    let mount_path = app.active_mount.mount_point.display().to_string();

    let status_text = if app.connect_state == ConnectState::Running {
        format!("… Connecting  {mount_path} ({label})")
    } else if mounted {
        format!("● Mounted  {mount_path}  ({label})")
    } else {
        format!("○ Not mounted  {mount_path}  ({label})")
    };

    let status_color = if app.connect_state == ConnectState::Running {
        Color::Yellow
    } else if mounted {
        Color::Green
    } else {
        Color::Red
    };

    let btn_connect = " Connect ";
    let btn_change = " Change mount ";
    let buttons_width = (btn_connect.len() + btn_change.len() + 2) as u16;
    let status_width = inner.width.saturating_sub(buttons_width);

    let status_area = Rect {
        x: inner.x,
        y: inner.y,
        width: status_width,
        height: 1,
    };
    frame.render_widget(
        Paragraph::new(status_text).style(Style::default().fg(status_color)),
        status_area,
    );

    let mut x = inner.x + status_width;

    if !mounted && app.connect_state != ConnectState::Running {
        let connect_rect = Rect {
            x,
            y: inner.y,
            width: btn_connect.len() as u16,
            height: 1,
        };
        frame.render_widget(
            Paragraph::new(btn_connect).style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            connect_rect,
        );
        app.register_click(connect_rect, ClickAction::Connect);
        x += btn_connect.len() as u16 + 1;
    }

    let change_rect = Rect {
        x,
        y: inner.y,
        width: btn_change.len() as u16,
        height: 1,
    };
    frame.render_widget(
        Paragraph::new(btn_change).style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        change_rect,
    );
    if app.connect_state != ConnectState::Running {
        app.register_click(change_rect, ClickAction::ChangeMount);
    }
}

fn draw_projects_row(frame: &mut Frame, app: &mut App, inner: Rect) {
    let local = app
        .projects_root()
        .map(|r| r.local.display().to_string())
        .unwrap_or_else(|| "(not configured)".to_string());
    let remote = app.projects_remote_root().display().to_string();

    let btn_pick = " Pick remote dir ";
    let buttons_width = btn_pick.len() as u16;
    let text_width = inner.width.saturating_sub(buttons_width);

    let line = format!("Projects  local: {local}  →  remote: {remote}");
    let text_area = Rect {
        x: inner.x,
        y: inner.y,
        width: text_width,
        height: 1,
    };
    frame.render_widget(
        Paragraph::new(line).style(Style::default().fg(Color::Gray)),
        text_area,
    );

    let pick_rect = Rect {
        x: inner.x + text_width,
        y: inner.y,
        width: btn_pick.len() as u16,
        height: 1,
    };
    let pick_style = if app.mounted {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    frame.render_widget(Paragraph::new(btn_pick).style(pick_style), pick_rect);
    if app.mounted && app.connect_state != ConnectState::Running {
        app.register_click(pick_rect, ClickAction::ChangeRemoteRoot);
    }
}
