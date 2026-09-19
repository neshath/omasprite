# Plugin boundary

OMASPRITE discovers `plugins/<id>/plugin.json` inside a project. A manifest has `id`, `version`, `entry`, and a list of capability names. The current host only admits declarative, non-privileged capabilities such as `palette` or `tiles`.

`filesystem` and `network` capabilities are rejected. Plugins are **not executed** by the editor yet; this is intentional until a subprocess or WASM sandbox with explicit user approval is introduced.
