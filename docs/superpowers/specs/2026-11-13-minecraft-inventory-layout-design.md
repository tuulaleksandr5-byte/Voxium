# Minecraft-style inventory layout for Voxium (2026-11-13)

## Goal and scope
Rebuild Voxium's **interactive inventory and functional-block windows** to use Minecraft Java's familiar spatial organization while retaining the Rust/Bevy inventory mechanics and the optional resource-pack backgrounds introduced in PR #4.

## Alternatives
- **Chosen: fixed pixel-accurate screen coordinates with a 3× UI scale**, layered over optional provided resource-pack GUI images. This matches Minecraft's 176×166 and 176×222 GUI canvases and preserves UI hit targets.
- A responsive two-column grid like the current game is easier to lay out but does not look like Minecraft.
- A fully new animated 3D UI and player render requires major renderer changes and is outside this request.

## Screens
Logical coordinates are at Minecraft's 176-pixel GUI width and multiplied by 3 in Bevy.
- Personal inventory 176×166: armor 4×1 at x=8/y=8,26,44,62; player-preview area at x=26/y=8; offhand x=77/y=62; 2×2 crafting at x=98,116/y=18,36; output x=154/y=28; 27 backpack slots starting x=8/y=84; nine hotbar slots x=8/y=142.
- Workbench 176×166: 3×3 crafting x=30/y=17,35,53; output x=124/y=35; backpack and hotbar at same bottom positions. Personal armor/crafting are not clickable while a workbench GUI is active.
- Furnace 176×166: input x=56/y=17, fuel x=56/y=53, output x=116/y=35; an unobtrusive progress indicator; backpack and hotbar at bottom.
- Chest 176×222 to align with `generic_54` texture: first three storage rows at x=8/y=18,36,54, backpack at y=140, hotbar at y=198. The pack background may display decorative slots for more storage, but only Voxium's existing 27 slots accept items.
- Creative: keep item catalogue and selectable grant target as a scrollable section below the main Minecraft-like canvas (not a promise of full Minecraft creative tabs).

## Visuals and behavior
- Square bevelled slot targets, pixel-art icons at 16×16 logical pixels, tooltip, cursor, shift-click and drag mechanics unchanged.
- The resource-pack GUI image is placed *behind* the interaction slots and marked non-pickable. When no pack is present, a procedural pixel/bevel frame is shown.
- A stylized static Voxium avatar placeholder fills the preview panel; no false promise of a rendered player model.
- Inventory remains usable at 1280×720, with scrolling when the chest or creative catalogue exceeds the viewport.
- Existing gameplay/save formats untouched.
- Implement on a new branch based on `feature/minecraft-optifine-resourcepacks`, no merge to main.

## Acceptance
- Querying the created Bevy entities shows correct counts, slot x/y positions, one result target and no unexpected slots for each screen.
- Existing input/crafting/catalogue tests continue to pass (updated expected chest count to reflect Minecraft's station-specific layout).
- Rust CI: fmt, check, clippy -D warnings, tests; Windows release build in actions.
