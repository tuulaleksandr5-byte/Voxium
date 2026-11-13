# Resource Pack Loader Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement the plan task-by-task.

**Goal:** Voxium loads supported Minecraft Java/OptiFine resource-pack images directly from ZIP files.
**Architecture:** New client-side ZIP loader translates supported assets into the existing 128px atlas, icon cache and UI handles; game-core and saves remain unchanged.
**Tech Stack:** Rust 1.99.1; Bevy 0.20.1; zip (deflate), image 0.25.
**Spec:** docs/superpowers/specs/2026-11-13-optifine-packs-design.md

## Global Constraints
- Source branch: feature/functional-stations-dark-inventory (preserve stations).
- No filesystem extraction of third-party ZIPs.
- No false claims of complete OptiFine compatibility.
- Old original art always remains fallback.

## Review Focus
- Malformed ZIP and very large deflated entries cannot crash the game.
- Unexpected OptiFine conditions cannot incorrectly override default GUI assets.
- Textures of multiple resolutions produce exactly 128×128 RGBA.
- Existing local atlas overrides work when no resource pack is selected.
- Pure UI assets from Colourful Containers are displayed, not confused with block textures.

### Task 1: Test then create ZIP loader
- [ ] Write focused unit tests for key validation, ZIP stored/deflate, and 16/32/128 scaling.
- [ ] Run failing tests on GitHub CI.
- [ ] Implement `apps/client/src/resource_packs.rs`, ZIP indexing, safe extraction limits, nearest neighbour resizing.
- [ ] Re-run tests and commit.

### Task 2: Connect world and item textures
- [ ] Test atlas layer replacement and missing file fallback.
- [ ] Implement `game_render::BlockAtlas::replace_layer_rgba`.
- [ ] Call pack layer mapping inside `graphics_assets::load_atlas`; add resource-pack override before existing item placeholders.
- [ ] Run tests; commit.

### Task 3: OptiFine basic container GUI artwork
- [ ] Test default GUI and unconditioned OptiFine `.properties` override.
- [ ] Load and crop relevant GUI PNGs into Bevy image handles on startup.
- [ ] Insert image previews into existing functional GUI screens, without breaking pointer interactions.
- [ ] Run tests; commit.

### Task 4: Package/verification
- [ ] Update `Cargo.lock` in GitHub runner for ZIP dependency; remove one-use updater.
- [ ] Run `cargo fmt --check`, `cargo check --workspace`, `cargo clippy`, `cargo test --workspace`, native Windows release build.
- [ ] Document installation and limitations; provide ZIP/EXE only after actual successful build.
