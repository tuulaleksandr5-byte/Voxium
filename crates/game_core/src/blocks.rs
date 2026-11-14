//! Stable block IDs. Saved worlds store these IDs, never display names.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlockDef {
    pub id: u8,
    pub name: &'static str,
    pub texture: &'static str,
    pub hardness: f32,
    pub collision: bool,
    pub opacity: bool,
    pub light_emission: u8,
}

pub const AIR: u8 = 0;
pub const GRASS: u8 = 1;
pub const DIRT: u8 = 2;
pub const STONE: u8 = 3;
pub const DEEP_STONE: u8 = 4;
pub const SAND: u8 = 5;
pub const GRAVEL: u8 = 6;
pub const WOOD: u8 = 7;
pub const LEAVES: u8 = 8;
pub const PLANKS: u8 = 9;
pub const COBBLE: u8 = 10;
pub const GLASS: u8 = 11;
pub const WATER: u8 = 12;
pub const BEDROCK: u8 = 13;
pub const SNOW: u8 = 14;
pub const SANDSTONE: u8 = 15;
pub const CHEST: u8 = 16;
pub const COAL_ORE: u8 = 17;
pub const IRON_ORE: u8 = 18;
pub const GOLD_ORE: u8 = 19;
pub const DIAMOND_ORE: u8 = 20;
pub const IRON_BLOCK: u8 = 21;
pub const GOLD_BLOCK: u8 = 22;
pub const DIAMOND_BLOCK: u8 = 23;
pub const CRAFTING_TABLE: u8 = 24;
pub const FURNACE: u8 = 25;
pub const BRICKS: u8 = 26;
pub const STONE_BRICKS: u8 = 27;
pub const MOSSY_COBBLE: u8 = 28;
pub const BIRCH_LOG: u8 = 29;
pub const BIRCH_LEAVES: u8 = 30;
pub const BIRCH_PLANKS: u8 = 31;
pub const CLAY: u8 = 32;
pub const BOOKSHELF: u8 = 33;
pub const LAMP: u8 = 34;
pub const ICE: u8 = 35;
pub const OBSIDIAN: u8 = 36;
pub const WOOL: u8 = 37;
pub const TERRACOTTA: u8 = 38;
pub const SMOOTH_STONE: u8 = 39;
pub const COAL_BLOCK: u8 = 40;
/// Работающая печь: состояние блока, а не отдельный предмет.
pub const FURNACE_LIT: u8 = 41;

const fn block(
    id: u8,
    name: &'static str,
    texture: &'static str,
    hardness: f32,
    collision: bool,
    opacity: bool,
) -> BlockDef {
    BlockDef {
        id,
        name,
        texture,
        hardness,
        collision,
        opacity,
        light_emission: 0,
    }
}

