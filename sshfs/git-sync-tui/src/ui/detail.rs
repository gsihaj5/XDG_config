use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::git::{format_commit_time, short_sha, side_label};
use crate::model::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Detail ")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let Some(repo) = app.selected_repo() else {
        let p = Paragraph::new("No repositories found.").style(Style::default().fg(Color::DarkGray));
        frame.render_widget(p, inner);
        return;
    };

    let mut lines = vec![
        format!("Path: {}", repo.relative_path),
        format!("Status: {}", repo.status.label()),
        String::new(),
    ];

    if let Some(ref local) = repo.local {
        lines.push("── Local ──".to_string());
        lines.push(format!("  Path:   {}", local.path.display()));
        lines.push(format!("  Branch: {}", local.branch));
        lines.push(format!("  SHA:    {}", short_sha(&local.head_sha)));
        lines.push(format!(
            "  Date:   {}",
            format_commit_time(local.commit_time)
        ));
        lines.push(format!(
            "  Dirty:  {}",
            if local.dirty { "yes" } else { "no" }
        ));
        if let Some(ref up) = local.upstream {
            lines.push(format!("  Upstream: {up} (+{}/-{})", local.ahead, local.behind));
        }
        lines.push(String::new());
    } else {
        lines.push("── Local: not found ──".to_string());
        lines.push(String::new());
    }

    if let Some(ref remote) = repo.remote {
        lines.push("── Remote (laptop) ──".to_string());
        lines.push(format!("  Path:   {}", remote.path.display()));
        lines.push(format!("  Branch: {}", remote.branch));
        lines.push(format!("  SHA:    {}", short_sha(&remote.head_sha)));
        lines.push(format!(
            "  Date:   {}",
            format_commit_time(remote.commit_time)
        ));
        lines.push(format!(
            "  Dirty:  {}",
            if remote.dirty { "yes" } else { "no" }
        ));
        if let Some(ref up) = remote.upstream {
            lines.push(format!(
                "  Upstream: {up} (+{}/-{})",
                remote.ahead, remote.behind
            ));
        }
        lines.push(String::new());
    } else if !app.mounted {
        lines.push("── Remote: unavailable (not mounted) ──".to_string());
        lines.push(String::new());
    } else {
        lines.push("── Remote: not found ──".to_string());
        lines.push(String::new());
    }

    if let (Some(newest), Some(oldest)) = (repo.newest, repo.oldest) {
        lines.push(format!(
            "Newest: {} | Oldest: {}",
            side_label(newest),
            side_label(oldest)
        ));
        if repo.status.can_sync() {
            lines.push(String::new());
            lines.push("Press Enter to sync (cursaves + git push/pull)".to_string());
        }
    }

    if app.config.cursor.enabled {
        lines.push(String::new());
        lines.push("── Cursor chats ──".to_string());
        if let Some(ref cs) = repo.cursor_status {
            if !cs.available {
                lines.push("  cursaves: not available".to_string());
            } else {
                let running = if cs.cursor_running || cs.db_busy {
                    "yes (import blocked)"
                } else {
                    "no"
                };
                lines.push(format!("  Cursor running: {running}"));
                if let Some(n) = cs.local_count {
                    lines.push(format!("  Local conversations: {n}"));
                }
                if let Some(n) = cs.snapshot_only {
                    if n > 0 {
                        lines.push(format!("  Not imported yet: {n}"));
                    }
                }
                if let Some(n) = cs.local_only {
                    if n > 0 {
                        lines.push(format!("  Not exported yet: {n}"));
                    }
                }
                if let Some(ref err) = cs.parse_error {
                    lines.push(format!("  status error: {err}"));
                }
            }
        } else if repo.local.is_none() {
            lines.push("  (no local path)".to_string());
        }
    }

    if !app.status_message.is_empty() {
        lines.push(String::new());
        lines.push(format!("ℹ {}", app.status_message));
    }

    let text = lines.join("\n");
    let p = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .style(Style::default());
    frame.render_widget(p, inner);
}
