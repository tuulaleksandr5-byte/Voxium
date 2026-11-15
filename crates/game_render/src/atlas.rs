//! Lossless in-memory assembly of the supplied block texture PNGs.

use image::{GenericImage, RgbaImage};
use std::io::Cursor;

const TILE_SIZE: u32 = 128;
const ATLAS_WIDTH: u32 = TILE_SIZE * 6;
const SOURCE_HEIGHT: u32 = TILE_SIZE * 3;
const ATLAS_HEIGHT: u32 = TILE_SIZE * crate::ATLAS_ROWS;
const EXTRA_FIRST_LAYER: usize = 36;
const SOURCE_TILE_COUNT: usize = 17;

/// Layer order for the fifteen additional 128×128 PNGs (layers 18…32).
pub const ADDITIONAL_TILE_NAMES: [&str; 15] = [
    "gold_ore",
    "coal_ore",
    "diamond_ore",
    "gold_block",
    "diamond_block",
    "iron_ore",
    "iron_block",
    "crafting_table_top",
    "crafting_table_front",
    "crafting_table_side",
    "crafting_table_bottom",
    "furnace_front",
    "furnace_front_on",
    "furnace_side",
    "furnace_top",
];
const ADDITIONAL_TILES: [&[u8]; 15] = [
    include_bytes!("../../../assets/textures/blocks/128x128/gold_ore.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/coal_ore.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/diamond_ore.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/gold_block.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/diamond_block.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/iron_ore.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/iron_block.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_top.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_front.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_side.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_bottom.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/furnace_front.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/furnace_front_on.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/furnace_side.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/furnace_top.png"),
];

