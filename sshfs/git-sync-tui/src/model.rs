use std::path::PathBuf;

use ratatui::layout::Rect;

use crate::config::{ResolvedConfig, ResolvedMountAlternative};
use crate::git::RepoPairState;
use crate::mount::{is_mounted, run_connect_script, ConnectResult};
use crate::scanner::pair_repos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickAction {
    Connect,
    ChangeMount,
    MountPickerRow(usize),
    MountPickerCancel,
    RepoRow(usize),
    SyncConfirmYes,
    SyncConfirmNo,
    ResultDismiss,
}

#[derive(Debug, Clone)]
pub struct ClickZone {
    pub rect: Rect,
    pub action: ClickAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modal {
    None,
    MountPicker,
    SyncConfirm,
    Result,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectState {
    Idle,
    Running,
    Done,
}

pub struct App {
    pub config: ResolvedConfig,
    pub active_mount: ResolvedMountAlternative,
    pub mounted: bool,
    pub repos: Vec<RepoPairState>,
    pub selected: usize,
    pub list_scroll: usize,
    pub modal: Modal,
    pub result_text: String,
    pub result_success: bool,
    pub connect_state: ConnectState,
    pub click_zones: Vec<ClickZone>,
    pub status_message: String,
    pub should_quit: bool,
    pub pending_sync_index: Option<usize>,
}

impl App {
    pub fn new(config: ResolvedConfig) -> Self {
        let active_mount = config
            .alternatives
            .iter()
            .find(|a| a.mount_point == config.default_mount_point)
            .cloned()
            .or_else(|| config.alternatives.first().cloned())
            .unwrap_or_else(|| ResolvedMountAlternative {
                label: "Default".to_string(),
                mount_point: config.default_mount_point.clone(),
                connect_script: config.default_connect_script.clone(),
            });

        let mounted = is_mounted(&active_mount.mount_point);

        let mut app = Self {
            config,
            active_mount,
            mounted,
            repos: Vec::new(),
            selected: 0,
            list_scroll: 0,
            modal: Modal::None,
            result_text: String::new(),
            result_success: false,
            connect_state: ConnectState::Idle,
            click_zones: Vec::new(),
            status_message: String::new(),
            should_quit: false,
            pending_sync_index: None,
        };
        app.refresh_repos();
        app
    }

    pub fn refresh_mount_status(&mut self) {
        self.mounted = is_mounted(&self.active_mount.mount_point);
    }

    pub fn refresh_repos(&mut self) {
        self.refresh_mount_status();
        let remote_available = self.mounted;

        let remote_roots: Vec<(PathBuf, PathBuf)> = self
            .config
            .roots
            .iter()
            .map(|r| {
                (
                    r.local.clone(),
                    self.config.remote_root(&self.active_mount.mount_point, r),
                )
            })
            .collect();

        let scanned = pair_repos(&self.config.roots, &remote_roots, remote_available);

        self.repos = scanned
            .iter()
            .filter_map(|s| {
                crate::git::classify_pair(s, remote_available, &self.config.cursor).ok()
            })
            .collect();

        if self.selected >= self.repos.len() && !self.repos.is_empty() {
            self.selected = self.repos.len() - 1;
        }
        self.clamp_scroll();
    }

    pub fn select_mount(&mut self, index: usize) {
        if let Some(alt) = self.config.alternatives.get(index).cloned() {
            self.active_mount = alt;
            self.refresh_repos();
            self.status_message = format!(
                "Switched to {} ({})",
                self.active_mount.label,
                self.active_mount.mount_point.display()
            );
        }
        self.modal = Modal::None;
    }

    pub fn run_connect(&mut self) {
        if self.connect_state == ConnectState::Running {
            return;
        }
        self.connect_state = ConnectState::Running;
        self.status_message = "Connecting...".to_string();

        let script = self.active_mount.connect_script.clone();
        let result = run_connect_script(&script);
        self.handle_connect_result(result);
    }

    pub fn handle_connect_result(&mut self, result: ConnectResult) {
        self.connect_state = ConnectState::Done;
        self.refresh_mount_status();

        if result.success && self.mounted {
            self.status_message = "Mounted successfully.".to_string();
            self.refresh_repos();
        } else if result.success && !self.mounted {
            self.result_success = false;
            self.result_text = format!(
                "Connect script finished but mount point is still not mounted.\n\n{}",
                result.output
            );
            self.modal = Modal::Result;
        } else {
            self.result_success = false;
            self.result_text = format!("Connect failed.\n\n{}", result.output);
            self.modal = Modal::Result;
        }
    }

    pub fn request_sync(&mut self) {
        if self.repos.is_empty() {
            return;
        }
        let repo = &self.repos[self.selected];
        if !repo.status.can_sync() {
            self.status_message = format!("Cannot sync: {}", repo.status.label());
            return;
        }
        self.pending_sync_index = Some(self.selected);
        self.modal = Modal::SyncConfirm;
    }

    pub fn confirm_sync(&mut self) {
        let Some(idx) = self.pending_sync_index.take() else {
            self.modal = Modal::None;
            return;
        };

        let pair = self.repos.get(idx).cloned();
        self.modal = Modal::None;

        if let Some(pair) = pair {
            let result = crate::git::execute_sync(&pair, &self.config.cursor);
            self.result_success = result.success;
            self.result_text = result.output;
            if result.chat_import_skipped {
                self.result_text.push_str(
                    "\n\nCursor was running: chats exported but not imported. Quit Cursor and sync again.",
                );
            }
            self.modal = Modal::Result;
            if result.success {
                self.refresh_repos();
            }
        }
    }

    pub fn cancel_modal(&mut self) {
        self.modal = Modal::None;
        self.pending_sync_index = None;
    }

    pub fn selected_repo(&self) -> Option<&RepoPairState> {
        self.repos.get(self.selected)
    }

    pub fn move_selection(&mut self, delta: i32) {
        if self.repos.is_empty() {
            return;
        }
        let len = self.repos.len() as i32;
        let next = (self.selected as i32 + delta).rem_euclid(len);
        self.selected = next as usize;
        self.clamp_scroll();
    }

    fn clamp_scroll(&mut self) {
        if self.repos.is_empty() {
            self.list_scroll = 0;
            return;
        }
        if self.selected < self.list_scroll {
            self.list_scroll = self.selected;
        }
        // visible height adjusted in UI; keep scroll reasonable
        let visible = 20usize;
        if self.selected >= self.list_scroll + visible {
            self.list_scroll = self.selected.saturating_sub(visible - 1);
        }
    }

    pub fn register_click(&mut self, rect: Rect, action: ClickAction) {
        self.click_zones.push(ClickZone { rect, action });
    }

    pub fn clear_clicks(&mut self) {
        self.click_zones.clear();
    }

    pub fn click_at(&mut self, col: u16, row: u16) -> bool {
        for zone in &self.click_zones {
            if zone.rect.contains(ratatui::layout::Position::new(col, row)) {
                match zone.action {
                    ClickAction::Connect => self.run_connect(),
                    ClickAction::ChangeMount => self.modal = Modal::MountPicker,
                    ClickAction::MountPickerRow(i) => self.select_mount(i),
                    ClickAction::MountPickerCancel => self.modal = Modal::None,
                    ClickAction::RepoRow(i) => {
                        self.selected = i;
                        self.clamp_scroll();
                    }
                    ClickAction::SyncConfirmYes => self.confirm_sync(),
                    ClickAction::SyncConfirmNo => self.cancel_modal(),
                    ClickAction::ResultDismiss => self.modal = Modal::None,
                }
                return true;
            }
        }
        false
    }
}
