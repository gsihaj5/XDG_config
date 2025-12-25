#!/bin/sh

FE_DIR="$1"
BE_DIR="$2"
NOTE_DIR="$3"

SLEEP_DUR=0.5

# -------- Workspace 2: Dev and logs terminals --------
i3-msg "workspace 1"

#start npm run dev
i3-msg "exec --no-startup-id ~/.config/i3/workspaces/scripts/open_nvim.sh "FE-nvim" \"$FE_DIR\""

# layout tab
i3-msg "split h"

sleep $SLEEP_DUR
i3-msg "exec --no-startup-id ~/.config/i3/workspaces/scripts/open_nvim.sh "NOTE" \"$NOTE_DIR\""

sleep $SLEEP_DUR

i3-msg "focus left"
i3-msg "split v"
i3-msg "exec --no-startup-id ~/.config/i3/workspaces/scripts/open_nvim.sh "BE-nvim" \"$BE_DIR\""

sleep $SLEEP_DUR
i3-msg "layout tabbed"
# -------- Workspace 2: Dev and logs terminals --------
i3-msg "workspace 2"

#start npm run dev
i3-msg "exec --no-startup-id ~/.config/i3/workspaces/scripts/open_fe_next.sh \"$FE_DIR\""

# split horizontally
i3-msg "split h"

sleep $SLEEP_DUR
i3-msg "exec --no-startup-id ~/.config/i3/workspaces/scripts/open_be_docker.sh \"$BE_DIR\""

sleep $SLEEP_DUR
# -------- Workspace 3: preview --------
i3-msg "workspace 3"

i3-msg "exec --no-startup-id brave-browser --profile-directory=\"Work\" --new-window http://localhost:3000"
