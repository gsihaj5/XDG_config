#!/bin/sh
# Idle: dim via xfce4-power-manager only. No auto-lock or suspend.

gsettings set org.cinnamon.desktop.screensaver idle-activation-enabled false 2>/dev/null
gsettings set org.cinnamon.desktop.screensaver lock-enabled false 2>/dev/null
gsettings set org.gnome.desktop.screensaver idle-activation-enabled false 2>/dev/null
gsettings set org.gnome.desktop.screensaver lock-enabled false 2>/dev/null
gsettings set org.gnome.settings-daemon.plugins.power sleep-inactive-ac-timeout 0 2>/dev/null
gsettings set org.gnome.settings-daemon.plugins.power sleep-inactive-battery-timeout 0 2>/dev/null
gsettings set org.gnome.settings-daemon.plugins.power sleep-inactive-ac-type nothing 2>/dev/null
gsettings set org.gnome.settings-daemon.plugins.power sleep-inactive-battery-type nothing 2>/dev/null
gsettings set org.gnome.settings-daemon.plugins.power idle-dim true 2>/dev/null

xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/blank-on-ac -s 5 --create -t int
xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/blank-on-battery -s 5 --create -t int
xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/brightness-level-on-ac -s 30 --create -t int
xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/brightness-level-on-battery -s 30 --create -t int
xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/lock-screen-suspend-hibernate -s false --create -t bool
xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/dpms-on-ac-off -s 0 --create -t int
xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/dpms-on-battery-off -s 0 --create -t int
xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/dpms-on-ac-sleep -s 0 --create -t int
xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/dpms-on-battery-sleep -s 0 --create -t int

# Keep X screensaver off so light-locker does not auto-lock on blank.
xset s off
xset s noblank

pidof xfce4-power-manager >/dev/null && xfce4-power-manager -r
