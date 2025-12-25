#!/bin/sh

#single screen workspace

FE_DIR="$1"
BE_DIR="$2"
NOTE_DIR="$3"
SLEEP_DUR=0.5

# -------- Workspace 1: Dev and logs terminals --------
i3-msg "workspace 1"

i3-msg "split v"

# Start NVIM 1
# 1 window
i3-msg "exec --no-startup-id ~/.config/i3/workspaces/scripts/open_nvim.sh "FE-nvim" \"$FE_DIR\""

sleep $SLEEP_DUR

# Start NPM
# second window on bottom
i3-msg "exec --no-startup-id ~/.config/i3/workspaces/scripts/open_fe_next.sh \"$FE_DIR\""
sleep $SLEEP_DUR

i3-msg "split h"

# Start Docker
# other window on bottom sid by side
i3-msg "exec --no-startup-id ~/.config/i3/workspaces/scripts/open_be_docker.sh \"$BE_DIR\""

sleep $SLEEP_DUR
# open BE on top
i3-msg "focus up"
i3-msg "split h"

i3-msg "exec --no-startup-id ~/.config/i3/workspaces/scripts/open_nvim.sh "NOTE" \"$NOTE_DIR\""

#open note
sleep $SLEEP_DUR

i3-msg "focus left"
i3-msg "split v"

i3-msg "exec --no-startup-id ~/.config/i3/workspaces/scripts/open_nvim.sh "BE-nvim" \"$BE_DIR\""

sleep $SLEEP_DUR

i3-msg "layout tabbed"

sleep $SLEEP_DUR

i3-msg "resize grow width 30px or 30ppt"
i3-msg "resize grow height 30px or 30ppt"

# # preview
# i3-msg "workspace 3"
#
# i3-msg "exec --no-startup-id brave-browser --profile-directory=\"Work\" --new-window http://localhost:3000"
