use game_core::{
    content::ContentRegistry,
    mining::{MiningConfig, MiningState, MiningTarget, Tool},
};

#[test]
fn supplied_txt_pins_all_72_applicable_duration_cells() {
    let config = MiningConfig::default();
    let catalog = ContentRegistry::default();
    let materials = ["", "wooden", "stone", "iron", "diamond", "golden"];
    let mut section = 0;
    let mut row = 0;
    let mut cells = 0;
    for line in include_str!("../../../docs/references/minecraft_mining_speed_1_20_1.txt").lines() {
        if line.starts_with("1. ") || line.starts_with("2. ") || line.starts_with("3. ") {
            section += 1;
            row = 0;
        }
        let parts: Vec<_> = line.split('|').map(str::trim).collect();
        if parts.len() < 2 || !parts[1].contains(',') {
            continue;
        }
        let blocks: &[u8] = match section {
            1 => &[game_core::SAND, game_core::GRAVEL, game_core::SNOW],
            2 => &[
                game_core::SANDSTONE,
                game_core::COBBLE,
                game_core::STONE,
                game_core::IRON_ORE,
                game_core::DIAMOND_ORE,
                game_core::GOLD_ORE,
            ],
            3 => &[
                game_core::IRON_BLOCK,
                game_core::DIAMOND_BLOCK,
                game_core::GOLD_BLOCK,
            ],
            _ => panic!("неизвестная секция TXT"),
        };
        let kind = if section == 1 { "shovel" } else { "pickaxe" };
        let item = (row != 0).then(|| {
            catalog
                .find(&format!("survival:{}_{kind}", materials[row]))
                .unwrap()
        });
        let tool = Tool::from_item(item, &catalog);
        for (column, block) in blocks.iter().enumerate() {
            let expected: f32 = parts[column + 1].replace(',', ".").parse().unwrap();
            assert_eq!(
                config.duration(*block, tool),
                Some(expected),
                "секция {section}, строка {row}, блок {block}"
            );
            cells += 1;
        }
        row += 1;
    }
    assert_eq!(cells, 72); // Шесть ячеек слоя снега исключены: такого блока нет.
}

#[test]
fn speed_and_harvest_are_independent_and_unknown_tools_use_hand() {
    let cfg = MiningConfig::default();
    let catalog = ContentRegistry::default();
    let tool = |name| Tool::from_item(catalog.find(name), &catalog);
    let gold = tool("survival:golden_pickaxe");
    assert_eq!(cfg.duration(game_core::IRON_ORE, gold), Some(1.30));
    assert!(!cfg.can_harvest(game_core::IRON_ORE, gold));
    assert!(cfg.can_harvest(game_core::COAL_ORE, gold));
    assert!(!cfg.can_harvest(game_core::GOLD_ORE, tool("survival:stone_pickaxe")));
    assert!(cfg.can_harvest(game_core::DIAMOND_ORE, tool("survival:iron_pickaxe")));
    assert_eq!(
        cfg.duration(game_core::STONE, tool("survival:diamond_axe")),
        Some(7.5)
    );
    assert_eq!(
        Tool::from_item(Some(game_core::content::ItemId(u16::MAX)), &catalog),
        Tool::Hand
    );
    for id in [
        game_core::AIR,
        game_core::WATER,
        game_core::BEDROCK,
        u8::MAX,
    ] {
        assert!(cfg.duration(id, Tool::Hand).is_none());
        assert!(!cfg.can_harvest(id, Tool::Hand));
    }
}

#[test]
fn config_rejects_invalid_times_ids_and_missing_defaults() {
    let source = include_str!("../../../assets/data/mining.ron");
    for invalid in [
        source.replacen("hardness: 0.6", "hardness: -1.0", 1),
        source.replacen("0.75", "0.0", 1),
        source.replacen("0.75", "-0.5", 1),
        source.replacen("0.75", "NaN", 1),
        source.replacen("0.75", "inf", 1),
        source.replacen("block: 2,", "block: 1,", 1),
        source.replacen("harvest_tier: 0", "harvest_tier: 5", 1),
        source.replacen("harvest_factor: 1.5", "harvest_factor: 0.0", 1),
        source.replacen("block: 1,", "block: 255,", 1),
    ] {
        assert!(MiningConfig::from_ron(&invalid).is_err());
    }
    assert!(MiningConfig::from_ron("(rules: [])").is_err());
}

