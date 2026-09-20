# Workstation Verification

Date: 2026-09-20

Branch: `codex/picotron-workstation`

Base commit: `4ff96cb refactor: make editor shell radically more apple-like`

## Automated checks

```text
27 unit tests passed
1 progression test passed
cargo build --locked passed
git diff --check passed
```

The remaining compiler warnings are the existing staged advanced contracts for layers, scene hooks, shader graphs, plugin discovery, and PNG helper methods. No new warning was introduced by the workstation shell.

## Native visual check

The isolated bundle `work/Omasprite-Picotron.app` opened successfully with bundle id `dev.neshath.omasprite.picotron` and displayed:

- HOME as the default surface
- NEW GAME and CONTINUE actions
- GFX, MAP, LOGIC, CHARACTER, and FX workspaces
- Simple and Advanced mode controls
- an opt-in Tools tray
- a persistent Play action
- the existing Snow Village status and runtime controls

The first automated click attempt closed the temporary native window before a second state snapshot could be collected. This is recorded as an automation limitation, not as a claim that the click path was visually re-verified. The New Game operation is a direct `World::default()` replacement and the underlying world/project behavior remains covered by the existing scene and runtime tests.

## Isolation check

The original checkout remained on `main` at `4ff96cb` with a clean working tree. All workstation changes are isolated in:

```text
/Users/tinkerspace/Documents/Codex/2026-09-16/omasprite-picotron-workstation
```
