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
pub const BARREL: u8 = 42;
pub const SMOKER: u8 = 43;
pub const SMOKER_LIT: u8 = 44;
pub const BLAST_FURNACE: u8 = 45;
pub const BLAST_FURNACE_LIT: u8 = 46;
pub const STONECUTTER: u8 = 47;
/// Общий сундук: все такие блоки открывают одно хранилище игрока.
pub const VAULT: u8 = 48;
// Декоративные блоки обновления «Палитра».
pub const WOOL_ORANGE: u8 = 49;
pub const WOOL_MAGENTA: u8 = 50;
pub const WOOL_LIGHT_BLUE: u8 = 51;
pub const WOOL_YELLOW: u8 = 52;
pub const WOOL_LIME: u8 = 53;
pub const WOOL_PINK: u8 = 54;
pub const WOOL_GRAY: u8 = 55;
pub const WOOL_LIGHT_GRAY: u8 = 56;
pub const WOOL_CYAN: u8 = 57;
pub const WOOL_PURPLE: u8 = 58;
pub const WOOL_BLUE: u8 = 59;
pub const WOOL_BROWN: u8 = 60;
pub const WOOL_GREEN: u8 = 61;
pub const WOOL_RED: u8 = 62;
pub const WOOL_BLACK: u8 = 63;
pub const CONCRETE_WHITE: u8 = 64;
pub const CONCRETE_ORANGE: u8 = 65;
pub const CONCRETE_MAGENTA: u8 = 66;
pub const CONCRETE_LIGHT_BLUE: u8 = 67;
pub const CONCRETE_YELLOW: u8 = 68;
pub const CONCRETE_LIME: u8 = 69;
pub const CONCRETE_PINK: u8 = 70;
pub const CONCRETE_GRAY: u8 = 71;
pub const CONCRETE_LIGHT_GRAY: u8 = 72;
pub const CONCRETE_CYAN: u8 = 73;
pub const CONCRETE_PURPLE: u8 = 74;
pub const CONCRETE_BLUE: u8 = 75;
pub const CONCRETE_BROWN: u8 = 76;
pub const CONCRETE_GREEN: u8 = 77;
pub const CONCRETE_RED: u8 = 78;
pub const CONCRETE_BLACK: u8 = 79;
pub const GRANITE: u8 = 80;
pub const POLISHED_GRANITE: u8 = 81;
pub const MARBLE: u8 = 82;
pub const POLISHED_MARBLE: u8 = 83;
pub const BASALT: u8 = 84;
pub const POLISHED_BASALT: u8 = 85;
pub const SLATE: u8 = 86;
pub const POLISHED_SLATE: u8 = 87;
pub const LIMESTONE: u8 = 88;
pub const POLISHED_LIMESTONE: u8 = 89;
pub const PINE_LOG: u8 = 90;
pub const PINE_LEAVES: u8 = 91;
pub const PINE_PLANKS: u8 = 92;
pub const SANDSTONE_BRICKS: u8 = 93;
pub const MOSSY_STONE_BRICKS: u8 = 94;
pub const CRACKED_STONE_BRICKS: u8 = 95;
pub const DARK_BRICKS: u8 = 96;
pub const STONE_TILES: u8 = 97;

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

