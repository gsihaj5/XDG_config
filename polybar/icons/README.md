# Workspace icons

Sharp SVG icons for polybar workspaces. Polybar cannot render SVG files directly, so the SVGs are compiled into `WorkspaceIcons.ttf`.

## Sources

| Workspace | File | Icon |
|-----------|------|------|
| 1 | `workspaces/ws01-terminal.svg` | Terminal (Lucide) |
| 2 | `workspaces/ws02-terminal.svg` | Terminal alt (Lucide) |
| 3 | `workspaces/ws03-browser.svg` | Browser (Lucide globe) |
| 4 | `workspaces/ws04-cursor.svg` | Cursor AI (Simple Icons) |
| 5-10 | `workspaces/ws05.svg` … `ws10.svg` | Generic Lucide icons |

## Rebuild font after editing SVGs

```bash
./polybar/icons/build-workspace-font.sh
~/.config/polybar/launch.sh
```

## Replace an icon

1. Edit or replace the SVG in `polybar/icons/workspaces/`
2. Run the rebuild script above
3. Reload polybar
