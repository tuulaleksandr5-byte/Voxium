# Visual refresh and revised controls

User supplied explicit visual references and requested continued implementation/release workflow. Keep native Rust/Bevy, Russian UI, saves and block IDs.

- Menu: embedded original voxel lake/cabin sunset illustration, dark teal translucent cards, amber primary actions, mint focus borders. Left-hand main menu, split create form/decorative image, world cards, grouped settings with actual draggable sliders. Retain keyboard form input and scrolling.
- Textures: 64×64 original repeatable pixel materials, stones/soil/wood/leaves/water/sand inspired by reference. Separate render material slots for grass top, soil bottom and log ends; persistent block IDs stay unchanged. Full 2D repeating grass UVs, eliminating stretched stripes.
- Controls: Space/Shift ascend/descend; Ctrl sprint/flight boost; ground Shift crouch. Double W latches sprint while W remains held and releases on W-up. Double Space still toggles flight; F3/F3+G and wheel remain.
- Verify regressions and native GUI, Windows client tests and Wine menus/controls. Refresh downloadable GitHub ZIP with checksum and report actual limits.
