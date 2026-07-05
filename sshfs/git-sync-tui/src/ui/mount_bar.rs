use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::model::{App, ClickAction, ConnectState};

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
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

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Mount ")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Layout: status text left, buttons right
    let btn_connect = " Connect ";
    let btn_change = " Change mount ";

    let buttons_width = (btn_connect.len() + btn_change.len() + 2) as u16;
    let status_width = inner.width.saturating_sub(buttons_width);

    let status_area = Rect {
        x: inner.x,
        y: inner.y,
        width: status_width,
        height: inner.height,
    };

    let p = Paragraph::new(status_text).style(Style::default().fg(status_color));
    frame.render_widget(p, status_area);

    let mut x = inner.x + status_width;

    if !mounted && app.connect_state != ConnectState::Running {
        let connect_rect = Rect {
            x,
            y: inner.y,
            width: btn_connect.len() as u16,
            height: 1,
        };
        let connect_style = Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
        frame.render_widget(
            Paragraph::new(btn_connect).style(connect_style),
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
    let change_style = Style::default()
        .fg(Color::Black)
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD);
    frame.render_widget(
        Paragraph::new(btn_change).style(change_style),
        change_rect,
    );

    if app.connect_state != ConnectState::Running {
        app.register_click(change_rect, ClickAction::ChangeMount);
    }
}
