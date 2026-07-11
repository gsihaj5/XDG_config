#!/bin/bash
set -euo pipefail

repo="$(cd "$(dirname "$0")/.." && pwd)"
conf_src="$repo/lightdm/slick-greeter.conf"
conf_dest=/etc/lightdm/slick-greeter.conf
wallpaper_dest=/usr/share/backgrounds/gerry-login-wallpaper.jpg
polybar_dest=/usr/local/share/lightdm/greeter-polybar
wrapper_dest=/usr/local/bin/lightdm-greeter-with-polybar
greeter_desktop_dest=/usr/share/xgreeters/slick-greeter-polybar.desktop
lightdm_dropin_dest=/etc/lightdm/lightdm.conf.d/95-greeter-polybar.conf
target_user="${SUDO_USER:-$USER}"
target_home="$(getent passwd "$target_user" | cut -d: -f6)"
icon_src="$target_home/.icons/Gruvbox-Plus-Dark"
icon_dest=/usr/share/icons/Gruvbox-Plus-Dark

if [[ $EUID -ne 0 ]]; then
    exec sudo "$0" "$@"
fi

expand_path() {
    local path="$1"
    path="${path/#\~/$target_home}"
    printf '%s\n' "$path"
}

detect_wallpaper() {
    local path i3_config waypaper_config

  i3_config="$target_home/XDG_config/i3/config"
  if [[ -f "$i3_config" ]]; then
    path="$(awk '/feh --bg-fill/ {
      for (i = 1; i <= NF; i++) {
        if ($i ~ /^(\/|~)/) { print $i; exit }
      }
    }' "$i3_config")"
    path="$(expand_path "$path")"
    if [[ -n "$path" && -f "$path" ]]; then
      printf '%s\n' "$path"
      return 0
    fi
  fi

  waypaper_config="$target_home/XDG_config/waypaper/config.ini"
  if [[ -f "$waypaper_config" ]]; then
    path="$(awk -F' = ' '$1 == "wallpaper" { print $2; exit }' "$waypaper_config")"
    path="$(expand_path "$path")"
    if [[ -n "$path" && -f "$path" ]]; then
      printf '%s\n' "$path"
      return 0
    fi
  fi

  path="$(ps -u "$target_user" -o args= 2>/dev/null | awk '/feh --bg-fill/ {
    for (i = 1; i <= NF; i++) {
      if ($i ~ /^(\/|~)/) { print $i; exit }
    }
  }')"
  path="$(expand_path "$path")"
  if [[ -n "$path" && -f "$path" ]]; then
    printf '%s\n' "$path"
    return 0
  fi

  path="$(sudo -u "$target_user" -H bash <<'EOF'
xfconf-query -c xfce4-desktop -l -v 2>/dev/null \
  | awk -F'\t' '/last-image/ { print $2 }' \
  | grep -v '/usr/share/xfce4/backdrops/linuxmint.jpg' \
  | head -1
EOF
)"
  path="${path//$'\n'/}"
  if [[ -n "$path" && -f "$path" ]]; then
    printf '%s\n' "$path"
    return 0
  fi

  return 1
}

wallpaper_src="$(detect_wallpaper || true)"
if [[ -z "${wallpaper_src:-}" ]]; then
  echo "Could not detect a desktop wallpaper; keeping existing login wallpaper if present." >&2
elif [[ ! -r "$wallpaper_src" ]]; then
  echo "Desktop wallpaper is not readable by root: $wallpaper_src" >&2
  echo "Run: chmod o+x \"$target_home\" and ensure the image is world-readable." >&2
else
  install -D -m 0644 "$wallpaper_src" "$wallpaper_dest"
  echo "Installed login wallpaper from $wallpaper_src"
fi

if [[ -d "$icon_src" && ! -e "$icon_dest" ]]; then
  ln -s "$icon_src" "$icon_dest"
  echo "Linked icon theme to $icon_dest"
fi

install -d -m 0755 "$polybar_dest/modules"
install -m 0644 "$repo/lightdm/greeter-polybar/config.ini" "$polybar_dest/"
install -m 0644 "$repo/lightdm/greeter-polybar/colors.ini" "$polybar_dest/"
install -m 0644 "$repo/lightdm/greeter-polybar/modules/"*.ini "$polybar_dest/modules/"
install -m 0755 "$repo/lightdm/greeter-polybar/launch.sh" "$polybar_dest/"
install -m 0755 "$repo/lightdm/greeter-with-polybar.sh" "$wrapper_dest"
install -m 0644 "$repo/lightdm/slick-greeter-polybar.desktop" "$greeter_desktop_dest"
install -D -m 0644 "$repo/lightdm/lightdm.conf.d/95-greeter-polybar.conf" "$lightdm_dropin_dest"
install -D -m 0644 "$conf_src" "$conf_dest"

echo "Installed greeter polybar and $conf_dest"
echo "Log out to preview the login screen."
