# OMARCY Studio

OMARCY Studio is a native, open-source pixel game editor for Omarchy Linux. It is designed to grow with the creator: a child can start with a pencil and Play button, while an advanced user can progressively unlock tilemaps, animation, lighting, visual logic, scripting, and plugins.

This repository is the **Game Editor Beta** proposal: a focused, working editor shell that proves the adaptive workspace and the first creation loop. It is intentionally honest about what is implemented and what is staged next.

## Current beta slice

- Native Rust desktop window powered by `eframe`/`egui` with the `wgpu` renderer.
- Sprite Lab / World Builder / Visual Logic workspace switching.
- Canvas preview with a crisp pixel-scene composition, frame controls, playback, onion-skin toggle, zoom, and tool cursor.
- World Builder map preview with overworld, town, and interior map styles, chunk grid, terrain, houses, objects, and layer guidance.
- Dialogue box authoring and preview with editable speaker and line fields, ready for portraits, choices, and event hooks.
- Beginner-friendly tool rail, layers, palette, project status, asset browser, and unlock path.
- Creator level model with a clear progression from sprite painting to plugins.
- Local-first project model ready to grow into a Git-friendly on-disk format.

## Run it

```sh
cargo run
```

Linux prerequisites are the standard `wgpu`/windowing development libraries for your distribution. On Arch/Omarchy, install the Rust toolchain and the desktop graphics dependencies from the system repositories.

## Design direction

The beta follows the supplied Omasprite-style reference: dark graphite chrome, chunky monospace controls, hot magenta and violet accents, lime progress signals, and a large scene-first canvas. See [PRODUCT.md](PRODUCT.md), [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), and [docs/ROADMAP.md](docs/ROADMAP.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Contributions should preserve local-first behavior, progressive disclosure, keyboard access, and the distinction between an implemented feature and a roadmap item.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
