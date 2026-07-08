#!/bin/bash
set -euo pipefail

repo="$(cd "$(dirname "$0")/.." && pwd)"
conf_src="$repo/lightdm/slick-greeter.conf"
conf_dest=/etc/lightdm/slick-greeter.conf
wallpaper_dest=/usr/share/backgrounds/gerry-login-wallpaper.jpg
target_user="${SUDO_USER:-$USER}"
target_home="$(getent passwd "$target_user" | cut -d: -f6)"
icon_src="$target_home/.icons/Gruvbox-Plus-Dark"
icon_dest=/usr/share/icons/Gruvbox-Plus-Dark

if [[ $EUID -ne 0 ]]; then
    exec sudo "$0" "$@"
fi

wallpaper_src="$(sudo -u "$target_user" -H bash <<'EOF'
xfconf-query -c xfce4-desktop -l -v 2>/dev/null \
  | awk -F'\t' '/last-image/ { print $2 }' \
  | grep -v '/usr/share/xfce4/backdrops/linuxmint.jpg' \
  | head -1
EOF
)"
wallpaper_src="${wallpaper_src//$'\n'/}"
if [[ -z "${wallpaper_src:-}" || ! -f "$wallpaper_src" ]]; then
  wallpaper_src="$(sudo -u "$target_user" -H xfconf-query -c xfce4-desktop \
    -p /backdrop/screen0/monitoreDP-1/workspace0/last-image 2>/dev/null || true)"
fi
if [[ -z "${wallpaper_src:-}" ]]; then
  echo "Could not detect a desktop wallpaper; keeping existing login wallpaper if present." >&2
elif [[ ! -r "$wallpaper_src" ]]; then
  echo "Desktop wallpaper is not world-readable: $wallpaper_src" >&2
  echo "Allow traversal with: chmod o+x \"$target_home\" (and ensure the file is readable)." >&2
else
  install -D -m 0644 "$wallpaper_src" "$wallpaper_dest"
  echo "Installed login wallpaper from $wallpaper_src"
fi

if [[ -d "$icon_src" && ! -e "$icon_dest" ]]; then
  ln -s "$icon_src" "$icon_dest"
  echo "Linked icon theme to $icon_dest"
fi

install -D -m 0644 "$conf_src" "$conf_dest"
echo "Installed $conf_dest"
