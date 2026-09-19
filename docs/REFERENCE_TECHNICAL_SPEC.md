# Reference technical spec

Source: the two user-supplied snowy RPG screenshots. They are references only; no characters, names, dialogue, buildings or maps are to be copied.

| Area | Observed | Inferred (not established by a still image) | Required for Omasprite |
|---|---|---|---|
| View | Roofs, ground and front-facing walls visible together | Elevated orthographic or perspective camera; second image may mix 3D scenery and sprites | Layered top-down rendering first; optional height/depth support; no full 3D dependency yet |
| Scale | Characters roughly one path/fence module wide; trees taller and wider | 16-pixel ground modules and approximately 16–32-pixel characters are plausible; image scaling prevents exact measurement | Configurable tile and sprite dimensions; start with 16×16 tiles and 16×24 character frames |
| Framing | First image is square; second is 400×240 with dialogue covering the upper area | Screenshots may be cropped; native game resolution cannot be established | Logical game resolution independent of the window; choose 320×180 initially, configurable |
| Layering | Characters overlap ground; tree crowns and roofs extend above footprints | Depth sorting uses feet/base position or layers | Background, terrain, objects, characters, foreground, UI; sort actors by foot Y |
| Collision | Fences, trees, walls and paths have clear spatial roles | Actual walkability and collision shapes are not visible | Separate collision data and an editor overlay; boundaries and NPC occupancy |
| Movement | Stationary character poses | Frame count, speed, smoothness and directions are unknown | Four directions with idle/walk state, timed animation and deterministic movement |
| Dialogue | Rounded light box, dark border, wrapped black text | Advance controls, branching and typewriter timing are unknown | Multi-page keyboard dialogue; speaker data, choices and event integration later |
| Environments | Snow, paths, trees, fence modules, building fronts, entrances, shading | Shadows may be baked; dynamic lighting is not demonstrated | Original tileset, multi-tile objects, transparency and authored shading; dynamic lighting optional |
| Maps | Entrances and paths suggest connected places | Neither screenshot proves transitions | Explicit destination map/spawn references; village, path, interior |
| Game state | No objective or save UI visible | Quests, inventory, persistence cannot be inferred | Add a small original objective and game save/load as requested separately |

## First proof

From a blank project: author terrain and collision, place player/NPC, author dialogue, add a linked map, play, interact, cross maps, save, close and restore progress. Images alone cannot prove this workflow; validate using the running application and persisted data.
