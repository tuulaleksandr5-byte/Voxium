//! Headless acceptance scenario: generated world -> build -> save -> reload -> remesh.
use game_core::{interaction::raycast, persistence::*, *};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn constructions_and_player_survive_a_new_world_instance() {
    let mut world = World::new(971);
    for pos in [[-1, -1], [-1, 0], [0, -1], [0, 0]] {
        world.insert(Chunk::generate(world.seed, pos));
    }
    let y = surface_height(world.seed, 0, 0).max(SEA_LEVEL) + 12;
    // Build across a negative-coordinate chunk corner.
    for x in -2..=2 {
        for z in -2..=2 {
            assert!(world.set([x, y, z], PLANKS));
        }
    }
    let hit = raycast([0.5, y as f32 + 4.0, 0.5], [0., -1., 0.], 6., |p| {
        world.block(p)
    })
    .unwrap();
    assert_eq!(hit.block, [0, y, 0]);
    assert_eq!(hit.previous, [0, y + 1, 0]);
    assert!(world.set(hit.block, AIR));
    let data = SaveData {
        name: "Boundary build".into(),
        seed: world.seed,
        player: [0.5, y as f32 + 1., 0.5],
        yaw: 0.5,
        pitch: -0.25,
        time: 532.,
        edits: world.edits.iter().map(|(p, b)| (*p, *b)).collect(),
        ..Default::default()
    };
    let root = std::env::temp_dir().join(format!(
        "voxel-scenario-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    save_world(&root, "build", &data).unwrap();
    drop(world);
    let loaded = load_world(&root, "build").unwrap();
    assert_eq!(loaded, data);
    let mut world = World::new(loaded.seed);
    world.edits = loaded.edits.into_iter().collect();
    for pos in [[-1, -1], [-1, 0], [0, -1], [0, 0]] {
        world.insert(Chunk::generate(world.seed, pos));
    }
    assert_eq!(world.block([0, y, 0]), AIR);
    for pos in [[-1, y, -1], [0, y, -1], [-1, y, 0], [1, y, 1]] {
        assert_eq!(world.block(pos), PLANKS);
    }
    let meshes = game_render::mesh_section([0, y.div_euclid(16), 0], |p| world.block(p));
    assert!(
        meshes
            .iter()
            .any(|(id, mesh)| *id == PLANKS && mesh.count_vertices() > 0)
    );
    std::fs::remove_dir_all(root).unwrap();
}
