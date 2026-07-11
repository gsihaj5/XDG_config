#!/bin/sh
SLEEP_DUR=0.5

# Let the existing workspace-init exec finish first
sleep 1

i3-msg workspace number 1
i3-msg 'exec --no-startup-id alacritty'
sleep $SLEEP_DUR

i3-msg workspace number 3
i3-msg 'exec --no-startup-id brave-browser --profile-directory=Work --new-window'
sleep $SLEEP_DUR

i3-msg workspace number 4
i3-msg 'exec --no-startup-id cursor'
sleep $SLEEP_DUR

i3-msg workspace number 1
