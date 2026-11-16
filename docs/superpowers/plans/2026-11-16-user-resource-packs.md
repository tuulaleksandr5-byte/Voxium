# User Resource Packs Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans to implement task-by-task.

**Goal:** Apply the user's nine available packs to Voxium assets and menus.
**Architecture:** Offline conversion into embedded assets; native Bevy rendering and UI.
**Tech Stack:** Rust 1.99.1, Bevy 0.20.1; Python/Pillow/fontTools for asset conversion.
**Spec:** ../specs/2026-11-16-user-resource-packs-design.md

## Global Constraints

- Preserve game rules, block/item IDs, save format, and dependency pins.
- Never claim access to the two oversized base archives.
- Preserve PNG source provenance, copy only requested resources, never execute archive content.
- Keep existing art as fallback when a supplied pack has no matching texture.

## Review Focus

- Transparent skin regions must not expose every breed's geometry simultaneously.
- Rig hierarchy must survive conversion and native animation must keep rest rotations.
- Faithful aliases must not change block face layer numbers or erase unmatched tiles.
- GUI textures must not move click targets away from visible item slots.
- The bitmap-derived font must contain Cyrillic and the main menu must fit a small window.

## Task 1: Convert supplied assets

Files: tools/import_resource_packs.py, tools/test_import_resource_packs.py,
assets/resource_packs/, apps/client/src/pack_assets.rs.
Produces: embedded model JSON, PNGs, block atlas, GUI sprites, font and provenance manifest.
- [x] Add and run failing conversion tests (alpha, transforms, hierarchy, aliases).
- [x] Implement safe selective ZIP reads and deterministic conversion.
- [x] Import supplied packs and inspect the manifest and preview sheets.

## Task 2: Native rendering integration

Files: apps/client/src/mob_packs.rs, mobs_client.rs, item_icons.rs, graphics_assets.rs;
crates/game_render/src/atlas.rs.
Consumes Task 1 embedded assets; produces native mob variants and Faithful world/icons.
- [x] Integrate custom model hierarchy, materials, rest rotations and deterministic variants.
- [x] Give Spawned Eggs priority in item icon loading.
- [x] Add full-atlas loading with size checks; select Faithful for pixel style.
- [x] Run focused checks and source/resource consistency tests.

## Task 3: Native GUI and menu

Files: apps/client/src/ui.rs, ui/presentation.rs, ui/inventory.rs.
Consumes embedded Icons, Colourful textures and bitmap-derived font.
- [x] Apply readable native containers using pack artwork without duplicating baked slots.
- [x] Update menu branding, actions and icons; retain interaction and layout contracts.
- [x] Run UI tests and inspect the live menu and representative mobs/containers.

## Task 4: Verify and package

- [x] Run formatting, workspace check, clippy and tests; record actual outcomes.
- [x] Review the whole diff and fix material issues.
- [x] Document imported coverage and limitations; prepare a reviewable Git branch and pull request.
