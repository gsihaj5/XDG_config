#!/bin/bash

ln -s ~/XDG_config/alacritty/ ~/.config/alacritty
ln -s ~/XDG_config/i3/ ~/.config/i3
ln -s ~/XDG_config/fusuma/ ~/.config/fusuma
ln -s ~/XDG_config/X11/ ~/.config/X11
ln -s ~/XDG_config/hypr/ ~/.config/hypr
ln -s ~/XDG_config/nvim/ ~/.config/nvim
ln -s ~/XDG_config/rofi/ ~/.config/rofi
ln -s ~/XDG_config/polybar/ ~/.config/polybar
ln -s ~/XDG_config/picom/ ~/.config/picom
ln -s ~/XDG_config/waybar/ ~/.config/waybar
ln -s ~/XDG_config/waypaper/ ~/.config/waypaper

# AppImage launchers and icons (merged into existing XDG dirs)
mkdir -p ~/.local/share/applications ~/.local/share/icons/hicolor
for desktop in ~/XDG_config/applications/*.desktop; do
    [ -f "$desktop" ] || continue
    ln -sf "$desktop" ~/.local/share/applications/
done
repo_hicolor=~/XDG_config/applications/icons/hicolor
if [ -d "$repo_hicolor" ]; then
    find "$repo_hicolor" -type f | while IFS= read -r icon; do
        relpath="${icon#$repo_hicolor/}"
        dest=~/.local/share/icons/hicolor/"$relpath"
        mkdir -p "$(dirname "$dest")"
        ln -sf "$icon" "$dest"
    done
fi
update-desktop-database ~/.local/share/applications 2>/dev/null || true
