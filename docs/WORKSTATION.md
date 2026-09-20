# OMASPRITE Workstation

OMASPRITE is moving toward a fantasy-workstation model: a small, friendly surface for making games, with a deeper editor underneath for creators who want more control.

The project is inspired by the *workflow* of Picotron, not its source code, artwork, branding, or cartridge format. OMASPRITE remains an independent GPL-3.0 Rust application with its own project and scene formats.

## The first five minutes

```text
HOME → NEW GAME → MAP → place things → PLAY
                    ↓
                  TALK
```

A child should be able to start with the Snow Village scene, paint a map, place a character or NPC, write one line of dialogue, and press Play without learning the entire editor.

## The workstation surface

The default shell has only these ideas:

```text
HOME   GFX   MAP   LOGIC   CHARACTER   FX       PLAY
```

- `HOME` starts or continues a game.
- `GFX` is the sprite and animation workspace.
- `MAP` is the tile, object, collision, and scene workspace.
- `LOGIC` is the dialogue/event foundation.
- `CHARACTER` is the directional animation workspace.
- `FX` contains lighting, shadows, weather, and particles.
- `PLAY` immediately starts the current game.

The tool tray is closed in Simple mode. It contains the deeper controls already implemented in the beta: palettes, layers, stamps, collision, lighting, assets, creator progress, project settings, and accessibility information.

## Simple and Advanced modes

Simple mode exposes the next useful action:

```text
Pencil   Fill   Eraser   Undo   Play
```

Advanced mode exposes the existing authoring systems without changing their data contracts:

- multi-layer scenes
- collision and map objects
- sprite frames and onion skin
- dialogue and event hooks
- directional character sets
- lighting, shadows, particles, and weather
- Lua export and plugin validation contracts
- project save/load and runtime debug information

The principle is progressive disclosure, not feature removal. A creator can grow from Simple mode into Advanced mode without moving to another application.

## Cartridge-style project model

The current project folders and validated JSON scenes remain the storage foundation. The workstation presents them as one game project rather than as a collection of technical files. Future packaging can add a single `.oma` bundle around the existing project folder without forcing the author to understand the bundle internals.

## Design rules

1. The Play button is always easy to find.
2. New users see one meaningful decision at a time.
3. Advanced controls are hidden, never removed.
4. Every important action has a keyboard path.
5. A project must be saved as real data; no fake recent-project state is presented as a saved game.
6. The editor and runtime share the same scene data so previews are trustworthy.

## Current boundary

The isolated workstation branch adds the simple Home surface, short workspace names, Simple/Advanced mode, and an opt-in tool tray. The existing sprite, world, dialogue, lighting, particle, runtime, Lua, plugin, and project systems remain the implementation foundation. The branch does not claim that every advanced contract is fully authored through the UI yet.
