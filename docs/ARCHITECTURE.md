# Architecture

OMARCHY Studio is organized around a native editor shell with replaceable domain modules. The beta keeps the first slice in one binary so it is easy to build and review; the boundaries are explicit before the codebase grows.

```text
src/main.rs
  └── app.rs          native lifecycle and workspace composition
      ├── model.rs    project, tool, workspace, progression data
      ├── ui.rs       reusable editor panels
      └── theme.rs    shared visual tokens and egui styling
```

## Planned modules

- `editor/sprite`: pixel buffers, layers, palettes, frame data, onion skin.
- `editor/tilemap`: chunked maps, stamps, collision, isometric/2.5D projections.
- `engine/renderer`: wgpu scene renderer, pixel lighting, shadows, camera.
- `engine/runtime`: play mode, input, ECS-facing scene data, deterministic preview.
- `editor/logic`: serializable visual graph with a Lua export boundary.
- `editor/character`: direction sets, animation clips, portraits, equipment layers, and palette variants.
- `engine/lighting`: palette-aware point lights, shadow masks, ambient/day-night state, and effect emitters.
- `editor/dialogue`: speaker/portrait lines, choices, conditions, and event hooks.
- `editor/assets`: content-addressed local asset index and importers.
- `plugins`: versioned capability manifest and sandboxed extension API.

## Data boundary

Projects will use a human-readable manifest plus small binary payloads:

```text
my-game/
  omarcy.toml
  assets/
    sprites/
    tiles/
    audio/
  scenes/
  scripts/
```

The editor must never require a cloud account to create, save, or play a project.

## Adaptive UI contract

Every advanced capability declares the minimum creator level and the workspace(s) where it appears. Unlocking changes discoverability, not file compatibility: opening a higher-level project should explain what is hidden and offer a safe path to unlock or inspect it.
