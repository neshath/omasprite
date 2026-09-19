# Runtime status

The runtime supports discrete arrow-key movement, an authored collision layer, terrain-derived collision, one NPC, multi-page dialogue, map portals, objective completion, and project game saves. `--play` opens a loose scene file or a project folder. The first fixture is a three-map original snowy RPG.

Next boundary: move simulation state, movement, interactions, scene transitions and game save data into a graphics-independent module. The renderer consumes that state; editor play starts a copy. Only then implement map switching and progress saves. Camera follow, animated assets and objective events remain acceptance work.
