# OMASPRITE Picotron Workstation Implementation Plan

> **For agentic workers:** implement this plan task-by-task, in order, checking off each step as it lands. Steps use checkbox (`- [ ]`) syntax for tracking. This plan is isolated on `codex/picotron-workstation`; the existing `main` checkout remains untouched.

**Goal:** Turn OMASPRITE into a simple, child-friendly fantasy workstation with Picotron’s immediate cartridge/workspace mental model while preserving the existing sprite, map, dialogue, lighting, runtime, and advanced feature contracts.

**Architecture:** Keep the current Rust/egui engine and project/runtime data contracts. Replace the always-visible professional editor chrome with a small workstation shell: Home, GFX, MAP, CODE/LOGIC, CHARACTER, FX, and PLAY. Advanced controls remain available behind a fold-out tray and creator-level disclosure rather than being removed.

**Tech Stack:** Rust 2021, eframe 0.29, egui 0.29, existing `ProjectStore`, `World`, `Sprite` editor, runtime, theme tokens, and native bundle.

---

### Task 1: Record the workstation contract

**Files:**
- Create: `docs/WORKSTATION.md`
- Modify: `README.md:1-120`

- [x] **Step 1: Write the product contract**

Document the default child workflow exactly:

```text
New Game → choose a starter → Paint → Place → Talk → Play
```

Document the Picotron-like surface model:

```text
HOME  GFX  MAP  LOGIC  CHARACTER  FX  PLAY
```

Document that the existing runtime systems remain available through Advanced mode and that OMASPRITE is inspired by Picotron without using Picotron source code, assets, branding, or cartridge format.

- [x] **Step 2: Update README claims**

Replace the current “left tool rail” description with the workstation model. Keep the existing implementation limits honest: the current beta remains a 16×16 authored scene and the advanced systems are still staged where the code says so.

- [x] **Step 3: Verify documentation references real code**

Run:

```sh
rg -n "Workspace|ProjectStore|World|Sprite|Logic|Effects|PLAY" src docs/WORKSTATION.md README.md
```

Expected: every named surface maps to an existing module or an explicitly labelled planned surface.

- [x] **Step 4: Commit**

```sh
git add docs/WORKSTATION.md README.md
git commit -m "docs: define picotron workstation direction"
```

### Task 2: Add a simple workstation state model

**Files:**
- Modify: `src/model.rs:1-90`
- Modify: `src/app.rs:1-100`
- Test: `src/model.rs` unit tests

- [x] **Step 1: Add the Home surface and display modes**

Extend the model with:

```rust
pub enum Workspace {
    Home,
    Sprite,
    World,
    Logic,
    Character,
    Effects,
}

pub enum CreatorMode {
    Simple,
    Advanced,
}
```

`Workspace::Home` is the default. `CreatorMode::Simple` is the default. Keep existing workspace variants and data behavior intact.

- [x] **Step 2: Add state to `StudioApp`**

Add:

```rust
pub mode: CreatorMode,
pub tray_open: bool,
pub recent_project: String,
```

Initialize `mode` to `Simple`, `tray_open` to `false`, and `recent_project` to the current project name. Do not alter `World`, scene serialization, or runtime behavior.

- [x] **Step 3: Add model tests**

Test that Home is the default display workspace and that each workspace has a short user-facing label: `HOME`, `GFX`, `MAP`, `LOGIC`, `CHARACTER`, and `FX`.

- [x] **Step 4: Run tests**

```sh
CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo test --locked
```

Expected: all existing tests plus the new model tests pass.

- [x] **Step 5: Commit**

```sh
git add src/model.rs src/app.rs
git commit -m "feat: add simple workstation state"
```

### Task 3: Replace the persistent editor chrome with a workstation shell

**Files:**
- Modify: `src/app.rs:100-760`
- Modify: `src/ui.rs:1-300`
- Modify: `src/theme.rs:1-90`

- [x] **Step 1: Build the Home screen**

