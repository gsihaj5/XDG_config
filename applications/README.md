# AppImage launchers

Desktop entries here are symlinked into `~/.local/share/applications/` by `attach.sh`, so they appear in rofi (`Mod+d` drun) and other XDG app menus.

## Add a new AppImage

1. Download `Foo-1.2.3.AppImage` to `~/tools/`
2. Make it executable and create a stable symlink:
   ```bash
   chmod +x ~/tools/Foo-1.2.3.AppImage
   ln -sf Foo-1.2.3.AppImage ~/tools/foo.AppImage
   ```
3. Copy `obsidian.desktop` to `foo.desktop` and edit:
   - `Name`, `Comment`, `Exec`, `Icon`, `StartupWMClass`, `Categories`
   - Use the stable symlink path in `Exec` (e.g. `/home/gerry/tools/foo.AppImage`)
4. Add an icon under `icons/hicolor/256x256/apps/` if your icon theme does not include it:
   ```bash
   cd /tmp && mkdir foo_extract && cd foo_extract
   ~/tools/foo.AppImage --appimage-extract usr/share/icons/hicolor/256x256/apps/*.png
   cp squashfs-root/usr/share/icons/hicolor/256x256/apps/*.png \
      ~/XDG_config/applications/icons/hicolor/256x256/apps/
   ```
5. Run `~/XDG_config/attach.sh`
6. Refresh the desktop database:
   ```bash
   update-desktop-database ~/.local/share/applications
   ```

## Update an AppImage version

Only update the symlink in `~/tools/`; the `.desktop` file stays the same:

```bash
ln -sf Foo-1.3.0.AppImage ~/tools/foo.AppImage
```

## Template

```ini
[Desktop Entry]
Name=APP_NAME
Comment=SHORT_DESCRIPTION
Exec=/home/gerry/tools/APP_SYMLINK.AppImage %F
Terminal=false
Type=Application
Icon=APP_ICON_NAME
StartupWMClass=APP_WM_CLASS
Categories=Utility;
```

Do not set `NoDisplay=true` if you want the app to appear in rofi.