pub const BLOCKS: [BlockDef; 42] = [
    block(AIR, "Air", "air", 0.0, false, false),
    block(GRASS, "Grass", "grass", 0.6, true, true),
    block(DIRT, "Dirt", "dirt", 0.5, true, true),
    block(STONE, "Stone", "stone", 1.5, true, true),
    block(DEEP_STONE, "Deep Stone", "deep_stone", 3.0, true, true),
    block(SAND, "Sand", "sand", 0.5, true, true),
    block(GRAVEL, "Gravel", "gravel", 0.6, true, true),
    block(WOOD, "Wood", "wood", 2.0, true, true),
    block(LEAVES, "Leaves", "leaves", 0.2, true, false),
    block(PLANKS, "Planks", "planks", 2.0, true, true),
    block(COBBLE, "Cobblestone", "cobble", 2.0, true, true),
    block(GLASS, "Glass", "glass", 0.3, true, false),
    block(WATER, "Water", "water", 100.0, false, false),
    block(BEDROCK, "Bedrock", "bedrock", -1.0, true, true),
    block(SNOW, "Snow", "snow", 0.2, true, true),
    block(SANDSTONE, "Sandstone", "sandstone", 0.8, true, true),
    // TODO(art): временно используем исходную текстуру досок, не изменяя PNG.
    block(CHEST, "Chest", "oak_planks", 2.5, true, true),
    block(COAL_ORE, "Coal Ore", "coal_ore", 3.0, true, true),
    block(IRON_ORE, "Iron Ore", "iron_ore", 3.0, true, true),
    block(GOLD_ORE, "Gold Ore", "gold_ore", 3.0, true, true),
    block(DIAMOND_ORE, "Diamond Ore", "diamond_ore", 3.0, true, true),
    block(IRON_BLOCK, "Iron Block", "iron_block", 5.0, true, true),
    block(GOLD_BLOCK, "Gold Block", "gold_block", 3.0, true, true),
    block(
        DIAMOND_BLOCK,
        "Diamond Block",
        "diamond_block",
        5.0,
        true,
        true,
    ),
    block(
        CRAFTING_TABLE,
        "Crafting Table",
        "crafting_table",
        2.5,
        true,
        true,
    ),
    block(FURNACE, "Furnace", "furnace", 3.5, true, true),
    block(BRICKS, "Bricks", "bricks", 2.0, true, true),
    block(
        STONE_BRICKS,
        "Stone Bricks",
        "stone_bricks",
        1.5,
        true,
        true,
    ),
    block(
        MOSSY_COBBLE,
        "Mossy Cobblestone",
        "mossy_cobblestone",
        2.0,
        true,
        true,
    ),
    block(BIRCH_LOG, "Birch Log", "birch_log", 2.0, true, true),
    block(
        BIRCH_LEAVES,
        "Birch Leaves",
        "birch_leaves",
        0.2,
        true,
        false,
    ),
    block(
        BIRCH_PLANKS,
        "Birch Planks",
        "birch_planks",
        2.0,
        true,
        true,
    ),
    block(CLAY, "Clay", "clay", 0.6, true, true),
    block(BOOKSHELF, "Bookshelf", "bookshelf", 1.5, true, true),
    BlockDef {
        light_emission: 15,
        ..block(LAMP, "Lamp", "lamp", 0.3, true, true)
    },
    block(ICE, "Ice", "ice", 0.5, true, false),
    block(OBSIDIAN, "Obsidian", "obsidian", 50.0, true, true),
    block(WOOL, "Wool", "wool", 0.8, true, true),
    block(TERRACOTTA, "Terracotta", "terracotta", 1.25, true, true),
    block(
        SMOOTH_STONE,
        "Smooth Stone",
        "smooth_stone",
        2.0,
        true,
        true,
    ),
    block(COAL_BLOCK, "Coal Block", "coal_block", 5.0, true, true),
    BlockDef {
        light_emission: 13,
        ..block(FURNACE_LIT, "Lit Furnace", "furnace", 3.5, true, true)
    },
];

/// Листва получает объёмную геометрию и не сливается в greedy-грани.
#[must_use]
pub fn is_leaves(id: u8) -> bool {
    matches!(id, LEAVES | BIRCH_LEAVES)
}

#[must_use]
pub fn is_solid(id: u8) -> bool {
    BLOCKS
        .get(usize::from(id))
        .is_some_and(|block| block.collision)
}
#[must_use]
pub fn is_opaque(id: u8) -> bool {
    BLOCKS
        .get(usize::from(id))
        .is_some_and(|block| block.opacity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progression_blocks_append_after_all_foundation_and_chest_ids() {
        let expected = [
            "Coal Ore",
            "Iron Ore",
            "Gold Ore",
            "Diamond Ore",
            "Iron Block",
            "Gold Block",
            "Diamond Block",
            "Crafting Table",
            "Furnace",
        ];
        assert_eq!(BLOCKS.len(), 42);
        for (offset, name) in expected.iter().enumerate() {
            let id = 17 + offset;
            assert_eq!(BLOCKS[id].id as usize, id);
            assert_eq!(BLOCKS[id].name, *name);
            assert!(is_solid(id as u8) && is_opaque(id as u8));
        }
    }

    #[test]
    fn chest_extends_registry_without_changing_foundation_ids() {
        assert_eq!(BLOCKS.len(), 42);
        let chest = BLOCKS.get(16).expect("сундук зарегистрирован");
        assert_eq!(chest.name, "Chest");
        assert!(is_solid(16));
        assert!(is_opaque(16));
        assert_eq!(BLOCKS.get(15).unwrap().name, "Sandstone");
    }

    #[test]
    fn registry_ids_and_properties_are_stable() {
        let names = [
            "Air",
            "Grass",
            "Dirt",
            "Stone",
            "Deep Stone",
            "Sand",
            "Gravel",
            "Wood",
            "Leaves",
            "Planks",
            "Cobblestone",
            "Glass",
            "Water",
            "Bedrock",
            "Snow",
            "Sandstone",
            "Chest",
        ];
        for (id, block) in BLOCKS[..17].iter().enumerate() {
            assert_eq!(block.id as usize, id);
            assert_eq!(block.name, names[id]);
            assert!(!block.texture.is_empty());
        }
        assert!(!is_solid(AIR));
        assert!(!is_solid(WATER));
        assert!(is_solid(GLASS));
        assert!(is_solid(LEAVES));
        assert!(!is_opaque(GLASS));
        assert!(!is_opaque(WATER));
        assert!(is_opaque(STONE));
        assert!(!is_solid(u8::MAX));
        assert!(!is_opaque(u8::MAX));
    }
}
