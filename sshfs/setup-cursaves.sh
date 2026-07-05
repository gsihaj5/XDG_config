#!/bin/bash
# One-time setup for Cursor chat sync (cursaves) on this machine.
set -euo pipefail

export PATH="${HOME}/.local/bin:${PATH}"

if ! command -v uv >/dev/null 2>&1; then
  echo "Installing uv..."
  curl -fsSL https://astral.sh/uv/install.sh | sh
  export PATH="${HOME}/.local/bin:${PATH}"
fi

if ! command -v cursaves >/dev/null 2>&1; then
  echo "Installing cursaves..."
  uv tool install "git+https://github.com/Callum-Ward/cursaves.git"
fi

if [[ -d "${HOME}/.cursaves/.git" ]]; then
  echo "~/.cursaves already initialized."
else
  if [[ -n "${1:-}" ]]; then
    cursaves init --remote "$1"
  else
    cursaves init
    echo ""
    echo "Local store created. To sync between machines, run:"
    echo "  $0 git@github.com:YOU/cursor-chats.git"
  fi
fi

echo "Done. cursaves: $(command -v cursaves)"
