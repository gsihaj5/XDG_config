use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};

use crate::config::CursorConfig;

#[derive(Debug, Clone, Default)]
pub struct CursorChatStatus {
    pub available: bool,
    pub cursor_running: bool,
    pub db_busy: bool,
    pub local_count: Option<u32>,
    pub snapshot_count: Option<u32>,
    pub in_both: Option<u32>,
    pub local_only: Option<u32>,
    pub snapshot_only: Option<u32>,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CursorSyncOutcome {
    pub output: String,
    pub import_skipped: bool,
    pub success: bool,
}

pub fn is_cursor_running() -> bool {
    if cursor_running_via_pgrep() {
        return true;
    }
    cursor_running_via_ps_macos()
}

fn cursor_running_via_pgrep() -> bool {
    let output = Command::new("pgrep")
        .args(["-af", "cursor"])
        .output()
        .ok();

    let Some(out) = output else {
        return false;
    };

    if !out.status.success() {
        return false;
    }

    let text = String::from_utf8_lossy(&out.stdout);
    text.lines().any(|line| is_main_cursor_process(line))
}

fn is_main_cursor_process(line: &str) -> bool {
    let lower = line.to_lowercase();
    if !lower.contains("cursor") {
        return false;
    }
    let skip = [
        "pgrep",
        "git-sync-tui",
        "cursaves",
        "cursor-helper",
        "crashpad",
        "chrome_crashpad",
        "zygote",
        "utility",
        "renderer",
        "gpu-process",
        "network",
    ];
    !skip.iter().any(|s| lower.contains(s))
}

fn cursor_running_via_ps_macos() -> bool {
    let output = Command::new("ps")
        .args(["-axo", "args"])
        .output()
        .ok();

    let Some(out) = output else {
        return false;
    };

    if !out.status.success() {
        return false;
    }

    String::from_utf8_lossy(&out.stdout)
        .lines()
        .any(|line| {
            line.contains("Cursor.app/Contents/MacOS/Cursor")
                && !line.contains("Helper")
                && !line.contains("Frameworks")
        })
}

pub fn is_db_busy() -> bool {
    let db = global_db_path();
    if !db.exists() {
        return false;
    }

    if db_busy_via_fuser(&db) {
        return true;
    }

    db_busy_via_sqlite(&db)
}

fn global_db_path() -> PathBuf {
    dirs_cursor_user_dir().join("globalStorage").join("state.vscdb")
}

fn dirs_cursor_user_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
            .join(".config")
            .join("Cursor")
            .join("User")
    } else {
        PathBuf::from(".config/Cursor/User")
    }
}

fn db_busy_via_fuser(db: &Path) -> bool {
    let output = Command::new("fuser")
        .arg(db)
        .output()
        .ok();

    let Some(out) = output else {
        return false;
    };

    out.status.success() && !out.stdout.is_empty()
}

fn db_busy_via_sqlite(db: &Path) -> bool {
    let output = Command::new("sqlite3")
        .arg(db)
        .arg("BEGIN IMMEDIATE; ROLLBACK;")
        .output()
        .ok();

    let Some(out) = output else {
        return false;
    };

    if out.status.success() {
        return false;
    }

    let stderr = String::from_utf8_lossy(&out.stderr).to_lowercase();
    stderr.contains("locked") || stderr.contains("busy")
}

pub fn cursaves_available(config: &CursorConfig) -> bool {
    if !config.enabled {
        return false;
    }
    Command::new(&config.cursaves_bin)
        .arg("--help")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn fetch_status(config: &CursorConfig, project_path: &Path) -> CursorChatStatus {
    let mut status = CursorChatStatus {
        available: cursaves_available(config),
        cursor_running: is_cursor_running(),
        db_busy: is_db_busy(),
        ..Default::default()
    };

    if !status.available {
        return status;
    }

    match run_cursaves(config, &["status", "-p", &path_arg(project_path)]) {
        Ok(output) => {
            parse_status_output(&output, &mut status);
        }
        Err(e) => {
            status.parse_error = Some(e.to_string());
        }
    }

    status
}

fn parse_status_output(output: &str, status: &mut CursorChatStatus) {
    for line in output.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Local conversations:") {
            status.local_count = parse_count(rest);
        } else if let Some(rest) = line.strip_prefix("Snapshot files:") {
            status.snapshot_count = parse_count(rest);
        } else if let Some(rest) = line.strip_prefix("In both:") {
            status.in_both = parse_count(rest);
        } else if let Some(rest) = line.strip_prefix("Local only (unexported):") {
            status.local_only = parse_count(rest);
        } else if let Some(rest) = line.strip_prefix("Snapshot only (not imported):") {
            status.snapshot_only = parse_count(rest);
        }
    }
}

