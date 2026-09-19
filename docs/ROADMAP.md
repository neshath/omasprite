# Roadmap

## Engineering milestones (acceptance takes precedence)

- M0: repository audit and reference technical specification.
- M1: project folders, validated save/open and persistence tests; live workflow acceptance pending.
- M2–4: pixel renderer, camera, independent collision, smooth player/animation.
- M5–7: generic entities, multi-page dialogue, linked scenes.
- M8–10: sprite editing, animation, scene inspector and isolated play controls.
- M11–12: original village/path/interior game, objective, game saves and live acceptance.

The historical checkboxes below describe preview panels, not completion of these engineering milestones.

## Game Editor Beta — current

- [x] Native editor shell and project status.
- [x] Adaptive creator level and unlock path.
- [x] Sprite Lab / World Builder / Visual Logic workspace foundation.
- [x] Canvas preview, playback, onion skin, zoom, tools, layers, palette, assets.
- [x] Top-down map preview for overworld, town, and interior workflows.
- [x] Dialogue box authoring/preview for story-driven scenes.
- [x] Character Workshop and Lighting & FX foundations.
- [x] Persist a real project manifest and scene/sprite undo/redo history.

## Beta 2 — make the loop real

- [x] Pixel buffer editing with file-backed PNG import/export.
- Character direction sets, animation clips, portraits, and palette variants.
- Layer compositing and animation frame data.
- [x] Tile stamps, map chunks, collision painting, and camera preview primitives.
- Dialogue choices, portraits, conditions, and scene event hooks.
- Data contracts now exist in `src/advanced.rs` for choices, conditions, portraits, and hooks; egui authoring/runtime wiring remains.
- [x] Play mode with a tiny deterministic 2D runtime.

## Beta 3 — mood and logic

- Sprite-based point lights, palette-aware shadows, and day/night presets.
- Particle/weather emitters and GPU-backed shader graph foundations.
- Visual logic nodes: event, condition, action, signal, and playtest.
- Human-readable Lua export for advanced creators.
- Data-only visual nodes, particle emitters, shader graph source contracts, reviewable Lua export, and capability-checked plugin manifests now exist in `src/advanced.rs`; execution and sandbox hosting remain.

## Proposal-ready direction

- Plugin SDK with capability permissions and versioned APIs.
- Omarchy package/distribution integration.
- Accessibility audit, screen-reader labels where supported, and full keyboard workflow.
- Example projects that are real, editable, and clearly labeled as examples.
