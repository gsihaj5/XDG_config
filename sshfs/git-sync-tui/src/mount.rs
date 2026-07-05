use std::path::Path;
use std::process::Command;

/// Returns true if `path` is a mount point.
pub fn is_mounted(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }

    if Command::new("mountpoint")
        .arg("-q")
        .arg(path)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        return true;
    }

    // Fallback: check /proc/mounts
    let path_str = path.to_string_lossy();
    if let Ok(mounts) = std::fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == path_str {
                return true;
            }
        }
    }

    false
}

pub struct ConnectResult {
    pub success: bool,
    pub output: String,
}

pub fn run_connect_script(script: &Path) -> ConnectResult {
    let output = Command::new("bash")
        .arg(script)
        .output();

    match output {
        Ok(out) => {
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
            ConnectResult {
                success: out.status.success(),
                output: if text.is_empty() {
                    if out.status.success() {
                        "Connect script finished successfully.".to_string()
                    } else {
                        format!("Connect script failed with status {}", out.status)
                    }
                } else {
                    text
                },
            }
        }
        Err(e) => ConnectResult {
            success: false,
            output: format!("Failed to run connect script: {e}"),
        },
    }
}
