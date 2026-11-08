# Clean HUD, exact atlas and block audio implementation plan

**Goal:** Keep only crosshair/hotbar on the default HUD; use the user's unchanged PNG for world surfaces and cached cube icons; add positional block sounds.

**Architecture:** A validated external/embedded `assets/atlas_128x128.png` is the sole production block-art source. Decode once for render materials and icon baking; retain original file bytes. No procedural texture fallback. Missing/invalid atlas keeps menus available and prevents opening a world with substitute art. CPU icon rasterization bakes 16 reusable RGBA icons from the same tile faces. BlockSound messages decouple successful edits/grounded movement from cached spatial playback. Offline generated original WAV files provide three variants per block/event and optional disk overrides.

**Constraints:** Bevy0.20.1, stable block IDs/saves and current controls. 128px tiles, 6×3 row-major. Hotbar nine slots, purple translucent panel, brighter selected border; icon face factors 1/.85/.70. Steps at0.4s walking,0.25s running, only while grounded and actually moving, none during flight/pause. Pitch .95…1.05. Missing sounds warn and never panic.

- [x] Remove permanent corner panels; F3 diagnostic panel hidden by default.
- [x] Remove procedural textures, implement required original PNG loading and user-facing missing-asset error.
- [x] Bake/test reusable atlas cube icons and replace flat hotbar swatches.
- [x] Generate saved WAV variants, implement cached positional audio with missing-file handling.
- [x] Hook successful break/place and movement step events, preserving collisions and sprint controls.
- [x] Verify native/Windows builds, tests, graphical smoke and offline sound assets; publish source.
- [x] Integrate and compare exact original PNG: awaiting a ZIP upload containing the image, which is currently only visible inline in chat.

Resolved9 November: the user supplied17 cut PNG files in textures_cut.zip. They are loaded directly with unchanged source bytes; a combined PNG is unnecessary. See ORIGINAL_TEXTURES.md.
