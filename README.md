# Horizon Max FPS

Horizon Max FPS is an open-source native client mod for Horizon Minecraft. Version 0.3.0 fixes the missing first-person hand and removes misleading default visual reductions.

## What version 0.3.0 fixes

- The first-person hand and held item are always rendered.
- No visual feature is disabled by default.
- Every aggressive rendering reduction is optional.
- Old saved settings can no longer hide the first-person hand.
- All mod text and settings are in English.

## Optional performance modes

- **Item-frame contents:** useful only in areas containing many item frames.
- **Minimal HUD:** hides the entire HUD and may provide a small gain on weak systems.
- **Hide living entities:** an extreme diagnostic option; it hides mobs and players.

These options reduce rendering work only in the situations they affect. They do not overclock the computer, change Horizon's renderer, or guarantee a higher FPS number. At very high frame rates, results can be smaller than normal measurement variation.

## Installation or update

1. Open Horizon Launcher.
2. Remove the older Horizon Max FPS entry if it is still installed.
3. Add this repository:
   `https://github.com/osmantugraseker35-lab/horizon-maxfps`
4. Restart Horizon.
5. Leave all optional modes disabled for normal gameplay.

## Compatibility

- Horizon game version: 26.3
- Horizon native API: 0.2
- Client-side only

## License

MIT License. You may use, study, modify, and redistribute the source code under the license terms.

## Author

Osman Tugra Seker
