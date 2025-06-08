# Polybar gruvbox theme

Simple and beautiful gruvbox dark theme for [polybar](https://github.com/polybar/polybar)

## Screenshots

Defalut view
![screenshot](./screenshots/1.png)

With i3 mode
![screenshot](./screenshots/2.png)


## Requirements
* polybar - 3.7.0 or above


## Install

Backup current config if exists

```sh
mv ~/.config/polybar ~/.config/polybar.backup
```
___

Download this repo
```sh
git clone --depth=1 https://github.com/emgyrz/polybar-gruvbox-theme.git ~/.config/polybar
```
___

Install fonts
```sh
# use your package manager to get JetBrains Mono
best_package_manager install jetbrains-mono 

# Material Icons (Round)
mkdir ~/.fonts && cp -R ~/.config/polybar/fonts/MaterialIcons ~/.fonts/ && fc-cache -f
```


## Configuration

Read polybar [docs](https://github.com/polybar/polybar/wiki).
___
Update device names:

* `modules/backlight.ini`:`card` - see available cars in `/sys/class/backlight/`
* `modules/battery.ini`:`battery,adapter` - see `/sys/class/power_supply/`
* `modules/temperature.ini`:`zone-type` - see content of `/sys/class/thermal/thermal_zone*/type`
* `modules/wired.ini`:`interface` - net device name can be found with command like `ifconfig`, `ip link` etc
* `modules/wlan.ini`:`interface` - ^
___
Select visible blocks in `modules.ini`
___
Add launch command when your WM starts. For example, for i3:
```sh
# ~/.config/i3/config
exec_always --no-startup-id ~/.config/polybar/launch.sh
```


