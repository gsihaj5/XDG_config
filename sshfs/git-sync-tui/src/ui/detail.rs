use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::git::{format_commit_time, short_sha, side_label, RepoPairState};
use crate::model::App;
use crate::scanner::ScannedRepo;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Detail ")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let Some(entry) = app.selected_entry() else {
        let p = Paragraph::new("No repositories found.").style(Style::default().fg(Color::DarkGray));
        frame.render_widget(p, inner);
        return;
    };

    let mut lines = vec![format!("Path: {}", entry.relative_path)];

    if app.is_detail_loading() {
        append_loading_detail(&mut lines, entry, app.mounted, app.loading_spinner_frame());
    } else if let Some(repo) = app.selected_repo() {
        append_loaded_detail(&mut lines, app, entry, repo);
    } else {
        append_loading_detail(&mut lines, entry, app.mounted, app.loading_spinner_frame());
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

fn append_loading_detail(
    lines: &mut Vec<String>,
    entry: &ScannedRepo,
    mounted: bool,
    spinner: &str,
) {
    lines.push(format!("Status: {spinner} Loading..."));
    lines.push(String::new());
    if let Some(ref p) = entry.local {
        lines.push("── Local ──".to_string());
        lines.push(format!("  Path:   {}", p.display()));
        lines.push(String::new());
    }
    append_remote_section(lines, entry, None, mounted);
    lines.push("Inspecting git state and cursor chats...".to_string());
}

fn append_loaded_detail(
    lines: &mut Vec<String>,
    app: &App,
    entry: &ScannedRepo,
    repo: &RepoPairState,
) {
    lines.push(format!("Status: {}", repo.status.label()));
    lines.push(String::new());

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

    append_remote_section(lines, entry, Some(repo), app.mounted);

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
}

fn append_remote_section(
    lines: &mut Vec<String>,
    entry: &ScannedRepo,
    repo: Option<&RepoPairState>,
    mounted: bool,
) {
    if !mounted {
        lines.push("── Remote: unavailable (not mounted) ──".to_string());
        lines.push(String::new());
        return;
    }

    lines.push("── Remote (laptop) ──".to_string());
    if let Some(ref checked) = entry.remote_checked {
        lines.push(format!("  Searched: {}", checked.display()));
    }

    if let Some(r) = repo {
        if let Some(ref remote) = r.remote {
            lines.push("  Found:    yes".to_string());
            lines.push(format!("  Path:     {}", remote.path.display()));
            lines.push(format!("  Branch:   {}", remote.branch));
            lines.push(format!("  SHA:      {}", short_sha(&remote.head_sha)));
            lines.push(format!(
                "  Date:     {}",
                format_commit_time(remote.commit_time)
            ));
            lines.push(format!(
                "  Dirty:    {}",
                if remote.dirty { "yes" } else { "no" }
            ));
            if let Some(ref up) = remote.upstream {
                lines.push(format!(
                    "  Upstream: {up} (+{}/-{})",
                    remote.ahead, remote.behind
                ));
            }
        } else {
            lines.push("  Found:    no (.git missing at searched path)".to_string());
        }
    } else if entry.remote.is_some() {
        lines.push("  Found:    yes (loading details...)".to_string());
    } else {
        lines.push("  Found:    no (.git missing at searched path)".to_string());
    }
    lines.push(String::new());
}
