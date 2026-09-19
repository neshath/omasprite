# Runtime status

The existing runtime supports discrete arrow-key movement, terrain-derived collision, one NPC, one dialogue line and height-step checks. `--play` currently opens a loose scene file. It still uses the world module shared with editor rendering.

Next boundary: move simulation state, movement, interactions, scene transitions and game save data into a graphics-independent module. The renderer consumes that state; editor play starts a copy. Only then implement map switching and progress saves. Camera follow, animated assets and objective events remain acceptance work.