pub const BLOCKS: [BlockDef; 98] = [
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
    block(BARREL, "Barrel", "barrel", 2.5, true, true),
    block(SMOKER, "Smoker", "smoker", 3.5, true, true),
    BlockDef {
        light_emission: 13,
        ..block(SMOKER_LIT, "Lit Smoker", "smoker", 3.5, true, true)
    },
    block(
        BLAST_FURNACE,
        "Blast Furnace",
        "blast_furnace",
        3.5,
        true,
        true,
    ),
    BlockDef {
        light_emission: 13,
        ..block(
            BLAST_FURNACE_LIT,
            "Lit Blast Furnace",
            "blast_furnace",
            3.5,
            true,
            true,
        )
    },
    block(STONECUTTER, "Stonecutter", "stonecutter", 3.5, true, true),
    block(VAULT, "Vault", "vault", 22.5, true, true),
    block(WOOL_ORANGE, "Orange Wool", "wool_orange", 0.8, true, true),
    block(
        WOOL_MAGENTA,
        "Magenta Wool",
        "wool_magenta",
        0.8,
        true,
        true,
    ),
    block(
        WOOL_LIGHT_BLUE,
        "Light_Blue Wool",
        "wool_light_blue",
        0.8,
        true,
        true,
    ),
    block(WOOL_YELLOW, "Yellow Wool", "wool_yellow", 0.8, true, true),
    block(WOOL_LIME, "Lime Wool", "wool_lime", 0.8, true, true),
    block(WOOL_PINK, "Pink Wool", "wool_pink", 0.8, true, true),
    block(WOOL_GRAY, "Gray Wool", "wool_gray", 0.8, true, true),
    block(
        WOOL_LIGHT_GRAY,
        "Light_Gray Wool",
        "wool_light_gray",
        0.8,
        true,
        true,
    ),
    block(WOOL_CYAN, "Cyan Wool", "wool_cyan", 0.8, true, true),
    block(WOOL_PURPLE, "Purple Wool", "wool_purple", 0.8, true, true),
    block(WOOL_BLUE, "Blue Wool", "wool_blue", 0.8, true, true),
    block(WOOL_BROWN, "Brown Wool", "wool_brown", 0.8, true, true),
    block(WOOL_GREEN, "Green Wool", "wool_green", 0.8, true, true),
    block(WOOL_RED, "Red Wool", "wool_red", 0.8, true, true),
    block(WOOL_BLACK, "Black Wool", "wool_black", 0.8, true, true),
    block(
        CONCRETE_WHITE,
        "White Concrete",
        "concrete_white",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_ORANGE,
        "Orange Concrete",
        "concrete_orange",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_MAGENTA,
        "Magenta Concrete",
        "concrete_magenta",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_LIGHT_BLUE,
        "Light_Blue Concrete",
        "concrete_light_blue",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_YELLOW,
        "Yellow Concrete",
        "concrete_yellow",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_LIME,
        "Lime Concrete",
        "concrete_lime",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_PINK,
        "Pink Concrete",
        "concrete_pink",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_GRAY,
        "Gray Concrete",
        "concrete_gray",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_LIGHT_GRAY,
        "Light_Gray Concrete",
        "concrete_light_gray",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_CYAN,
        "Cyan Concrete",
        "concrete_cyan",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_PURPLE,
        "Purple Concrete",
        "concrete_purple",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_BLUE,
        "Blue Concrete",
        "concrete_blue",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_BROWN,
        "Brown Concrete",
        "concrete_brown",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_GREEN,
        "Green Concrete",
        "concrete_green",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_RED,
        "Red Concrete",
        "concrete_red",
        1.8,
        true,
        true,
    ),
    block(
        CONCRETE_BLACK,
        "Black Concrete",
        "concrete_black",
        1.8,
        true,
        true,
    ),
    block(GRANITE, "Granite", "granite", 1.5, true, true),
    block(
        POLISHED_GRANITE,
        "Polished Granite",
        "polished_granite",
        1.5,
        true,
        true,
    ),
    block(MARBLE, "Marble", "marble", 1.5, true, true),
    block(
        POLISHED_MARBLE,
        "Polished Marble",
        "polished_marble",
        1.5,
        true,
        true,
    ),
    block(BASALT, "Basalt", "basalt", 1.5, true, true),
    block(
        POLISHED_BASALT,
        "Polished Basalt",
        "polished_basalt",
        1.5,
        true,
        true,
    ),
    block(SLATE, "Slate", "slate", 1.5, true, true),
    block(
        POLISHED_SLATE,
        "Polished Slate",
        "polished_slate",
        1.5,
        true,
        true,
    ),
    block(LIMESTONE, "Limestone", "limestone", 1.5, true, true),
    block(
        POLISHED_LIMESTONE,
        "Polished Limestone",
        "polished_limestone",
        1.5,
        true,
        true,
    ),
    block(PINE_LOG, "Pine Log", "pine_log", 2.0, true, true),
    block(PINE_LEAVES, "Pine Leaves", "pine_leaves", 0.2, true, false),
    block(PINE_PLANKS, "Pine Planks", "pine_planks", 2.0, true, true),
    block(
        SANDSTONE_BRICKS,
        "Sandstone Bricks",
        "sandstone_bricks",
        1.8,
        true,
        true,
    ),
    block(
        MOSSY_STONE_BRICKS,
        "Mossy Stone Bricks",
        "mossy_stone_bricks",
        1.8,
        true,
        true,
    ),
    block(
        CRACKED_STONE_BRICKS,
        "Cracked Stone Bricks",
        "cracked_stone_bricks",
        1.8,
        true,
        true,
    ),
    block(DARK_BRICKS, "Dark Bricks", "dark_bricks", 1.8, true, true),
    block(STONE_TILES, "Stone Tiles", "stone_tiles", 1.8, true, true),
];

/// Блок без состояния «горит»: в таком виде он выпадает и сохраняется.
#[must_use]
pub fn unlit(id: u8) -> u8 {
    match id {
        FURNACE_LIT => FURNACE,
        SMOKER_LIT => SMOKER,
        BLAST_FURNACE_LIT => BLAST_FURNACE,
        other => other,
    }
}

/// Листва получает объёмную геометрию и не сливается в greedy-грани.
#[must_use]
pub fn is_leaves(id: u8) -> bool {
    matches!(id, LEAVES | BIRCH_LEAVES | PINE_LEAVES)
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
        assert_eq!(BLOCKS.len(), 98);
        for (offset, name) in expected.iter().enumerate() {
            let id = 17 + offset;
            assert_eq!(BLOCKS[id].id as usize, id);
            assert_eq!(BLOCKS[id].name, *name);
            assert!(is_solid(id as u8) && is_opaque(id as u8));
        }
    }

    #[test]
    fn chest_extends_registry_without_changing_foundation_ids() {
        assert_eq!(BLOCKS.len(), 98);
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