macro_rules! pixel_tile {
    ($name:literal) => {
        (
            $name,
            include_bytes!(concat!(
                "../../../assets/textures/pixel/128x128/",
                $name,
                ".png"
            )) as &[u8],
        )
    };
}
/// Пиксельные тайлы новых блоков: слои 36…54 в обоих стилях графики.
const EXTRA_TILES: [(&str, &[u8]); 403] = [
    pixel_tile!("bricks"),
    pixel_tile!("stone_bricks"),
    pixel_tile!("mossy_cobblestone"),
    pixel_tile!("birch_log"),
    pixel_tile!("birch_log_top"),
    pixel_tile!("birch_leaves"),
    pixel_tile!("birch_planks"),
    pixel_tile!("clay"),
    pixel_tile!("bookshelf"),
    pixel_tile!("lamp"),
    pixel_tile!("ice"),
    pixel_tile!("obsidian"),
    pixel_tile!("wool"),
    pixel_tile!("terracotta"),
    pixel_tile!("smooth_stone"),
    pixel_tile!("coal_block"),
    pixel_tile!("chest_front"),
    pixel_tile!("chest_side"),
    pixel_tile!("chest_top"),
    pixel_tile!("barrel_side"),
    pixel_tile!("barrel_top"),
    pixel_tile!("smoker_front"),
    pixel_tile!("smoker_front_on"),
    pixel_tile!("smoker_side"),
    pixel_tile!("blast_furnace_front"),
    pixel_tile!("blast_furnace_front_on"),
    pixel_tile!("blast_furnace_side"),
    pixel_tile!("stonecutter_top"),
    pixel_tile!("stonecutter_side"),
    pixel_tile!("vault_front"),
    pixel_tile!("vault_side"),
    pixel_tile!("vault_top"),
    pixel_tile!("wool_orange"),
    pixel_tile!("wool_magenta"),
    pixel_tile!("wool_light_blue"),
    pixel_tile!("wool_yellow"),
    pixel_tile!("wool_lime"),
    pixel_tile!("wool_pink"),
    pixel_tile!("wool_gray"),
    pixel_tile!("wool_light_gray"),
    pixel_tile!("wool_cyan"),
    pixel_tile!("wool_purple"),
    pixel_tile!("wool_blue"),
    pixel_tile!("wool_brown"),
    pixel_tile!("wool_green"),
    pixel_tile!("wool_red"),
    pixel_tile!("wool_black"),
    pixel_tile!("concrete_white"),
    pixel_tile!("concrete_orange"),
    pixel_tile!("concrete_magenta"),
    pixel_tile!("concrete_light_blue"),
    pixel_tile!("concrete_yellow"),
    pixel_tile!("concrete_lime"),
    pixel_tile!("concrete_pink"),
    pixel_tile!("concrete_gray"),
    pixel_tile!("concrete_light_gray"),
    pixel_tile!("concrete_cyan"),
    pixel_tile!("concrete_purple"),
    pixel_tile!("concrete_blue"),
    pixel_tile!("concrete_brown"),
    pixel_tile!("concrete_green"),
    pixel_tile!("concrete_red"),
    pixel_tile!("concrete_black"),
    pixel_tile!("granite"),
    pixel_tile!("polished_granite"),
    pixel_tile!("marble"),
    pixel_tile!("polished_marble"),
    pixel_tile!("basalt"),
    pixel_tile!("polished_basalt"),
    pixel_tile!("slate"),
    pixel_tile!("polished_slate"),
    pixel_tile!("limestone"),
    pixel_tile!("polished_limestone"),
    pixel_tile!("pine_log"),
    pixel_tile!("pine_log_top"),
    pixel_tile!("pine_leaves"),
    pixel_tile!("pine_planks"),
    pixel_tile!("sandstone_bricks"),
    pixel_tile!("mossy_stone_bricks"),
    pixel_tile!("cracked_stone_bricks"),
    pixel_tile!("dark_bricks"),
    pixel_tile!("stone_tiles"),
    pixel_tile!("poppy"),
    pixel_tile!("dandelion"),
    pixel_tile!("cornflower"),
    pixel_tile!("daisy"),
    pixel_tile!("tall_grass"),
    pixel_tile!("acacia_log"),
    pixel_tile!("acacia_log_top"),
    pixel_tile!("acacia_leaves"),
    pixel_tile!("acacia_planks"),
    pixel_tile!("jungle_log"),
    pixel_tile!("jungle_log_top"),
    pixel_tile!("jungle_leaves"),
    pixel_tile!("jungle_planks"),
    pixel_tile!("dark_oak_log"),
    pixel_tile!("dark_oak_log_top"),
    pixel_tile!("dark_oak_leaves"),
    pixel_tile!("dark_oak_planks"),
    pixel_tile!("mangrove_log"),
    pixel_tile!("mangrove_log_top"),
    pixel_tile!("mangrove_leaves"),
    pixel_tile!("mangrove_planks"),
    pixel_tile!("cherry_log"),
    pixel_tile!("cherry_log_top"),
    pixel_tile!("cherry_leaves"),
    pixel_tile!("cherry_planks"),
    pixel_tile!("copper_ore"),
    pixel_tile!("emerald_ore"),
    pixel_tile!("lapis_ore"),
    pixel_tile!("copper_block"),
    pixel_tile!("emerald_block"),
    pixel_tile!("lapis_block"),
    pixel_tile!("glass_white"),
    pixel_tile!("glass_orange"),
    pixel_tile!("glass_magenta"),
    pixel_tile!("glass_light_blue"),
    pixel_tile!("glass_yellow"),
    pixel_tile!("glass_lime"),
    pixel_tile!("glass_pink"),
    pixel_tile!("glass_gray"),
    pixel_tile!("glass_light_gray"),
    pixel_tile!("glass_cyan"),
    pixel_tile!("glass_purple"),
    pixel_tile!("glass_blue"),
    pixel_tile!("glass_brown"),
    pixel_tile!("glass_green"),
    pixel_tile!("glass_red"),
    pixel_tile!("glass_black"),
    pixel_tile!("terracotta_white"),
    pixel_tile!("terracotta_orange"),
    pixel_tile!("terracotta_magenta"),
    pixel_tile!("terracotta_light_blue"),
    pixel_tile!("terracotta_yellow"),
    pixel_tile!("terracotta_lime"),
    pixel_tile!("terracotta_pink"),
    pixel_tile!("terracotta_gray"),
    pixel_tile!("terracotta_light_gray"),
    pixel_tile!("terracotta_cyan"),
    pixel_tile!("terracotta_purple"),
    pixel_tile!("terracotta_blue"),
    pixel_tile!("terracotta_brown"),
    pixel_tile!("terracotta_green"),
    pixel_tile!("terracotta_red"),
    pixel_tile!("terracotta_black"),
    pixel_tile!("sponge"),
    pixel_tile!("melon_side"),
    pixel_tile!("melon_top"),
    pixel_tile!("pumpkin_side"),
    pixel_tile!("pumpkin_top"),
    pixel_tile!("hay_bale_side"),
    pixel_tile!("hay_bale_top"),
    pixel_tile!("bone_block_side"),
    pixel_tile!("bone_block_top"),
    pixel_tile!("mud"),
    pixel_tile!("packed_mud"),
    pixel_tile!("mud_bricks"),
    pixel_tile!("moss_block"),
    pixel_tile!("podzol_top"),
    pixel_tile!("podzol_side"),
    pixel_tile!("coarse_dirt"),
    pixel_tile!("red_sand"),
    pixel_tile!("red_sandstone"),
    pixel_tile!("calcite"),
    pixel_tile!("tuff"),
    pixel_tile!("amethyst_block"),
    pixel_tile!("quartz_block"),
    pixel_tile!("lava"),
    pixel_tile!("mangrove_roots"),
    pixel_tile!("muddy_mangrove_roots"),
    pixel_tile!("spruce_leaves"),
    pixel_tile!("azalea_leaves"),
    pixel_tile!("flowering_azalea_leaves"),
    pixel_tile!("oak_sapling"),
    pixel_tile!("spruce_sapling"),
    pixel_tile!("birch_sapling"),
    pixel_tile!("jungle_sapling"),
    pixel_tile!("acacia_sapling"),
    pixel_tile!("cherry_sapling"),
    pixel_tile!("dark_oak_sapling"),
    pixel_tile!("mangrove_propagule"),
    pixel_tile!("cobweb"),
    pixel_tile!("grass"),
    pixel_tile!("fern"),
    pixel_tile!("dead_bush"),
    pixel_tile!("seagrass"),
    pixel_tile!("tall_seagrass"),
    pixel_tile!("torchflower"),
    pixel_tile!("blue_orchid"),
    pixel_tile!("allium"),
    pixel_tile!("azure_bluet"),
    pixel_tile!("red_tulip"),
    pixel_tile!("orange_tulip"),
    pixel_tile!("white_tulip"),
    pixel_tile!("pink_tulip"),
    pixel_tile!("wither_rose"),
    pixel_tile!("lily_of_the_valley"),
    pixel_tile!("brown_mushroom"),
    pixel_tile!("red_mushroom"),
    pixel_tile!("torch"),
    pixel_tile!("fire"),
    pixel_tile!("soul_fire"),
    pixel_tile!("powered_rail"),
    pixel_tile!("detector_rail"),
    pixel_tile!("diorite"),
    pixel_tile!("polished_diorite"),
    pixel_tile!("andesite"),
    pixel_tile!("polished_andesite"),
    pixel_tile!("spruce_planks"),
    pixel_tile!("bamboo_planks"),
    pixel_tile!("bamboo_mosaic"),
    pixel_tile!("suspicious_sand"),
    pixel_tile!("suspicious_gravel"),
    pixel_tile!("deepslate_gold_ore"),
    pixel_tile!("deepslate_iron_ore"),
    pixel_tile!("deepslate_coal_ore"),
    pixel_tile!("nether_gold_ore"),
    pixel_tile!("spruce_log_top"),
    pixel_tile!("spruce_log"),
    pixel_tile!("bamboo_block_top"),
    pixel_tile!("bamboo_block"),
    pixel_tile!("stripped_spruce_log_top"),
    pixel_tile!("stripped_spruce_log"),
    pixel_tile!("stripped_birch_log_top"),
    pixel_tile!("stripped_birch_log"),
    pixel_tile!("stripped_jungle_log_top"),
    pixel_tile!("stripped_jungle_log"),
    pixel_tile!("stripped_acacia_log_top"),
    pixel_tile!("stripped_acacia_log"),
    pixel_tile!("stripped_cherry_log_top"),
    pixel_tile!("stripped_cherry_log"),
    pixel_tile!("stripped_dark_oak_log_top"),
    pixel_tile!("stripped_dark_oak_log"),
    pixel_tile!("stripped_oak_log_top"),
    pixel_tile!("stripped_oak_log"),
    pixel_tile!("stripped_mangrove_log_top"),
    pixel_tile!("stripped_mangrove_log"),
    pixel_tile!("stripped_bamboo_block_top"),
    pixel_tile!("stripped_bamboo_block"),
    pixel_tile!("wet_sponge"),
    pixel_tile!("deepslate_lapis_ore"),
    pixel_tile!("dispenser_side"),
    pixel_tile!("dispenser_front"),
    pixel_tile!("chiseled_sandstone"),
    pixel_tile!("cut_sandstone"),
    pixel_tile!("note_block"),
    pixel_tile!("sticky_piston_top"),
    pixel_tile!("piston_side"),
    pixel_tile!("piston_bottom"),
    pixel_tile!("piston_top"),
    pixel_tile!("tnt_top"),
    pixel_tile!("tnt_side"),
    pixel_tile!("chiseled_bookshelf_side"),
    pixel_tile!("chiseled_bookshelf_front"),
    pixel_tile!("bed_top_white"),
    pixel_tile!("bed_side_white"),
    pixel_tile!("bed_top_orange"),
    pixel_tile!("bed_side_orange"),
    pixel_tile!("bed_top_magenta"),
    pixel_tile!("bed_side_magenta"),
    pixel_tile!("bed_top_light_blue"),
    pixel_tile!("bed_side_light_blue"),
    pixel_tile!("bed_top_yellow"),
    pixel_tile!("bed_side_yellow"),
    pixel_tile!("bed_top_lime"),
    pixel_tile!("bed_side_lime"),
    pixel_tile!("bed_top_pink"),
    pixel_tile!("bed_side_pink"),
    pixel_tile!("bed_top_gray"),
    pixel_tile!("bed_side_gray"),
    pixel_tile!("bed_top_light_gray"),
    pixel_tile!("bed_side_light_gray"),
    pixel_tile!("bed_top_cyan"),
    pixel_tile!("bed_side_cyan"),
    pixel_tile!("bed_top_purple"),
    pixel_tile!("bed_side_purple"),
    pixel_tile!("bed_top_blue"),
    pixel_tile!("bed_side_blue"),
    pixel_tile!("bed_top_brown"),
    pixel_tile!("bed_side_brown"),
    pixel_tile!("bed_top_green"),
    pixel_tile!("bed_side_green"),
    pixel_tile!("bed_top_red"),
    pixel_tile!("bed_side_red"),
    pixel_tile!("bed_top_black"),
    pixel_tile!("bed_side_black"),
    pixel_tile!("wheat"),
    pixel_tile!("sugar_cane"),
    pixel_tile!("redstone_wire"),
    pixel_tile!("redstone_torch"),
    pixel_tile!("soul_torch"),
    pixel_tile!("ladder"),
    pixel_tile!("rail"),
    pixel_tile!("nether_portal"),
    pixel_tile!("iron_bars"),
    pixel_tile!("chain"),
    pixel_tile!("glass_pane"),
    pixel_tile!("spawner"),
    pixel_tile!("redstone_ore"),
    pixel_tile!("deepslate_redstone_ore"),
    pixel_tile!("deepslate_diamond_ore"),
    pixel_tile!("farmland_top"),
    pixel_tile!("cactus_side"),
    pixel_tile!("cactus_top"),
    pixel_tile!("jukebox_side"),
    pixel_tile!("jukebox_top"),
    pixel_tile!("netherrack"),
    pixel_tile!("soul_sand"),
    pixel_tile!("soul_soil"),
    pixel_tile!("glowstone"),
    pixel_tile!("carved_pumpkin_front"),
    pixel_tile!("jack_o_lantern_front"),
    pixel_tile!("cake_top"),
    pixel_tile!("cake_side"),
    pixel_tile!("repeater_top"),
    pixel_tile!("oak_door_bottom"),
    pixel_tile!("oak_door_top"),
    pixel_tile!("iron_door_bottom"),
    pixel_tile!("iron_door_top"),
    pixel_tile!("chiseled_stone_bricks"),
    pixel_tile!("brown_mushroom_block"),
    pixel_tile!("red_mushroom_block"),
    pixel_tile!("mushroom_stem"),
    pixel_tile!("crimson_planks"),
    pixel_tile!("warped_planks"),
    pixel_tile!("lever"),
    pixel_tile!("oak_trapdoor"),
    pixel_tile!("spruce_trapdoor"),
    pixel_tile!("birch_trapdoor"),
    pixel_tile!("jungle_trapdoor"),
    pixel_tile!("acacia_trapdoor"),
    pixel_tile!("cherry_trapdoor"),
    pixel_tile!("dark_oak_trapdoor"),
    pixel_tile!("mangrove_trapdoor"),
    pixel_tile!("bamboo_trapdoor"),
    pixel_tile!("vine"),
    pixel_tile!("glow_lichen"),
    pixel_tile!("lily_pad"),
    pixel_tile!("nether_wart"),
    pixel_tile!("cocoa"),
    pixel_tile!("tripwire"),
    pixel_tile!("carrots"),
    pixel_tile!("potatoes"),
    pixel_tile!("pumpkin_stem"),
    pixel_tile!("melon_stem"),
    pixel_tile!("attached_pumpkin_stem"),
    pixel_tile!("attached_melon_stem"),
    pixel_tile!("brewing_stand"),
    pixel_tile!("mycelium_top"),
    pixel_tile!("mycelium_side"),
    pixel_tile!("nether_bricks"),
    pixel_tile!("enchanting_table_top"),
    pixel_tile!("enchanting_table_side"),
    pixel_tile!("cauldron_side"),
    pixel_tile!("cauldron_top"),
    pixel_tile!("powder_snow"),
    pixel_tile!("end_portal"),
    pixel_tile!("end_portal_frame_top"),
    pixel_tile!("end_portal_frame_side"),
    pixel_tile!("end_stone"),
    pixel_tile!("dragon_egg"),
    pixel_tile!("redstone_lamp"),
    pixel_tile!("deepslate_emerald_ore"),
    pixel_tile!("ender_chest"),
    pixel_tile!("tripwire_hook"),
    pixel_tile!("command_block"),
    pixel_tile!("beacon"),
    pixel_tile!("flower_pot"),
    pixel_tile!("daylight_detector_top"),
    pixel_tile!("daylight_detector_side"),
    pixel_tile!("anvil"),
    pixel_tile!("hopper"),
    pixel_tile!("quartz_ore"),
    pixel_tile!("chiseled_quartz_block"),
    pixel_tile!("redstone_block"),
    pixel_tile!("comparator_top"),
    pixel_tile!("skeleton_skull"),
    pixel_tile!("wither_skull"),
    pixel_tile!("zombie_head"),
    pixel_tile!("player_head"),
    pixel_tile!("creeper_head"),
    pixel_tile!("dragon_head"),
    pixel_tile!("piglin_head"),
    pixel_tile!("activator_rail"),
    pixel_tile!("barrier"),
    pixel_tile!("light"),
    pixel_tile!("sunflower"),
    pixel_tile!("lilac"),
    pixel_tile!("rose_bush"),
    pixel_tile!("peony"),
    pixel_tile!("large_fern"),
    pixel_tile!("quartz_pillar_top"),
    pixel_tile!("quartz_pillar_side"),
    pixel_tile!("dropper_front"),
    pixel_tile!("slime_block"),
    pixel_tile!("iron_trapdoor"),
    pixel_tile!("prismarine"),
    pixel_tile!("prismarine_bricks"),
    pixel_tile!("dark_prismarine"),
    pixel_tile!("sea_lantern"),
    pixel_tile!("packed_ice"),
    pixel_tile!("chiseled_red_sandstone"),
    pixel_tile!("cut_red_sandstone"),
];
/// Пиксельные версии семнадцати исходных тайлов (слои 0…16).
const PIXEL_SOURCE_TILES: [(&str, &[u8]); 17] = [
    pixel_tile!("grass_top"),
    pixel_tile!("grass_side"),
    pixel_tile!("dirt"),
    pixel_tile!("stone"),
    pixel_tile!("deepslate"),
    pixel_tile!("sand"),
    pixel_tile!("gravel"),
    pixel_tile!("oak_log"),
    pixel_tile!("oak_log_top"),
    pixel_tile!("oak_leaves"),
    pixel_tile!("oak_planks"),
    pixel_tile!("cobblestone"),
    pixel_tile!("glass"),
    pixel_tile!("water"),
    pixel_tile!("snow"),
    pixel_tile!("sandstone"),
    pixel_tile!("sandstone_top"),
];
/// Пиксельные версии пятнадцати дополнительных тайлов (слои 18…32).
const PIXEL_ADDITIONAL_TILES: [(&str, &[u8]); 15] = [
    pixel_tile!("gold_ore"),
    pixel_tile!("coal_ore"),
    pixel_tile!("diamond_ore"),
    pixel_tile!("gold_block"),
    pixel_tile!("diamond_block"),
    pixel_tile!("iron_ore"),
    pixel_tile!("iron_block"),
    pixel_tile!("crafting_table_top"),
    pixel_tile!("crafting_table_front"),
    pixel_tile!("crafting_table_side"),
    pixel_tile!("crafting_table_bottom"),
    pixel_tile!("furnace_front"),
    pixel_tile!("furnace_front_on"),
    pixel_tile!("furnace_side"),
    pixel_tile!("furnace_top"),
];