#[test]
fn timer_resets_identity_and_consumes_completion_once_with_uncapped_time() {
    let mut state = MiningState::default();
    let target = MiningTarget {
        position: [1, 2, 3],
        block: game_core::STONE,
        item: None,
        slot: 0,
    };
    assert!(!state.advance(target, 0.4, 1.0));
    assert!(!state.consume(target, 1.0));
    assert!(state.advance(target, 0.6, 1.0));
    assert!(!state.advance(target, 1.0, 1.0));
    assert!(state.consume(target, 1.0));
    assert!(!state.consume(target, 1.0));
    for changed in [
        MiningTarget {
            position: [2, 2, 3],
            ..target
        },
        MiningTarget {
            block: game_core::COBBLE,
            ..target
        },
        MiningTarget {
            item: Some(game_core::content::ItemId(1)),
            ..target
        },
        MiningTarget { slot: 1, ..target },
    ] {
        state.reset();
        state.advance(target, 0.9, 1.0);
        assert!(!state.advance(changed, 0.2, 1.0));
        assert!(!state.consume(target, 1.0));
        state.reset();
        assert!(!state.consume(changed, 1.0));
    }
    assert!(!state.advance(target, f32::NAN, 1.0));
    assert!(!state.advance(target, 1.0, f32::INFINITY));
}

#[test]
fn fallback_defaults_cover_unlisted_blocks_with_correct_tools() {
    let cfg = MiningConfig::default();
    let catalog = ContentRegistry::default();
    let tool = |name| Tool::from_item(catalog.find(name), &catalog);
    for (block, hand, name, equipped) in [
        (game_core::GRASS, 0.9, "survival:diamond_shovel", 0.1125),
        (game_core::PLANKS, 3.0, "survival:golden_axe", 0.25),
        (game_core::GLASS, 0.45, "survival:diamond_pickaxe", 0.45),
        (game_core::CHEST, 3.75, "survival:iron_axe", 0.625),
        (
            game_core::CRAFTING_TABLE,
            3.75,
            "survival:stone_axe",
            0.9375,
        ),
        (game_core::DIRT, 0.75, "survival:wooden_shovel", 0.375),
        (game_core::WOOD, 3.0, "survival:wooden_axe", 1.5),
        (game_core::LEAVES, 0.3, "survival:wooden_hoe", 0.15),
        (game_core::DEEP_STONE, 15.0, "survival:stone_pickaxe", 1.125),
        (game_core::COAL_ORE, 15.0, "survival:wooden_pickaxe", 2.25),
        (game_core::FURNACE, 17.5, "survival:stone_pickaxe", 1.3125),
    ] {
        assert!((cfg.duration(block, Tool::Hand).unwrap() - hand).abs() < 0.00001);
        assert!((cfg.duration(block, tool(name)).unwrap() - equipped).abs() < 0.00001);
    }
    let source = include_str!("../../../assets/data/mining.ron");
    assert!(MiningConfig::from_ron(&source.replace("2.0, 4.0, 6.0", "1e-40, 4.0, 6.0")).is_err());
}

#[test]
fn tool_recognition_whitelists_namespace_material_kind_and_catalog_type() {
    for (name, kind) in [
        ("custom:diamond_pickaxe", "Tool"),
        ("survival:netherite_pickaxe", "Tool"),
        ("survival:diamond_pickaxe_extra", "Tool"),
        ("survival:diamond_sword", "Tool"),
        ("survival:diamond_pickaxe", "Material"),
    ] {
        let catalog = ContentRegistry::from_ron(&format!(
            "[(id: \"{name}\", name: \"x\", max_stack: 1, kind: {kind})]"
        ))
        .unwrap();
        assert_eq!(Tool::from_item(catalog.find(name), &catalog), Tool::Hand);
    }
}
