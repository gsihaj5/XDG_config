#!/bin/bash

PRO_DIR="$2"
TITLE="$1"

alacritty --title "$TITLE" -e zsh -lic "
cd $PRO_DIR
nvim
zsh
"
