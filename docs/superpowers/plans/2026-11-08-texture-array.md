# Texture array and painted lighting implementation plan

**Goal:** Use 128px painted tiles with repeatable greedy UVs, isolated mipmaps, configurable face mapping, corner AO and soft outdoor lighting.

**Architecture:** `game_render` owns the validated RON face table, atlas slicing and extended PBR material. Meshes contain tile-local repeating UVs and vertex AO. The client embeds shaders and defaults, accepts an external atlas, and owns sun, fog and gradient sky.

**Constraints:** Stable block IDs and existing saves. 6×3 atlas, 128×128 tiles; all filters linear, anisotropy 8. Preserve current menu and flight controls. The requested PNG is absent from this checkout; request the original file and explicitly identify the temporary fallback.

**Review focus:** Layer/mip upload ordering; rotated or upside-down side textures; transparent glass without alpha in the source; section corner AO changes after border edits; runtime shader compilation and standalone Windows packaging.

- [x] Test RON mappings/range errors, tile extraction and per-layer mip isolation.
- [x] Implement array material, embedded shader and validated optional PNG override.
- [x] Test and implement AO-aware greedy merging and repeating UVs.
- [x] Add gradient sky, filtered directional shadows and smooth fog.
- [x] Run workspace checks and native graphical smoke test.
- [x] Build/test Windows executable, document the missing original PNG, publish source and verified ZIP.

- [ ] Connect the exact original user PNG and compare visual fidelity: blocked on receiving `atlas_128x128.png` as a file. The fallback is explicitly documented in the build and download instructions.
