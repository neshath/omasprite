# Contributing

Thanks for helping build a welcoming pixel game editor.

## Before opening a change

1. Read [PRODUCT.md](PRODUCT.md) and keep the adaptive creator experience central.
2. Run `cargo fmt --check` and `cargo test`.
3. For UI work, include a short screenshot or screen recording when practical.
4. Say what is implemented, what is stubbed, and what remains on the roadmap.

## Good first contributions

- Add keyboard shortcuts and visible focus states.
- Extract project progression data into serializable formats.
- Implement a small pixel-buffer command with undo/redo.
- Add a real sample project under `examples/`.

## Scope

Prefer small vertical slices over speculative framework work. Keep the application local-first and avoid adding network services to the editor core.
