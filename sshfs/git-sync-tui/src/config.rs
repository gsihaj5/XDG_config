use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigFile {
    pub mount: MountConfig,
    #[serde(default)]
    pub roots: Vec<RootPair>,
    #[serde(default)]
    pub cursor: CursorConfigFile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CursorConfigFile {
    #[serde(default = "default_cursor_enabled")]
    pub enabled: bool,
    #[serde(default = "default_cursaves_bin")]
    pub cursaves_bin: String,
    #[serde(default = "default_true")]
    pub require_quit_for_import: bool,
    #[serde(default = "default_true")]
    pub push_when_cursor_open: bool,
}

fn default_cursor_enabled() -> bool {
    true
}

fn default_cursaves_bin() -> String {
    "cursaves".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for CursorConfigFile {
    fn default() -> Self {
        Self {
            enabled: default_cursor_enabled(),
            cursaves_bin: default_cursaves_bin(),
            require_quit_for_import: true,
            push_when_cursor_open: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CursorConfig {
    pub enabled: bool,
    pub cursaves_bin: String,
    pub require_quit_for_import: bool,
    pub push_when_cursor_open: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MountConfig {
    pub default_mount_point: String,
    pub default_connect_script: String,
    #[serde(default)]
    pub alternatives: Vec<MountAlternative>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MountAlternative {
    pub label: String,
    pub mount_point: String,
    pub connect_script: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RootPair {
    pub local: String,
    pub remote_mount_suffix: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedMountAlternative {
    pub label: String,
    pub mount_point: PathBuf,
    pub connect_script: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ResolvedRootPair {
    pub local: PathBuf,
    pub remote_suffix: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub default_mount_point: PathBuf,
    pub default_connect_script: PathBuf,
    pub alternatives: Vec<ResolvedMountAlternative>,
    pub roots: Vec<ResolvedRootPair>,
    pub cursor: CursorConfig,
}

pub fn expand_path(raw: &str) -> PathBuf {
    if raw.starts_with('~') {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(raw.replacen('~', &home, 1));
        }
    }
    PathBuf::from(raw)
}

pub fn default_config_path() -> PathBuf {
    expand_path("~/.config/git-sync-tui/config.toml")
}

pub fn load_config(path: &Path) -> Result<ResolvedConfig> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("read config {}", path.display()))?;
    let file: ConfigFile =
        toml::from_str(&content).with_context(|| format!("parse config {}", path.display()))?;
    resolve_config(file)
}

fn resolve_config(file: ConfigFile) -> Result<ResolvedConfig> {
    let default_mount_point = expand_path(&file.mount.default_mount_point);
    let default_connect_script = expand_path(&file.mount.default_connect_script);

    let mut alternatives = file
        .mount
        .alternatives
        .into_iter()
        .map(|a| ResolvedMountAlternative {
            label: a.label,
            mount_point: expand_path(&a.mount_point),
            connect_script: expand_path(&a.connect_script),
        })
        .collect::<Vec<_>>();

    if alternatives.is_empty() {
        alternatives.push(ResolvedMountAlternative {
            label: "Default".to_string(),
            mount_point: default_mount_point.clone(),
            connect_script: default_connect_script.clone(),
        });
    }

    let roots = file
        .roots
        .into_iter()
        .map(|r| ResolvedRootPair {
            local: expand_path(&r.local),
            remote_suffix: r.remote_mount_suffix,
        })
        .collect();

    let cursor = CursorConfig {
        enabled: file.cursor.enabled,
        cursaves_bin: file.cursor.cursaves_bin,
        require_quit_for_import: file.cursor.require_quit_for_import,
        push_when_cursor_open: file.cursor.push_when_cursor_open,
    };

    Ok(ResolvedConfig {
        default_mount_point,
        default_connect_script,
        alternatives,
        roots,
        cursor,
    })
}

impl ResolvedConfig {
    pub fn remote_root(&self, active_mount: &Path, root: &ResolvedRootPair) -> PathBuf {
        active_mount.join(&root.remote_suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_tilde() {
        std::env::set_var("HOME", "/home/test");
        assert_eq!(expand_path("~/projects"), PathBuf::from("/home/test/projects"));
    }
}
