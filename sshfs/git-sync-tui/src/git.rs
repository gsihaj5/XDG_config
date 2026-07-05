use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, TimeZone};

use crate::config::CursorConfig;
use crate::cursor::{self, CursorChatStatus};
use crate::scanner::ScannedRepo;

#[derive(Debug, Clone)]
pub struct RepoSide {
    pub path: PathBuf,
    pub branch: String,
    pub head_sha: String,
    pub commit_time: i64,
    pub dirty: bool,
    pub upstream: Option<String>,
    pub ahead: u32,
    pub behind: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncStatus {
    InSync,
    Ready,
    Dirty,
    Diverged,
    BranchMismatch,
    NoUpstream,
    MissingPair,
    RemoteUnavailable,
}

impl SyncStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::InSync => "In sync",
            Self::Ready => "Ready to sync",
            Self::Dirty => "Uncommitted changes",
            Self::Diverged => "Diverged — manual fix needed",
            Self::BranchMismatch => "Branch mismatch",
            Self::NoUpstream => "No upstream configured",
            Self::MissingPair => "Missing on one machine",
            Self::RemoteUnavailable => "Remote unavailable",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::InSync => "✓",
            Self::Ready => "●",
            Self::Dirty | Self::Diverged | Self::BranchMismatch | Self::NoUpstream => "⚠",
            Self::MissingPair => "?",
            Self::RemoteUnavailable => "○",
        }
    }

    pub fn can_sync(self) -> bool {
        matches!(self, Self::Ready)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Local,
    Remote,
}

#[derive(Debug, Clone)]
pub struct RepoPairState {
    pub relative_path: String,
    pub local: Option<RepoSide>,
    pub remote: Option<RepoSide>,
    pub status: SyncStatus,
    pub newest: Option<Side>,
    pub oldest: Option<Side>,
    pub cursor_status: Option<CursorChatStatus>,
}

pub fn inspect_side(path: &Path) -> Result<RepoSide> {
    let branch = git_output(path, &["rev-parse", "--abbrev-ref", "HEAD"])?;
    let head_sha = git_output(path, &["rev-parse", "HEAD"])?;
    let commit_time: i64 = git_output(path, &["log", "-1", "--format=%ct"])?
        .trim()
        .parse()
        .unwrap_or(0);
    let dirty = !git_output(path, &["status", "--porcelain"])?.trim().is_empty();

    let upstream = git_output(path, &["rev-parse", "--abbrev-ref", "@{upstream}"])
        .ok()
        .filter(|s| !s.is_empty() && s != "HEAD");

    let (ahead, behind) = if let Some(ref up) = upstream {
        let counts = git_output(
            path,
            &["rev-list", "--left-right", "--count", &format!("HEAD...{up}")],
        )
        .unwrap_or_else(|_| "0\t0".to_string());
        parse_ahead_behind(&counts)
    } else {
        (0, 0)
    };

    Ok(RepoSide {
        path: path.to_path_buf(),
        branch,
        head_sha,
        commit_time,
        dirty,
        upstream,
        ahead,
        behind,
    })
}

fn parse_ahead_behind(s: &str) -> (u32, u32) {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() >= 2 {
        (
            parts[0].parse().unwrap_or(0),
            parts[1].parse().unwrap_or(0),
        )
    } else {
        (0, 0)
    }
}

pub fn classify_pair(
    scanned: &ScannedRepo,
    remote_mounted: bool,
    cursor_config: &CursorConfig,
) -> Result<RepoPairState> {
    let local = scanned
        .local
        .as_ref()
        .map(|p| inspect_side(p))
        .transpose()?;
    let remote = if remote_mounted {
        scanned
            .remote
            .as_ref()
            .map(|p| inspect_side(p))
            .transpose()?
    } else {
        None
    };

    let (status, newest, oldest) = compute_status(local.as_ref(), remote.as_ref(), remote_mounted);

    let cursor_status = local.as_ref().map(|l| cursor::fetch_status(cursor_config, &l.path));

    Ok(RepoPairState {
        relative_path: scanned.relative_path.clone(),
        local,
        remote,
        status,
        newest,
        oldest,
        cursor_status,
    })
}

fn compute_status(
    local: Option<&RepoSide>,
    remote: Option<&RepoSide>,
    remote_mounted: bool,
) -> (SyncStatus, Option<Side>, Option<Side>) {
    if !remote_mounted {
        if local.is_some() {
            return (SyncStatus::RemoteUnavailable, None, None);
        }
        return (SyncStatus::MissingPair, None, None);
    }

    match (local, remote) {
        (None, None) => (SyncStatus::MissingPair, None, None),
        (Some(_), None) | (None, Some(_)) => (SyncStatus::MissingPair, None, None),
        (Some(l), Some(r)) => compare_sides(l, r),
    }
}

fn compare_sides(
    local: &RepoSide,
    remote: &RepoSide,
) -> (SyncStatus, Option<Side>, Option<Side>) {
    if local.dirty || remote.dirty {
        return (SyncStatus::Dirty, None, None);
    }

    if local.branch != remote.branch {
        return (SyncStatus::BranchMismatch, None, None);
    }

    if local.head_sha == remote.head_sha {
        return (SyncStatus::InSync, None, None);
    }

    let local_is_ancestor =
        is_ancestor(&local.path, &local.head_sha, &remote.head_sha).unwrap_or(false);
    let remote_is_ancestor =
        is_ancestor(&remote.path, &remote.head_sha, &local.head_sha).unwrap_or(false);

    let (newest, oldest) = if local_is_ancestor && !remote_is_ancestor {
        (Some(Side::Remote), Some(Side::Local))
    } else if remote_is_ancestor && !local_is_ancestor {
        (Some(Side::Local), Some(Side::Remote))
    } else {
        return (SyncStatus::Diverged, None, None);
    };

    let newest_side = match newest {
        Some(Side::Local) => local,
        Some(Side::Remote) => remote,
        None => unreachable!(),
    };
    let oldest_side = match oldest {
        Some(Side::Local) => local,
        Some(Side::Remote) => remote,
        None => unreachable!(),
    };

    if newest_side.upstream.is_none() || oldest_side.upstream.is_none() {
        return (SyncStatus::NoUpstream, newest, oldest);
    }

    // Newest will push to origin; oldest can fast-forward pull because
    // oldest HEAD is an ancestor of newest HEAD (linear history).
    (SyncStatus::Ready, newest, oldest)
}

