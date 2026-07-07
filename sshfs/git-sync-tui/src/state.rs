use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::config::{default_config_path, expand_path};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppState {
    /// Remote folder name under mount point, keyed by mount alternative label.
    #[serde(default)]
    pub remote_projects_suffix: HashMap<String, String>,
}

impl AppState {
    pub fn state_path() -> PathBuf {
        default_config_path()
            .parent()
            .map(|p| p.join("state.toml"))
            .unwrap_or_else(|| expand_path("~/.config/git-sync-tui/state.toml"))
    }

    pub fn load() -> Self {
        let path = Self::state_path();
        if !path.exists() {
            return Self::default();
        }
        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::state_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create {}", parent.display()))?;
        }
        let content = toml::to_string_pretty(self).context("serialize state")?;
        fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
        Ok(())
    }

    pub fn suffix_for_mount(&self, mount_label: &str, default: &str) -> String {
        self.remote_projects_suffix
            .get(mount_label)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    pub fn set_suffix_for_mount(&mut self, mount_label: &str, suffix: &str) {
        self.remote_projects_suffix
            .insert(mount_label.to_string(), suffix.to_string());
    }
}

/// List immediate child directories under `mount` for the remote-root picker.
pub fn list_mount_subdirs(mount: &Path) -> Vec<(String, PathBuf)> {
    let mut dirs = Vec::new();
    let Ok(entries) = fs::read_dir(mount) else {
        return dirs;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        dirs.push((name, path));
    }

    dirs.sort_by(|a, b| a.0.cmp(&b.0));
    dirs
}
