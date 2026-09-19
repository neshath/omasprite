# Project format v1

Milestone 1 introduces a project directory with `project.json`, `scenes/`, `tilesets/`, `sprites/`, `animations/`, `entities/`, `dialogues/`, `audio/`, `scripts/`, and `saves/`.

The manifest contains `version`, `name`, `entry_scene`, and a sorted `scenes` mapping of stable scene IDs to relative JSON paths. Asset directories are reserved; no asset importer is claimed yet.

Scene v1 stores 16×16 tiles/heights plus an independent 16×16 collision layer, one NPC position, spawn, dialogue string and ambient value. Tile IDs are 0 snow, 1 path, 2 water, 3 tree, 4 building. Older scene files without `collision` are migrated in memory as fully walkable.

Save writes a new immutable scene revision, then writes/syncs `project.json.pending`, then renames it over the manifest. The previous scene remains on disk. An interrupted manifest write leaves the prior project loadable. A stale pending file blocks further saves and is retained for inspection; the editor reports the error. There is no automatic revision pruning or multi-writer support.

New requires a nonexistent directory and an existing parent. Existing folders are never replaced. Open validates every scene and rejects unknown versions, path traversal and out-of-project symlinks before switching the active document. Future migrations must be explicit; unknown versions are not guessed.

Game progress saves are separate from project authoring saves and are not implemented in this milestone.
