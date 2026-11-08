# Controls, Russian UI and visual fixes

User requests fullscreen settings stability, correct grass, flight/sprint/wheel, Russian, F3 coordinates/FPS, F3+G chunk boundaries. Existing authorized workflow continues through tests, Windows build and GitHub package update.

- Reproduced failures: settings overwrite fullscreen monitor size with saved window size; grass top/bottom sample repeating side stripes.
- Fix window settings only when values differ, never resize fullscreen surface. Windowed resolution is applied after leaving fullscreen.
- Keep grass side UV upright; horizontal faces sample turf/soil respectively.
- Double Space toggles collision-respecting creative flight; Space/Ctrl ascend/descend, either Shift accelerates, scroll cycles nine slots with wrap. Freeze motion in missing chunks.
- F3 release toggles debug; consuming F3+G suppresses F3 debug toggle. Boundary grid is a bounded neighborhood of real 16-wide chunks/sections, with correct negative coordinates.
- Russian default menus/HUD/materials/messages; embed Cyrillic DejaVu font and license. Keep saves backward compatible.
- Verify regression tests, build, Clippy, native/Wine GUI and publish updated source/package to existing authorized GitHub branches. Native Windows validation remains outstanding.