/// Имена дополнительных пиксельных тайлов в порядке слоёв, начиная с 36.
pub fn extra_tile_names() -> impl Iterator<Item = &'static str> {
    EXTRA_TILES.iter().map(|(name, _)| *name)
}

/// Decoded original block artwork, shared by world materials and hotbar icons.
///
/// Assembly copies decoded RGBA pixels without resizing, flipping, repainting,
/// or re-encoding the source PNG files. The internal 6×6 atlas appends the
/// embedded additional tiles at layers 18…32. Tile assembly leaves layer 17
/// transparent; whole-atlas loading preserves all eighteen source tiles.
/// Layers 33…35 remain transparent.
#[derive(Clone, Debug)]
pub struct BlockAtlas {
    pub(crate) rgba: RgbaImage,
    /// Пиксельный стиль: тайлы 32×32, заранее увеличенные 4× без сглаживания.
    pub pixelated: bool,
}

impl BlockAtlas {
    /// Decode an original 768×384 PNG with six columns and three rows.
    pub fn from_png(bytes: &[u8]) -> Result<Self, String> {
        Self::from_legacy_rgba(&decode_png(bytes, (ATLAS_WIDTH, SOURCE_HEIGHT), "atlas")?)
    }

    /// Assemble exactly seventeen original 128×128 PNGs in this layer order:
    /// grass_top, grass_side, dirt, stone, deepslate, sand, gravel, oak_log,
    /// oak_log_top, oak_leaves, oak_planks, cobblestone, glass, water, snow,
    /// sandstone, sandstone_top. The unused eighteenth layer stays transparent.
    pub fn from_tiles(tiles: &[&[u8]]) -> Result<Self, String> {
        if tiles.len() != SOURCE_TILE_COUNT {
            return Err(format!(
                "expected {SOURCE_TILE_COUNT} block texture PNGs, got {}",
                tiles.len()
            ));
        }
        let mut rgba = RgbaImage::new(ATLAS_WIDTH, ATLAS_HEIGHT);
        for (layer, bytes) in tiles.iter().enumerate() {
            let tile = decode_png(bytes, (TILE_SIZE, TILE_SIZE), &format!("tile {layer}"))?;
            rgba.copy_from(
                &tile,
                (layer as u32 % 6) * TILE_SIZE,
                (layer as u32 / 6) * TILE_SIZE,
            )
            .map_err(|e| format!("cannot copy tile {layer}: {e}"))?;
        }
        Self {
            rgba,
            pixelated: false,
        }
        .with_extra_tiles()?
        .with_additional_tiles(&ADDITIONAL_TILES)
    }

