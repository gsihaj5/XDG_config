#!/usr/bin/env bash
ws="${1:?workspace number required}"

case "$ws" in
  1|2) icon="󰆍" ;;
  3) icon="󰖟" ;;
  4) icon="󰍩" ;;
  *) icon="󰆲" ;;
esac

state="$(i3-msg -t get_workspaces 2>/dev/null | jq -r --argjson n "$ws" '.[] | select(.num == $n) | "\(.urgent) \(.focused) \(.visible)"' | head -1)"

color="#a89984"
if [ -n "$state" ]; then
  read -r urgent focused visible <<< "$state"
  if [ "$urgent" = "true" ]; then
    color="#fb4934"
  elif [ "$focused" = "true" ]; then
    color="#b8bb26"
  elif [ "$visible" = "true" ]; then
    color="#ebdbb2"
  fi
fi

printf '%%{F%s}%s%%{F-}' "$color" "$icon"