fn parse_count(s: &str) -> Option<u32> {
    s.trim().parse().ok()
}

pub fn sync_chats(config: &CursorConfig, project_path: &Path) -> Result<CursorSyncOutcome> {
    if !config.enabled {
        return Ok(CursorSyncOutcome {
            output: String::new(),
            import_skipped: false,
            success: true,
        });
    }

    if !cursaves_available(config) {
        return Ok(CursorSyncOutcome {
            output: format!(
                "Cursor chat sync skipped: '{}' not found or not working.",
                config.cursaves_bin
            ),
            import_skipped: false,
            success: true,
        });
    }

    let path = path_arg(project_path);
    let cursor_open = is_cursor_running() || (config.require_quit_for_import && is_db_busy());

    let mut log = String::new();
    let mut import_skipped = false;

    if cursor_open && config.require_quit_for_import {
        import_skipped = true;
        log.push_str("=== cursaves push (Cursor running — export only) ===\n");
        if config.push_when_cursor_open {
            let push_out = run_cursaves(config, &["push", "-p", &path, "--all"])?;
            log.push_str(&push_out);
            log.push_str(
                "\n⚠ Cursor is running — chats exported but not imported.\n\
                 Quit Cursor fully, then sync again to import chats from the other machine.\n",
            );
        } else {
            log.push_str("Skipped: Cursor is running and push_when_cursor_open is false.\n");
        }
    } else {
        log.push_str("=== cursaves sync ===\n");
        let sync_out = run_cursaves_in_dir(config, project_path, &["sync"])?;
        log.push_str(&sync_out);
        if sync_out.to_lowercase().contains("import") {
            log.push_str("\nRestart Cursor (quit and reopen) to see imported chats.\n");
        }
    }

    Ok(CursorSyncOutcome {
        output: log,
        import_skipped,
        success: true,
    })
}

fn run_cursaves(config: &CursorConfig, args: &[&str]) -> Result<String> {
    run_cursaves_with_dir(config, None, args)
}

fn run_cursaves_in_dir(config: &CursorConfig, dir: &Path, args: &[&str]) -> Result<String> {
    run_cursaves_with_dir(config, Some(dir), args)
}

fn run_cursaves_with_dir(
    config: &CursorConfig,
    dir: Option<&Path>,
    args: &[&str],
) -> Result<String> {
    let mut cmd = Command::new(&config.cursaves_bin);
    cmd.args(args);
    if let Some(d) = dir {
        cmd.current_dir(d);
    }
    let out = cmd
        .output()
        .with_context(|| format!("run {} {}", config.cursaves_bin, args.join(" ")))?;

    let mut text = String::new();
    if !out.stdout.is_empty() {
        text.push_str(&String::from_utf8_lossy(&out.stdout));
    }
    if !out.stderr.is_empty() {
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        text.push_str(&String::from_utf8_lossy(&out.stderr));
    }

    if out.status.success() {
        Ok(text)
    } else {
        anyhow::bail!("{}", text.trim())
    }
}

fn path_arg(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_status_lines() {
        let mut status = CursorChatStatus::default();
        let sample = "\
Project: /home/gerry/projects/foo
Local conversations: 3
 Snapshot files: 2
 In both: 1
 Local only (unexported): 2
 Snapshot only (not imported): 1
";
        parse_status_output(sample, &mut status);
        assert_eq!(status.local_count, Some(3));
        assert_eq!(status.snapshot_count, Some(2));
        assert_eq!(status.in_both, Some(1));
        assert_eq!(status.local_only, Some(2));
        assert_eq!(status.snapshot_only, Some(1));
    }

    #[test]
    fn main_cursor_process_filter() {
        assert!(is_main_cursor_process(
            "1234 /usr/share/cursor/cursor --type=browser"
        ));
        assert!(!is_main_cursor_process(
            "1234 /usr/share/cursor/cursor --type=renderer"
        ));
        assert!(!is_main_cursor_process("1234 cursaves push"));
    }
}
