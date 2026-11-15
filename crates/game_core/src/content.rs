//! Проверенный каталог предметов; числовые ID действуют только в текущем запуске.

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ItemId(pub u16);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum ArmorSlot {
    Head,
    Chest,
    Legs,
    Feet,
}

impl ArmorSlot {
    #[must_use]
    pub fn index(self) -> usize {
        match self {
            Self::Head => 0,
            Self::Chest => 1,
            Self::Legs => 2,
            Self::Feet => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ItemKind {
    Block { block: String },
    Material,
    Tool,
    Armor { slot: ArmorSlot },
    Food,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub enum BreakDrop {
    #[default]
    SelfItem,
    Nothing,
    Item(String),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub max_stack: u16,
    pub kind: ItemKind,
    #[serde(default)]
    pub break_drop: BreakDrop,
}

#[derive(Clone, Debug)]
pub struct ContentRegistry {
    definitions: Vec<ItemDef>,
    by_name: HashMap<String, ItemId>,
    by_block: HashMap<crate::BlockId, ItemId>,
}

// Имена соответствуют стабильным сохранённым ID; новые блоки только дописываются.
#[allow(clippy::large_const_arrays)]
pub(crate) const BLOCK_NAMES: [&str; 1041] = [
    "foundation:air",
    "foundation:grass",
    "foundation:dirt",
    "foundation:stone",
    "foundation:deepslate",
    "foundation:sand",
    "foundation:gravel",
    "foundation:oak_log",
    "foundation:oak_leaves",
    "foundation:oak_planks",
    "foundation:cobblestone",
    "foundation:glass",
    "foundation:water",
    "foundation:bedrock",
    "foundation:snow",
    "foundation:sandstone",
    "survival:chest",
    "survival:coal_ore",
    "survival:iron_ore",
    "survival:gold_ore",
    "survival:diamond_ore",
    "survival:iron_block",
    "survival:gold_block",
    "survival:diamond_block",
    "survival:crafting_table",
    "survival:furnace",
    "builder:bricks",
    "builder:stone_bricks",
    "builder:mossy_cobblestone",
    "builder:birch_log",
    "builder:birch_leaves",
    "builder:birch_planks",
    "builder:clay",
    "builder:bookshelf",
    "builder:lamp",
    "builder:ice",
    "builder:obsidian",
    "builder:wool",
    "builder:terracotta",
    "builder:smooth_stone",
    "builder:coal_block",
    "builder:furnace_lit",
    "maker:barrel",
    "maker:smoker",
    "maker:smoker_lit",
    "maker:blast_furnace",
    "maker:blast_furnace_lit",
    "maker:stonecutter",
    "maker:vault",
    "palette:wool_orange",
    "palette:wool_magenta",
    "palette:wool_light_blue",
    "palette:wool_yellow",
    "palette:wool_lime",
    "palette:wool_pink",
    "palette:wool_gray",
    "palette:wool_light_gray",
    "palette:wool_cyan",
    "palette:wool_purple",
    "palette:wool_blue",
    "palette:wool_brown",
    "palette:wool_green",
    "palette:wool_red",
    "palette:wool_black",
    "palette:concrete_white",
    "palette:concrete_orange",
    "palette:concrete_magenta",
    "palette:concrete_light_blue",
    "palette:concrete_yellow",
    "palette:concrete_lime",
    "palette:concrete_pink",
    "palette:concrete_gray",
    "palette:concrete_light_gray",
    "palette:concrete_cyan",
    "palette:concrete_purple",
    "palette:concrete_blue",
    "palette:concrete_brown",
    "palette:concrete_green",
    "palette:concrete_red",
    "palette:concrete_black",
    "palette:granite",
    "palette:polished_granite",
    "palette:marble",
    "palette:polished_marble",
    "palette:basalt",
    "palette:polished_basalt",
    "palette:slate",
    "palette:polished_slate",
    "palette:limestone",
    "palette:polished_limestone",
    "palette:pine_log",
    "palette:pine_leaves",
    "palette:pine_planks",
    "palette:sandstone_bricks",
    "palette:mossy_stone_bricks",
    "palette:cracked_stone_bricks",
    "palette:dark_bricks",
    "palette:stone_tiles",
    "flora:poppy",
    "flora:dandelion",
    "flora:cornflower",
    "flora:daisy",
    "flora:tall_grass",
    "mine:acacia_log",
    "mine:acacia_leaves",
    "mine:acacia_planks",
    "mine:jungle_log",
    "mine:jungle_leaves",
    "mine:jungle_planks",
    "mine:dark_oak_log",
    "mine:dark_oak_leaves",
    "mine:dark_oak_planks",
    "mine:mangrove_log",
    "mine:mangrove_leaves",
    "mine:mangrove_planks",
    "mine:cherry_log",
    "mine:cherry_leaves",
    "mine:cherry_planks",
    "mine:copper_ore",
    "mine:emerald_ore",
    "mine:lapis_ore",
    "mine:copper_block",
    "mine:emerald_block",
    "mine:lapis_block",
    "mine:glass_white",
    "mine:glass_orange",
    "mine:glass_magenta",
    "mine:glass_light_blue",
    "mine:glass_yellow",
    "mine:glass_lime",
    "mine:glass_pink",
    "mine:glass_gray",
    "mine:glass_light_gray",
    "mine:glass_cyan",
    "mine:glass_purple",
    "mine:glass_blue",
    "mine:glass_brown",
    "mine:glass_green",
    "mine:glass_red",
    "mine:glass_black",
    "mine:terracotta_white",
    "mine:terracotta_orange",
    "mine:terracotta_magenta",
    "mine:terracotta_light_blue",
    "mine:terracotta_yellow",
    "mine:terracotta_lime",
    "mine:terracotta_pink",
    "mine:terracotta_gray",
    "mine:terracotta_light_gray",
    "mine:terracotta_cyan",
    "mine:terracotta_purple",
    "mine:terracotta_blue",
    "mine:terracotta_brown",
    "mine:terracotta_green",
    "mine:terracotta_red",
    "mine:terracotta_black",
    "mine:sponge",
    "mine:melon",
    "mine:pumpkin",
    "mine:hay_bale",
    "mine:bone_block",
    "mine:mud",
    "mine:packed_mud",
    "mine:mud_bricks",
    "mine:moss_block",
    "mine:podzol",
    "mine:coarse_dirt",
    "mine:red_sand",
    "mine:red_sandstone",
    "mine:calcite",
    "mine:tuff",
    "mine:amethyst_block",
    "mine:quartz_block",
    "mine:oak_slab",
    "mine:birch_slab",
    "mine:pine_slab",
    "mine:stone_slab",
    "mine:cobblestone_slab",
    "mine:stone_brick_slab",
    "mine:smooth_stone_slab",
    "mine:sandstone_slab",
    "mine:brick_slab",
    "mine:granite_slab",
    "mine:marble_slab",
    "mine:dark_brick_slab",
    "fluid:water_1",
    "fluid:water_2",
    "fluid:water_3",
    "fluid:water_4",
    "fluid:water_5",
    "fluid:water_6",
    "fluid:water_7",
    "fluid:lava",
    "fluid:lava_1",
    "fluid:lava_2",
    "fluid:lava_3",
    "voxium:diorite",
    "voxium:polished_diorite",
    "voxium:andesite",
    "voxium:polished_andesite",
    "voxium:spruce_planks",
    "voxium:bamboo_planks",
    "voxium:bamboo_mosaic",
    "voxium:suspicious_sand",
    "voxium:suspicious_gravel",
    "voxium:deepslate_gold_ore",
    "voxium:deepslate_iron_ore",
    "voxium:deepslate_coal_ore",
    "voxium:nether_gold_ore",
    "voxium:spruce_log",
    "voxium:bamboo_block",
    "voxium:stripped_spruce_log",
    "voxium:stripped_birch_log",
    "voxium:stripped_jungle_log",
    "voxium:stripped_acacia_log",
    "voxium:stripped_cherry_log",
    "voxium:stripped_dark_oak_log",
    "voxium:stripped_oak_log",
    "voxium:stripped_mangrove_log",
    "voxium:stripped_bamboo_block",
    "voxium:oak_wood",
    "voxium:spruce_wood",
    "voxium:birch_wood",
    "voxium:jungle_wood",
    "voxium:acacia_wood",
    "voxium:cherry_wood",
    "voxium:dark_oak_wood",
    "voxium:mangrove_wood",
    "voxium:stripped_oak_wood",
    "voxium:stripped_spruce_wood",
    "voxium:stripped_birch_wood",
    "voxium:stripped_jungle_wood",
    "voxium:stripped_acacia_wood",
    "voxium:stripped_cherry_wood",
    "voxium:stripped_dark_oak_wood",
    "voxium:stripped_mangrove_wood",
    "voxium:wet_sponge",
    "voxium:deepslate_lapis_ore",
    "voxium:dispenser",
    "voxium:chiseled_sandstone",
    "voxium:cut_sandstone",
    "voxium:note_block",
    "voxium:sticky_piston",
    "voxium:piston",
    "voxium:tnt",
    "voxium:chiseled_bookshelf",
    "voxium:mangrove_roots",
    "voxium:muddy_mangrove_roots",
    "voxium:spruce_leaves",
    "voxium:azalea_leaves",
    "voxium:flowering_azalea_leaves",
    "voxium:oak_sapling",
    "voxium:spruce_sapling",
    "voxium:birch_sapling",
    "voxium:jungle_sapling",
    "voxium:acacia_sapling",
    "voxium:cherry_sapling",
    "voxium:dark_oak_sapling",
    "voxium:mangrove_propagule",
    "voxium:cobweb",
    "voxium:grass",
    "voxium:fern",
    "voxium:dead_bush",
    "voxium:seagrass",
    "voxium:tall_seagrass",
    "voxium:torchflower",
    "voxium:blue_orchid",
    "voxium:allium",
    "voxium:azure_bluet",
    "voxium:red_tulip",
    "voxium:orange_tulip",
    "voxium:white_tulip",
    "voxium:pink_tulip",
    "voxium:wither_rose",
    "voxium:lily_of_the_valley",
    "voxium:brown_mushroom",
    "voxium:red_mushroom",
    "voxium:torch",
    "voxium:wall_torch",
    "voxium:fire",
    "voxium:soul_fire",
    "voxium:powered_rail",
    "voxium:detector_rail",
    "voxium:white_bed",
    "voxium:orange_bed",
    "voxium:magenta_bed",
    "voxium:light_blue_bed",
    "voxium:yellow_bed",
    "voxium:lime_bed",
    "voxium:pink_bed",
    "voxium:gray_bed",
    "voxium:light_gray_bed",
    "voxium:cyan_bed",
    "voxium:purple_bed",
    "voxium:blue_bed",
    "voxium:brown_bed",
    "voxium:green_bed",
    "voxium:red_bed",
    "voxium:black_bed",
    "voxium:piston_head",
    "voxium:moving_piston",
    "voxium:dispenser_south",
    "voxium:dispenser_west",
    "voxium:dispenser_east",
    "voxium:dispenser_up",
    "voxium:dispenser_down",
    "voxium:piston_south",
    "voxium:piston_west",
    "voxium:piston_east",
    "voxium:piston_up",
    "voxium:piston_down",
    "voxium:sticky_piston_south",
    "voxium:sticky_piston_west",
    "voxium:sticky_piston_east",
    "voxium:sticky_piston_up",
    "voxium:sticky_piston_down",
    "voxium:wall_torch_south",
    "voxium:wall_torch_west",
    "voxium:wall_torch_east",
    "voxium:powered_rail_x",
    "voxium:detector_rail_x",
    "voxium:spawner",
    "voxium:oak_stairs",
    "voxium:oak_stairs_south",
    "voxium:oak_stairs_west",
    "voxium:oak_stairs_east",
    "voxium:redstone_wire",
    "voxium:deepslate_diamond_ore",
    "voxium:wheat",
    "voxium:farmland",
    "voxium:oak_sign",
    "voxium:oak_sign_south",
    "voxium:oak_sign_west",
    "voxium:oak_sign_east",
    "voxium:spruce_sign",
    "voxium:spruce_sign_south",
    "voxium:spruce_sign_west",
    "voxium:spruce_sign_east",
    "voxium:birch_sign",
    "voxium:birch_sign_south",
    "voxium:birch_sign_west",
    "voxium:birch_sign_east",
    "voxium:jungle_sign",
    "voxium:jungle_sign_south",
    "voxium:jungle_sign_west",
    "voxium:jungle_sign_east",
    "voxium:acacia_sign",
    "voxium:acacia_sign_south",
    "voxium:acacia_sign_west",
    "voxium:acacia_sign_east",
    "voxium:cherry_sign",
    "voxium:cherry_sign_south",
    "voxium:cherry_sign_west",
    "voxium:cherry_sign_east",
    "voxium:dark_oak_sign",
    "voxium:dark_oak_sign_south",
    "voxium:dark_oak_sign_west",
    "voxium:dark_oak_sign_east",
    "voxium:mangrove_sign",
    "voxium:mangrove_sign_south",
    "voxium:mangrove_sign_west",
    "voxium:mangrove_sign_east",
    "voxium:bamboo_sign",
    "voxium:bamboo_sign_south",
    "voxium:bamboo_sign_west",
    "voxium:bamboo_sign_east",
    "voxium:oak_door",
    "voxium:oak_door_lower_closed_south",
    "voxium:oak_door_lower_closed_west",
    "voxium:oak_door_lower_closed_east",
    "voxium:oak_door_lower_open_north",
    "voxium:oak_door_lower_open_south",
    "voxium:oak_door_lower_open_west",
    "voxium:oak_door_lower_open_east",
    "voxium:oak_door_upper_closed_north",
    "voxium:oak_door_upper_closed_south",
    "voxium:oak_door_upper_closed_west",
    "voxium:oak_door_upper_closed_east",
    "voxium:oak_door_upper_open_north",
    "voxium:oak_door_upper_open_south",
    "voxium:oak_door_upper_open_west",
    "voxium:oak_door_upper_open_east",
    "voxium:ladder",
    "voxium:ladder_south",
    "voxium:ladder_west",
    "voxium:ladder_east",
    "voxium:rail",
    "voxium:rail_x",
    "voxium:cobblestone_stairs",
    "voxium:cobblestone_stairs_south",
    "voxium:cobblestone_stairs_west",
    "voxium:cobblestone_stairs_east",
    "voxium:oak_wall_sign",
    "voxium:oak_wall_sign_south",
    "voxium:oak_wall_sign_west",
    "voxium:oak_wall_sign_east",
    "voxium:spruce_wall_sign",
    "voxium:spruce_wall_sign_south",
    "voxium:spruce_wall_sign_west",
    "voxium:spruce_wall_sign_east",
    "voxium:birch_wall_sign",
    "voxium:birch_wall_sign_south",
    "voxium:birch_wall_sign_west",
    "voxium:birch_wall_sign_east",
    "voxium:jungle_wall_sign",
    "voxium:jungle_wall_sign_south",
    "voxium:jungle_wall_sign_west",
    "voxium:jungle_wall_sign_east",
    "voxium:acacia_wall_sign",
    "voxium:acacia_wall_sign_south",
    "voxium:acacia_wall_sign_west",
    "voxium:acacia_wall_sign_east",
    "voxium:cherry_wall_sign",
    "voxium:cherry_wall_sign_south",
    "voxium:cherry_wall_sign_west",
    "voxium:cherry_wall_sign_east",
    "voxium:dark_oak_wall_sign",
    "voxium:dark_oak_wall_sign_south",
    "voxium:dark_oak_wall_sign_west",
    "voxium:dark_oak_wall_sign_east",
    "voxium:mangrove_wall_sign",
    "voxium:mangrove_wall_sign_south",
    "voxium:mangrove_wall_sign_west",
    "voxium:mangrove_wall_sign_east",
    "voxium:bamboo_wall_sign",
    "voxium:bamboo_wall_sign_south",
    "voxium:bamboo_wall_sign_west",
    "voxium:bamboo_wall_sign_east",
    "voxium:oak_hanging_sign",
    "voxium:oak_hanging_sign_z",
    "voxium:spruce_hanging_sign",
    "voxium:spruce_hanging_sign_z",
    "voxium:birch_hanging_sign",
    "voxium:birch_hanging_sign_z",
    "voxium:acacia_hanging_sign",
    "voxium:acacia_hanging_sign_z",
    "voxium:cherry_hanging_sign",
    "voxium:cherry_hanging_sign_z",
    "voxium:jungle_hanging_sign",
    "voxium:jungle_hanging_sign_z",
    "voxium:dark_oak_hanging_sign",
    "voxium:dark_oak_hanging_sign_z",
    "voxium:crimson_hanging_sign",
    "voxium:crimson_hanging_sign_z",
    "voxium:warped_hanging_sign",
    "voxium:warped_hanging_sign_z",
    "voxium:mangrove_hanging_sign",
    "voxium:mangrove_hanging_sign_z",
    "voxium:bamboo_hanging_sign",
    "voxium:bamboo_hanging_sign_z",
    "voxium:oak_wall_hanging_sign",
    "voxium:oak_wall_hanging_sign_z",
    "voxium:spruce_wall_hanging_sign",
    "voxium:spruce_wall_hanging_sign_z",
    "voxium:birch_wall_hanging_sign",
    "voxium:birch_wall_hanging_sign_z",
    "voxium:acacia_wall_hanging_sign",
    "voxium:acacia_wall_hanging_sign_z",
    "voxium:cherry_wall_hanging_sign",
    "voxium:cherry_wall_hanging_sign_z",
    "voxium:jungle_wall_hanging_sign",
    "voxium:jungle_wall_hanging_sign_z",
    "voxium:dark_oak_wall_hanging_sign",
    "voxium:dark_oak_wall_hanging_sign_z",
    "voxium:crimson_wall_hanging_sign",
    "voxium:crimson_wall_hanging_sign_z",
    "voxium:warped_wall_hanging_sign",
    "voxium:warped_wall_hanging_sign_z",
    "voxium:mangrove_wall_hanging_sign",
    "voxium:mangrove_wall_hanging_sign_z",
    "voxium:bamboo_wall_hanging_sign",
    "voxium:bamboo_wall_hanging_sign_z",
    "voxium:lever",
    "voxium:lever_off_south",
    "voxium:lever_off_west",
    "voxium:lever_off_east",
    "voxium:lever_on_north",
    "voxium:lever_on_south",
    "voxium:lever_on_west",
    "voxium:lever_on_east",
    "voxium:stone_pressure_plate",
    "voxium:iron_door",
    "voxium:iron_door_lower_closed_south",
    "voxium:iron_door_lower_closed_west",
    "voxium:iron_door_lower_closed_east",
    "voxium:iron_door_lower_open_north",
    "voxium:iron_door_lower_open_south",
    "voxium:iron_door_lower_open_west",
    "voxium:iron_door_lower_open_east",
    "voxium:iron_door_upper_closed_north",
    "voxium:iron_door_upper_closed_south",
    "voxium:iron_door_upper_closed_west",
    "voxium:iron_door_upper_closed_east",
    "voxium:iron_door_upper_open_north",
    "voxium:iron_door_upper_open_south",
    "voxium:iron_door_upper_open_west",
    "voxium:iron_door_upper_open_east",
    "voxium:oak_pressure_plate",
    "voxium:spruce_pressure_plate",
    "voxium:birch_pressure_plate",
    "voxium:jungle_pressure_plate",
    "voxium:acacia_pressure_plate",
    "voxium:cherry_pressure_plate",
    "voxium:dark_oak_pressure_plate",
    "voxium:mangrove_pressure_plate",
    "voxium:bamboo_pressure_plate",
    "voxium:redstone_ore",
    "voxium:deepslate_redstone_ore",
    "voxium:redstone_torch",
    "voxium:redstone_wall_torch",
    "voxium:redstone_wall_torch_south",
    "voxium:redstone_wall_torch_west",
    "voxium:redstone_wall_torch_east",
    "voxium:stone_button",
    "voxium:stone_button_south",
    "voxium:stone_button_west",
    "voxium:stone_button_east",
    "voxium:snow_layer",
    "voxium:cactus",
    "voxium:sugar_cane",
    "voxium:jukebox",
    "voxium:oak_fence",
    "voxium:netherrack",
    "voxium:soul_sand",
    "voxium:soul_soil",
    "voxium:soul_torch",
    "voxium:soul_wall_torch",
    "voxium:soul_wall_torch_south",
    "voxium:soul_wall_torch_west",
    "voxium:soul_wall_torch_east",
    "voxium:glowstone",
    "voxium:nether_portal",
    "voxium:nether_portal_z",
    "voxium:carved_pumpkin",
    "voxium:carved_pumpkin_south",
    "voxium:carved_pumpkin_west",
    "voxium:carved_pumpkin_east",
    "voxium:jack_o_lantern",
    "voxium:jack_o_lantern_south",
    "voxium:jack_o_lantern_west",
    "voxium:jack_o_lantern_east",
    "voxium:cake",
    "voxium:repeater",
    "voxium:repeater_south",
    "voxium:repeater_west",
    "voxium:repeater_east",
    "voxium:oak_trapdoor",
    "voxium:oak_trapdoor_open_north",
    "voxium:oak_trapdoor_open_south",
    "voxium:oak_trapdoor_open_west",
    "voxium:oak_trapdoor_open_east",
    "voxium:spruce_trapdoor",
    "voxium:spruce_trapdoor_open_north",
    "voxium:spruce_trapdoor_open_south",
    "voxium:spruce_trapdoor_open_west",
    "voxium:spruce_trapdoor_open_east",
    "voxium:birch_trapdoor",
    "voxium:birch_trapdoor_open_north",
    "voxium:birch_trapdoor_open_south",
    "voxium:birch_trapdoor_open_west",
    "voxium:birch_trapdoor_open_east",
    "voxium:jungle_trapdoor",
    "voxium:jungle_trapdoor_open_north",
    "voxium:jungle_trapdoor_open_south",
    "voxium:jungle_trapdoor_open_west",
    "voxium:jungle_trapdoor_open_east",
    "voxium:acacia_trapdoor",
    "voxium:acacia_trapdoor_open_north",
    "voxium:acacia_trapdoor_open_south",
    "voxium:acacia_trapdoor_open_west",
    "voxium:acacia_trapdoor_open_east",
    "voxium:cherry_trapdoor",
    "voxium:cherry_trapdoor_open_north",
    "voxium:cherry_trapdoor_open_south",
    "voxium:cherry_trapdoor_open_west",
    "voxium:cherry_trapdoor_open_east",
    "voxium:dark_oak_trapdoor",
    "voxium:dark_oak_trapdoor_open_north",
    "voxium:dark_oak_trapdoor_open_south",
    "voxium:dark_oak_trapdoor_open_west",
    "voxium:dark_oak_trapdoor_open_east",
    "voxium:mangrove_trapdoor",
    "voxium:mangrove_trapdoor_open_north",
    "voxium:mangrove_trapdoor_open_south",
    "voxium:mangrove_trapdoor_open_west",
    "voxium:mangrove_trapdoor_open_east",
    "voxium:bamboo_trapdoor",
    "voxium:bamboo_trapdoor_open_north",
    "voxium:bamboo_trapdoor_open_south",
    "voxium:bamboo_trapdoor_open_west",
    "voxium:bamboo_trapdoor_open_east",
    "voxium:chiseled_stone_bricks",
    "voxium:infested_stone",
    "voxium:infested_cobblestone",
    "voxium:infested_stone_bricks",
    "voxium:infested_mossy_stone_bricks",
    "voxium:infested_cracked_stone_bricks",
    "voxium:infested_chiseled_stone_bricks",
    "voxium:brown_mushroom_block",
    "voxium:red_mushroom_block",
    "voxium:mushroom_stem",
    "voxium:iron_bars",
    "voxium:chain",
    "voxium:glass_pane",
    "voxium:attached_pumpkin_stem",
    "voxium:attached_melon_stem",
    "voxium:pumpkin_stem",
    "voxium:melon_stem",
    "voxium:vine",
    "voxium:glow_lichen",
    "voxium:oak_fence_gate",
    "voxium:oak_fence_gate_closed_south",
    "voxium:oak_fence_gate_closed_west",
    "voxium:oak_fence_gate_closed_east",
    "voxium:oak_fence_gate_open_north",
    "voxium:oak_fence_gate_open_south",
    "voxium:oak_fence_gate_open_west",
    "voxium:oak_fence_gate_open_east",
    "voxium:brick_stairs",
    "voxium:brick_stairs_south",
    "voxium:brick_stairs_west",
    "voxium:brick_stairs_east",
    "voxium:stone_brick_stairs",
    "voxium:stone_brick_stairs_south",
    "voxium:stone_brick_stairs_west",
    "voxium:stone_brick_stairs_east",
    "voxium:mud_brick_stairs",
    "voxium:mud_brick_stairs_south",
    "voxium:mud_brick_stairs_west",
    "voxium:mud_brick_stairs_east",
    "voxium:mycelium",
    "voxium:lily_pad",
    "voxium:nether_bricks",
    "voxium:nether_brick_fence",
    "voxium:nether_brick_stairs",
    "voxium:nether_brick_stairs_south",
    "voxium:nether_brick_stairs_west",
    "voxium:nether_brick_stairs_east",
    "voxium:nether_wart",
    "voxium:enchanting_table",
    "voxium:brewing_stand",
    "voxium:cauldron",
    "voxium:water_cauldron",
    "voxium:lava_cauldron",
    "voxium:powder_snow_cauldron",
    "voxium:end_portal",
    "voxium:end_portal_frame",
    "voxium:end_stone",
    "voxium:dragon_egg",
    "voxium:redstone_lamp",
    "voxium:cocoa",
    "voxium:sandstone_stairs",
    "voxium:sandstone_stairs_south",
    "voxium:sandstone_stairs_west",
    "voxium:sandstone_stairs_east",
    "voxium:deepslate_emerald_ore",
    "voxium:ender_chest",
    "voxium:tripwire_hook",
    "voxium:tripwire_hook_south",
    "voxium:tripwire_hook_west",
    "voxium:tripwire_hook_east",
    "voxium:tripwire",
    "voxium:spruce_stairs",
    "voxium:spruce_stairs_south",
    "voxium:spruce_stairs_west",
    "voxium:spruce_stairs_east",
    "voxium:birch_stairs",
    "voxium:birch_stairs_south",
    "voxium:birch_stairs_west",
    "voxium:birch_stairs_east",
    "voxium:jungle_stairs",
    "voxium:jungle_stairs_south",
    "voxium:jungle_stairs_west",
    "voxium:jungle_stairs_east",
    "voxium:command_block",
    "voxium:beacon",
    "voxium:cobblestone_wall",
    "voxium:mossy_cobblestone_wall",
    "voxium:flower_pot",
    "voxium:potted_torchflower",
    "voxium:potted_oak_sapling",
    "voxium:potted_spruce_sapling",
    "voxium:potted_birch_sapling",
    "voxium:potted_jungle_sapling",
    "voxium:potted_acacia_sapling",
    "voxium:potted_cherry_sapling",
    "voxium:potted_dark_oak_sapling",
    "voxium:potted_mangrove_propagule",
    "voxium:potted_fern",
    "voxium:potted_dandelion",
    "voxium:potted_poppy",
    "voxium:potted_blue_orchid",
    "voxium:potted_allium",
    "voxium:potted_azure_bluet",
    "voxium:potted_red_tulip",
    "voxium:potted_orange_tulip",
    "voxium:potted_white_tulip",
    "voxium:potted_pink_tulip",
    "voxium:potted_oxeye_daisy",
    "voxium:potted_cornflower",
    "voxium:potted_lily_of_the_valley",
    "voxium:potted_wither_rose",
    "voxium:potted_red_mushroom",
    "voxium:potted_brown_mushroom",
    "voxium:potted_dead_bush",
    "voxium:potted_cactus",
    "voxium:carrots",
    "voxium:potatoes",
    "voxium:oak_button",
    "voxium:oak_button_south",
    "voxium:oak_button_west",
    "voxium:oak_button_east",
    "voxium:spruce_button",
    "voxium:spruce_button_south",
    "voxium:spruce_button_west",
    "voxium:spruce_button_east",
    "voxium:birch_button",
    "voxium:birch_button_south",
    "voxium:birch_button_west",
    "voxium:birch_button_east",
    "voxium:jungle_button",
    "voxium:jungle_button_south",
    "voxium:jungle_button_west",
    "voxium:jungle_button_east",
    "voxium:acacia_button",
    "voxium:acacia_button_south",
    "voxium:acacia_button_west",
    "voxium:acacia_button_east",
    "voxium:cherry_button",
    "voxium:cherry_button_south",
    "voxium:cherry_button_west",
    "voxium:cherry_button_east",
    "voxium:dark_oak_button",
    "voxium:dark_oak_button_south",
    "voxium:dark_oak_button_west",
    "voxium:dark_oak_button_east",
    "voxium:mangrove_button",
    "voxium:mangrove_button_south",
    "voxium:mangrove_button_west",
    "voxium:mangrove_button_east",
    "voxium:bamboo_button",
    "voxium:bamboo_button_south",
    "voxium:bamboo_button_west",
    "voxium:bamboo_button_east",
    "voxium:skeleton_skull",
    "voxium:skeleton_wall_skull",
    "voxium:skeleton_wall_skull_south",
    "voxium:skeleton_wall_skull_west",
    "voxium:skeleton_wall_skull_east",
    "voxium:wither_skeleton_skull",
    "voxium:wither_skeleton_wall_skull",
    "voxium:wither_skeleton_wall_skull_south",
    "voxium:wither_skeleton_wall_skull_west",
    "voxium:wither_skeleton_wall_skull_east",
    "voxium:zombie_head",
    "voxium:zombie_wall_head",
    "voxium:zombie_wall_head_south",
    "voxium:zombie_wall_head_west",
    "voxium:zombie_wall_head_east",
    "voxium:player_head",
    "voxium:player_wall_head",
    "voxium:player_wall_head_south",
    "voxium:player_wall_head_west",
    "voxium:player_wall_head_east",
    "voxium:creeper_head",
    "voxium:creeper_wall_head",
    "voxium:creeper_wall_head_south",
    "voxium:creeper_wall_head_west",
    "voxium:creeper_wall_head_east",
    "voxium:dragon_head",
    "voxium:dragon_wall_head",
    "voxium:dragon_wall_head_south",
    "voxium:dragon_wall_head_west",
    "voxium:dragon_wall_head_east",
    "voxium:piglin_head",
    "voxium:piglin_wall_head",
    "voxium:piglin_wall_head_south",
    "voxium:piglin_wall_head_west",
    "voxium:piglin_wall_head_east",
    "voxium:anvil",
    "voxium:anvil_z",
    "voxium:chipped_anvil",
    "voxium:chipped_anvil_z",
    "voxium:damaged_anvil",
    "voxium:damaged_anvil_z",
    "voxium:trapped_chest",
    "voxium:light_weighted_pressure_plate",
    "voxium:heavy_weighted_pressure_plate",
    "voxium:comparator",
    "voxium:comparator_south",
    "voxium:comparator_west",
    "voxium:comparator_east",
    "voxium:daylight_detector",
    "voxium:redstone_block",
    "voxium:nether_quartz_ore",
    "voxium:hopper",
    "voxium:chiseled_quartz_block",
    "voxium:quartz_pillar",
    "voxium:quartz_stairs",
    "voxium:quartz_stairs_south",
    "voxium:quartz_stairs_west",
    "voxium:quartz_stairs_east",
    "voxium:activator_rail",
    "voxium:activator_rail_x",
    "voxium:dropper",
    "voxium:dropper_south",
    "voxium:dropper_west",
    "voxium:dropper_east",
    "voxium:dropper_up",
    "voxium:dropper_down",
    "voxium:white_stained_glass_pane",
    "voxium:orange_stained_glass_pane",
    "voxium:magenta_stained_glass_pane",
    "voxium:light_blue_stained_glass_pane",
    "voxium:yellow_stained_glass_pane",
    "voxium:lime_stained_glass_pane",
    "voxium:pink_stained_glass_pane",
    "voxium:gray_stained_glass_pane",
    "voxium:light_gray_stained_glass_pane",
    "voxium:cyan_stained_glass_pane",
    "voxium:purple_stained_glass_pane",
    "voxium:blue_stained_glass_pane",
    "voxium:brown_stained_glass_pane",
    "voxium:green_stained_glass_pane",
    "voxium:red_stained_glass_pane",
    "voxium:black_stained_glass_pane",
    "voxium:acacia_stairs",
    "voxium:acacia_stairs_south",
    "voxium:acacia_stairs_west",
    "voxium:acacia_stairs_east",
    "voxium:cherry_stairs",
    "voxium:cherry_stairs_south",
    "voxium:cherry_stairs_west",
    "voxium:cherry_stairs_east",
    "voxium:dark_oak_stairs",
    "voxium:dark_oak_stairs_south",
    "voxium:dark_oak_stairs_west",
    "voxium:dark_oak_stairs_east",
    "voxium:mangrove_stairs",
    "voxium:mangrove_stairs_south",
    "voxium:mangrove_stairs_west",
    "voxium:mangrove_stairs_east",
    "voxium:bamboo_stairs",
    "voxium:bamboo_stairs_south",
    "voxium:bamboo_stairs_west",
    "voxium:bamboo_stairs_east",
    "voxium:bamboo_mosaic_stairs",
    "voxium:bamboo_mosaic_stairs_south",
    "voxium:bamboo_mosaic_stairs_west",
    "voxium:bamboo_mosaic_stairs_east",
    "voxium:slime_block",
    "voxium:barrier",
    "voxium:light",
    "voxium:iron_trapdoor",
    "voxium:iron_trapdoor_open_north",
    "voxium:iron_trapdoor_open_south",
    "voxium:iron_trapdoor_open_west",
    "voxium:iron_trapdoor_open_east",
    "voxium:prismarine",
    "voxium:prismarine_bricks",
    "voxium:dark_prismarine",
    "voxium:prismarine_stairs",
    "voxium:prismarine_stairs_south",
    "voxium:prismarine_stairs_west",
    "voxium:prismarine_stairs_east",
    "voxium:prismarine_brick_stairs",
    "voxium:prismarine_brick_stairs_south",
    "voxium:prismarine_brick_stairs_west",
    "voxium:prismarine_brick_stairs_east",
    "voxium:dark_prismarine_stairs",
    "voxium:dark_prismarine_stairs_south",
    "voxium:dark_prismarine_stairs_west",
    "voxium:dark_prismarine_stairs_east",
    "voxium:prismarine_slab",
    "voxium:prismarine_brick_slab",
    "voxium:dark_prismarine_slab",
    "voxium:sea_lantern",
    "voxium:white_carpet",
    "voxium:orange_carpet",
    "voxium:magenta_carpet",
    "voxium:light_blue_carpet",
    "voxium:yellow_carpet",
    "voxium:lime_carpet",
    "voxium:pink_carpet",
    "voxium:gray_carpet",
    "voxium:light_gray_carpet",
    "voxium:cyan_carpet",
    "voxium:purple_carpet",
    "voxium:blue_carpet",
    "voxium:brown_carpet",
    "voxium:green_carpet",
    "voxium:red_carpet",
    "voxium:black_carpet",
    "voxium:packed_ice",
    "voxium:sunflower",
    "voxium:lilac",
    "voxium:rose_bush",
    "voxium:peony",
    "voxium:large_fern",
    "voxium:white_banner",
    "voxium:white_banner_south",
    "voxium:white_banner_west",
    "voxium:white_banner_east",
    "voxium:orange_banner",
    "voxium:orange_banner_south",
    "voxium:orange_banner_west",
    "voxium:orange_banner_east",
    "voxium:magenta_banner",
    "voxium:magenta_banner_south",
    "voxium:magenta_banner_west",
    "voxium:magenta_banner_east",
    "voxium:light_blue_banner",
    "voxium:light_blue_banner_south",
    "voxium:light_blue_banner_west",
    "voxium:light_blue_banner_east",
    "voxium:yellow_banner",
    "voxium:yellow_banner_south",
    "voxium:yellow_banner_west",
    "voxium:yellow_banner_east",
    "voxium:lime_banner",
    "voxium:lime_banner_south",
    "voxium:lime_banner_west",
    "voxium:lime_banner_east",
    "voxium:pink_banner",
    "voxium:pink_banner_south",
    "voxium:pink_banner_west",
    "voxium:pink_banner_east",
    "voxium:gray_banner",
    "voxium:gray_banner_south",
    "voxium:gray_banner_west",
    "voxium:gray_banner_east",
    "voxium:light_gray_banner",
    "voxium:light_gray_banner_south",
    "voxium:light_gray_banner_west",
    "voxium:light_gray_banner_east",
    "voxium:cyan_banner",
    "voxium:cyan_banner_south",
    "voxium:cyan_banner_west",
    "voxium:cyan_banner_east",
    "voxium:purple_banner",
    "voxium:purple_banner_south",
    "voxium:purple_banner_west",
    "voxium:purple_banner_east",
    "voxium:blue_banner",
    "voxium:blue_banner_south",
    "voxium:blue_banner_west",
    "voxium:blue_banner_east",
    "voxium:brown_banner",
    "voxium:brown_banner_south",
    "voxium:brown_banner_west",
    "voxium:brown_banner_east",
    "voxium:green_banner",
    "voxium:green_banner_south",
    "voxium:green_banner_west",
    "voxium:green_banner_east",
    "voxium:red_banner",
    "voxium:red_banner_south",
    "voxium:red_banner_west",
    "voxium:red_banner_east",
    "voxium:black_banner",
    "voxium:black_banner_south",
    "voxium:black_banner_west",
    "voxium:black_banner_east",
    "voxium:white_wall_banner",
    "voxium:white_wall_banner_south",
    "voxium:white_wall_banner_west",
    "voxium:white_wall_banner_east",
    "voxium:orange_wall_banner",
    "voxium:orange_wall_banner_south",
    "voxium:orange_wall_banner_west",
    "voxium:orange_wall_banner_east",
    "voxium:magenta_wall_banner",
    "voxium:magenta_wall_banner_south",
    "voxium:magenta_wall_banner_west",
    "voxium:magenta_wall_banner_east",
    "voxium:light_blue_wall_banner",
    "voxium:light_blue_wall_banner_south",
    "voxium:light_blue_wall_banner_west",
    "voxium:light_blue_wall_banner_east",
    "voxium:yellow_wall_banner",
    "voxium:yellow_wall_banner_south",
    "voxium:yellow_wall_banner_west",
    "voxium:yellow_wall_banner_east",
    "voxium:lime_wall_banner",
    "voxium:lime_wall_banner_south",
    "voxium:lime_wall_banner_west",
    "voxium:lime_wall_banner_east",
    "voxium:pink_wall_banner",
    "voxium:pink_wall_banner_south",
    "voxium:pink_wall_banner_west",
    "voxium:pink_wall_banner_east",
    "voxium:gray_wall_banner",
    "voxium:gray_wall_banner_south",
    "voxium:gray_wall_banner_west",
    "voxium:gray_wall_banner_east",
    "voxium:light_gray_wall_banner",
    "voxium:light_gray_wall_banner_south",
    "voxium:light_gray_wall_banner_west",
    "voxium:light_gray_wall_banner_east",
    "voxium:cyan_wall_banner",
    "voxium:cyan_wall_banner_south",
    "voxium:cyan_wall_banner_west",
    "voxium:cyan_wall_banner_east",
    "voxium:purple_wall_banner",
    "voxium:purple_wall_banner_south",
    "voxium:purple_wall_banner_west",
    "voxium:purple_wall_banner_east",
    "voxium:blue_wall_banner",
    "voxium:blue_wall_banner_south",
    "voxium:blue_wall_banner_west",
    "voxium:blue_wall_banner_east",
    "voxium:brown_wall_banner",
    "voxium:brown_wall_banner_south",
    "voxium:brown_wall_banner_west",
    "voxium:brown_wall_banner_east",
    "voxium:green_wall_banner",
    "voxium:green_wall_banner_south",
    "voxium:green_wall_banner_west",
    "voxium:green_wall_banner_east",
    "voxium:red_wall_banner",
    "voxium:red_wall_banner_south",
    "voxium:red_wall_banner_west",
    "voxium:red_wall_banner_east",
    "voxium:black_wall_banner",
    "voxium:black_wall_banner_south",
    "voxium:black_wall_banner_west",
    "voxium:black_wall_banner_east",
    "voxium:chiseled_red_sandstone",
    "voxium:cut_red_sandstone",
    "voxium:red_sandstone_stairs",
    "voxium:red_sandstone_stairs_south",
    "voxium:red_sandstone_stairs_west",
    "voxium:red_sandstone_stairs_east",
    "voxium:spruce_slab",
    "voxium:jungle_slab",
    "voxium:acacia_slab",
    "voxium:cherry_slab",
    "voxium:dark_oak_slab",
    "voxium:mangrove_slab",
    "voxium:bamboo_slab",
    "voxium:bamboo_mosaic_slab",
];

impl Default for ContentRegistry {
    fn default() -> Self {
        Self::from_ron(include_str!("../../../assets/data/items.ron"))
            .expect("встроенный каталог предметов должен быть корректным")
    }
}

impl ContentRegistry {
    pub fn from_ron(source: &str) -> Result<Self, String> {
        let definitions: Vec<ItemDef> = ron::from_str(source)
            .map_err(|error| format!("Ошибка RON каталога предметов: {error}"))?;
        Self::checked(definitions)
    }

    fn checked(definitions: Vec<ItemDef>) -> Result<Self, String> {
        if definitions.len() > usize::from(u16::MAX) + 1 {
            return Err("Слишком много предметов для числовых ID".into());
        }
        let mut by_name = HashMap::new();
        let mut by_block = HashMap::new();
        for (index, item) in definitions.iter().enumerate() {
            if !valid_namespaced_id(&item.id) {
                return Err(format!("Некорректный ID предмета: {}", item.id));
            }
            if item.name.trim().is_empty() {
                return Err(format!("Пустое название предмета: {}", item.id));
            }
            if !(1..=64).contains(&item.max_stack) {
                return Err(format!("Размер стека должен быть от 1 до 64: {}", item.id));
            }
            if matches!(item.kind, ItemKind::Tool | ItemKind::Armor { .. }) && item.max_stack != 1 {
                return Err(format!("Инструменты и броня требуют стек 1: {}", item.id));
            }
            let id = ItemId(index as u16);
            if by_name.insert(item.id.clone(), id).is_some() {
                return Err(format!("Повторяющийся ID предмета: {}", item.id));
            }
            if let ItemKind::Block { block } = &item.kind {
                let block_id = BLOCK_NAMES
                    .iter()
                    .position(|name| name == block)
                    .ok_or_else(|| format!("Неизвестный блок {block} у предмета {}", item.id))?
                    as crate::BlockId;
                if matches!(block_id, 0 | 13)
                    || crate::drop_base(block_id) != block_id
                    || (crate::is_fluid(block_id)
                        && block_id != crate::WATER
                        && block_id != crate::LAVA)
                {
                    return Err(format!("Блок {block} не может быть предметом"));
                }
                if by_block.insert(block_id, id).is_some() {
                    return Err(format!("Несколько предметов ссылаются на блок {block}"));
                }
            }
        }
        for item in &definitions {
            if let BreakDrop::Item(reference) = &item.break_drop
                && !by_name.contains_key(reference)
            {
                return Err(format!(
                    "Неизвестный предмет добычи {reference} у {}",
                    item.id
                ));
            }
        }
        Ok(Self {
            definitions,
            by_name,
            by_block,
        })
    }

    pub fn reload(&self, source: &str) -> Result<Self, String> {
        let candidate = Self::from_ron(source)?;
        let mut definitions = Vec::with_capacity(candidate.len());
        for previous in &self.definitions {
            let next_id = candidate
                .find(&previous.id)
                .ok_or_else(|| format!("Нельзя удалить существующий предмет: {}", previous.id))?;
            let next = &candidate.definitions[usize::from(next_id.0)];
            if next.kind != previous.kind {
                return Err(format!(
                    "Нельзя изменить тип существующего предмета: {}",
                    previous.id
                ));
            }
            definitions.push(next.clone());
        }
        for next in candidate.definitions {
            if !self.by_name.contains_key(&next.id) {
                definitions.push(next);
            }
        }
        Self::checked(definitions)
    }

    #[must_use]
    pub fn get(&self, id: ItemId) -> Option<&ItemDef> {
        self.definitions.get(usize::from(id.0))
    }

    #[must_use]
    pub fn find(&self, id: &str) -> Option<ItemId> {
        self.by_name.get(id).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &ItemDef)> {
        self.definitions
            .iter()
            .enumerate()
            .map(|(index, item)| (ItemId(index as u16), item))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    #[must_use]
    pub fn item_for_block(&self, block: crate::BlockId) -> Option<ItemId> {
        self.by_block.get(&block).copied()
    }

    #[must_use]
    pub fn block_for_item(&self, id: ItemId) -> Option<crate::BlockId> {
        let ItemKind::Block { block } = &self.get(id)?.kind else {
            return None;
        };
        BLOCK_NAMES
            .iter()
            .position(|name| name == block)
            .map(|index| index as crate::BlockId)
    }

    #[must_use]
    pub fn drop_for_block(&self, block: crate::BlockId) -> Option<ItemId> {
        // Верхняя половина двери ничего не роняет: предмет даёт нижняя.
        if crate::models::is_door_upper(block) {
            return None;
        }
        // Горящая печь выпадает обычной печью.
        let block = crate::drop_base(block);
        let item = self.item_for_block(block)?;
        match &self.get(item)?.break_drop {
            BreakDrop::SelfItem => Some(item),
            BreakDrop::Nothing => None,
            BreakDrop::Item(reference) => self.find(reference),
        }
    }

    #[must_use]
    pub fn max_stack(&self, id: ItemId) -> u16 {
        self.get(id).map_or(0, |item| item.max_stack)
    }
}

/// ID блока по имени вида `voxium:oak_door` или просто `oak_door`.
#[must_use]
pub fn block_by_name(name: &str) -> Option<crate::BlockId> {
    BLOCK_NAMES
        .iter()
        .position(|candidate| {
            *candidate == name
                || candidate
                    .split_once(':')
                    .is_some_and(|(_, short)| short == name)
        })
        .map(|index| index as crate::BlockId)
}

fn valid_namespaced_id(id: &str) -> bool {
    let Some((namespace, path)) = id.split_once(':') else {
        return false;
    };
    let valid = |character: u8| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, b'_' | b'-' | b'.')
    };
    !namespace.is_empty()
        && !path.is_empty()
        && namespace.bytes().all(valid)
        && path
            .bytes()
            .all(|character| valid(character) || character == b'/')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progression_catalog_adds_stable_blocks_drops_tools_and_armor() {
        let registry = ContentRegistry::default();
        let blocks = [
            "coal_ore",
            "iron_ore",
            "gold_ore",
            "diamond_ore",
            "iron_block",
            "gold_block",
            "diamond_block",
            "crafting_table",
            "furnace",
        ];
        for (offset, name) in blocks.iter().enumerate() {
            let id = registry
                .find(&format!("survival:{name}"))
                .expect("new block item");
            assert_eq!(
                registry.block_for_item(id),
                Some(17 + offset as crate::BlockId)
            );
            assert_eq!(
                registry.item_for_block(17 + offset as crate::BlockId),
                Some(id)
            );
        }
        for (block, drop) in [
            (17, "coal"),
            (18, "raw_iron"),
            (19, "raw_gold"),
            (20, "diamond"),
        ] {
            assert_eq!(
                registry.drop_for_block(block),
                Some(registry.find(&format!("survival:{drop}")).unwrap())
            );
        }
        for block in 21..=25 {
            assert_eq!(
                registry.drop_for_block(block),
                registry.item_for_block(block)
            );
        }
        for name in [
            "raw_iron",
            "raw_gold",
            "iron_ingot",
            "gold_ingot",
            "diamond",
            "leather",
        ] {
            let id = registry.find(&format!("survival:{name}")).unwrap();
            assert_eq!(registry.get(id).unwrap().kind, ItemKind::Material);
            assert_eq!(registry.max_stack(id), 64);
        }
        for material in ["wooden", "stone", "iron", "golden", "diamond"] {
            for tool in ["axe", "hoe", "pickaxe", "shovel", "sword"] {
                let def = registry
                    .get(
                        registry
                            .find(&format!("survival:{material}_{tool}"))
                            .unwrap(),
                    )
                    .unwrap();
                assert_eq!(def.kind, ItemKind::Tool);
                assert_eq!(def.max_stack, 1);
                assert!(!def.name.contains("образец"));
            }
        }
        for material in ["leather", "iron", "golden", "diamond"] {
            for (part, slot) in [
                ("helmet", ArmorSlot::Head),
                ("chestplate", ArmorSlot::Chest),
                ("leggings", ArmorSlot::Legs),
                ("boots", ArmorSlot::Feet),
            ] {
                let def = registry
                    .get(
                        registry
                            .find(&format!("survival:{material}_{part}"))
                            .unwrap(),
                    )
                    .unwrap();
                assert_eq!(def.kind, ItemKind::Armor { slot });
                assert_eq!(def.max_stack, 1);
            }
        }
        assert_eq!(registry.len(), 682);
    }

    fn material(id: &str, max_stack: u16) -> String {
        format!("(id: \"{id}\", name: \"Материал\", max_stack: {max_stack}, kind: Material)")
    }

    #[test]
    fn default_catalog_preserves_original_stage_three_blocks_and_fixtures() {
        let registry = ContentRegistry::default();
        assert_eq!(registry.len(), 682);
        assert!(!registry.is_empty());
        let names = [
            "foundation:air",
            "foundation:grass",
            "foundation:dirt",
            "foundation:stone",
            "foundation:deepslate",
            "foundation:sand",
            "foundation:gravel",
            "foundation:oak_log",
            "foundation:oak_leaves",
            "foundation:oak_planks",
            "foundation:cobblestone",
            "foundation:glass",
            "foundation:water",
            "foundation:bedrock",
            "foundation:snow",
            "foundation:sandstone",
            "survival:chest",
        ];
        for (block, name) in names.iter().enumerate() {
            if block == 0 || block == 13 {
                assert_eq!(registry.item_for_block(block as crate::BlockId), None);
            } else {
                let item = registry.item_for_block(block as crate::BlockId).unwrap();
                assert_eq!(&registry.get(item).unwrap().id, name);
                assert_eq!(registry.block_for_item(item), Some(block as crate::BlockId));
            }
        }
        assert!(registry.item_for_block(17).is_some());
        assert_eq!(
            registry.max_stack(registry.find("survival:egg").unwrap()),
            16
        );
        assert_eq!(
            registry.max_stack(registry.find("survival:wooden_pickaxe").unwrap()),
            1
        );
        assert_eq!(registry.find("survival:wool"), None);
        assert_eq!(registry.find("survival:raw_beef"), None);
        assert_eq!(registry.get(ItemId(u16::MAX)).map(|d| &d.id), None);
        assert_eq!(registry.max_stack(ItemId(u16::MAX)), 0);
        assert_eq!(
            registry.block_for_item(registry.find("survival:egg").unwrap()),
            None
        );
        for (slot, index) in [
            (ArmorSlot::Head, 0),
            (ArmorSlot::Chest, 1),
            (ArmorSlot::Legs, 2),
            (ArmorSlot::Feet, 3),
        ] {
            assert_eq!(slot.index(), index);
            assert_eq!(
                registry
                    .iter()
                    .filter(|(_, item)| item.kind == ItemKind::Armor { slot })
                    .count(),
                4
            );
        }
    }

    #[test]
    fn break_drops_are_data_driven_and_resolve_forward_item_references() {
        let registry = ContentRegistry::default();
        assert_eq!(registry.drop_for_block(1), registry.item_for_block(2));
        assert_eq!(registry.drop_for_block(3), registry.item_for_block(10));
        for block in [0, 11, 12, 13, crate::BlockId::MAX] {
            assert_eq!(registry.drop_for_block(block), None);
        }
        for block in [2, 4, 5, 6, 7, 8, 9, 10, 14, 15, 16] {
            assert_eq!(
                registry.drop_for_block(block),
                registry.item_for_block(block)
            );
        }
        let source = r#"[(id:"test:block",name:"Блок",max_stack:64,kind:Block(block:"foundation:stone"),break_drop:Item("test:material")),(id:"test:material",name:"Материал",max_stack:16,kind:Material)]"#;
        let registry = ContentRegistry::from_ron(source).unwrap();
        assert_eq!(registry.drop_for_block(3), registry.find("test:material"));
        assert!(
            ContentRegistry::from_ron(&source.replace("test:material", "test:missing").replacen(
                r#"id:"test:missing""#,
                r#"id:"test:material""#,
                1
            ))
            .is_err()
        );
        let changed = registry
            .reload(&source.replace(r#"break_drop:Item("test:material")"#, "break_drop:Nothing"))
            .unwrap();
        assert_eq!(changed.drop_for_block(3), None);
        assert_eq!(registry.drop_for_block(3), registry.find("test:material"));
    }

    #[test]
    fn empty_catalog_is_valid_and_runtime_iteration_is_stable() {
        let empty = ContentRegistry::from_ron("[]").unwrap();
        assert!(empty.is_empty());
        let source = format!("[{}, {}]", material("test:z", 64), material("test:a", 16));
        let registry = ContentRegistry::from_ron(&source).unwrap();
        assert_eq!(
            registry
                .iter()
                .map(|(id, def)| (id, def.id.as_str()))
                .collect::<Vec<_>>(),
            vec![(ItemId(0), "test:z"), (ItemId(1), "test:a")]
        );
    }

    #[test]
    fn reload_preserves_ids_appends_additions_and_updates_properties() {
        let original = ContentRegistry::from_ron(&format!(
            "[{},{}]",
            material("test:a", 64),
            material("test:b", 16)
        ))
        .unwrap();
        let updated = original
            .reload(&format!(
                "[{},{},{}]",
                material("test:c", 32),
                material("test:b", 8),
                material("test:a", 16)
            ))
            .unwrap();
        assert_eq!(updated.find("test:a"), original.find("test:a"));
        assert_eq!(updated.find("test:b"), original.find("test:b"));
        assert_eq!(updated.find("test:c"), Some(ItemId(2)));
        assert_eq!(updated.max_stack(ItemId(0)), 16);
        assert_eq!(original.max_stack(ItemId(0)), 64);
        assert_eq!(
            updated
                .iter()
                .map(|(_, def)| def.id.as_str())
                .collect::<Vec<_>>(),
            ["test:a", "test:b", "test:c"]
        );
    }

    #[test]
    fn reload_rejects_removal_kind_and_block_or_armor_slot_changes_atomically() {
        let original = ContentRegistry::default();
        assert!(original.reload("[]").is_err());
        let ron = include_str!("../../../assets/data/items.ron");
        let remapped = ron
            .replace("block: \"foundation:grass\"", "block: \"test:temporary\"")
            .replace("block: \"foundation:dirt\"", "block: \"foundation:grass\"")
            .replace("block: \"test:temporary\"", "block: \"foundation:dirt\"");
        assert!(ContentRegistry::from_ron(&remapped).is_ok());
        assert!(original.reload(&remapped).is_err());
        for changed in [
            ron.replace("kind: Material", "kind: Food"),
            ron.replace("block: \"foundation:grass\"", "block: \"foundation:air\""),
            ron.replace("slot: Head", "slot: Feet"),
        ] {
            assert!(original.reload(&changed).is_err());
        }
        assert_eq!(original.len(), 682);
        assert_eq!(
            original.max_stack(original.find("survival:egg").unwrap()),
            16
        );
    }

    #[test]
    fn numeric_id_capacity_accepts_last_u16_and_rejects_overflow() {
        let mut source = String::from("[");
        for index in 0..=u16::MAX {
            source.push_str(&material(&format!("test:item_{index}"), 64));
            source.push(',');
        }
        source.push(']');
        let registry = ContentRegistry::from_ron(&source).unwrap();
        assert_eq!(registry.len(), 65_536);
        assert_eq!(registry.find("test:item_65535"), Some(ItemId(u16::MAX)));
        assert_eq!(
            registry.get(ItemId(u16::MAX)).unwrap().id,
            "test:item_65535"
        );
        source.pop();
        source.push_str(&material("test:overflow", 64));
        source.push(']');
        assert!(ContentRegistry::from_ron(&source).is_err());
        assert!(registry.reload(&source).is_err());
        assert_eq!(registry.len(), 65_536);
    }

    #[test]
    fn malformed_names_duplicates_caps_unknown_fields_and_block_references_fail() {
        for id in [
            "",
            "stone",
            ":stone",
            "test:",
            "test:a:b",
            "Test:a",
            "test:a b",
            "test:камень",
        ] {
            assert!(
                ContentRegistry::from_ron(&format!("[{}]", material(id, 64))).is_err(),
                "{id}"
            );
        }
        assert!(
            ContentRegistry::from_ron(&format!(
                "[{},{}]",
                material("test:a", 64),
                material("test:a", 64)
            ))
            .is_err()
        );
        for record in [
            material("test:a", 0),
            material("test:a", 65),
            "(id:\"test:a\",name:\"\",max_stack:64,kind:Material)".into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Tool)".into(),
            "(id:\"test:a\",name:\"A\",max_stack:16,kind:Armor(slot:Head))".into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Material,typo:true)".into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Block(block:\"survival:unknown_ore\"))"
                .into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Block(block:\"foundation:bedrock\"))"
                .into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Block(block:\"foundation:air\"))".into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Block(block:\"foundation:grass\",typo:1))"
                .into(),
        ] {
            assert!(
                ContentRegistry::from_ron(&format!("[{record}]")).is_err(),
                "{record}"
            );
        }
        let duplicate_block = "[(id:\"test:a\",name:\"A\",max_stack:64,kind:Block(block:\"foundation:grass\")),(id:\"test:b\",name:\"B\",max_stack:64,kind:Block(block:\"foundation:grass\"))]";
        assert!(ContentRegistry::from_ron(duplicate_block).is_err());
    }
}

// Эти типы проверяют только предложенные схемы; игровые системы их не загружают.
#[cfg(test)]
#[allow(dead_code)]
mod example_schemas {
    use super::*;
    use std::collections::HashSet;

    #[derive(Debug, Deserialize)]
    enum ToolType {
        Hand,
        Pickaxe,
    }
    #[derive(Debug, Deserialize)]
    enum Distribution {
        Triangular,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    enum RecipeKind {
        Shapeless,
        Smelting,
    }
    #[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
    enum Biome {
        Plains,
        Forest,
        Desert,
        Mountains,
        Ocean,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct DropRef {
        item: String,
        min: u16,
        max: u16,
        chance: f32,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Textures {
        top: u16,
        side: u16,
        bottom: u16,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ExampleBlock {
        id: String,
        name: String,
        textures: Textures,
        hardness: f32,
        tool_type: ToolType,
        min_tier: u8,
        requires_tool: bool,
        drops: Vec<DropRef>,
        xp: u16,
        collision: bool,
        opacity: bool,
        light_emission: u8,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct MaterialStats {
        durability: u16,
        speed: f32,
        sword_damage: f32,
        tier: u8,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ExampleTool {
        id: String,
        tool_type: ToolType,
        material_stats: MaterialStats,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct CountedItem {
        item: String,
        count: u16,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Recipe {
        id: String,
        kind: RecipeKind,
        inputs: Vec<CountedItem>,
        output: CountedItem,
        seconds: Option<f32>,
        xp: Option<f32>,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct XpRange {
        min: u16,
        max: u16,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct OreTextures {
        stone_layer: u16,
        deepslate_layer: u16,
        deepslate_below_y: i32,
        overlay_layer: u16,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Ore {
        id: String,
        replaces: Vec<String>,
        y_min: i32,
        y_max: i32,
        distribution: Distribution,
        veins_per_chunk: u16,
        max_vein_size: u16,
        hardness: f32,
        tool_type: ToolType,
        min_tier: u8,
        requires_tool: bool,
        drops: Vec<DropRef>,
        xp: XpRange,
        textures: OreTextures,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Food {
        item: String,
        hunger: f32,
        saturation: f32,
        eat_seconds: f32,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Hitbox {
        width: f32,
        height: f32,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Cow {
        id: String,
        name: String,
        max_health: f32,
        speed: f32,
        hitbox: Hitbox,
        model: String,
        biome_weights: HashMap<Biome, f32>,
        drops: Vec<DropRef>,
        xp: XpRange,
        fall_damage: bool,
        shearing: Option<String>,
        egg_interval_seconds: Option<f32>,
    }

    #[test]
    fn example_catalog_references_resolve() {
        let items: Vec<ItemDef> =
            ron::from_str(include_str!("../../../assets/data/examples/items.ron")).unwrap();
        let blocks: Vec<ExampleBlock> =
            ron::from_str(include_str!("../../../assets/data/examples/blocks.ron")).unwrap();
        let tools: Vec<ExampleTool> =
            ron::from_str(include_str!("../../../assets/data/examples/tools.ron")).unwrap();
        let recipes: Vec<Recipe> = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .from_str(include_str!("../../../assets/data/examples/recipes.ron"))
            .unwrap();
        let ores: Vec<Ore> =
            ron::from_str(include_str!("../../../assets/data/examples/ores.ron")).unwrap();
        let foods: Vec<Food> =
            ron::from_str(include_str!("../../../assets/data/examples/food.ron")).unwrap();
        let cow: Cow = ron::from_str(include_str!(
            "../../../assets/data/examples/animals/cow.ron"
        ))
        .unwrap();
        let by_name: HashMap<_, _> = items.iter().map(|item| (item.id.as_str(), item)).collect();
        assert_eq!(by_name.len(), items.len());
        let mut block_names: HashSet<&str> = BLOCK_NAMES[..16].iter().copied().collect();
        let ore_names: HashSet<_> = ores.iter().map(|ore| ore.id.as_str()).collect();
        assert_eq!(ore_names.len(), ores.len());
        for block in &blocks {
            assert!(valid_namespaced_id(&block.id));
            block_names.insert(&block.id);
        }
        block_names.extend(ore_names);
        for item in &items {
            assert!(valid_namespaced_id(&item.id));
            assert!((1..=64).contains(&item.max_stack));
            if let ItemKind::Block { block } = &item.kind {
                assert!(block_names.contains(block.as_str()), "{}", block);
            }
        }
        let check_drops = |drops: &[DropRef]| {
            for drop in drops {
                let item = by_name
                    .get(drop.item.as_str())
                    .expect("ссылка добычи на предмет");
                assert!(drop.min > 0 && drop.min <= drop.max && drop.max <= item.max_stack);
                assert!(drop.chance.is_finite() && (0.0..=1.0).contains(&drop.chance));
            }
        };
        for block in &blocks {
            check_drops(&block.drops);
        }
        for ore in &ores {
            assert!(valid_namespaced_id(&ore.id));
            assert!(ore.y_min <= ore.y_max);
            for replacement in &ore.replaces {
                assert!(block_names.contains(replacement.as_str()));
            }
            check_drops(&ore.drops);
        }
        for tool in &tools {
            assert_eq!(by_name[tool.id.as_str()].kind, ItemKind::Tool);
        }
        let pickaxe = tools
            .iter()
            .find(|tool| tool.id == "survival:wooden_pickaxe")
            .unwrap();
        assert_eq!(pickaxe.material_stats.durability, 59);
        assert_eq!(pickaxe.material_stats.speed, 2.0);
        assert_eq!(pickaxe.material_stats.tier, 0);
        let recipe_names: HashSet<_> = recipes.iter().map(|recipe| recipe.id.as_str()).collect();
        assert_eq!(recipe_names.len(), recipes.len());
        for recipe in &recipes {
            assert!(valid_namespaced_id(&recipe.id));
            assert!(!recipe.inputs.is_empty());
            for stack in recipe.inputs.iter().chain(std::iter::once(&recipe.output)) {
                let item = by_name
                    .get(stack.item.as_str())
                    .expect("ссылка рецепта на предмет");
                assert!(stack.count > 0 && stack.count <= item.max_stack);
            }
            if recipe.kind == RecipeKind::Smelting {
                assert!(
                    recipe
                        .seconds
                        .is_some_and(|value| value.is_finite() && value > 0.0)
                );
                assert!(
                    recipe
                        .xp
                        .is_some_and(|value| value.is_finite() && value >= 0.0)
                );
            }
        }
        let stone = blocks
            .iter()
            .find(|block| block.id == "foundation:stone")
            .unwrap();
        assert_eq!(stone.drops[0].item, "foundation:cobblestone");
        let planks = recipes
            .iter()
            .find(|recipe| recipe.id == "survival:planks_from_log")
            .unwrap();
        assert_eq!(planks.inputs[0].item, "foundation:oak_log");
        assert_eq!(planks.inputs[0].count, 1);
        assert_eq!(planks.output.item, "foundation:oak_planks");
        assert_eq!(planks.output.count, 4);
        let food_names: HashSet<_> = foods.iter().map(|food| food.item.as_str()).collect();
        assert_eq!(food_names.len(), foods.len());
        for food in &foods {
            assert_eq!(by_name[food.item.as_str()].kind, ItemKind::Food);
            assert!(food.hunger.is_finite() && food.hunger > 0.0);
            assert!(food.saturation.is_finite() && food.saturation >= 0.0);
            assert!(food.eat_seconds.is_finite() && food.eat_seconds > 0.0);
        }
        for meat in ["beef", "pork", "mutton", "chicken"] {
            let raw = format!("survival:raw_{meat}");
            let cooked = format!("survival:cooked_{meat}");
            let recipe = recipes.iter().find(|recipe| recipe.id == cooked).unwrap();
            assert_eq!(recipe.inputs[0].item, raw);
            assert_eq!(recipe.output.item, cooked);
            assert!(food_names.contains(raw.as_str()) && food_names.contains(cooked.as_str()));
        }
        check_drops(&cow.drops);
        assert_eq!(cow.id, "survival:cow");
        assert!(cow.drops.iter().any(|drop| drop.item == "survival:leather"));
        assert!(
            cow.drops
                .iter()
                .any(|drop| drop.item == "survival:raw_beef")
        );
        // Шерсть, мясо и животные остаются примерами будущих этапов.
        let active = ContentRegistry::default();
        for future in ["survival:wool", "survival:raw_beef"] {
            assert_eq!(active.find(future), None);
        }
    }
}
