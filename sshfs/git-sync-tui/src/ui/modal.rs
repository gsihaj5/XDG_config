use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::git::{format_commit_time, short_sha, side_label};
use crate::model::{App, ClickAction};

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}

pub fn draw_mount_picker(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(50, 12, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Select mount point ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = Vec::new();
    for alt in &app.config.alternatives {
        let marker = if alt.mount_point == app.active_mount.mount_point {
            ">"
        } else {
            " "
        };
        lines.push(format!(
            "{marker} {:<16} {}",
            alt.label,
            alt.mount_point.display()
        ));
    }
    lines.push(String::new());
    lines.push("Click a row to select · [ Cancel ]".to_string());

    let text = lines.join("\n");
    let p = Paragraph::new(text).style(Style::default());
    frame.render_widget(p, inner);

    // Clickable rows for alternatives
    let alt_count = app.config.alternatives.len();
    for i in 0..alt_count {
        let row_rect = Rect {
            x: inner.x,
            y: inner.y + i as u16,
            width: inner.width,
            height: 1,
        };
        app.register_click(row_rect, ClickAction::MountPickerRow(i));
    }

    // Cancel button on last line
    let cancel_y = inner.y + alt_count as u16 + 1;
    let cancel_rect = Rect {
        x: inner.x + inner.width.saturating_sub(12),
        y: cancel_y,
        width: 10,
        height: 1,
    };
    frame.render_widget(
        Paragraph::new(" Cancel ").style(Style::default().fg(Color::Black).bg(Color::Red)),
        cancel_rect,
    );
    app.register_click(cancel_rect, ClickAction::MountPickerCancel);
}

pub fn draw_remote_root_picker(frame: &mut Frame, app: &mut App) {
    let choice_count = app.remote_root_choices.len();
    let height = (choice_count + 5).min(20) as u16;
    let area = centered_rect(58, height, frame.area());
    frame.render_widget(Clear, area);

    let mount = app.active_mount.mount_point.display().to_string();
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Remote projects folder under {mount} "))
        .title_alignment(Alignment::Center)
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![
        "Choose which directory on the mount corresponds to local ~/projects:".to_string(),
        String::new(),
    ];

    for (i, (name, path)) in app.remote_root_choices.iter().enumerate() {
        let marker = if name == &app.remote_projects_suffix {
            ">"
        } else {
            " "
        };
        lines.push(format!("{marker} {:<20} {}", name, path.display()));
        let _ = i;
    }

    lines.push(String::new());
    lines.push("Click a row to select · [ Cancel ]".to_string());

    let text = lines.join("\n");
    frame.render_widget(Paragraph::new(text).style(Style::default()), inner);

    for i in 0..choice_count {
        let row_rect = Rect {
            x: inner.x,
            y: inner.y + 2 + i as u16,
            width: inner.width,
            height: 1,
        };
        app.register_click(row_rect, ClickAction::RemoteRootPickerRow(i));
    }

    let cancel_y = inner.y + 2 + choice_count as u16 + 1;
    let cancel_rect = Rect {
        x: inner.x + inner.width.saturating_sub(12),
        y: cancel_y,
        width: 10,
        height: 1,
    };
    frame.render_widget(
        Paragraph::new(" Cancel ").style(Style::default().fg(Color::Black).bg(Color::Red)),
        cancel_rect,
    );
    app.register_click(cancel_rect, ClickAction::RemoteRootPickerCancel);
}

pub fn draw_sync_confirm(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(60, 22, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Confirm sync ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec!["This will:".to_string()];

    if let Some(repo) = app.selected_repo() {
        lines.push(format!("  Repo: {}", repo.relative_path));
        lines.push(String::new());

        if let (Some(newest), Some(oldest)) = (repo.newest, repo.oldest) {
            let newest_side = match newest {
                crate::git::Side::Local => repo.local.as_ref(),
                crate::git::Side::Remote => repo.remote.as_ref(),
            };
            let oldest_side = match oldest {
                crate::git::Side::Local => repo.local.as_ref(),
                crate::git::Side::Remote => repo.remote.as_ref(),
            };

            if let Some(n) = newest_side {
                lines.push(format!(
                    "  1. git push in {} ({})",
                    side_label(newest),
                    n.path.display()
                ));
                lines.push(format!(
                    "     branch {} @ {}",
                    n.branch,
                    short_sha(&n.head_sha)
                ));
            }
            if let Some(o) = oldest_side {
                lines.push(format!(
                    "  2. git pull in {} ({})",
                    side_label(oldest),
                    o.path.display()
                ));
                lines.push(format!(
                    "     branch {} @ {} ({})",
                    o.branch,
                    short_sha(&o.head_sha),
                    format_commit_time(o.commit_time)
                ));
            }
        }
    }

    lines.push(String::new());
    lines.push("Cursor chats:".to_string());
    if app.config.cursor.enabled {
        let cursor_open = app.cursor_cache.cursor_running() || app.cursor_cache.db_busy();
        if cursor_open && app.config.cursor.require_quit_for_import {
            lines.push("  ● Cursor running — import skipped (export only)".to_string());
        } else {
            lines.push("  ✓ Cursor not running — full chat sync".to_string());
        }
    } else {
        lines.push("  (disabled in config)".to_string());
    }

    lines.push(String::new());
    lines.push("  [ Yes ]    [ No ]".to_string());
    lines.push(String::new());
    lines.push("Keyboard: y = yes, n/Esc = no".to_string());

    let text = lines.join("\n");
    let p = Paragraph::new(text).wrap(Wrap { trim: true });
    frame.render_widget(p, inner);

    let yes_rect = Rect {
        x: inner.x + 2,
        y: inner.y + inner.height.saturating_sub(3),
        width: 7,
        height: 1,
    };
    let no_rect = Rect {
        x: inner.x + 13,
        y: inner.y + inner.height.saturating_sub(3),
        width: 6,
        height: 1,
    };

    frame.render_widget(
        Paragraph::new(" Yes ").style(Style::default().fg(Color::Black).bg(Color::Green)),
        yes_rect,
    );
    frame.render_widget(
        Paragraph::new(" No ").style(Style::default().fg(Color::Black).bg(Color::Red)),
        no_rect,
    );

    app.register_click(yes_rect, ClickAction::SyncConfirmYes);
    app.register_click(no_rect, ClickAction::SyncConfirmNo);
}

pub fn draw_result(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(65, 20, frame.area());
    frame.render_widget(Clear, area);

    let title = if app.result_success {
        " Sync complete "
    } else {
        " Sync / connect result "
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_alignment(Alignment::Center)
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut text = app.result_text.clone();
    text.push_str("\n\n[ OK ]  (click or press Enter/Esc)");
    let color = if app.result_success {
        Color::Green
    } else {
        Color::Red
    };

    let p = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(color));
    frame.render_widget(p, inner);

    let ok_rect = Rect {
        x: inner.x + 2,
        y: inner.y + inner.height.saturating_sub(2),
        width: 6,
        height: 1,
    };
    frame.render_widget(
        Paragraph::new(" OK ").style(Style::default().fg(Color::Black).bg(Color::Blue)),
        ok_rect,
    );
    app.register_click(ok_rect, ClickAction::ResultDismiss);
}
