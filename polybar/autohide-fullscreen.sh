#!/usr/bin/env bash
# Hide polybar while the focused i3 workspace has a fullscreen container.

POLYBAR_MSG="${POLYBAR_MSG:-$HOME/.local/bin/polybar-msg}"
PIDFILE="${XDG_RUNTIME_DIR:-/tmp}/polybar-autohide-fullscreen.pid"

if [ -f "$PIDFILE" ]; then
  old="$(cat "$PIDFILE" 2>/dev/null || true)"
  if [ -n "$old" ] && [ "$old" != "$$" ]; then
    kill "$old" 2>/dev/null || true
    for _ in 1 2 3 4 5 6 7 8 9 10; do
      kill -0 "$old" 2>/dev/null || break
      sleep 0.1
    done
  fi
fi
echo $$ >"$PIDFILE"
# Only remove pidfile if we still own it (avoids reload race with predecessor).
trap '[ "$(cat "$PIDFILE" 2>/dev/null)" = "$$" ] && rm -f "$PIDFILE"' EXIT

is_fullscreen() {
  # Workspace nodes can report fullscreen_mode=1 even when no client is
  # fullscreen (and scratchpad always does). Only count non-workspace cons.
  i3-msg -t get_tree 2>/dev/null | jq -e '
    first(
      .. | objects
      | select(.type? == "workspace")
      | select(any(.. | objects; .focused? == true))
    )
    | any(.. | objects;
        .type? != "workspace"
        and (.fullscreen_mode // 0) != 0
        and (.fullscreen_mode // "none") != "none"
      )
  ' >/dev/null 2>&1
}

update_bar() {
  if is_fullscreen; then
    "$POLYBAR_MSG" cmd hide 2>/dev/null || true
  else
    "$POLYBAR_MSG" cmd show 2>/dev/null || true
  fi
}

update_bar
# Reconnect if i3 IPC drops across reload/restart.
while true; do
  while IFS= read -r _; do
    update_bar
  done < <(i3-msg -t subscribe -m '["window","workspace"]')
  sleep 0.5
done
