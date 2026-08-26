#!/bin/sh
if hyprctl monitors >/dev/null 2>&1; then
  hyprlauncher
else
  rofi -show drun
fi
