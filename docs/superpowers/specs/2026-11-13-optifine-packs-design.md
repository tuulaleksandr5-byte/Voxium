# Voxium ZIP resource packs and OptiFine GUI (incremental design, 2026-11-13)

## Intent
Load existing Minecraft Java resource packs, including the supplied Colourful Containers Dark Mode 1.18 pack, without changing Voxium save files or abandoning existing 128×128 texture arrays.

## Considered approaches
1. **Translate resource-pack images at load time into Voxium's existing atlas and item icon assets (chosen).** Small rendering change and immediate block/item compatibility; does not automatically implement OptiFine's mesh/shader feature set.
2. Replace the entire renderer with a Minecraft-compatible model/texture system. High risk, would block current gameplay and need model variants, state mapping and shader pipeline.
3. Import ZIP externally into a custom Voxium directory. Simpler at runtime, but not direct ZIP support and not user-friendly.

## First deliverable
- Discover one active `assets/resourcepacks/active.zip` alongside EXE or current working directory; allow explicit `VOXIUM_RESOURCE_PACK` path.
- Read ZIP entries in-memory using the Rust zip crate; never extract arbitrary files; validate total/file limits.
- Resolve `assets/minecraft/textures/block/*.png` and `textures/item/*.png` (including legacy `blocks`/`items` names), with documented mappings for the supported Voxium blocks and items.
- Accept 16, 32, 64, 128, 256 or 512 px square textures, nearest-neighbour resize to internal 128×128, respecting source alpha. Invalid/missing files leave the original texture intact. Items and block icons both reflect the pack.
- Read basic unconditioned OptiFine `optifine/gui/container/**/*.properties` overrides (`container`, `texture`) and default Minecraft container PNGs. Present the relevant image within Voxium inventory/workbench/furnace/chest panels while retaining the functional buttons and slots.
- Load at startup, without hot-reload. Do not silently reinterpret OptiFine conditional rules.

## Not in this increment
CTM geometry and neighbour selection, CEM custom entity models, CIT predicate matching, animated textures, shader packs, PBR normal/specular/emissive maps, blockstate/model JSON geometry, full pixel-perfect Minecraft GUI layout. Those are separate renderer and gameplay stages.

## Quality
Test ZIP reading, limits, absent/broken resources, variable resolutions, mappings, and OptiFine GUI source selection. Run Linux Rust CI and Windows x64 build in GitHub Actions. Keep staged PR based on functional-stations branch; do not merge directly into main.
