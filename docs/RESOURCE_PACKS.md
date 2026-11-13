# Minecraft / OptiFine resource packs (first-stage compatibility)

The Windows build can load one Minecraft Java resource pack directly from a ZIP at game startup.

## Install

1. Keep the ZIP **unextracted**.
2. Next to `Voxium.exe`, create the folder `assets/resourcepacks/`.
3. Copy the resource pack there and rename it to `active.zip` (result: `assets/resourcepacks/active.zip`).
4. Launch Voxium. Restart to apply a different pack.

Alternatively set the `VOXIUM_RESOURCE_PACK` environment variable to the absolute path of your ZIP.

The ZIP must contain `assets/minecraft/` at its root. PNGs not present in the pack leave the built-in Voxium art unchanged. A corrupt ZIP also preserves the built-in art. This release never extracts entries onto disk.

## What is implemented

- Minecraft Java block textures in `assets/minecraft/textures/block/*.png` (or legacy `textures/blocks`) mapped to Voxium's existing 32 atlas slots. Examples: `grass_block_top`, `oak_planks`, `stone`, `iron_ore`, `crafting_table_front`.
- Material and tool icons in `assets/minecraft/textures/item/*.png` (or legacy `textures/items`) are loaded before Voxium's embedded icon fallback.
- 16×16, 32×32, 64×64, 128×128, 256×256 and 512×512 square PNGs are converted to the renderer's fixed 128×128 atlas and icon slots using nearest-neighbour scaling; source ZIP entries are untouched. Alpha is preserved.
- Minecraft inventory, crafting table, furnace and chest GUI images from `textures/gui/container/` and simple unconditional OptiFine `optifine/gui/container/**/*.properties` can be previewed in the matching Voxium panel without replacing its interactive slots. The uploaded Colourful Containers Dark Mode 1.18 pack contains these GUI assets.
- Invalid images/unknown entries are skipped without changing the remaining textures. The total uncompressed size of interesting files is capped at 256 MiB.

## What is **not** supported (yet)

Do not interpret "OptiFine resource pack" as full OptiFine compatibility. This loader does **not** implement:
- Connected Textures (CTM), random/animated block textures or neighbour-dependent render logic
- Custom Item Textures rules (CIT), custom entity models (CEM), or JSON blockstate/3D model geometry
- PBR normal, specular or emissive maps, OptiFine/Iris shader packs or their shaders
- Conditional custom GUI rules tied to NBT, names, biomes, professions, etc.
- Minecraft's exact inventory slot layout or font replacements

The OptiFine custom GUI parser applies only rules without unsupported conditions; other rules are ignored safely. Block geometry still uses Voxium's voxel meshes. Full visual interoperability requires later renderer stages.

## Validation

Tests in `apps/client/src/resource_packs.rs` cover ZIP parsing, PNG mapping, scaling including alpha, basic OptiFine GUI rule selection, skipping conditional rules, invalid input and unsafe paths. CI checks format, Clippy, tests and native Windows builds.
