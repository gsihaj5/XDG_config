#!/bin/bash

FE_DIR="$1"
TITLE="${2:-FE NPM}"

# Always keep the terminal open after npm exits (incl. Ctrl-C / non-zero).
# ERR_EXIT in zshrc would otherwise abort before the keep-alive shell.
alacritty --title "$TITLE" -e zsh -lic "
cd $FE_DIR || exit 1
npm run dev || true
exec zsh
"
