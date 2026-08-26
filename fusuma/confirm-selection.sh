#!/bin/sh
if [ "${XDG_SESSION_TYPE:-}" = wayland ] && command -v wtype >/dev/null 2>&1; then
  wtype -k Return
else
  xdotool key Return
fi
