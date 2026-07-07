#!/bin/bash

BE_DIR="$1"
TITLE="${2:-BE Term}"

alacritty --title "$TITLE" -e zsh -lic "
cd $BE_DIR
docker compose up -d
zsh
"
