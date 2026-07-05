# git-sync-tui

TUI to compare and sync git repos between this machine and an SSHFS-mounted laptop home.

Also syncs **Cursor agent chat history** via [cursaves](https://github.com/Callum-Ward/cursaves) when `[cursor]` is enabled in config.

## Setup

### 1. Config

From anywhere (adjust if your clone path differs):

```bash
mkdir -p ~/.config/git-sync-tui
cp ~/XDG_config/sshfs/git-sync-tui/config.example.toml ~/.config/git-sync-tui/config.toml
# edit mount paths and roots
```

Or from inside this directory:

```bash
mkdir -p ~/.config/git-sync-tui
cp config.example.toml ~/.config/git-sync-tui/config.toml
```

### 2. cursaves (one-time per machine)

```bash
uv tool install git+https://github.com/Callum-Ward/cursaves.git
cursaves init --remote git@github.com:YOU/cursor-chats.git   # private repo
```

Use the **same** remote on laptop and PC. Chat snapshots live in `~/.cursaves/`, separate from your project git remotes.

### 3. Build & run

```bash
cargo build --release
./target/release/git-sync-tui
```

## Sync behavior

On **Enter** (confirm sync):

1. **cursaves** — export/import chats for the repo's local path
   - Cursor **open**: export only (`cursaves push`); quit Cursor and sync again to import
   - Cursor **closed**: full `cursaves sync`
2. **git** — push newest side, fetch + pull oldest side

Restart Cursor after import to see chats in the sidebar.

## Cursor safety

git-sync-tui detects a running Cursor via `pgrep` (Linux) and optionally `fuser`/`sqlite3` on `~/.config/Cursor/User/globalStorage/state.vscdb`. Import is blocked while Cursor is open to avoid DB corruption.
