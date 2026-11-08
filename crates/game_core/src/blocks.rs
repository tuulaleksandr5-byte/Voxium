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

pub const BLOCKS: [BlockDef; 16] = [
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
];

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
        ];
        for (id, block) in BLOCKS.iter().enumerate() {
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
