# Accessibility and keyboard audit

The editor uses native egui accessible controls for buttons, checkboxes, sliders, and text fields. Keyboard shortcuts are visible in the `Keyboard & accessibility` inspector section:

- Space: Play or Stop.
- Escape: Stop play.
- Arrow keys: move in play mode.
- Enter: interact / advance dialogue.
- 1–9: select an available dialogue choice.

Manual screen-reader and full tab-order verification on an Omarchy Linux desktop is still required before a release. This document deliberately records that as a release gate rather than claiming completion from automated tests alone.
