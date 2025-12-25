#!/bin/bash

FE_DIR="$1"

alacritty -e zsh -lic "
cd $FE_DIR
npm run dev &
NPM_PID=\$!
trap 'echo Killing npm...; kill \$NPM_PID; wait; echo Press any key...; read -n 1; bash' SIGINT
wait \$NPM_PID
echo 'React exited. Press any key...'
read -n 1
zsh
"
