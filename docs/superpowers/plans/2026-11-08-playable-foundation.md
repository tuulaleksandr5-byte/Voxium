# Playable Foundation Implementation Plan

> **For agentic workers:** Use superpowers:subagent-driven-development for independent modules and integration review.

**Goal:** Replace the empty bootstrap with a playable creative voxel foundation in this repository.
**Architecture:** Keep deterministic, headless block storage/generation/interactions in game_core; put section greedy meshes and original procedural pixel materials in game_render; coordinate streaming, player, UI and persistence in the Bevy client. No entity per block.
**Tech Stack:** Rust 1.99.1 / edition 2024, Bevy =0.20.1, serde and versioned checked JSON saves.
**Spec:** ../specs/2026-11-08-v1-requirements.md

## Global Constraints
- WORLD_MIN_Y=-64, WORLD_MAX_Y=320 inclusive, SEA_LEVEL=63, sections 16³.
- Native Rust/Bevy only. Own procedural materials use references for palette, not extracted sprites.
- The existing cloud checkout is isolated. Work on feature/playable-foundation, without another worktree.
- Preserve original coordinate APIs and tests. Never claim Windows runtime or 60 FPS without measurement.
- User explicitly asks to begin implementation without stopping at a plan; routine design choices are delegated to the implementer.

## Review Focus
- Negative and boundary coordinates: edits invalidate adjacent meshes and survive unload/reload.
- Corrupt or interrupted saves: fail explicitly or recover valid backup, never silently lose constructions.
- Unloaded terrain: freeze gravity until collision data is available.
- Chunk task results: discard results for worlds or revisions that have changed.
- Repeated UI/world transitions: release cursor and remove old entities/tasks.

### Task 1: Headless world
- [x] Test stable registry, negative sections, deterministic generation, bounds and edits.
- [x] Implement game_core blocks/world modules using the contract below, then run headless tests.

### Task 2: Meshing and original materials
- [x] Test single cube, adjacent cubes, solid section, transparent boundary and cross-section culling.
- [x] Implement game_render greedy meshes and nearest/repeating pixel textures with per-block materials.

### Task 3: Saves
- [x] Test save round trip, integrity failure, backup recovery, version/ID/path validation.
- [x] Implement game_core::persistence, independent of Bevy, using explicit save directory.

### Task 4: Playable client
- [x] Implement asynchronous near-first chunk streaming and bounded mesh uploads.
- [x] Add first-person movement, gravity, collision, DDA block picking/building, hotbar and catalog.
- [x] Add menu/create/load/pause/save/settings and persisted world/player/settings.
- [x] Verify format, check, Clippy, tests, build and Xvfb/software Vulkan startup; exercise create/edit/save/reload.
- [x] Document controls, scope limits and remaining v1 acceptance checks; commit coherent changes and review.

## Shared contract
Block IDs are u8: AIR=0 GRASS=1 DIRT=2 STONE=3 DEEP_STONE=4 SAND=5 GRAVEL=6 WOOD=7 LEAVES=8 PLANKS=9 COBBLE=10 GLASS=11 WATER=12 BEDROCK=13 SNOW=14 SANDSTONE=15.
Root exports: BLOCKS: [BlockDef;16], BlockDef {id:u8,name:&'static str,texture:&'static str,hardness:f32,collision:bool,opacity:bool,light_emission:u8}; is_solid(u8), is_opaque(u8).
World {pub seed:u64,pub chunks:HashMap<[i32;2],Arc<Chunk>>,pub edits:HashMap<[i32;3],u8>,pub dirty:HashSet<[i32;3]>}; World::new(seed), block([i32;3])->u8, set([i32;3],u8)->bool, insert(Chunk), unload([i32;2]); chunk has pub position:[i32;2], pub sections:HashMap<i32,Section>; Chunk::generate(seed,position), block([i32;3])->u8 (WORLD position); Section blocks compact u8, all-air sections omitted. surface_height(seed,x,z)->i32. All exported from game_core.
game_render::mesh_section(section:[i32;3],sample:impl Fn([i32;3])->u8)->Vec<(u8,Mesh)>; positions section-local, outward CCW indices, repeated UVs. game_render::create_materials(&mut Assets<Image>,&mut Assets<StandardMaterial>)->Vec<Handle<StandardMaterial>> indexed block ID.
game_core::persistence::{SaveData {format_version:u32,generator_version:u32,name:String,seed:u64,player:[f32;3],yaw:f32,pitch:f32,time:f64,edits:Vec<([i32;3],u8)>},save_world(root:&Path,id:&str,data:&SaveData)->Result<(),String>,load_world(root,id)->Result<SaveData,String>,list_worlds(root)->Result<Vec<(String,String)>,String>}.


## Execution record
- Implemented independently: world/registry; meshing/materials; persistence; interaction/physics; UI. Root integrated streaming, camera/player, settings and application lifecycle.
- Independent reviewers inspected core/render and client; fixed contact rounding, ceiling clearance (including unloaded neighbors), save-version protection, failed-autosave retry, stored volume activation, menu clipping.
- Ruling: keep this cloud checkout and create feature/playable-foundation branch — onboarding isolation already exists and the user asks to use the existing repository.
- Ruling: implement a playable pre-1.0 stage instead of claiming a finished1.0 — Windows/GPU validation unavailable, large-save performance and far-coordinate precision remain work.
- Ruling: procedural16×16 original materials, not sprite extraction — images are references and 3D faces need repeatable materials.
- Validation evidence and known limits: docs/VALIDATION.md and README.md. No push/merge/publication performed.
