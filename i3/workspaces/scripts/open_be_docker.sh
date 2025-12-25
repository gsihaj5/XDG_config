#!/bin/bash

BE_DIR="$1"

alacritty -e zsh -lic "
cd $BE_DIR
docker compose up -d
zsh
"
