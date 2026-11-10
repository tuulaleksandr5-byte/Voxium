use game_core::{
    AIR, BEDROCK, COAL_ORE, Chunk, DEEP_STONE, DIAMOND_ORE, GOLD_ORE, IRON_ORE, STONE, WORLD_MAX_Y,
    WORLD_MIN_Y, World,
    ores::{OreConfig, WorldGenerator},
};

#[test]
fn default_ore_bands_replace_only_stone_and_are_load_order_independent() {
    let generator = WorldGenerator::from_snapshot(2, Some(OreConfig::default())).unwrap();
    let mut counts = [0; 4];
    let positions = [[-1, -1], [0, -1], [-1, 0], [0, 0]];
    let mut world = World::new(42);
    for pos in positions {
        world.insert(generator.generate(42, pos));
    }
    for pos in positions.into_iter().rev() {
        let chunk = generator.generate(42, pos);
        assert_eq!(&*world.chunks[&pos], &chunk);
        let legacy = Chunk::generate(42, pos);
        for y in WORLD_MIN_Y..=WORLD_MAX_Y {
            for z in 0..16 {
                for x in 0..16 {
                    let p = [pos[0] * 16 + x, y, pos[1] * 16 + z];
                    let id = chunk.block(p);
                    if let Some(i) = [COAL_ORE, IRON_ORE, GOLD_ORE, DIAMOND_ORE]
                        .iter()
                        .position(|b| *b == id)
                    {
                        counts[i] += 1;
                        assert!(matches!(legacy.block(p), STONE | DEEP_STONE));
                        let (min, max) = [(0, 192), (-64, 72), (-64, 32), (-64, 16)][i];
                        assert!((min..=max).contains(&y));
                    } else {
                        assert_eq!(id, legacy.block(p));
                    }
                }
            }
        }
    }
    assert!(counts.iter().all(|n| *n > 0), "{counts:?}");
    assert_ne!(
        generator.generate(42, [0, 0]),
        generator.generate(43, [0, 0])
    );
}

#[test]
fn version_selection_requires_valid_snapshot_and_leaves_legacy_exact() {
    assert!(WorldGenerator::from_snapshot(2, None).is_err());
    assert!(WorldGenerator::from_snapshot(3, Some(OreConfig::default())).is_err());
    assert!(WorldGenerator::from_snapshot(1, Some(OreConfig::default())).is_err());
    let legacy = WorldGenerator::from_snapshot(1, None).unwrap();
    assert_eq!(legacy.generate(3, [-1, -1]), Chunk::generate(3, [-1, -1]));
}

#[test]
fn invalid_extreme_chunks_stay_empty_and_edge_chunks_do_not_overflow() {
    let generator = WorldGenerator::from_snapshot(2, Some(OreConfig::default())).unwrap();
    for pos in [
        [i32::MIN, 0],
        [0, i32::MAX],
        [i32::MAX, i32::MIN],
        [134_217_728, -1],
        [-134_217_729, 0],
        [0, -134_217_729],
    ] {
        let chunk = generator.generate(42, pos);
        assert!(chunk.sections.is_empty());
        assert_eq!(chunk, Chunk::generate(42, pos));
    }
    for pos in [
        [-134_217_728, 0],
        [134_217_727, -1],
        [-134_217_728, 134_217_727],
        [134_217_727, -134_217_728],
    ] {
        let chunk = generator.generate(42, pos);
        let x = (i64::from(pos[0]) * 16) as i32;
        assert_eq!(chunk.block([x, -64, pos[1] * 16]), BEDROCK);
        assert_eq!(chunk.block([x, 321, pos[1] * 16]), AIR);
    }
}

#[test]
fn config_rejects_unknown_duplicate_missing_or_unbounded_rules() {
    let source = include_str!("../../../assets/data/ores.ron");
    for invalid in [
        source.replacen("block: 17", "block: 3", 1),
        source.replacen("block: 18", "block: 17", 1),
        source.replacen("min_y: 0", "min_y: -65", 1),
        source.replacen("max_y: 192", "max_y: 321", 1),
        source.replacen("max_y: 192", "max_y: -1", 1),
        source.replacen("veins: 20", "veins: 0", 1),
        source.replacen("veins: 20", "veins: 65", 1),
        source.replacen("blocks: 17", "blocks: 0", 1),
        source.replacen("blocks: 17", "blocks: 65", 1),
        source.replacen("veins: 20", "veins: 20, future: true", 1),
    ] {
        assert!(OreConfig::from_ron(&invalid).is_err(), "{invalid}");
    }
    assert!(OreConfig::from_ron("(ores: [])").is_err());
}
