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
    assert!(WorldGenerator::from_snapshot(7, Some(OreConfig::default())).is_err());
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

#[test]
fn generator_three_only_adds_birch_clay_and_obsidian_on_top_of_generator_two() {
    use game_core::{
        BIRCH_LEAVES, BIRCH_LOG, CLAY, GRAVEL, LEAVES, OBSIDIAN, SAND, WOOD, biome_at, world::Biome,
    };
    let two = WorldGenerator::from_snapshot(2, Some(OreConfig::default())).unwrap();
    let three = WorldGenerator::from_snapshot(3, Some(OreConfig::default())).unwrap();
    assert_eq!((two.version(), three.version()), (2, 3));
    assert!(WorldGenerator::from_snapshot(3, None).is_err());
    assert!(WorldGenerator::from_snapshot(7, Some(OreConfig::default())).is_err());
    let seed = 5;
    // Лесной чанк гарантирует деревья, остальные дают воду и глубину.
    let forest = (-40..40)
        .flat_map(|x| (-40..40).map(move |z| [x, z]))
        .find(|p| biome_at(seed, p[0] * 16 + 8, p[1] * 16 + 8) == Biome::Forest)
        .expect("seed has a forest");
    let mut found = [0_usize; 4];
    for dx in -4..=4 {
        for dz in -4..=4 {
            let pos = [forest[0] + dx, forest[1] + dz];
            let old = two.generate(seed, pos);
            let new = three.generate(seed, pos);
            assert_eq!(new, three.generate(seed, pos));
            for y in WORLD_MIN_Y..=WORLD_MAX_Y {
                for z in 0..16 {
                    for x in 0..16 {
                        let p = [pos[0] * 16 + x, y, pos[1] * 16 + z];
                        let (before, after) = (old.block(p), new.block(p));
                        if before == after {
                            continue;
                        }
                        let index = match (before, after) {
                            (WOOD, BIRCH_LOG) => 0,
                            (LEAVES, BIRCH_LEAVES) => 1,
                            (SAND | GRAVEL, CLAY) => 2,
                            (DEEP_STONE, OBSIDIAN) => 3,
                            other => panic!("unexpected replacement {other:?} at {p:?}"),
                        };
                        found[index] += 1;
                    }
                }
            }
        }
    }
    assert!(found.iter().all(|count| *count > 0), "{found:?}");
}

#[test]
fn generator_four_adds_pines_rocks_and_plants_only_where_expected() {
    use game_core::{
        BASALT, CALCITE, COBBLED_DEEPSLATE, CORNFLOWER, DAISY, DANDELION, GRANITE, GRASS, LEAVES,
        POPPY, SANDSTONE, SPRUCE_LEAVES, SPRUCE_LOG, TALL_GRASS, WOOD, biome_at, world::Biome,
    };
    let three = WorldGenerator::from_snapshot(3, Some(OreConfig::default())).unwrap();
    let four = WorldGenerator::from_snapshot(4, Some(OreConfig::default())).unwrap();
    assert_eq!(four.version(), 4);
    let seed = 5;
    let forest = (-40..40)
        .flat_map(|x| (-40..40).map(move |z| [x, z]))
        .find(|p| biome_at(seed, p[0] * 16 + 8, p[1] * 16 + 8) == Biome::Forest)
        .expect("seed has a forest");
    // [сосна, породы, цветы, трава]
    let mut found = [0_usize; 4];
    for dx in -4..=4 {
        for dz in -4..=4 {
            let pos = [forest[0] + dx, forest[1] + dz];
            let old = three.generate(seed, pos);
            let new = four.generate(seed, pos);
            assert_eq!(new, four.generate(seed, pos));
            for y in WORLD_MIN_Y..=WORLD_MAX_Y {
                for z in 0..16 {
                    for x in 0..16 {
                        let p = [pos[0] * 16 + x, y, pos[1] * 16 + z];
                        let (before, after) = (old.block(p), new.block(p));
                        if before == after {
                            continue;
                        }
                        let index = match (before, after) {
                            (WOOD, SPRUCE_LOG) | (LEAVES, SPRUCE_LEAVES) => 0,
                            (STONE, SANDSTONE | GRANITE | CALCITE | COBBLED_DEEPSLATE)
                            | (DEEP_STONE, BASALT) => 1,
                            (AIR, POPPY | DANDELION | CORNFLOWER | DAISY) => 2,
                            (AIR, TALL_GRASS) => 3,
                            other => panic!("unexpected replacement {other:?} at {p:?}"),
                        };
                        if index >= 2 {
                            assert_eq!(new.block([p[0], p[1] - 1, p[2]]), GRASS);
                        }
                        found[index] += 1;
                    }
                }
            }
        }
    }
    assert!(found.iter().all(|count| *count > 0), "{found:?}");
}

