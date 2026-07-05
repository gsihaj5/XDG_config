use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::config::ResolvedRootPair;

/// Discover git repositories under `root`, returning map of relative path -> absolute path.
pub fn discover_repos(root: &Path) -> Vec<(String, PathBuf)> {
    if !root.is_dir() {
        return Vec::new();
    }

    let mut repos = Vec::new();
    walk_for_git(root, root, &mut repos);
    repos.sort_by(|a, b| a.0.cmp(&b.0));
    repos
}

fn walk_for_git(base: &Path, current: &Path, out: &mut Vec<(String, PathBuf)>) {
    let git_marker = current.join(".git");
    if git_marker.exists() {
        let rel = current
            .strip_prefix(base)
            .unwrap_or(current)
            .to_string_lossy()
            .to_string();
        out.push((rel, current.to_path_buf()));
        return;
    }

    let entries = match std::fs::read_dir(current) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name == ".git" || name.starts_with('.') {
            continue;
        }
        walk_for_git(base, &path, out);
    }
}

#[derive(Debug, Clone)]
pub struct ScannedRepo {
    pub relative_path: String,
    pub local: Option<PathBuf>,
    pub remote: Option<PathBuf>,
    /// Path checked on the remote for this repo (`remote_root/relative_path`).
    pub remote_checked: Option<PathBuf>,
}

pub fn pair_repos(
    roots: &[ResolvedRootPair],
    remote_roots: &[(PathBuf, PathBuf)], // (local_root, remote_root)
    remote_available: bool,
) -> Vec<ScannedRepo> {
    let mut map: HashMap<String, ScannedRepo> = HashMap::new();

    for (local_root, remote_root) in remote_roots {
        for (rel, path) in discover_repos(local_root) {
            let remote_checked = if remote_available {
                Some(remote_root.join(&rel))
            } else {
                None
            };

            let remote = remote_checked.as_ref().and_then(|remote_path| {
                if remote_path.join(".git").exists() {
                    Some(remote_path.clone())
                } else {
                    None
                }
            });

            let entry = map.entry(rel.clone()).or_insert_with(|| ScannedRepo {
                relative_path: rel.clone(),
                local: None,
                remote: None,
                remote_checked: None,
            });
            entry.local = Some(path);
            entry.remote_checked = remote_checked;
            if let Some(rp) = remote {
                entry.remote = Some(rp);
            }
        }
    }

    let _ = roots;

    let mut repos: Vec<_> = map.into_values().collect();
    repos.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    repos
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn discovers_nested_repo() {
        let tmp = std::env::temp_dir().join("git-sync-tui-test");
        let _ = fs::remove_dir_all(&tmp);
        let repo = tmp.join("proj/nested");
        fs::create_dir_all(&repo).unwrap();
        fs::create_dir_all(repo.join(".git")).unwrap();

        let found = discover_repos(&tmp);
        assert!(found.iter().any(|(r, _)| r == "proj/nested"));

        let _ = fs::remove_dir_all(&tmp);
    }
}