    pub(crate) fn from_legacy_rgba(source: &RgbaImage) -> Result<Self, String> {
        let mut rgba = RgbaImage::new(ATLAS_WIDTH, ATLAS_HEIGHT);
        rgba.copy_from(source, 0, 0).map_err(|e| e.to_string())?;
        Self {
            rgba,
            pixelated: false,
        }
        .with_extra_tiles()?
        .with_additional_tiles(&ADDITIONAL_TILES)
    }

    /// Полностью пиксельный набор: оригинальная графика 32×32, увеличенная в
    /// 4 раза без сглаживания. Исходные PNG при этом не читаются и не меняются.
    pub fn pixel() -> Result<Self, String> {
        let mut atlas = Self {
            rgba: RgbaImage::new(ATLAS_WIDTH, ATLAS_HEIGHT),
            pixelated: true,
        }
        .with_extra_tiles()?;
        for (layer, (name, bytes)) in PIXEL_SOURCE_TILES.iter().enumerate() {
            atlas.copy_tile(layer, bytes, name)?;
        }
        for (offset, (name, bytes)) in PIXEL_ADDITIONAL_TILES.iter().enumerate() {
            atlas.copy_tile(offset + 18, bytes, name)?;
        }
        Ok(atlas)
    }

    fn with_extra_tiles(mut self) -> Result<Self, String> {
        for (offset, (name, bytes)) in EXTRA_TILES.iter().enumerate() {
            self.copy_tile(EXTRA_FIRST_LAYER + offset, bytes, name)?;
        }
        Ok(self)
    }