fn is_ancestor(repo: &Path, ancestor: &str, descendant: &str) -> Result<bool> {
    if ancestor.is_empty() || descendant.is_empty() {
        return Ok(false);
    }
    Ok(Command::new("git")
        .current_dir(repo)
        .args(["merge-base", "--is-ancestor", ancestor, descendant])
        .status()
        .context("git merge-base")?
        .success())
}

pub struct SyncResult {
    pub success: bool,
    pub output: String,
    pub chat_import_skipped: bool,
}

pub fn execute_sync(pair: &RepoPairState, cursor_config: &CursorConfig) -> SyncResult {
    let Some(newest) = pair.newest else {
        return SyncResult {
            success: false,
            output: "Cannot sync: newest side unknown.".to_string(),
            chat_import_skipped: false,
        };
    };
    let Some(oldest) = pair.oldest else {
        return SyncResult {
            success: false,
            output: "Cannot sync: oldest side unknown.".to_string(),
            chat_import_skipped: false,
        };
    };

    let newest_path = side_path(pair, newest);
    let oldest_path = side_path(pair, oldest);

    let Some(newest_path) = newest_path else {
        return SyncResult {
            success: false,
            output: "Newest path not found.".to_string(),
            chat_import_skipped: false,
        };
    };
    let Some(oldest_path) = oldest_path else {
        return SyncResult {
            success: false,
            output: "Oldest path not found.".to_string(),
            chat_import_skipped: false,
        };
    };

    let mut log = String::new();
    let mut chat_import_skipped = false;

    if let Some(ref local) = pair.local {
        match cursor::sync_chats(cursor_config, &local.path) {
            Ok(outcome) => {
                if !outcome.output.is_empty() {
                    log.push_str(&outcome.output);
                    if !log.ends_with('\n') {
                        log.push('\n');
                    }
                    log.push('\n');
                }
                chat_import_skipped = outcome.import_skipped;
                if !outcome.success {
                    return SyncResult {
                        success: false,
                        output: log,
                        chat_import_skipped,
                    };
                }
            }
            Err(e) => {
                log.push_str(&format!("=== cursaves (failed) ===\n{e}\n\n"));
                return SyncResult {
                    success: false,
                    output: log,
                    chat_import_skipped,
                };
            }
        }
    }

    log.push_str(&format!("=== git push in {} ===\n", newest_path.display()));
    match git_run_capture(&newest_path, &["push"]) {
        Ok(out) => log.push_str(&out),
        Err(e) => {
            log.push_str(&format!("{e}\n"));
            return SyncResult {
                success: false,
                output: log,
                chat_import_skipped,
            };
        }
    }

    log.push_str(&format!("\n=== git fetch in {} ===\n", oldest_path.display()));
    match git_run_capture(&oldest_path, &["fetch"]) {
        Ok(out) => log.push_str(&out),
        Err(e) => {
            log.push_str(&format!("{e}\n"));
            return SyncResult {
                success: false,
                output: log,
                chat_import_skipped,
            };
        }
    }

    log.push_str(&format!("\n=== git pull in {} ===\n", oldest_path.display()));
    match git_run_capture(&oldest_path, &["pull"]) {
        Ok(out) => log.push_str(&out),
        Err(e) => {
            log.push_str(&format!("{e}\n"));
            return SyncResult {
                success: false,
                output: log,
                chat_import_skipped,
            };
        }
    }

    if chat_import_skipped {
        log.push_str(
            "\nNote: Cursor chats were exported only. Quit Cursor and sync again to import.\n",
        );
    }

    SyncResult {
        success: true,
        output: log,
        chat_import_skipped,
    }
}

fn side_path(pair: &RepoPairState, side: Side) -> Option<&PathBuf> {
    match side {
        Side::Local => pair.local.as_ref().map(|s| &s.path),
        Side::Remote => pair.remote.as_ref().map(|s| &s.path),
    }
}

fn git_output(repo: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .current_dir(repo)
        .args(args)
        .output()
        .with_context(|| format!("git {} in {}", args.join(" "), repo.display()))?;

    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr);
        anyhow::bail!("git {} failed: {}", args.join(" "), stderr.trim())
    }
}

fn git_run_capture(repo: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .current_dir(repo)
        .args(args)
        .output()
        .with_context(|| format!("git {} in {}", args.join(" "), repo.display()))?;

    let mut text = String::new();
    if !out.stdout.is_empty() {
        text.push_str(&String::from_utf8_lossy(&out.stdout));
    }
    if !out.stderr.is_empty() {
        if !text.is_empty() {
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

pub fn format_commit_time(ts: i64) -> String {
    if ts == 0 {
        return "unknown".to_string();
    }
    Local
        .timestamp_opt(ts, 0)
        .single()
        .map(|dt: DateTime<Local>| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn short_sha(sha: &str) -> String {
    sha.chars().take(8).collect()
}

pub fn side_label(side: Side) -> &'static str {
    match side {
        Side::Local => "local",
        Side::Remote => "remote (laptop)",
    }
}
