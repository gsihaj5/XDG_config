#!/usr/bin/env bash
# Rebuild WorkspaceIcons.ttf from SVG sources in ./workspaces/
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/workspaces" && pwd)"
FONT_DIR="$ROOT/font"
FONT_INSTALL="$HOME/.local/share/fonts/WorkspaceIcons"

mkdir -p "$FONT_DIR" "$FONT_INSTALL"

npx --yes icon-font-generator \
  "$ROOT"/ws01-terminal.svg \
  "$ROOT"/ws02-terminal.svg \
  "$ROOT"/ws03-browser.svg \
  "$ROOT"/ws04-cursor.svg \
  "$ROOT"/ws05.svg \
  "$ROOT"/ws06.svg \
  "$ROOT"/ws07.svg \
  "$ROOT"/ws08.svg \
  "$ROOT"/ws09.svg \
  "$ROOT"/ws10.svg \
  --out "$FONT_DIR" \
  --name WorkspaceIcons \
  --csspath "$FONT_DIR/workspace-icons.css" \
  --prefix . \
  --height=1000 \
  --types=ttf

cp "$FONT_DIR/WorkspaceIcons.ttf" "$FONT_INSTALL/"
fc-cache -f "$FONT_INSTALL"
echo "Installed $FONT_INSTALL/WorkspaceIcons.ttf"