    fn copy_tile(&mut self, layer: usize, bytes: &[u8], name: &str) -> Result<(), String> {
        let tile = decode_png(bytes, (TILE_SIZE, TILE_SIZE), name)?;
        self.rgba
            .copy_from(
                &tile,
                (layer as u32 % 6) * TILE_SIZE,
                (layer as u32 / 6) * TILE_SIZE,
            )
            .map_err(|e| format!("cannot copy tile {layer}: {e}"))
    }

    /// Replace layers 18…32 with exactly fifteen supplied 128×128 PNGs.
    /// Source layers 0…17 are preserved, and all PNG pixels are copied exactly.
    /// The returned atlas can be shared by GPU materials and UI icon baking.
    pub fn with_additional_tiles(mut self, tiles: &[&[u8]]) -> Result<Self, String> {
        if tiles.len() != ADDITIONAL_TILE_NAMES.len() {
            return Err(format!(
                "expected 15 additional block texture PNGs, got {}",
                tiles.len()
            ));
        }
        for (offset, bytes) in tiles.iter().enumerate() {
            self.replace_additional_tile(offset, bytes)?;
        }
        Ok(self)
    }

    /// Заменить один дополнительный PNG; индекс 0…14 соответствует слоям 18…32.
    /// Проверка индекса и полное декодирование предшествуют изменению атласа.
    /// При ошибке текущие пиксели, включая все исходные слои, сохраняются.
    pub fn replace_additional_tile(&mut self, offset: usize, bytes: &[u8]) -> Result<(), String> {
        let name = ADDITIONAL_TILE_NAMES
            .get(offset)
            .ok_or_else(|| format!("additional tile index must be in 0..15, got {offset}"))?;
        let tile = decode_png(bytes, (TILE_SIZE, TILE_SIZE), name)?;
        let layer = offset + 18;
        self.rgba
            .copy_from(
                &tile,
                (layer as u32 % 6) * TILE_SIZE,
                (layer as u32 / 6) * TILE_SIZE,
            )
            .map_err(|e| format!("cannot copy tile {layer}: {e}"))
    }
}

