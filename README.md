# Horizon Max FPS

Horizon Max FPS is an open-source native client mod for Horizon Minecraft.

## Version 0.4.0

This release removes every rendering callback from the previous versions. Those callbacks could reduce performance because they ran repeatedly during entity and GUI rendering.

The mod now does one focused job:

- Requests a 1000 FPS foreground limit.
- Never hides the first-person hand or held item.
- Never hides entities, the HUD, item frames, blocks, particles, or effects.
- Never runs per-entity or per-frame rendering cancellation code.
- Uses no networking, telemetry, automation, or data collection.

## Important performance note

A frame-rate limit is not the same as generated FPS. The mod can request a 1000 FPS ceiling, but the actual result still depends on the CPU, GPU, resolution, render distance, world complexity, other mods, and Horizon itself. It cannot guarantee 1000 generated frames per second.

## Clean update

1. Fully close Horizon and Minecraft.
2. Remove the older Horizon Max FPS installation.
3. Add this repository again:
   `https://github.com/osmantugraseker35-lab/horizon-maxfps`
4. Start Horizon.
5. Make sure VSync is disabled in the game settings.
6. Set the normal game frame-rate limit to its highest available value.

## Compatibility

- Horizon game version: 26.3
- Horizon native API: 0.2
- Client-side only

## License

MIT License.

## Author

Osman Tugra Seker
