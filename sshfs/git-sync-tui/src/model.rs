use std::path::PathBuf;

use ratatui::layout::Rect;

use crate::config::{ResolvedConfig, ResolvedMountAlternative, ResolvedRootPair};
use crate::cursor::CursorCache;
use crate::detail_loader::DetailLoader;
use crate::git::RepoPairState;
use crate::mount::{is_mounted, run_connect_script, ConnectResult};
use crate::scanner::{pair_repos, ScannedRepo};
use crate::state::{list_mount_subdirs, AppState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickAction {
    Connect,
    ChangeMount,
    ChangeRemoteRoot,
    MountPickerRow(usize),
    MountPickerCancel,
    RemoteRootPickerRow(usize),
    RemoteRootPickerCancel,
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
    RemoteRootPicker,
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
    pub app_state: AppState,
    pub active_mount: ResolvedMountAlternative,
    pub mounted: bool,
    /// Index into `config.roots` for the projects pair (default: 0).
    pub projects_root_index: usize,
    pub remote_projects_suffix: String,
    pub remote_root_choices: Vec<(String, PathBuf)>,
    pub entries: Vec<ScannedRepo>,
    pub detail: Option<RepoPairState>,
    pub detail_index: Option<usize>,
    pub detail_loader: DetailLoader,
    pub detail_spinner: u32,
    pub cursor_cache: CursorCache,
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

        let app_state = AppState::load();
        let projects_root_index = 0;
        let projects_default = config
            .roots
            .get(projects_root_index)
            .map(|r| r.remote_suffix.as_str())
            .unwrap_or("projects");
        let remote_projects_suffix =
            app_state.suffix_for_mount(&active_mount.label, projects_default);

        let mut app = Self {
            config,
            app_state,
            active_mount,
            mounted,
            projects_root_index,
            remote_projects_suffix,
            remote_root_choices: Vec::new(),
            entries: Vec::new(),
            detail: None,
            detail_index: None,
            detail_loader: DetailLoader::new(),
            detail_spinner: 0,
            cursor_cache: CursorCache::default(),
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
        app.refresh_list();
        app.request_detail_load();
        app
    }

    pub fn projects_root(&self) -> Option<&ResolvedRootPair> {
        self.config.roots.get(self.projects_root_index)
    }

    pub fn projects_remote_root(&self) -> PathBuf {
        self.active_mount
            .mount_point
            .join(&self.remote_projects_suffix)
    }

    fn reload_suffix_for_active_mount(&mut self) {
        let default = self
            .config
            .roots
            .get(self.projects_root_index)
            .map(|r| r.remote_suffix.as_str())
            .unwrap_or("projects");
        self.remote_projects_suffix = self
            .app_state
            .suffix_for_mount(&self.active_mount.label, default);
    }

    fn remote_roots_for_scan(&self) -> Vec<(PathBuf, PathBuf)> {
        self.config
            .roots
            .iter()
            .enumerate()
            .map(|(i, r)| {
                let remote = if i == self.projects_root_index {
                    self.active_mount
                        .mount_point
                        .join(&self.remote_projects_suffix)
                } else {
                    self.config
                        .remote_root(&self.active_mount.mount_point, r)
                };
                (r.local.clone(), remote)
            })
            .collect()
    }

    pub fn open_remote_root_picker(&mut self) {
        if !self.mounted {
            self.status_message =
                "Mount the remote first to choose a projects folder.".to_string();
            return;
        }
        self.remote_root_choices = list_mount_subdirs(&self.active_mount.mount_point);
        if self.remote_root_choices.is_empty() {
            self.status_message = format!(
                "No directories found under {}.",
                self.active_mount.mount_point.display()
            );
            return;
        }
        self.modal = Modal::RemoteRootPicker;
    }

    pub fn select_remote_root(&mut self, choice_index: usize) {
        let Some((name, _)) = self.remote_root_choices.get(choice_index) else {
            self.modal = Modal::None;
            return;
        };
        let suffix = name.clone();
        self.remote_projects_suffix = suffix.clone();
        self.app_state
            .set_suffix_for_mount(&self.active_mount.label, &suffix);
        if let Err(e) = self.app_state.save() {
            self.status_message = format!("Saved choice locally but failed to persist: {e}");
        } else {
            self.status_message = format!("Remote projects root: {}", self.projects_remote_root().display());
        }
        self.modal = Modal::None;
        self.refresh_repos();
    }

    pub fn refresh_mount_status(&mut self) {
        self.mounted = is_mounted(&self.active_mount.mount_point);
    }

    pub fn refresh_list(&mut self) {
        self.refresh_mount_status();
        let remote_available = self.mounted;
        let remote_roots = self.remote_roots_for_scan();

        self.entries = pair_repos(&self.config.roots, &remote_roots, remote_available);

        if self.selected >= self.entries.len() && !self.entries.is_empty() {
            self.selected = self.entries.len() - 1;
        }
        self.clamp_scroll();

        if self.detail_index != Some(self.selected) {
            self.detail = None;
            self.detail_index = None;
        }
    }

    pub fn is_detail_loading(&self) -> bool {
        self.detail_loader.is_loading(self.selected)
    }

    pub fn loading_spinner_frame(&self) -> &'static str {
        const FRAMES: [&str; 4] = ["⠋", "⠙", "⠹", "⠸"];
        FRAMES[(self.detail_spinner as usize / 8) % FRAMES.len()]
    }

    pub fn tick_spinner(&mut self) {
        if self.is_detail_loading() {
            self.detail_spinner = self.detail_spinner.wrapping_add(1);
        }
    }

    /// Poll background loader; call every frame from the event loop.
    pub fn poll_detail_load(&mut self) {
        let Some((index, result)) = self.detail_loader.poll(self.selected) else {
            return;
        };

        match result {
            Ok(state) => {
                self.detail = Some(state);
                self.detail_index = Some(index);
                self.cursor_cache.invalidate_runtime();
            }
            Err(e) => {
                self.detail = None;
                self.detail_index = None;
                self.status_message = format!("Failed to inspect repo: {e}");
            }
        }
    }

    fn request_detail_load(&mut self) {
        let Some(entry) = self.entries.get(self.selected).cloned() else {
            self.detail_loader.cancel();
            self.detail = None;
            self.detail_index = None;
            return;
        };

        self.detail = None;
        self.detail_index = None;
        self.cursor_cache.invalidate_runtime();

        self.detail_loader.spawn(
            self.selected,
            entry,
            self.mounted,
            self.config.cursor.clone(),
        );
    }

    pub fn refresh_repos(&mut self) {
        self.refresh_list();
        self.request_detail_load();
    }

    pub fn select_mount(&mut self, index: usize) {
        if let Some(alt) = self.config.alternatives.get(index).cloned() {
            self.active_mount = alt;
            self.reload_suffix_for_active_mount();
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
        if self.entries.is_empty() {
            return;
        }
        if self.is_detail_loading() {
            self.status_message = "Still loading repo details...".to_string();
            return;
        }
        let Some(repo) = self.selected_repo() else {
            self.status_message = "Repo details not ready yet.".to_string();
            return;
        };
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

        let pair = self
            .detail
            .as_ref()
            .filter(|_| self.detail_index == Some(idx))
            .cloned();
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

    pub fn selected_entry(&self) -> Option<&ScannedRepo> {
        self.entries.get(self.selected)
    }

    pub fn selected_repo(&self) -> Option<&RepoPairState> {
        if self.detail_index == Some(self.selected) && !self.is_detail_loading() {
            self.detail.as_ref()
        } else {
            None
        }
    }

    pub fn list_icon(&self, index: usize) -> &'static str {
        if self.detail_loader.is_loading(index) {
            return "…";
        }
        if self.detail_index == Some(index) {
            return self
                .detail
                .as_ref()
                .map(|d| d.status.icon())
                .unwrap_or("·");
        }
        "·"
    }

    fn select_index(&mut self, index: usize) {
        if index == self.selected {
            return;
        }
        self.selected = index;
        self.clamp_scroll();
        self.request_detail_load();
    }

    pub fn move_selection(&mut self, delta: i32) {
        if self.entries.is_empty() {
            return;
        }
        let len = self.entries.len() as i32;
        let next = (self.selected as i32 + delta).rem_euclid(len);
        self.select_index(next as usize);
    }

    fn clamp_scroll(&mut self) {
        if self.entries.is_empty() {
            self.list_scroll = 0;
            return;
        }
        if self.selected < self.list_scroll {
            self.list_scroll = self.selected;
        }
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
                    ClickAction::ChangeRemoteRoot => self.open_remote_root_picker(),
                    ClickAction::MountPickerRow(i) => self.select_mount(i),
                    ClickAction::MountPickerCancel => self.modal = Modal::None,
                    ClickAction::RemoteRootPickerRow(i) => self.select_remote_root(i),
                    ClickAction::RemoteRootPickerCancel => self.modal = Modal::None,
                    ClickAction::RepoRow(i) => self.select_index(i),
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
