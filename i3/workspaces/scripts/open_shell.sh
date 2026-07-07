#!/bin/bash

PRO_DIR="$1"
TITLE="${2:-Shell}"

alacritty --title "$TITLE" -e zsh -lic "
cd $PRO_DIR
zsh
"
