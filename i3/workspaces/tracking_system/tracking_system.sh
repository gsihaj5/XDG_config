#!/bin/bash

FE_DIR="~/projects/tracking_system/FE_tracking_system/"
BE_DIR="~/projects/tracking_system/BE_tracking_system/"
NOTE_DIR="~/projects/notes/tracking_system/"
SCRIPT_DIR="~/.config/i3/workspaces/tracking_system"

if xrandr | grep -q "HDMI-2 connected"; then
    echo "HDMI-2 connected"
    echo "using single screen workspace"

    exec $SCRIPT_DIR/tracking_system_ss_ws.sh $FE_DIR $BE_DIR $NOTE_DIR
else
    echo "only eDP1"
fi

exit
