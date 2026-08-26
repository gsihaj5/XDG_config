#!/usr/bin/env bash

POLYBAR="${POLYBAR:-$HOME/.local/bin/polybar}"
POLYBAR_MSG="${POLYBAR_MSG:-$HOME/.local/bin/polybar-msg}"
CONFIG="$HOME/.config/polybar/config.ini"

"$POLYBAR_MSG" cmd quit 2>/dev/null || true
pkill -x polybar 2>/dev/null || true

for _ in $(seq 1 10); do
  pgrep -u "$UID" -x polybar >/dev/null || break
  sleep 0.2
done

monitor_width() {
  local mon="$1"
  if [ -n "$mon" ] && type -P xrandr >/dev/null 2>&1; then
    xrandr --query | awk -v m="$mon" '
      $1 == m {
        for (i = 1; i <= NF; i++) {
          if ($i ~ /^[0-9]+x[0-9]+/) {
            split($i, a, "x")
            print a[1]
            exit
          }
        }
      }
    '
    return
  fi

  if type -P xdpyinfo >/dev/null 2>&1; then
    xdpyinfo | awk '/dimensions:/{print $2}' | cut -dx -f1
  fi
}

set_bar_geometry() {
  local width="$1"
  local margin=4
  local left_w=400
  local center_w=350
  local right_w=320

  export LEFT_BAR_OFFSET=$margin
  export LEFT_BAR_WIDTH=$left_w
  export CENTER_BAR_WIDTH=$center_w
  export CENTER_BAR_OFFSET=$(( (width - center_w) / 2 ))
  export RIGHT_BAR_WIDTH=$right_w
  export RIGHT_BAR_OFFSET=$(( width - right_w - margin ))
}

launch_bars() {
  local monitor="$1"
  local width

  width="$(monitor_width "$monitor")"
  [ -n "$width" ] || width=1366
  set_bar_geometry "$width"

  for bar in left center right; do
    if [ -n "$monitor" ]; then
      MONITOR=$monitor "$POLYBAR" --config="$CONFIG" "$bar" &
    else
      "$POLYBAR" --config="$CONFIG" "$bar" &
    fi
  done
}

if type -P xrandr >/dev/null 2>&1; then
  mapfile -t monitors < <(xrandr --query | awk '/ connected/{print $1}')
else
  monitors=()
fi

if [ "${#monitors[@]}" -eq 0 ]; then
  launch_bars ""
else
  for m in "${monitors[@]}"; do
    launch_bars "$m"
  done
fi

# Hide bars while a client is fullscreen (i3 override-redirect bars stay on top otherwise).
"$HOME/.config/polybar/autohide-fullscreen.sh" &