Render a single friendly landing surface with four primary actions:

```text
NEW GAME       OPEN GAME
CONTINUE       PLAY DEMO
```

The first action creates/opens the existing project flow; it must not fabricate saved user data. The empty state explains “Start with a scene, a character, and a Play button.”

- [x] **Step 2: Build the top workstation bar**

Replace the current always-visible Apple-style three-pane shell with a single compact bar containing:

```text
OMASPRITE   [HOME] [GFX] [MAP] [LOGIC] [CHARACTER] [FX]   PLAY  SAVE  ☰
```

Use text labels with keyboard mnemonics, not missing-font glyphs. `PLAY` must continue to call `World::start`; `SAVE` must continue to call `World::save`.

- [x] **Step 3: Add the fold-out tool tray**

Keep the existing tools, palette, layers, asset shelf, creator progress, and accessibility controls inside a tray opened by `☰ Tools`. The tray is closed by default in Simple mode and open by default in Advanced mode. No existing tool action is deleted.

- [x] **Step 4: Add progressive disclosure**

Simple mode shows only:

```text
Pencil  Eraser  Fill  Undo  Play
```

Advanced mode reveals the existing stamp, path/collision, lighting, shadow, particle, layers, dialogue, and project controls. Existing workspace content remains reachable through the same `World`, `Sprite`, `Logic`, `Character`, and `Effects` methods.

- [x] **Step 5: Add a compact bottom status line**

Show only the current project, unsaved state, and one keyboard hint. Remove duplicate project labels and decorative slogans.

- [x] **Step 6: Run formatting and build**

```sh
cargo fmt
CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo build --locked
```

Expected: the workstation shell compiles without changing scene/runtime serialization.

- [x] **Step 7: Commit**

```sh
git add src/app.rs src/ui.rs src/theme.rs
git commit -m "feat: add picotron-style workstation shell"
```

### Task 4: Verify the child workflow and preserve the existing version

**Files:**
- Modify: `docs/WORKSTATION.md`
- Create: `docs/WORKSTATION-VERIFICATION.md`

- [x] **Step 1: Run the full test suite**

```sh
CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo test --locked
```

Expected: 26 existing unit tests, the progression test, and the new model tests all pass.

- [x] **Step 2: Build and launch the isolated bundle**

```sh
CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo build --locked
cp target/debug/omarchy-studio work/Omasprite-Picotron.app/Contents/MacOS/omasprite
codesign --force --sign - work/Omasprite-Picotron.app
codesign --verify --deep --strict work/Omasprite-Picotron.app
open work/Omasprite-Picotron.app
```

- [x] **Step 3: Verify the workflow visually**

Confirm in the native window:

```text
HOME opens first
NEW GAME is visible without opening advanced panels
GFX and MAP remain one-click away
PLAY starts the existing Snow Village runtime
Advanced mode exposes the existing controls
```

- [x] **Step 4: Verify the original checkout is untouched**

Run from the original checkout:

```sh
git -C ../referenced-chatgpt-conversation-this-is-an status --short
git -C ../referenced-chatgpt-conversation-this-is-an branch --show-current
```

Expected: clean `main` with commit `4ff96cb`; all workstation changes exist only in `../omasprite-picotron-workstation` on `codex/picotron-workstation`.

- [x] **Step 5: Commit verification**

```sh
git add docs/WORKSTATION.md docs/WORKSTATION-VERIFICATION.md
git commit -m "docs: verify workstation workflow"
```

---

## Self-review

- Existing Apple-style `main` remains untouched because all implementation work is isolated in a separate worktree and branch.
- Existing scene, runtime, sprite, dialogue, lighting, particle, Lua, plugin, and project systems remain the implementation foundation.
- No Picotron source code, assets, branding, or proprietary cartridge format is copied.
- The first slice does not pretend to deliver a complete export pipeline; it delivers the simpler workstation shell and verifies the existing playable loop through it.
