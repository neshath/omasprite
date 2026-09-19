# Verification

Use `cargo fmt --check`, `cargo test --locked`, and `cargo build --locked`.

For limited disk space use `CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1` with Cargo. This reduces generated artifacts without changing game logic.

## Milestone 1 coverage

- Create/edit/save/reopen through the actual filesystem and JSON format.
- Reject creation over existing projects.
- Invalid scene save preserves the readable project.
- Interrupted manifest save preserves the previous scene.
- Unknown versions and path traversal fail before opening.

Unit tests do not prove visual quality or mouse/keyboard usability. Record a separate live acceptance result for New → paint → Save → close → Open before calling the milestone fully verified.

Live check on macOS: application launched with accessible controls and a visible World canvas. The first New-project attempt failed because native text-entry automation did not update the relative destination field and the launch working directory was read-only. Default destination was changed to the user's Documents directory. Full create/paint/reopen UI acceptance remains pending; the storage tests cover persistence independently.

## Remaining acceptance

The current automated acceptance covers sprite editing primitives, animated frames, authored collision, dialogue advance, scene transitions, objective completion, progress restore, and the original three-map RPG route. Native mouse/keyboard automation for New → paint → Save → close → Open remains environment-dependent and was not counted as passed. Camera follow, multiple NPC entities, branching dialogue, arbitrary image import, and standalone export remain roadmap work.