#[test]
fn generator_five_adds_tree_species_and_ores_without_touching_anything_else() {
    use game_core::{
        ACACIA_LEAVES, ACACIA_LOG, CHERRY_LEAVES, CHERRY_LOG, COPPER_ORE, DARK_OAK_LEAVES,
        DARK_OAK_LOG, EMERALD_ORE, GRASS, JUNGLE_LEAVES, JUNGLE_LOG, LAPIS_ORE, LEAVES,
        MANGROVE_LEAVES, MANGROVE_LOG, MELON, PUMPKIN, RED_SAND, SAND, WOOD, biome_at,
        world::Biome,
    };
    let four = WorldGenerator::from_snapshot(4, Some(OreConfig::default())).unwrap();
    let five = WorldGenerator::from_snapshot(5, Some(OreConfig::default())).unwrap();
    assert_eq!(five.version(), 5);
    let seed = 5;
    let forest = (-40..40)
        .flat_map(|x| (-40..40).map(move |z| [x, z]))
        .find(|p| biome_at(seed, p[0] * 16 + 8, p[1] * 16 + 8) == Biome::Forest)
        .expect("seed has a forest");
    // [деревья, руды]
    let mut found = [0_usize; 2];
    for dx in -4..=4 {
        for dz in -4..=4 {
            let pos = [forest[0] + dx, forest[1] + dz];
            let old = four.generate(seed, pos);
            let new = five.generate(seed, pos);
            assert_eq!(new, five.generate(seed, pos));
            for y in WORLD_MIN_Y..=WORLD_MAX_Y {
                for z in 0..16 {
                    for x in 0..16 {
                        let p = [pos[0] * 16 + x, y, pos[1] * 16 + z];
                        let (before, after) = (old.block(p), new.block(p));
                        if before == after {
                            continue;
                        }
                        match (before, after) {
                            (
                                WOOD,
                                ACACIA_LOG | CHERRY_LOG | DARK_OAK_LOG | JUNGLE_LOG | MANGROVE_LOG,
                            )
                            | (
                                LEAVES,
                                ACACIA_LEAVES | CHERRY_LEAVES | DARK_OAK_LEAVES | JUNGLE_LEAVES
                                | MANGROVE_LEAVES,
                            ) => found[0] += 1,
                            (STONE | DEEP_STONE, COPPER_ORE | LAPIS_ORE | EMERALD_ORE) => {
                                found[1] += 1
                            }
                            (SAND, RED_SAND) => {}
                            (AIR, PUMPKIN | MELON) => {
                                assert_eq!(new.block([p[0], p[1] - 1, p[2]]), GRASS)
                            }
                            other => panic!("unexpected replacement {other:?} at {p:?}"),
                        }
                    }
                }
            }
        }
    }
    assert!(found.iter().all(|count| *count > 0), "{found:?}");
}

#[test]
fn generator_six_adds_only_deep_ores_rocks_spruces_and_ground_plants() {
    use game_core::{
        ACACIA_LEAVES, AIR, ALLIUM, ANDESITE, AZALEA_LEAVES, AZURE_BLUET, BLUE_ORCHID,
        BROWN_MUSHROOM, COAL_ORE, DEAD_BUSH, DEEPSLATE_COAL_ORE, DEEPSLATE_GOLD_ORE,
        DEEPSLATE_IRON_ORE, DEEPSLATE_LAPIS_ORE, DIORITE, FERN, GOLD_ORE, GRASS, IRON_ORE,
        LAPIS_ORE, LEAVES, LILY_OF_THE_VALLEY, ORANGE_TULIP, PINK_TULIP, RED_MUSHROOM, RED_TULIP,
        SAND, SHORT_GRASS, SPRUCE_LEAVES, SPRUCE_LOG, STONE, WHITE_TULIP, WOOD, biome_at,
        world::Biome,
    };
    let _ = (ACACIA_LEAVES, AZALEA_LEAVES);
    let five = WorldGenerator::from_snapshot(5, Some(OreConfig::default())).unwrap();
    let six = WorldGenerator::from_snapshot(6, Some(OreConfig::default())).unwrap();
    assert_eq!(six.version(), 6);
    let seed = 5;
    let forest = (-40..40)
        .flat_map(|x| (-40..40).map(move |z| [x, z]))
        .find(|p| biome_at(seed, p[0] * 16 + 8, p[1] * 16 + 8) == Biome::Forest)
        .expect("seed has a forest");
    // [ели, породы, глубинные руды, растения]
    let mut found = [0_usize; 4];
    for dx in -4..=4 {
        for dz in -4..=4 {
            let pos = [forest[0] + dx, forest[1] + dz];
            let old = five.generate(seed, pos);
            let new = six.generate(seed, pos);
            assert_eq!(new, six.generate(seed, pos));
            for y in WORLD_MIN_Y..=WORLD_MAX_Y {
                for z in 0..16 {
                    for x in 0..16 {
                        let p = [pos[0] * 16 + x, y, pos[1] * 16 + z];
                        let (before, after) = (old.block(p), new.block(p));
                        if before == after {
                            continue;
                        }
                        let index = match (before, after) {
                            (WOOD, SPRUCE_LOG) | (LEAVES, SPRUCE_LEAVES) => 0,
                            (STONE, DIORITE | ANDESITE) => 1,
                            (
                                COAL_ORE | IRON_ORE | GOLD_ORE | LAPIS_ORE,
                                DEEPSLATE_COAL_ORE | DEEPSLATE_IRON_ORE | DEEPSLATE_GOLD_ORE
                                | DEEPSLATE_LAPIS_ORE,
                            ) => 2,
                            (
                                AIR,
                                ALLIUM | AZURE_BLUET | RED_TULIP | ORANGE_TULIP | WHITE_TULIP
                                | PINK_TULIP | BLUE_ORCHID | LILY_OF_THE_VALLEY | FERN
                                | SHORT_GRASS | BROWN_MUSHROOM | RED_MUSHROOM | DEAD_BUSH,
                            ) => {
                                let below = new.block([p[0], p[1] - 1, p[2]]);
                                assert!(below == GRASS || below == SAND);
                                3
                            }
                            other => panic!("unexpected replacement {other:?} at {p:?}"),
                        };
                        found[index] += 1;
                    }
                }
            }
        }
    }
    // Глубинные руды встречаются не в каждой окрестности леса, остальное — всегда.
    assert!(found[0] > 0 && found[1] > 0 && found[3] > 0, "{found:?}");
}