fn decode_png(bytes: &[u8], expected: (u32, u32), name: &str) -> Result<RgbaImage, String> {
    let dimensions = image::ImageReader::with_format(Cursor::new(bytes), image::ImageFormat::Png)
        .into_dimensions()
        .map_err(|e| format!("cannot read {name} PNG: {e}"))?;
    if dimensions != expected {
        return Err(format!(
            "{name} must be {}×{}, got {}×{}",
            expected.0, expected.1, dimensions.0, dimensions.1
        ));
    }
    image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .map_err(|e| format!("cannot decode {name} PNG: {e}"))
        .map(image::DynamicImage::into_rgba8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas_array;

    const ORIGINAL_TILES: [&[u8]; SOURCE_TILE_COUNT] = [
        include_bytes!("../../../assets/textures/grass_top.png"),
        include_bytes!("../../../assets/textures/grass_side.png"),
        include_bytes!("../../../assets/textures/dirt.png"),
        include_bytes!("../../../assets/textures/stone.png"),
        include_bytes!("../../../assets/textures/deepslate.png"),
        include_bytes!("../../../assets/textures/sand.png"),
        include_bytes!("../../../assets/textures/gravel.png"),
        include_bytes!("../../../assets/textures/oak_log.png"),
        include_bytes!("../../../assets/textures/oak_log_top.png"),
        include_bytes!("../../../assets/textures/oak_leaves.png"),
        include_bytes!("../../../assets/textures/oak_planks.png"),
        include_bytes!("../../../assets/textures/cobblestone.png"),
        include_bytes!("../../../assets/textures/glass.png"),
        include_bytes!("../../../assets/textures/water.png"),
        include_bytes!("../../../assets/textures/snow.png"),
        include_bytes!("../../../assets/textures/sandstone.png"),
        include_bytes!("../../../assets/textures/sandstone_top.png"),
    ];

    #[test]
    fn single_additional_replacement_rejects_invalid_input_without_mutating_any_layer() {
        let mut atlas = BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap();
        let before = atlas.rgba.clone();
        for offset in [15, 18, 36, usize::MAX] {
            assert!(
                atlas
                    .replace_additional_tile(offset, ADDITIONAL_TILES[0])
                    .is_err()
            );
            assert_eq!(atlas.rgba, before);
        }
        let mut wrong_size = Cursor::new(Vec::new());
        RgbaImage::new(128, 127)
            .write_to(&mut wrong_size, image::ImageFormat::Png)
            .unwrap();
        for offset in [0, 14] {
            for invalid in [b"invalid PNG".as_slice(), wrong_size.get_ref()] {
                assert!(atlas.replace_additional_tile(offset, invalid).is_err());
                assert_eq!(atlas.rgba, before);
            }
        }
    }

    #[test]
    fn legacy_whole_atlas_and_raw_rgba_preserve_all_eighteen_source_layers() {
        let source = RgbaImage::from_fn(768, 384, |x, y| {
            image::Rgba([
                (x % 256) as u8,
                (y % 256) as u8,
                ((x + y) % 256) as u8,
                ((x * 3 + y) % 256) as u8,
            ])
        });
        let mut png = Cursor::new(Vec::new());
        source.write_to(&mut png, image::ImageFormat::Png).unwrap();
        let atlas = BlockAtlas::from_png(png.get_ref()).unwrap();
        assert_eq!(
            image::imageops::crop_imm(&atlas.rgba, 0, 0, 768, 384).to_image(),
            source
        );
        let legacy_gpu = atlas_array(source.as_raw()).unwrap();
        let expanded_gpu = atlas_array(atlas.rgba.as_raw()).unwrap();
        assert_eq!(legacy_gpu.data, expanded_gpu.data);
        let data = expanded_gpu.data.unwrap();
        let stride = data.len() / crate::MATERIAL_COUNT;
        for layer in 0..18 {
            let expected =
                image::imageops::crop_imm(&source, (layer % 6) * 128, (layer / 6) * 128, 128, 128)
                    .to_image();
            assert_eq!(
                &data[layer as usize * stride..][..128 * 128 * 4],
                expected.as_raw(),
                "legacy layer {layer}"
            );
        }
    }

    #[test]
    fn supplied_additional_tiles_replace_defaults_without_touching_originals() {
        let atlas = BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap();
        let mut png = Cursor::new(Vec::new());
        let source = RgbaImage::from_fn(128, 128, |x, y| image::Rgba([x as u8, y as u8, 42, 100]));
        source.write_to(&mut png, image::ImageFormat::Png).unwrap();
        let updated = atlas
            .clone()
            .with_additional_tiles(&[png.get_ref().as_slice(); 15])
            .unwrap();
        assert_eq!(
            image::imageops::crop_imm(&updated.rgba, 0, 0, 768, 384).to_image(),
            image::imageops::crop_imm(&atlas.rgba, 0, 0, 768, 384).to_image()
        );
        for layer in 18..33 {
            assert_eq!(
                image::imageops::crop_imm(
                    &updated.rgba,
                    (layer % 6) * 128,
                    (layer / 6) * 128,
                    128,
                    128
                )
                .to_image(),
                source
            );
        }
        assert!(atlas.clone().with_additional_tiles(&[]).is_err());
        assert!(
            atlas
                .clone()
                .with_additional_tiles(&ADDITIONAL_TILES[..14])
                .is_err()
        );
        assert!(
            atlas
                .clone()
                .with_additional_tiles(&[ADDITIONAL_TILES[0]; 16])
                .is_err()
        );
        assert!(
            atlas
                .clone()
                .with_additional_tiles(&[b"invalid PNG".as_slice(); 15])
                .unwrap_err()
                .contains("gold_ore")
        );
        let mut wrong_size = Cursor::new(Vec::new());
        RgbaImage::new(127, 128)
            .write_to(&mut wrong_size, image::ImageFormat::Png)
            .unwrap();
        assert!(
            atlas
                .with_additional_tiles(&[wrong_size.get_ref().as_slice(); 15])
                .unwrap_err()
                .contains("128×128")
        );
    }

    #[test]
    fn legacy_apis_append_new_tiles_with_exact_rgba_and_empty_reserved_layers() {
        let mut source = Cursor::new(Vec::new());
        RgbaImage::from_pixel(768, 384, image::Rgba([23, 45, 67, 89]))
            .write_to(&mut source, image::ImageFormat::Png)
            .unwrap();
        let tiles: [&[u8]; 15] = [
            include_bytes!("../../../assets/textures/blocks/128x128/gold_ore.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/coal_ore.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/diamond_ore.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/gold_block.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/diamond_block.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/iron_ore.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/iron_block.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_top.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_front.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_side.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_bottom.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/furnace_front.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/furnace_front_on.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/furnace_side.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/furnace_top.png"),
        ];
        for atlas in [
            BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap(),
            BlockAtlas::from_png(source.get_ref()).unwrap(),
        ] {
            assert_eq!(atlas.rgba.dimensions(), (768, ATLAS_HEIGHT));
            let gpu = atlas_array(atlas.rgba.as_raw()).unwrap();
            let data = gpu.data.unwrap();
            let stride = data.len() / crate::MATERIAL_COUNT;
            for (offset, png) in tiles.iter().enumerate() {
                let layer = offset + 18;
                let expected = image::load_from_memory(png).unwrap().into_rgba8();
                let actual = image::imageops::crop_imm(
                    &atlas.rgba,
                    (layer as u32 % 6) * 128,
                    (layer as u32 / 6) * 128,
                    128,
                    128,
                )
                .to_image();
                assert_eq!(actual, expected, "atlas layer {layer}");
                assert_eq!(
                    &data[layer * stride..][..128 * 128 * 4],
                    expected.as_raw(),
                    "GPU layer {layer}"
                );
            }
            for layer in [33, 34, 35] {
                assert!(
                    data[layer * stride..(layer + 1) * stride]
                        .iter()
                        .all(|&b| b == 0)
                );
            }
        }
    }

    #[test]
    fn uploaded_tiles_preserve_every_rgba_pixel_in_atlas_and_gpu_base_layers() {
        let atlas = BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap();
        let gpu = atlas_array(atlas.rgba.as_raw()).unwrap();
        let gpu_bytes = gpu.data.as_ref().unwrap();
        let layer_stride = (0..8).map(|m| (128_usize >> m).pow(2) * 4).sum::<usize>();
        for (layer, bytes) in ORIGINAL_TILES.iter().enumerate() {
            let original = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
                .unwrap()
                .into_rgba8();
            assert_eq!(original.dimensions(), (TILE_SIZE, TILE_SIZE));
            for (x, y, pixel) in original.enumerate_pixels() {
                assert_eq!(
                    atlas.rgba.get_pixel(
                        (layer as u32 % 6) * TILE_SIZE + x,
                        (layer as u32 / 6) * TILE_SIZE + y
                    ),
                    pixel,
                    "original tile {layer} pixel {x},{y} changed"
                );
            }
            assert_eq!(
                &gpu_bytes[layer * layer_stride..][..original.as_raw().len()],
                original.as_raw(),
                "GPU base RGBA differs from original tile {layer}"
            );
        }
        assert!(
            gpu_bytes[SOURCE_TILE_COUNT * layer_stride..18 * layer_stride]
                .iter()
                .all(|&v| v == 0)
        );
    }

    #[test]
    fn uploaded_tiles_keep_rgba_in_each_isolated_mip_chain() {
        let atlas = BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap();
        let combined = atlas_array(atlas.rgba.as_raw()).unwrap();
        let combined_bytes = combined.data.as_ref().unwrap();
        let layer_stride = combined_bytes.len() / crate::MATERIAL_COUNT;
        for (layer, bytes) in ORIGINAL_TILES.iter().enumerate() {
            // Repeat just this source tile into every slot. Its mip chain must
            // match the chain built among the seventeen different real images,
            // so no filtering tap can bleed neighboring artwork or alpha.
            let isolated = BlockAtlas::from_tiles(&[*bytes; SOURCE_TILE_COUNT]).unwrap();
            let isolated_gpu = atlas_array(isolated.rgba.as_raw()).unwrap();
            assert_eq!(
                &combined_bytes[layer * layer_stride..][..layer_stride],
                &isolated_gpu.data.as_ref().unwrap()[..layer_stride],
                "RGBA mip chain for source tile {layer} mixes neighboring tiles"
            );
        }
    }

    #[test]
    fn tile_count_dimensions_and_invalid_pngs_are_rejected() {
        assert!(BlockAtlas::from_tiles(&[]).unwrap_err().contains("17"));
        assert!(BlockAtlas::from_tiles(&ORIGINAL_TILES[..16]).is_err());
        let mut too_many = ORIGINAL_TILES.to_vec();
        too_many.push(ORIGINAL_TILES[0]);
        assert!(BlockAtlas::from_tiles(&too_many).is_err());
        let mut wrong_size = Cursor::new(Vec::new());
        RgbaImage::new(128, 127)
            .write_to(&mut wrong_size, image::ImageFormat::Png)
            .unwrap();
        let mut tiles = ORIGINAL_TILES;
        tiles[6] = wrong_size.get_ref();
        let error = BlockAtlas::from_tiles(&tiles).unwrap_err();
        assert!(error.contains("tile 6") && error.contains("128×128"));
        tiles[6] = b"invalid PNG";
        assert!(
            BlockAtlas::from_tiles(&tiles)
                .unwrap_err()
                .contains("tile 6")
        );
    }

    #[test]
    fn pixel_set_fills_every_mapped_layer_and_keeps_leaf_cutouts() {
        let atlas = BlockAtlas::pixel().unwrap();
        assert!(atlas.pixelated);
        assert!(!BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap().pixelated);
        assert_eq!(extra_tile_names().count(), EXTRA_TILES.len());
        let tile = |layer: u32| {
            image::imageops::crop_imm(&atlas.rgba, (layer % 6) * 128, (layer / 6) * 128, 128, 128)
                .to_image()
        };
        for faces in crate::BlockTextures::default().0.iter().skip(1) {
            for layer in [
                Some(faces.top),
                Some(faces.side),
                Some(faces.bottom),
                faces.front,
            ]
            .into_iter()
            .flatten()
            {
                assert!(
                    tile(u32::from(layer)).pixels().any(|p| p[3] > 0),
                    "layer {layer} is empty"
                );
            }
        }
        for layer in [9, 41] {
            let leaves = tile(layer);
            assert!(leaves.pixels().any(|p| p[3] == 0));
            assert!(leaves.pixels().any(|p| p[3] == 255));
        }
        // 32×32 → 128×128 без сглаживания: каждый блок 4×4 одноцветный.
        let stone = tile(3);
        for (x, y, pixel) in stone.enumerate_pixels() {
            assert_eq!(pixel, stone.get_pixel(x / 4 * 4, y / 4 * 4));
        }
    }
}
