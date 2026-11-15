//! Stable block IDs. Saved worlds store these IDs, never display names.

/// Номер блока. Два байта вместо одного: типов блоков больше 256.
pub type BlockId = u16;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlockDef {
    pub id: BlockId,
    pub name: &'static str,
    pub texture: &'static str,
    pub hardness: f32,
    pub collision: bool,
    pub opacity: bool,
    pub light_emission: u8,
}

pub const AIR: BlockId = 0;
pub const GRASS: BlockId = 1;
pub const DIRT: BlockId = 2;
pub const STONE: BlockId = 3;
pub const DEEP_STONE: BlockId = 4;
pub const SAND: BlockId = 5;
pub const GRAVEL: BlockId = 6;
pub const WOOD: BlockId = 7;
pub const LEAVES: BlockId = 8;
pub const PLANKS: BlockId = 9;
pub const COBBLE: BlockId = 10;
pub const GLASS: BlockId = 11;
pub const WATER: BlockId = 12;
pub const BEDROCK: BlockId = 13;
pub const SNOW: BlockId = 14;
pub const SANDSTONE: BlockId = 15;
pub const CHEST: BlockId = 16;
pub const COAL_ORE: BlockId = 17;
pub const IRON_ORE: BlockId = 18;
pub const GOLD_ORE: BlockId = 19;
pub const DIAMOND_ORE: BlockId = 20;
pub const IRON_BLOCK: BlockId = 21;
pub const GOLD_BLOCK: BlockId = 22;
pub const DIAMOND_BLOCK: BlockId = 23;
pub const CRAFTING_TABLE: BlockId = 24;
pub const FURNACE: BlockId = 25;
pub const BRICKS: BlockId = 26;
pub const STONE_BRICKS: BlockId = 27;
pub const MOSSY_COBBLE: BlockId = 28;
pub const BIRCH_LOG: BlockId = 29;
pub const BIRCH_LEAVES: BlockId = 30;
pub const BIRCH_PLANKS: BlockId = 31;
pub const CLAY: BlockId = 32;
pub const BOOKSHELF: BlockId = 33;
pub const LAMP: BlockId = 34;
pub const ICE: BlockId = 35;
pub const OBSIDIAN: BlockId = 36;
pub const WOOL: BlockId = 37;
pub const TERRACOTTA: BlockId = 38;
pub const SMOOTH_STONE: BlockId = 39;
pub const COAL_BLOCK: BlockId = 40;
/// Работающая печь: состояние блока, а не отдельный предмет.
pub const FURNACE_LIT: BlockId = 41;
pub const BARREL: BlockId = 42;
pub const SMOKER: BlockId = 43;
pub const SMOKER_LIT: BlockId = 44;
pub const BLAST_FURNACE: BlockId = 45;
pub const BLAST_FURNACE_LIT: BlockId = 46;
pub const STONECUTTER: BlockId = 47;
/// Общий сундук: все такие блоки открывают одно хранилище игрока.
pub const VAULT: BlockId = 48;
// Декоративные блоки обновления «Палитра».
pub const WOOL_ORANGE: BlockId = 49;
pub const WOOL_MAGENTA: BlockId = 50;
pub const WOOL_LIGHT_BLUE: BlockId = 51;
pub const WOOL_YELLOW: BlockId = 52;
pub const WOOL_LIME: BlockId = 53;
pub const WOOL_PINK: BlockId = 54;
pub const WOOL_GRAY: BlockId = 55;
pub const WOOL_LIGHT_GRAY: BlockId = 56;
pub const WOOL_CYAN: BlockId = 57;
pub const WOOL_PURPLE: BlockId = 58;
pub const WOOL_BLUE: BlockId = 59;
pub const WOOL_BROWN: BlockId = 60;
pub const WOOL_GREEN: BlockId = 61;
pub const WOOL_RED: BlockId = 62;
pub const WOOL_BLACK: BlockId = 63;
pub const CONCRETE_WHITE: BlockId = 64;
pub const CONCRETE_ORANGE: BlockId = 65;
pub const CONCRETE_MAGENTA: BlockId = 66;
pub const CONCRETE_LIGHT_BLUE: BlockId = 67;
pub const CONCRETE_YELLOW: BlockId = 68;
pub const CONCRETE_LIME: BlockId = 69;
pub const CONCRETE_PINK: BlockId = 70;
pub const CONCRETE_GRAY: BlockId = 71;
pub const CONCRETE_LIGHT_GRAY: BlockId = 72;
pub const CONCRETE_CYAN: BlockId = 73;
pub const CONCRETE_PURPLE: BlockId = 74;
pub const CONCRETE_BLUE: BlockId = 75;
pub const CONCRETE_BROWN: BlockId = 76;
pub const CONCRETE_GREEN: BlockId = 77;
pub const CONCRETE_RED: BlockId = 78;
pub const CONCRETE_BLACK: BlockId = 79;
pub const GRANITE: BlockId = 80;
pub const POLISHED_GRANITE: BlockId = 81;
pub const MARBLE: BlockId = 82;
pub const POLISHED_MARBLE: BlockId = 83;
pub const BASALT: BlockId = 84;
pub const POLISHED_BASALT: BlockId = 85;
pub const SLATE: BlockId = 86;
pub const POLISHED_SLATE: BlockId = 87;
pub const LIMESTONE: BlockId = 88;
pub const POLISHED_LIMESTONE: BlockId = 89;
pub const PINE_LOG: BlockId = 90;
pub const PINE_LEAVES: BlockId = 91;
pub const PINE_PLANKS: BlockId = 92;
pub const SANDSTONE_BRICKS: BlockId = 93;
pub const MOSSY_STONE_BRICKS: BlockId = 94;
pub const CRACKED_STONE_BRICKS: BlockId = 95;
pub const DARK_BRICKS: BlockId = 96;
pub const STONE_TILES: BlockId = 97;
// Растения: крестовая модель без столкновений.
pub const POPPY: BlockId = 98;
pub const DANDELION: BlockId = 99;
pub const CORNFLOWER: BlockId = 100;
pub const DAISY: BlockId = 101;
pub const TALL_GRASS: BlockId = 102;
// Большая партия: породы дерева, руды, стекло, терракота, природа, плиты.
pub const ACACIA_LOG: BlockId = 103;
pub const ACACIA_LEAVES: BlockId = 104;
pub const ACACIA_PLANKS: BlockId = 105;
pub const JUNGLE_LOG: BlockId = 106;
pub const JUNGLE_LEAVES: BlockId = 107;
pub const JUNGLE_PLANKS: BlockId = 108;
pub const DARK_OAK_LOG: BlockId = 109;
pub const DARK_OAK_LEAVES: BlockId = 110;
pub const DARK_OAK_PLANKS: BlockId = 111;
pub const MANGROVE_LOG: BlockId = 112;
pub const MANGROVE_LEAVES: BlockId = 113;
pub const MANGROVE_PLANKS: BlockId = 114;
pub const CHERRY_LOG: BlockId = 115;
pub const CHERRY_LEAVES: BlockId = 116;
pub const CHERRY_PLANKS: BlockId = 117;
pub const COPPER_ORE: BlockId = 118;
pub const EMERALD_ORE: BlockId = 119;
pub const LAPIS_ORE: BlockId = 120;
pub const COPPER_BLOCK: BlockId = 121;
pub const EMERALD_BLOCK: BlockId = 122;
pub const LAPIS_BLOCK: BlockId = 123;
pub const GLASS_WHITE: BlockId = 124;
pub const GLASS_ORANGE: BlockId = 125;
pub const GLASS_MAGENTA: BlockId = 126;
pub const GLASS_LIGHT_BLUE: BlockId = 127;
pub const GLASS_YELLOW: BlockId = 128;
pub const GLASS_LIME: BlockId = 129;
pub const GLASS_PINK: BlockId = 130;
pub const GLASS_GRAY: BlockId = 131;
pub const GLASS_LIGHT_GRAY: BlockId = 132;
pub const GLASS_CYAN: BlockId = 133;
pub const GLASS_PURPLE: BlockId = 134;
pub const GLASS_BLUE: BlockId = 135;
pub const GLASS_BROWN: BlockId = 136;
pub const GLASS_GREEN: BlockId = 137;
pub const GLASS_RED: BlockId = 138;
pub const GLASS_BLACK: BlockId = 139;
pub const TERRACOTTA_WHITE: BlockId = 140;
pub const TERRACOTTA_ORANGE: BlockId = 141;
pub const TERRACOTTA_MAGENTA: BlockId = 142;
pub const TERRACOTTA_LIGHT_BLUE: BlockId = 143;
pub const TERRACOTTA_YELLOW: BlockId = 144;
pub const TERRACOTTA_LIME: BlockId = 145;
pub const TERRACOTTA_PINK: BlockId = 146;
pub const TERRACOTTA_GRAY: BlockId = 147;
pub const TERRACOTTA_LIGHT_GRAY: BlockId = 148;
pub const TERRACOTTA_CYAN: BlockId = 149;
pub const TERRACOTTA_PURPLE: BlockId = 150;
pub const TERRACOTTA_BLUE: BlockId = 151;
pub const TERRACOTTA_BROWN: BlockId = 152;
pub const TERRACOTTA_GREEN: BlockId = 153;
pub const TERRACOTTA_RED: BlockId = 154;
pub const TERRACOTTA_BLACK: BlockId = 155;
pub const SPONGE: BlockId = 156;
pub const MELON: BlockId = 157;
pub const PUMPKIN: BlockId = 158;
pub const HAY_BALE: BlockId = 159;
pub const BONE_BLOCK: BlockId = 160;
pub const MUD: BlockId = 161;
pub const PACKED_MUD: BlockId = 162;
pub const MUD_BRICKS: BlockId = 163;
pub const MOSS_BLOCK: BlockId = 164;
pub const PODZOL: BlockId = 165;
pub const COARSE_DIRT: BlockId = 166;
pub const RED_SAND: BlockId = 167;
pub const RED_SANDSTONE: BlockId = 168;
pub const CALCITE: BlockId = 169;
pub const TUFF: BlockId = 170;
pub const AMETHYST_BLOCK: BlockId = 171;
pub const QUARTZ_BLOCK: BlockId = 172;
pub const OAK_SLAB: BlockId = 173;
pub const BIRCH_SLAB: BlockId = 174;
pub const PINE_SLAB: BlockId = 175;
pub const STONE_SLAB: BlockId = 176;
pub const COBBLESTONE_SLAB: BlockId = 177;
pub const STONE_BRICK_SLAB: BlockId = 178;
pub const SMOOTH_STONE_SLAB: BlockId = 179;
pub const SANDSTONE_SLAB: BlockId = 180;
pub const BRICK_SLAB: BlockId = 181;
pub const GRANITE_SLAB: BlockId = 182;
pub const MARBLE_SLAB: BlockId = 183;
pub const DARK_BRICK_SLAB: BlockId = 184;
// Жидкости: уровни течения. Источники — WATER (12) и LAVA.
pub const FLOW_WATER_1: BlockId = 185;
pub const FLOW_WATER_2: BlockId = 186;
pub const FLOW_WATER_3: BlockId = 187;
pub const FLOW_WATER_4: BlockId = 188;
pub const FLOW_WATER_5: BlockId = 189;
pub const FLOW_WATER_6: BlockId = 190;
pub const FLOW_WATER_7: BlockId = 191;
pub const LAVA: BlockId = 192;
pub const FLOW_LAVA_1: BlockId = 193;
pub const FLOW_LAVA_2: BlockId = 194;
pub const FLOW_LAVA_3: BlockId = 195;
// Часть 1 списка блоков Minecraft: камень, дерево, растения, кровати и прочее.
pub const DIORITE: BlockId = 196;
pub const POLISHED_DIORITE: BlockId = 197;
pub const ANDESITE: BlockId = 198;
pub const POLISHED_ANDESITE: BlockId = 199;
pub const SPRUCE_PLANKS: BlockId = 200;
pub const BAMBOO_PLANKS: BlockId = 201;
pub const BAMBOO_MOSAIC: BlockId = 202;
pub const SUSPICIOUS_SAND: BlockId = 203;
pub const SUSPICIOUS_GRAVEL: BlockId = 204;
pub const DEEPSLATE_GOLD_ORE: BlockId = 205;
pub const DEEPSLATE_IRON_ORE: BlockId = 206;
pub const DEEPSLATE_COAL_ORE: BlockId = 207;
pub const NETHER_GOLD_ORE: BlockId = 208;
pub const SPRUCE_LOG: BlockId = 209;
pub const BAMBOO_BLOCK: BlockId = 210;
pub const STRIPPED_SPRUCE_LOG: BlockId = 211;
pub const STRIPPED_BIRCH_LOG: BlockId = 212;
pub const STRIPPED_JUNGLE_LOG: BlockId = 213;
pub const STRIPPED_ACACIA_LOG: BlockId = 214;
pub const STRIPPED_CHERRY_LOG: BlockId = 215;
pub const STRIPPED_DARK_OAK_LOG: BlockId = 216;
pub const STRIPPED_OAK_LOG: BlockId = 217;
pub const STRIPPED_MANGROVE_LOG: BlockId = 218;
pub const STRIPPED_BAMBOO_BLOCK: BlockId = 219;
pub const OAK_WOOD: BlockId = 220;
pub const SPRUCE_WOOD: BlockId = 221;
pub const BIRCH_WOOD: BlockId = 222;
pub const JUNGLE_WOOD: BlockId = 223;
pub const ACACIA_WOOD: BlockId = 224;
pub const CHERRY_WOOD: BlockId = 225;
pub const DARK_OAK_WOOD: BlockId = 226;
pub const MANGROVE_WOOD: BlockId = 227;
pub const STRIPPED_OAK_WOOD: BlockId = 228;
pub const STRIPPED_SPRUCE_WOOD: BlockId = 229;
pub const STRIPPED_BIRCH_WOOD: BlockId = 230;
pub const STRIPPED_JUNGLE_WOOD: BlockId = 231;
pub const STRIPPED_ACACIA_WOOD: BlockId = 232;
pub const STRIPPED_CHERRY_WOOD: BlockId = 233;
pub const STRIPPED_DARK_OAK_WOOD: BlockId = 234;
pub const STRIPPED_MANGROVE_WOOD: BlockId = 235;
pub const WET_SPONGE: BlockId = 236;
pub const DEEPSLATE_LAPIS_ORE: BlockId = 237;
pub const DISPENSER: BlockId = 238;
pub const CHISELED_SANDSTONE: BlockId = 239;
pub const CUT_SANDSTONE: BlockId = 240;
pub const NOTE_BLOCK: BlockId = 241;
pub const STICKY_PISTON: BlockId = 242;
pub const PISTON: BlockId = 243;
pub const TNT: BlockId = 244;
pub const CHISELED_BOOKSHELF: BlockId = 245;
pub const MANGROVE_ROOTS: BlockId = 246;
pub const MUDDY_MANGROVE_ROOTS: BlockId = 247;
pub const SPRUCE_LEAVES: BlockId = 248;
pub const AZALEA_LEAVES: BlockId = 249;
pub const FLOWERING_AZALEA_LEAVES: BlockId = 250;
pub const OAK_SAPLING: BlockId = 251;
pub const SPRUCE_SAPLING: BlockId = 252;
pub const BIRCH_SAPLING: BlockId = 253;
pub const JUNGLE_SAPLING: BlockId = 254;
pub const ACACIA_SAPLING: BlockId = 255;
pub const CHERRY_SAPLING: BlockId = 256;
pub const DARK_OAK_SAPLING: BlockId = 257;
pub const MANGROVE_PROPAGULE: BlockId = 258;
pub const COBWEB: BlockId = 259;
pub const SHORT_GRASS: BlockId = 260;
pub const FERN: BlockId = 261;
pub const DEAD_BUSH: BlockId = 262;
pub const SEAGRASS: BlockId = 263;
pub const TALL_SEAGRASS: BlockId = 264;
pub const TORCHFLOWER: BlockId = 265;
pub const BLUE_ORCHID: BlockId = 266;
pub const ALLIUM: BlockId = 267;
pub const AZURE_BLUET: BlockId = 268;
pub const RED_TULIP: BlockId = 269;
pub const ORANGE_TULIP: BlockId = 270;
pub const WHITE_TULIP: BlockId = 271;
pub const PINK_TULIP: BlockId = 272;
pub const WITHER_ROSE: BlockId = 273;
pub const LILY_OF_THE_VALLEY: BlockId = 274;
pub const BROWN_MUSHROOM: BlockId = 275;
pub const RED_MUSHROOM: BlockId = 276;
pub const TORCH: BlockId = 277;
pub const WALL_TORCH: BlockId = 278;
pub const FIRE: BlockId = 279;
pub const SOUL_FIRE: BlockId = 280;
pub const POWERED_RAIL: BlockId = 281;
pub const DETECTOR_RAIL: BlockId = 282;
pub const WHITE_BED: BlockId = 283;
pub const ORANGE_BED: BlockId = 284;
pub const MAGENTA_BED: BlockId = 285;
pub const LIGHT_BLUE_BED: BlockId = 286;
pub const YELLOW_BED: BlockId = 287;
pub const LIME_BED: BlockId = 288;
pub const PINK_BED: BlockId = 289;
pub const GRAY_BED: BlockId = 290;
pub const LIGHT_GRAY_BED: BlockId = 291;
pub const CYAN_BED: BlockId = 292;
pub const PURPLE_BED: BlockId = 293;
pub const BLUE_BED: BlockId = 294;
pub const BROWN_BED: BlockId = 295;
pub const GREEN_BED: BlockId = 296;
pub const RED_BED: BlockId = 297;
pub const BLACK_BED: BlockId = 298;
pub const PISTON_HEAD: BlockId = 299;
pub const MOVING_PISTON: BlockId = 300;
/// Растения этой партии идут подряд: OAK_SAPLING…SOUL_FIRE.
// Направления: отдельный ID на каждое (север — основной блок).
pub const DISPENSER_SOUTH: BlockId = 301;
pub const DISPENSER_WEST: BlockId = 302;
pub const DISPENSER_EAST: BlockId = 303;
pub const DISPENSER_UP: BlockId = 304;
pub const DISPENSER_DOWN: BlockId = 305;
pub const PISTON_SOUTH: BlockId = 306;
pub const PISTON_WEST: BlockId = 307;
pub const PISTON_EAST: BlockId = 308;
pub const PISTON_UP: BlockId = 309;
pub const PISTON_DOWN: BlockId = 310;
pub const STICKY_PISTON_SOUTH: BlockId = 311;
pub const STICKY_PISTON_WEST: BlockId = 312;
pub const STICKY_PISTON_EAST: BlockId = 313;
pub const STICKY_PISTON_UP: BlockId = 314;
pub const STICKY_PISTON_DOWN: BlockId = 315;
pub const WALL_TORCH_SOUTH: BlockId = 316;
pub const WALL_TORCH_WEST: BlockId = 317;
pub const WALL_TORCH_EAST: BlockId = 318;
pub const POWERED_RAIL_X: BlockId = 319;
pub const DETECTOR_RAIL_X: BlockId = 320;
pub const SPAWNER: BlockId = 321;
pub const OAK_STAIRS: BlockId = 322;
pub const REDSTONE_WIRE: BlockId = 326;
pub const DEEPSLATE_DIAMOND_ORE: BlockId = 327;
pub const WHEAT: BlockId = 328;
pub const FARMLAND: BlockId = 329;
pub const OAK_SIGN: BlockId = 330;
pub const SPRUCE_SIGN: BlockId = 334;
pub const BIRCH_SIGN: BlockId = 338;
pub const JUNGLE_SIGN: BlockId = 342;
pub const ACACIA_SIGN: BlockId = 346;
pub const CHERRY_SIGN: BlockId = 350;
pub const DARK_OAK_SIGN: BlockId = 354;
pub const MANGROVE_SIGN: BlockId = 358;
pub const BAMBOO_SIGN: BlockId = 362;
pub const OAK_DOOR: BlockId = 366;
pub const LADDER: BlockId = 382;
pub const RAIL: BlockId = 386;
pub const COBBLESTONE_STAIRS: BlockId = 388;
pub const OAK_WALL_SIGN: BlockId = 392;
pub const SPRUCE_WALL_SIGN: BlockId = 396;
pub const BIRCH_WALL_SIGN: BlockId = 400;
pub const JUNGLE_WALL_SIGN: BlockId = 404;
pub const ACACIA_WALL_SIGN: BlockId = 408;
pub const CHERRY_WALL_SIGN: BlockId = 412;
pub const DARK_OAK_WALL_SIGN: BlockId = 416;
pub const MANGROVE_WALL_SIGN: BlockId = 420;
pub const BAMBOO_WALL_SIGN: BlockId = 424;
pub const OAK_HANGING_SIGN: BlockId = 428;
pub const SPRUCE_HANGING_SIGN: BlockId = 430;
pub const BIRCH_HANGING_SIGN: BlockId = 432;
pub const ACACIA_HANGING_SIGN: BlockId = 434;
pub const CHERRY_HANGING_SIGN: BlockId = 436;
pub const JUNGLE_HANGING_SIGN: BlockId = 438;
pub const DARK_OAK_HANGING_SIGN: BlockId = 440;
pub const CRIMSON_HANGING_SIGN: BlockId = 442;
pub const WARPED_HANGING_SIGN: BlockId = 444;
pub const MANGROVE_HANGING_SIGN: BlockId = 446;
pub const BAMBOO_HANGING_SIGN: BlockId = 448;
pub const OAK_WALL_HANGING_SIGN: BlockId = 450;
pub const SPRUCE_WALL_HANGING_SIGN: BlockId = 452;
pub const BIRCH_WALL_HANGING_SIGN: BlockId = 454;
pub const ACACIA_WALL_HANGING_SIGN: BlockId = 456;
pub const CHERRY_WALL_HANGING_SIGN: BlockId = 458;
pub const JUNGLE_WALL_HANGING_SIGN: BlockId = 460;
pub const DARK_OAK_WALL_HANGING_SIGN: BlockId = 462;
pub const CRIMSON_WALL_HANGING_SIGN: BlockId = 464;
pub const WARPED_WALL_HANGING_SIGN: BlockId = 466;
pub const MANGROVE_WALL_HANGING_SIGN: BlockId = 468;
pub const BAMBOO_WALL_HANGING_SIGN: BlockId = 470;
pub const LEVER: BlockId = 472;
pub const STONE_PRESSURE_PLATE: BlockId = 480;
pub const IRON_DOOR: BlockId = 481;
pub const OAK_PRESSURE_PLATE: BlockId = 497;
pub const SPRUCE_PRESSURE_PLATE: BlockId = 498;
pub const BIRCH_PRESSURE_PLATE: BlockId = 499;
pub const JUNGLE_PRESSURE_PLATE: BlockId = 500;
pub const ACACIA_PRESSURE_PLATE: BlockId = 501;
pub const CHERRY_PRESSURE_PLATE: BlockId = 502;
pub const DARK_OAK_PRESSURE_PLATE: BlockId = 503;
pub const MANGROVE_PRESSURE_PLATE: BlockId = 504;
pub const BAMBOO_PRESSURE_PLATE: BlockId = 505;
pub const REDSTONE_ORE: BlockId = 506;
pub const DEEPSLATE_REDSTONE_ORE: BlockId = 507;
pub const REDSTONE_TORCH: BlockId = 508;
pub const REDSTONE_WALL_TORCH: BlockId = 509;
pub const STONE_BUTTON: BlockId = 513;
pub const SNOW_LAYER: BlockId = 517;
pub const CACTUS: BlockId = 518;
pub const SUGAR_CANE: BlockId = 519;
pub const JUKEBOX: BlockId = 520;
pub const OAK_FENCE: BlockId = 521;
pub const NETHERRACK: BlockId = 522;
pub const SOUL_SAND: BlockId = 523;
pub const SOUL_SOIL: BlockId = 524;
pub const SOUL_TORCH: BlockId = 525;
pub const SOUL_WALL_TORCH: BlockId = 526;
pub const GLOWSTONE: BlockId = 530;
pub const NETHER_PORTAL: BlockId = 531;
pub const CARVED_PUMPKIN: BlockId = 533;
pub const JACK_O_LANTERN: BlockId = 537;
pub const CAKE: BlockId = 541;
pub const REPEATER: BlockId = 542;
pub const OAK_TRAPDOOR: BlockId = 546;
pub const SPRUCE_TRAPDOOR: BlockId = 551;
pub const BIRCH_TRAPDOOR: BlockId = 556;
pub const JUNGLE_TRAPDOOR: BlockId = 561;
pub const ACACIA_TRAPDOOR: BlockId = 566;
pub const CHERRY_TRAPDOOR: BlockId = 571;
pub const DARK_OAK_TRAPDOOR: BlockId = 576;
pub const MANGROVE_TRAPDOOR: BlockId = 581;
pub const BAMBOO_TRAPDOOR: BlockId = 586;
pub const CHISELED_STONE_BRICKS: BlockId = 591;
pub const INFESTED_STONE: BlockId = 592;
pub const INFESTED_COBBLESTONE: BlockId = 593;
pub const INFESTED_STONE_BRICKS: BlockId = 594;
pub const INFESTED_MOSSY_STONE_BRICKS: BlockId = 595;
pub const INFESTED_CRACKED_STONE_BRICKS: BlockId = 596;
pub const INFESTED_CHISELED_STONE_BRICKS: BlockId = 597;
pub const BROWN_MUSHROOM_BLOCK: BlockId = 598;
pub const RED_MUSHROOM_BLOCK: BlockId = 599;
pub const MUSHROOM_STEM: BlockId = 600;
pub const IRON_BARS: BlockId = 601;
pub const CHAIN: BlockId = 602;
pub const GLASS_PANE: BlockId = 603;
/// Блоки части 2: первый и последний ID.
/// Что означает номер варианта внутри семейства.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FamilyKind {
    /// Четыре стороны света: север, юг, запад, восток.
    H4,
    /// Ось: 0 вдоль X, 1 вдоль Z.
    AX2,
    /// Рычаг: выключен/включён × четыре стороны.
    Lever,
    /// Дверь: нижняя/верхняя половина × закрыта/открыта × четыре стороны.
    Door,
    /// Люк: закрыт, затем открыт на четыре стороны.
    Trap,
    /// Рельсы: вдоль Z, вдоль X.
    Rail,
}

/// Семейства блоков с состоянием: (первый ID, число вариантов, вид).
pub const FAMILIES: [(BlockId, u8, FamilyKind); 63] = [
    (322, 4, FamilyKind::H4),
    (330, 4, FamilyKind::H4),
    (334, 4, FamilyKind::H4),
    (338, 4, FamilyKind::H4),
    (342, 4, FamilyKind::H4),
    (346, 4, FamilyKind::H4),
    (350, 4, FamilyKind::H4),
    (354, 4, FamilyKind::H4),
    (358, 4, FamilyKind::H4),
    (362, 4, FamilyKind::H4),
    (366, 16, FamilyKind::Door),
    (382, 4, FamilyKind::H4),
    (386, 2, FamilyKind::Rail),
    (388, 4, FamilyKind::H4),
    (392, 4, FamilyKind::H4),
    (396, 4, FamilyKind::H4),
    (400, 4, FamilyKind::H4),
    (404, 4, FamilyKind::H4),
    (408, 4, FamilyKind::H4),
    (412, 4, FamilyKind::H4),
    (416, 4, FamilyKind::H4),
    (420, 4, FamilyKind::H4),
    (424, 4, FamilyKind::H4),
    (428, 2, FamilyKind::AX2),
    (430, 2, FamilyKind::AX2),
    (432, 2, FamilyKind::AX2),
    (434, 2, FamilyKind::AX2),
    (436, 2, FamilyKind::AX2),
    (438, 2, FamilyKind::AX2),
    (440, 2, FamilyKind::AX2),
    (442, 2, FamilyKind::AX2),
    (444, 2, FamilyKind::AX2),
    (446, 2, FamilyKind::AX2),
    (448, 2, FamilyKind::AX2),
    (450, 2, FamilyKind::AX2),
    (452, 2, FamilyKind::AX2),
    (454, 2, FamilyKind::AX2),
    (456, 2, FamilyKind::AX2),
    (458, 2, FamilyKind::AX2),
    (460, 2, FamilyKind::AX2),
    (462, 2, FamilyKind::AX2),
    (464, 2, FamilyKind::AX2),
    (466, 2, FamilyKind::AX2),
    (468, 2, FamilyKind::AX2),
    (470, 2, FamilyKind::AX2),
    (472, 8, FamilyKind::Lever),
    (481, 16, FamilyKind::Door),
    (509, 4, FamilyKind::H4),
    (513, 4, FamilyKind::H4),
    (526, 4, FamilyKind::H4),
    (531, 2, FamilyKind::AX2),
    (533, 4, FamilyKind::H4),
    (537, 4, FamilyKind::H4),
    (542, 4, FamilyKind::H4),
    (546, 5, FamilyKind::Trap),
    (551, 5, FamilyKind::Trap),
    (556, 5, FamilyKind::Trap),
    (561, 5, FamilyKind::Trap),
    (566, 5, FamilyKind::Trap),
    (571, 5, FamilyKind::Trap),
    (576, 5, FamilyKind::Trap),
    (581, 5, FamilyKind::Trap),
    (586, 5, FamilyKind::Trap),
];

pub const PART2_FIRST: BlockId = 321;
pub const PART2_LAST: BlockId = 603;
pub const PLANT2_FIRST: BlockId = 251;
pub const PLANT2_LAST: BlockId = 280;

const fn block(
    id: BlockId,
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

#[allow(clippy::large_const_arrays)]
pub const BLOCKS: [BlockDef; 604] = [
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
    block(POPPY, "Poppy", "poppy", 0.05, false, false),
    block(DANDELION, "Dandelion", "dandelion", 0.05, false, false),
    block(CORNFLOWER, "Cornflower", "cornflower", 0.05, false, false),
    block(DAISY, "Daisy", "daisy", 0.05, false, false),
    block(TALL_GRASS, "Tall Grass", "tall_grass", 0.05, false, false),
    block(ACACIA_LOG, "Acacia Log", "acacia_log", 2.0, true, true),
    block(
        ACACIA_LEAVES,
        "Acacia Leaves",
        "acacia_leaves",
        0.2,
        true,
        false,
    ),
    block(
        ACACIA_PLANKS,
        "Acacia Planks",
        "acacia_planks",
        2.0,
        true,
        true,
    ),
    block(JUNGLE_LOG, "Jungle Log", "jungle_log", 2.0, true, true),
    block(
        JUNGLE_LEAVES,
        "Jungle Leaves",
        "jungle_leaves",
        0.2,
        true,
        false,
    ),
    block(
        JUNGLE_PLANKS,
        "Jungle Planks",
        "jungle_planks",
        2.0,
        true,
        true,
    ),
    block(
        DARK_OAK_LOG,
        "Dark Oak Log",
        "dark_oak_log",
        2.0,
        true,
        true,
    ),
    block(
        DARK_OAK_LEAVES,
        "Dark Oak Leaves",
        "dark_oak_leaves",
        0.2,
        true,
        false,
    ),
    block(
        DARK_OAK_PLANKS,
        "Dark Oak Planks",
        "dark_oak_planks",
        2.0,
        true,
        true,
    ),
    block(
        MANGROVE_LOG,
        "Mangrove Log",
        "mangrove_log",
        2.0,
        true,
        true,
    ),
    block(
        MANGROVE_LEAVES,
        "Mangrove Leaves",
        "mangrove_leaves",
        0.2,
        true,
        false,
    ),
    block(
        MANGROVE_PLANKS,
        "Mangrove Planks",
        "mangrove_planks",
        2.0,
        true,
        true,
    ),
    block(CHERRY_LOG, "Cherry Log", "cherry_log", 2.0, true, true),
    block(
        CHERRY_LEAVES,
        "Cherry Leaves",
        "cherry_leaves",
        0.2,
        true,
        false,
    ),
    block(
        CHERRY_PLANKS,
        "Cherry Planks",
        "cherry_planks",
        2.0,
        true,
        true,
    ),
    block(COPPER_ORE, "Copper Ore", "copper_ore", 3.0, true, true),
    block(EMERALD_ORE, "Emerald Ore", "emerald_ore", 3.0, true, true),
    block(LAPIS_ORE, "Lapis Ore", "lapis_ore", 3.0, true, true),
    block(
        COPPER_BLOCK,
        "Copper Block",
        "copper_block",
        3.0,
        true,
        true,
    ),
    block(
        EMERALD_BLOCK,
        "Emerald Block",
        "emerald_block",
        3.0,
        true,
        true,
    ),
    block(LAPIS_BLOCK, "Lapis Block", "lapis_block", 3.0, true, true),
    block(GLASS_WHITE, "Glass White", "glass_white", 0.3, true, false),
    block(
        GLASS_ORANGE,
        "Glass Orange",
        "glass_orange",
        0.3,
        true,
        false,
    ),
    block(
        GLASS_MAGENTA,
        "Glass Magenta",
        "glass_magenta",
        0.3,
        true,
        false,
    ),
    block(
        GLASS_LIGHT_BLUE,
        "Glass Light Blue",
        "glass_light_blue",
        0.3,
        true,
        false,
    ),
    block(
        GLASS_YELLOW,
        "Glass Yellow",
        "glass_yellow",
        0.3,
        true,
        false,
    ),
    block(GLASS_LIME, "Glass Lime", "glass_lime", 0.3, true, false),
    block(GLASS_PINK, "Glass Pink", "glass_pink", 0.3, true, false),
    block(GLASS_GRAY, "Glass Gray", "glass_gray", 0.3, true, false),
    block(
        GLASS_LIGHT_GRAY,
        "Glass Light Gray",
        "glass_light_gray",
        0.3,
        true,
        false,
    ),
    block(GLASS_CYAN, "Glass Cyan", "glass_cyan", 0.3, true, false),
    block(
        GLASS_PURPLE,
        "Glass Purple",
        "glass_purple",
        0.3,
        true,
        false,
    ),
    block(GLASS_BLUE, "Glass Blue", "glass_blue", 0.3, true, false),
    block(GLASS_BROWN, "Glass Brown", "glass_brown", 0.3, true, false),
    block(GLASS_GREEN, "Glass Green", "glass_green", 0.3, true, false),
    block(GLASS_RED, "Glass Red", "glass_red", 0.3, true, false),
    block(GLASS_BLACK, "Glass Black", "glass_black", 0.3, true, false),
    block(
        TERRACOTTA_WHITE,
        "Terracotta White",
        "terracotta_white",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_ORANGE,
        "Terracotta Orange",
        "terracotta_orange",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_MAGENTA,
        "Terracotta Magenta",
        "terracotta_magenta",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_LIGHT_BLUE,
        "Terracotta Light Blue",
        "terracotta_light_blue",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_YELLOW,
        "Terracotta Yellow",
        "terracotta_yellow",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_LIME,
        "Terracotta Lime",
        "terracotta_lime",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_PINK,
        "Terracotta Pink",
        "terracotta_pink",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_GRAY,
        "Terracotta Gray",
        "terracotta_gray",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_LIGHT_GRAY,
        "Terracotta Light Gray",
        "terracotta_light_gray",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_CYAN,
        "Terracotta Cyan",
        "terracotta_cyan",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_PURPLE,
        "Terracotta Purple",
        "terracotta_purple",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_BLUE,
        "Terracotta Blue",
        "terracotta_blue",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_BROWN,
        "Terracotta Brown",
        "terracotta_brown",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_GREEN,
        "Terracotta Green",
        "terracotta_green",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_RED,
        "Terracotta Red",
        "terracotta_red",
        1.25,
        true,
        true,
    ),
    block(
        TERRACOTTA_BLACK,
        "Terracotta Black",
        "terracotta_black",
        1.25,
        true,
        true,
    ),
    block(SPONGE, "Sponge", "sponge", 0.6, true, true),
    block(MELON, "Melon", "melon", 1.0, true, true),
    block(PUMPKIN, "Pumpkin", "pumpkin", 1.0, true, true),
    block(HAY_BALE, "Hay Bale", "hay_bale", 0.5, true, true),
    block(BONE_BLOCK, "Bone Block", "bone_block", 2.0, true, true),
    block(MUD, "Mud", "mud", 0.5, true, true),
    block(PACKED_MUD, "Packed Mud", "packed_mud", 1.0, true, true),
    block(MUD_BRICKS, "Mud Bricks", "mud_bricks", 1.5, true, true),
    block(MOSS_BLOCK, "Moss Block", "moss_block", 0.1, true, true),
    block(PODZOL, "Podzol", "podzol", 0.5, true, true),
    block(COARSE_DIRT, "Coarse Dirt", "coarse_dirt", 0.5, true, true),
    block(RED_SAND, "Red Sand", "red_sand", 0.5, true, true),
    block(
        RED_SANDSTONE,
        "Red Sandstone",
        "red_sandstone",
        0.8,
        true,
        true,
    ),
    block(CALCITE, "Calcite", "calcite", 0.75, true, true),
    block(TUFF, "Tuff", "tuff", 1.5, true, true),
    block(
        AMETHYST_BLOCK,
        "Amethyst Block",
        "amethyst_block",
        1.5,
        true,
        true,
    ),
    block(
        QUARTZ_BLOCK,
        "Quartz Block",
        "quartz_block",
        0.8,
        true,
        true,
    ),
    block(OAK_SLAB, "Oak Slab", "oak_slab", 1.5, true, false),
    block(BIRCH_SLAB, "Birch Slab", "birch_slab", 1.5, true, false),
    block(PINE_SLAB, "Pine Slab", "pine_slab", 1.5, true, false),
    block(STONE_SLAB, "Stone Slab", "stone_slab", 1.5, true, false),
    block(
        COBBLESTONE_SLAB,
        "Cobblestone Slab",
        "cobblestone_slab",
        1.5,
        true,
        false,
    ),
    block(
        STONE_BRICK_SLAB,
        "Stone Brick Slab",
        "stone_brick_slab",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_STONE_SLAB,
        "Smooth Stone Slab",
        "smooth_stone_slab",
        1.5,
        true,
        false,
    ),
    block(
        SANDSTONE_SLAB,
        "Sandstone Slab",
        "sandstone_slab",
        1.5,
        true,
        false,
    ),
    block(BRICK_SLAB, "Brick Slab", "brick_slab", 1.5, true, false),
    block(
        GRANITE_SLAB,
        "Granite Slab",
        "granite_slab",
        1.5,
        true,
        false,
    ),
    block(MARBLE_SLAB, "Marble Slab", "marble_slab", 1.5, true, false),
    block(
        DARK_BRICK_SLAB,
        "Dark Brick Slab",
        "dark_brick_slab",
        1.5,
        true,
        false,
    ),
    block(
        FLOW_WATER_1,
        "Flowing Water 1",
        "water",
        100.0,
        false,
        false,
    ),
    block(
        FLOW_WATER_2,
        "Flowing Water 2",
        "water",
        100.0,
        false,
        false,
    ),
    block(
        FLOW_WATER_3,
        "Flowing Water 3",
        "water",
        100.0,
        false,
        false,
    ),
    block(
        FLOW_WATER_4,
        "Flowing Water 4",
        "water",
        100.0,
        false,
        false,
    ),
    block(
        FLOW_WATER_5,
        "Flowing Water 5",
        "water",
        100.0,
        false,
        false,
    ),
    block(
        FLOW_WATER_6,
        "Flowing Water 6",
        "water",
        100.0,
        false,
        false,
    ),
    block(
        FLOW_WATER_7,
        "Flowing Water 7",
        "water",
        100.0,
        false,
        false,
    ),
    BlockDef {
        light_emission: 15,
        ..block(LAVA, "Lava", "lava", 100.0, false, false)
    },
    BlockDef {
        light_emission: 15,
        ..block(FLOW_LAVA_1, "Flowing Lava 1", "lava", 100.0, false, false)
    },
    BlockDef {
        light_emission: 15,
        ..block(FLOW_LAVA_2, "Flowing Lava 2", "lava", 100.0, false, false)
    },
    BlockDef {
        light_emission: 15,
        ..block(FLOW_LAVA_3, "Flowing Lava 3", "lava", 100.0, false, false)
    },
    block(DIORITE, "Diorite", "diorite", 1.5, true, true),
    block(
        POLISHED_DIORITE,
        "Polished Diorite",
        "polished_diorite",
        1.5,
        true,
        true,
    ),
    block(ANDESITE, "Andesite", "andesite", 1.5, true, true),
    block(
        POLISHED_ANDESITE,
        "Polished Andesite",
        "polished_andesite",
        1.5,
        true,
        true,
    ),
    block(
        SPRUCE_PLANKS,
        "Spruce Planks",
        "spruce_planks",
        2.0,
        true,
        true,
    ),
    block(
        BAMBOO_PLANKS,
        "Bamboo Planks",
        "bamboo_planks",
        2.0,
        true,
        true,
    ),
    block(
        BAMBOO_MOSAIC,
        "Bamboo Mosaic",
        "bamboo_mosaic",
        2.0,
        true,
        true,
    ),
    block(
        SUSPICIOUS_SAND,
        "Suspicious Sand",
        "suspicious_sand",
        0.25,
        true,
        true,
    ),
    block(
        SUSPICIOUS_GRAVEL,
        "Suspicious Gravel",
        "suspicious_gravel",
        0.25,
        true,
        true,
    ),
    block(
        DEEPSLATE_GOLD_ORE,
        "Deepslate Gold Ore",
        "deepslate_gold_ore",
        4.5,
        true,
        true,
    ),
    block(
        DEEPSLATE_IRON_ORE,
        "Deepslate Iron Ore",
        "deepslate_iron_ore",
        4.5,
        true,
        true,
    ),
    block(
        DEEPSLATE_COAL_ORE,
        "Deepslate Coal Ore",
        "deepslate_coal_ore",
        4.5,
        true,
        true,
    ),
    block(
        NETHER_GOLD_ORE,
        "Nether Gold Ore",
        "nether_gold_ore",
        4.5,
        true,
        true,
    ),
    block(SPRUCE_LOG, "Spruce Log", "spruce_log", 2.0, true, true),
    block(
        BAMBOO_BLOCK,
        "Bamboo Block",
        "bamboo_block",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_SPRUCE_LOG,
        "Stripped Spruce Log",
        "stripped_spruce_log",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_BIRCH_LOG,
        "Stripped Birch Log",
        "stripped_birch_log",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_JUNGLE_LOG,
        "Stripped Jungle Log",
        "stripped_jungle_log",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_ACACIA_LOG,
        "Stripped Acacia Log",
        "stripped_acacia_log",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_CHERRY_LOG,
        "Stripped Cherry Log",
        "stripped_cherry_log",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_DARK_OAK_LOG,
        "Stripped Dark Oak Log",
        "stripped_dark_oak_log",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_OAK_LOG,
        "Stripped Oak Log",
        "stripped_oak_log",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_MANGROVE_LOG,
        "Stripped Mangrove Log",
        "stripped_mangrove_log",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_BAMBOO_BLOCK,
        "Stripped Bamboo Block",
        "stripped_bamboo_block",
        2.0,
        true,
        true,
    ),
    block(OAK_WOOD, "Oak Wood", "oak_wood", 2.0, true, true),
    block(SPRUCE_WOOD, "Spruce Wood", "spruce_wood", 2.0, true, true),
    block(BIRCH_WOOD, "Birch Wood", "birch_wood", 2.0, true, true),
    block(JUNGLE_WOOD, "Jungle Wood", "jungle_wood", 2.0, true, true),
    block(ACACIA_WOOD, "Acacia Wood", "acacia_wood", 2.0, true, true),
    block(CHERRY_WOOD, "Cherry Wood", "cherry_wood", 2.0, true, true),
    block(
        DARK_OAK_WOOD,
        "Dark Oak Wood",
        "dark_oak_wood",
        2.0,
        true,
        true,
    ),
    block(
        MANGROVE_WOOD,
        "Mangrove Wood",
        "mangrove_wood",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_OAK_WOOD,
        "Stripped Oak Wood",
        "stripped_oak_wood",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_SPRUCE_WOOD,
        "Stripped Spruce Wood",
        "stripped_spruce_wood",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_BIRCH_WOOD,
        "Stripped Birch Wood",
        "stripped_birch_wood",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_JUNGLE_WOOD,
        "Stripped Jungle Wood",
        "stripped_jungle_wood",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_ACACIA_WOOD,
        "Stripped Acacia Wood",
        "stripped_acacia_wood",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_CHERRY_WOOD,
        "Stripped Cherry Wood",
        "stripped_cherry_wood",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_DARK_OAK_WOOD,
        "Stripped Dark Oak Wood",
        "stripped_dark_oak_wood",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_MANGROVE_WOOD,
        "Stripped Mangrove Wood",
        "stripped_mangrove_wood",
        2.0,
        true,
        true,
    ),
    block(WET_SPONGE, "Wet Sponge", "wet_sponge", 0.6, true, true),
    block(
        DEEPSLATE_LAPIS_ORE,
        "Deepslate Lapis Ore",
        "deepslate_lapis_ore",
        4.5,
        true,
        true,
    ),
    block(DISPENSER, "Dispenser", "dispenser", 3.5, true, true),
    block(
        CHISELED_SANDSTONE,
        "Chiseled Sandstone",
        "chiseled_sandstone",
        0.8,
        true,
        true,
    ),
    block(
        CUT_SANDSTONE,
        "Cut Sandstone",
        "cut_sandstone",
        0.8,
        true,
        true,
    ),
    block(NOTE_BLOCK, "Note Block", "note_block", 0.8, true, true),
    block(
        STICKY_PISTON,
        "Sticky Piston",
        "sticky_piston",
        1.5,
        true,
        true,
    ),
    block(PISTON, "Piston", "piston", 1.5, true, true),
    block(TNT, "Tnt", "tnt", 0.05, true, true),
    block(
        CHISELED_BOOKSHELF,
        "Chiseled Bookshelf",
        "chiseled_bookshelf",
        1.5,
        true,
        true,
    ),
    block(
        MANGROVE_ROOTS,
        "Mangrove Roots",
        "mangrove_roots",
        0.7,
        true,
        false,
    ),
    block(
        MUDDY_MANGROVE_ROOTS,
        "Muddy Mangrove Roots",
        "muddy_mangrove_roots",
        0.7,
        true,
        false,
    ),
    block(
        SPRUCE_LEAVES,
        "Spruce Leaves",
        "spruce_leaves",
        0.2,
        true,
        false,
    ),
    block(
        AZALEA_LEAVES,
        "Azalea Leaves",
        "azalea_leaves",
        0.2,
        true,
        false,
    ),
    block(
        FLOWERING_AZALEA_LEAVES,
        "Flowering Azalea Leaves",
        "flowering_azalea_leaves",
        0.2,
        true,
        false,
    ),
    block(
        OAK_SAPLING,
        "Oak Sapling",
        "oak_sapling",
        0.05,
        false,
        false,
    ),
    block(
        SPRUCE_SAPLING,
        "Spruce Sapling",
        "spruce_sapling",
        0.05,
        false,
        false,
    ),
    block(
        BIRCH_SAPLING,
        "Birch Sapling",
        "birch_sapling",
        0.05,
        false,
        false,
    ),
    block(
        JUNGLE_SAPLING,
        "Jungle Sapling",
        "jungle_sapling",
        0.05,
        false,
        false,
    ),
    block(
        ACACIA_SAPLING,
        "Acacia Sapling",
        "acacia_sapling",
        0.05,
        false,
        false,
    ),
    block(
        CHERRY_SAPLING,
        "Cherry Sapling",
        "cherry_sapling",
        0.05,
        false,
        false,
    ),
    block(
        DARK_OAK_SAPLING,
        "Dark Oak Sapling",
        "dark_oak_sapling",
        0.05,
        false,
        false,
    ),
    block(
        MANGROVE_PROPAGULE,
        "Mangrove Propagule",
        "mangrove_propagule",
        0.05,
        false,
        false,
    ),
    block(COBWEB, "Cobweb", "cobweb", 0.05, false, false),
    block(SHORT_GRASS, "Grass", "grass", 0.05, false, false),
    block(FERN, "Fern", "fern", 0.05, false, false),
    block(DEAD_BUSH, "Dead Bush", "dead_bush", 0.05, false, false),
    block(SEAGRASS, "Seagrass", "seagrass", 0.05, false, false),
    block(
        TALL_SEAGRASS,
        "Tall Seagrass",
        "tall_seagrass",
        0.05,
        false,
        false,
    ),
    block(
        TORCHFLOWER,
        "Torchflower",
        "torchflower",
        0.05,
        false,
        false,
    ),
    block(
        BLUE_ORCHID,
        "Blue Orchid",
        "blue_orchid",
        0.05,
        false,
        false,
    ),
    block(ALLIUM, "Allium", "allium", 0.05, false, false),
    block(
        AZURE_BLUET,
        "Azure Bluet",
        "azure_bluet",
        0.05,
        false,
        false,
    ),
    block(RED_TULIP, "Red Tulip", "red_tulip", 0.05, false, false),
    block(
        ORANGE_TULIP,
        "Orange Tulip",
        "orange_tulip",
        0.05,
        false,
        false,
    ),
    block(
        WHITE_TULIP,
        "White Tulip",
        "white_tulip",
        0.05,
        false,
        false,
    ),
    block(PINK_TULIP, "Pink Tulip", "pink_tulip", 0.05, false, false),
    block(
        WITHER_ROSE,
        "Wither Rose",
        "wither_rose",
        0.05,
        false,
        false,
    ),
    block(
        LILY_OF_THE_VALLEY,
        "Lily Of The Valley",
        "lily_of_the_valley",
        0.05,
        false,
        false,
    ),
    BlockDef {
        light_emission: 1,
        ..block(
            BROWN_MUSHROOM,
            "Brown Mushroom",
            "brown_mushroom",
            0.05,
            false,
            false,
        )
    },
    block(
        RED_MUSHROOM,
        "Red Mushroom",
        "red_mushroom",
        0.05,
        false,
        false,
    ),
    BlockDef {
        light_emission: 14,
        ..block(TORCH, "Torch", "torch", 0.05, false, false)
    },
    BlockDef {
        light_emission: 14,
        ..block(WALL_TORCH, "Wall Torch", "wall_torch", 0.05, false, false)
    },
    BlockDef {
        light_emission: 15,
        ..block(FIRE, "Fire", "fire", 0.05, false, false)
    },
    BlockDef {
        light_emission: 10,
        ..block(SOUL_FIRE, "Soul Fire", "soul_fire", 0.05, false, false)
    },
    block(
        POWERED_RAIL,
        "Powered Rail",
        "powered_rail",
        0.7,
        false,
        false,
    ),
    block(
        DETECTOR_RAIL,
        "Detector Rail",
        "detector_rail",
        0.7,
        false,
        false,
    ),
    block(WHITE_BED, "White Bed", "white_bed", 0.2, true, false),
    block(ORANGE_BED, "Orange Bed", "orange_bed", 0.2, true, false),
    block(MAGENTA_BED, "Magenta Bed", "magenta_bed", 0.2, true, false),
    block(
        LIGHT_BLUE_BED,
        "Light Blue Bed",
        "light_blue_bed",
        0.2,
        true,
        false,
    ),
    block(YELLOW_BED, "Yellow Bed", "yellow_bed", 0.2, true, false),
    block(LIME_BED, "Lime Bed", "lime_bed", 0.2, true, false),
    block(PINK_BED, "Pink Bed", "pink_bed", 0.2, true, false),
    block(GRAY_BED, "Gray Bed", "gray_bed", 0.2, true, false),
    block(
        LIGHT_GRAY_BED,
        "Light Gray Bed",
        "light_gray_bed",
        0.2,
        true,
        false,
    ),
    block(CYAN_BED, "Cyan Bed", "cyan_bed", 0.2, true, false),
    block(PURPLE_BED, "Purple Bed", "purple_bed", 0.2, true, false),
    block(BLUE_BED, "Blue Bed", "blue_bed", 0.2, true, false),
    block(BROWN_BED, "Brown Bed", "brown_bed", 0.2, true, false),
    block(GREEN_BED, "Green Bed", "green_bed", 0.2, true, false),
    block(RED_BED, "Red Bed", "red_bed", 0.2, true, false),
    block(BLACK_BED, "Black Bed", "black_bed", 0.2, true, false),
    block(PISTON_HEAD, "Piston Head", "piston_head", 1.5, true, false),
    block(
        MOVING_PISTON,
        "Moving Piston",
        "moving_piston",
        1.5,
        false,
        false,
    ),
    block(
        DISPENSER_SOUTH,
        "Dispenser South",
        "dispenser_south",
        3.5,
        true,
        true,
    ),
    block(
        DISPENSER_WEST,
        "Dispenser West",
        "dispenser_west",
        3.5,
        true,
        true,
    ),
    block(
        DISPENSER_EAST,
        "Dispenser East",
        "dispenser_east",
        3.5,
        true,
        true,
    ),
    block(
        DISPENSER_UP,
        "Dispenser Up",
        "dispenser_up",
        3.5,
        true,
        true,
    ),
    block(
        DISPENSER_DOWN,
        "Dispenser Down",
        "dispenser_down",
        3.5,
        true,
        true,
    ),
    block(
        PISTON_SOUTH,
        "Piston South",
        "piston_south",
        1.5,
        true,
        true,
    ),
    block(PISTON_WEST, "Piston West", "piston_west", 1.5, true, true),
    block(PISTON_EAST, "Piston East", "piston_east", 1.5, true, true),
    block(PISTON_UP, "Piston Up", "piston_up", 1.5, true, true),
    block(PISTON_DOWN, "Piston Down", "piston_down", 1.5, true, true),
    block(
        STICKY_PISTON_SOUTH,
        "Sticky Piston South",
        "sticky_piston_south",
        1.5,
        true,
        true,
    ),
    block(
        STICKY_PISTON_WEST,
        "Sticky Piston West",
        "sticky_piston_west",
        1.5,
        true,
        true,
    ),
    block(
        STICKY_PISTON_EAST,
        "Sticky Piston East",
        "sticky_piston_east",
        1.5,
        true,
        true,
    ),
    block(
        STICKY_PISTON_UP,
        "Sticky Piston Up",
        "sticky_piston_up",
        1.5,
        true,
        true,
    ),
    block(
        STICKY_PISTON_DOWN,
        "Sticky Piston Down",
        "sticky_piston_down",
        1.5,
        true,
        true,
    ),
    BlockDef {
        light_emission: 14,
        ..block(
            WALL_TORCH_SOUTH,
            "Wall Torch South",
            "wall_torch_south",
            0.05,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 14,
        ..block(
            WALL_TORCH_WEST,
            "Wall Torch West",
            "wall_torch_west",
            0.05,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 14,
        ..block(
            WALL_TORCH_EAST,
            "Wall Torch East",
            "wall_torch_east",
            0.05,
            false,
            false,
        )
    },
    block(
        POWERED_RAIL_X,
        "Powered Rail X",
        "powered_rail_x",
        0.7,
        false,
        false,
    ),
    block(
        DETECTOR_RAIL_X,
        "Detector Rail X",
        "detector_rail_x",
        0.7,
        false,
        false,
    ),
    block(SPAWNER, "Spawner", "spawner", 5.0, true, false),
    block(OAK_STAIRS, "Oak Stairs", "oak_stairs", 2.0, true, false),
    block(
        OAK_STAIRS + 1,
        "Oak Stairs (south)",
        "oak_stairs",
        2.0,
        true,
        false,
    ),
    block(
        OAK_STAIRS + 2,
        "Oak Stairs (west)",
        "oak_stairs",
        2.0,
        true,
        false,
    ),
    block(
        OAK_STAIRS + 3,
        "Oak Stairs (east)",
        "oak_stairs",
        2.0,
        true,
        false,
    ),
    block(
        REDSTONE_WIRE,
        "Redstone Wire",
        "redstone_wire",
        0.05,
        false,
        false,
    ),
    block(
        DEEPSLATE_DIAMOND_ORE,
        "Deepslate Diamond Ore",
        "deepslate_diamond_ore",
        4.5,
        true,
        true,
    ),
    block(WHEAT, "Wheat", "wheat", 0.05, false, false),
    block(FARMLAND, "Farmland", "farmland", 0.6, true, false),
    block(OAK_SIGN, "Oak Sign", "oak_sign", 1.0, false, false),
    block(
        OAK_SIGN + 1,
        "Oak Sign (south)",
        "oak_sign",
        1.0,
        false,
        false,
    ),
    block(
        OAK_SIGN + 2,
        "Oak Sign (west)",
        "oak_sign",
        1.0,
        false,
        false,
    ),
    block(
        OAK_SIGN + 3,
        "Oak Sign (east)",
        "oak_sign",
        1.0,
        false,
        false,
    ),
    block(SPRUCE_SIGN, "Spruce Sign", "spruce_sign", 1.0, false, false),
    block(
        SPRUCE_SIGN + 1,
        "Spruce Sign (south)",
        "spruce_sign",
        1.0,
        false,
        false,
    ),
    block(
        SPRUCE_SIGN + 2,
        "Spruce Sign (west)",
        "spruce_sign",
        1.0,
        false,
        false,
    ),
    block(
        SPRUCE_SIGN + 3,
        "Spruce Sign (east)",
        "spruce_sign",
        1.0,
        false,
        false,
    ),
    block(BIRCH_SIGN, "Birch Sign", "birch_sign", 1.0, false, false),
    block(
        BIRCH_SIGN + 1,
        "Birch Sign (south)",
        "birch_sign",
        1.0,
        false,
        false,
    ),
    block(
        BIRCH_SIGN + 2,
        "Birch Sign (west)",
        "birch_sign",
        1.0,
        false,
        false,
    ),
    block(
        BIRCH_SIGN + 3,
        "Birch Sign (east)",
        "birch_sign",
        1.0,
        false,
        false,
    ),
    block(JUNGLE_SIGN, "Jungle Sign", "jungle_sign", 1.0, false, false),
    block(
        JUNGLE_SIGN + 1,
        "Jungle Sign (south)",
        "jungle_sign",
        1.0,
        false,
        false,
    ),
    block(
        JUNGLE_SIGN + 2,
        "Jungle Sign (west)",
        "jungle_sign",
        1.0,
        false,
        false,
    ),
    block(
        JUNGLE_SIGN + 3,
        "Jungle Sign (east)",
        "jungle_sign",
        1.0,
        false,
        false,
    ),
    block(ACACIA_SIGN, "Acacia Sign", "acacia_sign", 1.0, false, false),
    block(
        ACACIA_SIGN + 1,
        "Acacia Sign (south)",
        "acacia_sign",
        1.0,
        false,
        false,
    ),
    block(
        ACACIA_SIGN + 2,
        "Acacia Sign (west)",
        "acacia_sign",
        1.0,
        false,
        false,
    ),
    block(
        ACACIA_SIGN + 3,
        "Acacia Sign (east)",
        "acacia_sign",
        1.0,
        false,
        false,
    ),
    block(CHERRY_SIGN, "Cherry Sign", "cherry_sign", 1.0, false, false),
    block(
        CHERRY_SIGN + 1,
        "Cherry Sign (south)",
        "cherry_sign",
        1.0,
        false,
        false,
    ),
    block(
        CHERRY_SIGN + 2,
        "Cherry Sign (west)",
        "cherry_sign",
        1.0,
        false,
        false,
    ),
    block(
        CHERRY_SIGN + 3,
        "Cherry Sign (east)",
        "cherry_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_SIGN,
        "Dark Oak Sign",
        "dark_oak_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_SIGN + 1,
        "Dark Oak Sign (south)",
        "dark_oak_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_SIGN + 2,
        "Dark Oak Sign (west)",
        "dark_oak_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_SIGN + 3,
        "Dark Oak Sign (east)",
        "dark_oak_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_SIGN,
        "Mangrove Sign",
        "mangrove_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_SIGN + 1,
        "Mangrove Sign (south)",
        "mangrove_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_SIGN + 2,
        "Mangrove Sign (west)",
        "mangrove_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_SIGN + 3,
        "Mangrove Sign (east)",
        "mangrove_sign",
        1.0,
        false,
        false,
    ),
    block(BAMBOO_SIGN, "Bamboo Sign", "bamboo_sign", 1.0, false, false),
    block(
        BAMBOO_SIGN + 1,
        "Bamboo Sign (south)",
        "bamboo_sign",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_SIGN + 2,
        "Bamboo Sign (west)",
        "bamboo_sign",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_SIGN + 3,
        "Bamboo Sign (east)",
        "bamboo_sign",
        1.0,
        false,
        false,
    ),
    block(OAK_DOOR, "Oak Door", "oak_door", 3.0, true, false),
    block(
        OAK_DOOR + 1,
        "Oak Door (lower closed south)",
        "oak_door",
        3.0,
        true,
        false,
    ),
    block(
        OAK_DOOR + 2,
        "Oak Door (lower closed west)",
        "oak_door",
        3.0,
        true,
        false,
    ),
    block(
        OAK_DOOR + 3,
        "Oak Door (lower closed east)",
        "oak_door",
        3.0,
        true,
        false,
    ),
    block(
        OAK_DOOR + 4,
        "Oak Door (lower open north)",
        "oak_door",
        3.0,
        false,
        false,
    ),
    block(
        OAK_DOOR + 5,
        "Oak Door (lower open south)",
        "oak_door",
        3.0,
        false,
        false,
    ),
    block(
        OAK_DOOR + 6,
        "Oak Door (lower open west)",
        "oak_door",
        3.0,
        false,
        false,
    ),
    block(
        OAK_DOOR + 7,
        "Oak Door (lower open east)",
        "oak_door",
        3.0,
        false,
        false,
    ),
    block(
        OAK_DOOR + 8,
        "Oak Door (upper closed north)",
        "oak_door",
        3.0,
        true,
        false,
    ),
    block(
        OAK_DOOR + 9,
        "Oak Door (upper closed south)",
        "oak_door",
        3.0,
        true,
        false,
    ),
    block(
        OAK_DOOR + 10,
        "Oak Door (upper closed west)",
        "oak_door",
        3.0,
        true,
        false,
    ),
    block(
        OAK_DOOR + 11,
        "Oak Door (upper closed east)",
        "oak_door",
        3.0,
        true,
        false,
    ),
    block(
        OAK_DOOR + 12,
        "Oak Door (upper open north)",
        "oak_door",
        3.0,
        false,
        false,
    ),
    block(
        OAK_DOOR + 13,
        "Oak Door (upper open south)",
        "oak_door",
        3.0,
        false,
        false,
    ),
    block(
        OAK_DOOR + 14,
        "Oak Door (upper open west)",
        "oak_door",
        3.0,
        false,
        false,
    ),
    block(
        OAK_DOOR + 15,
        "Oak Door (upper open east)",
        "oak_door",
        3.0,
        false,
        false,
    ),
    block(LADDER, "Ladder", "ladder", 0.4, false, false),
    block(LADDER + 1, "Ladder (south)", "ladder", 0.4, false, false),
    block(LADDER + 2, "Ladder (west)", "ladder", 0.4, false, false),
    block(LADDER + 3, "Ladder (east)", "ladder", 0.4, false, false),
    block(RAIL, "Rail", "rail", 0.7, false, false),
    block(RAIL + 1, "Rail (x)", "rail", 0.7, false, false),
    block(
        COBBLESTONE_STAIRS,
        "Cobblestone Stairs",
        "cobblestone_stairs",
        2.0,
        true,
        false,
    ),
    block(
        COBBLESTONE_STAIRS + 1,
        "Cobblestone Stairs (south)",
        "cobblestone_stairs",
        2.0,
        true,
        false,
    ),
    block(
        COBBLESTONE_STAIRS + 2,
        "Cobblestone Stairs (west)",
        "cobblestone_stairs",
        2.0,
        true,
        false,
    ),
    block(
        COBBLESTONE_STAIRS + 3,
        "Cobblestone Stairs (east)",
        "cobblestone_stairs",
        2.0,
        true,
        false,
    ),
    block(
        OAK_WALL_SIGN,
        "Oak Wall Sign",
        "oak_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        OAK_WALL_SIGN + 1,
        "Oak Wall Sign (south)",
        "oak_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        OAK_WALL_SIGN + 2,
        "Oak Wall Sign (west)",
        "oak_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        OAK_WALL_SIGN + 3,
        "Oak Wall Sign (east)",
        "oak_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        SPRUCE_WALL_SIGN,
        "Spruce Wall Sign",
        "spruce_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        SPRUCE_WALL_SIGN + 1,
        "Spruce Wall Sign (south)",
        "spruce_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        SPRUCE_WALL_SIGN + 2,
        "Spruce Wall Sign (west)",
        "spruce_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        SPRUCE_WALL_SIGN + 3,
        "Spruce Wall Sign (east)",
        "spruce_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        BIRCH_WALL_SIGN,
        "Birch Wall Sign",
        "birch_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        BIRCH_WALL_SIGN + 1,
        "Birch Wall Sign (south)",
        "birch_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        BIRCH_WALL_SIGN + 2,
        "Birch Wall Sign (west)",
        "birch_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        BIRCH_WALL_SIGN + 3,
        "Birch Wall Sign (east)",
        "birch_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        JUNGLE_WALL_SIGN,
        "Jungle Wall Sign",
        "jungle_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        JUNGLE_WALL_SIGN + 1,
        "Jungle Wall Sign (south)",
        "jungle_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        JUNGLE_WALL_SIGN + 2,
        "Jungle Wall Sign (west)",
        "jungle_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        JUNGLE_WALL_SIGN + 3,
        "Jungle Wall Sign (east)",
        "jungle_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        ACACIA_WALL_SIGN,
        "Acacia Wall Sign",
        "acacia_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        ACACIA_WALL_SIGN + 1,
        "Acacia Wall Sign (south)",
        "acacia_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        ACACIA_WALL_SIGN + 2,
        "Acacia Wall Sign (west)",
        "acacia_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        ACACIA_WALL_SIGN + 3,
        "Acacia Wall Sign (east)",
        "acacia_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        CHERRY_WALL_SIGN,
        "Cherry Wall Sign",
        "cherry_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        CHERRY_WALL_SIGN + 1,
        "Cherry Wall Sign (south)",
        "cherry_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        CHERRY_WALL_SIGN + 2,
        "Cherry Wall Sign (west)",
        "cherry_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        CHERRY_WALL_SIGN + 3,
        "Cherry Wall Sign (east)",
        "cherry_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_WALL_SIGN,
        "Dark Oak Wall Sign",
        "dark_oak_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_WALL_SIGN + 1,
        "Dark Oak Wall Sign (south)",
        "dark_oak_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_WALL_SIGN + 2,
        "Dark Oak Wall Sign (west)",
        "dark_oak_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_WALL_SIGN + 3,
        "Dark Oak Wall Sign (east)",
        "dark_oak_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_WALL_SIGN,
        "Mangrove Wall Sign",
        "mangrove_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_WALL_SIGN + 1,
        "Mangrove Wall Sign (south)",
        "mangrove_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_WALL_SIGN + 2,
        "Mangrove Wall Sign (west)",
        "mangrove_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_WALL_SIGN + 3,
        "Mangrove Wall Sign (east)",
        "mangrove_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_WALL_SIGN,
        "Bamboo Wall Sign",
        "bamboo_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_WALL_SIGN + 1,
        "Bamboo Wall Sign (south)",
        "bamboo_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_WALL_SIGN + 2,
        "Bamboo Wall Sign (west)",
        "bamboo_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_WALL_SIGN + 3,
        "Bamboo Wall Sign (east)",
        "bamboo_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        OAK_HANGING_SIGN,
        "Oak Hanging Sign",
        "oak_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        OAK_HANGING_SIGN + 1,
        "Oak Hanging Sign (z)",
        "oak_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        SPRUCE_HANGING_SIGN,
        "Spruce Hanging Sign",
        "spruce_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        SPRUCE_HANGING_SIGN + 1,
        "Spruce Hanging Sign (z)",
        "spruce_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        BIRCH_HANGING_SIGN,
        "Birch Hanging Sign",
        "birch_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        BIRCH_HANGING_SIGN + 1,
        "Birch Hanging Sign (z)",
        "birch_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        ACACIA_HANGING_SIGN,
        "Acacia Hanging Sign",
        "acacia_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        ACACIA_HANGING_SIGN + 1,
        "Acacia Hanging Sign (z)",
        "acacia_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        CHERRY_HANGING_SIGN,
        "Cherry Hanging Sign",
        "cherry_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        CHERRY_HANGING_SIGN + 1,
        "Cherry Hanging Sign (z)",
        "cherry_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        JUNGLE_HANGING_SIGN,
        "Jungle Hanging Sign",
        "jungle_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        JUNGLE_HANGING_SIGN + 1,
        "Jungle Hanging Sign (z)",
        "jungle_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_HANGING_SIGN,
        "Dark Oak Hanging Sign",
        "dark_oak_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_HANGING_SIGN + 1,
        "Dark Oak Hanging Sign (z)",
        "dark_oak_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_HANGING_SIGN,
        "Crimson Hanging Sign",
        "crimson_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_HANGING_SIGN + 1,
        "Crimson Hanging Sign (z)",
        "crimson_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        WARPED_HANGING_SIGN,
        "Warped Hanging Sign",
        "warped_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        WARPED_HANGING_SIGN + 1,
        "Warped Hanging Sign (z)",
        "warped_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_HANGING_SIGN,
        "Mangrove Hanging Sign",
        "mangrove_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_HANGING_SIGN + 1,
        "Mangrove Hanging Sign (z)",
        "mangrove_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_HANGING_SIGN,
        "Bamboo Hanging Sign",
        "bamboo_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_HANGING_SIGN + 1,
        "Bamboo Hanging Sign (z)",
        "bamboo_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        OAK_WALL_HANGING_SIGN,
        "Oak Wall Hanging Sign",
        "oak_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        OAK_WALL_HANGING_SIGN + 1,
        "Oak Wall Hanging Sign (z)",
        "oak_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        SPRUCE_WALL_HANGING_SIGN,
        "Spruce Wall Hanging Sign",
        "spruce_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        SPRUCE_WALL_HANGING_SIGN + 1,
        "Spruce Wall Hanging Sign (z)",
        "spruce_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        BIRCH_WALL_HANGING_SIGN,
        "Birch Wall Hanging Sign",
        "birch_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        BIRCH_WALL_HANGING_SIGN + 1,
        "Birch Wall Hanging Sign (z)",
        "birch_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        ACACIA_WALL_HANGING_SIGN,
        "Acacia Wall Hanging Sign",
        "acacia_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        ACACIA_WALL_HANGING_SIGN + 1,
        "Acacia Wall Hanging Sign (z)",
        "acacia_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        CHERRY_WALL_HANGING_SIGN,
        "Cherry Wall Hanging Sign",
        "cherry_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        CHERRY_WALL_HANGING_SIGN + 1,
        "Cherry Wall Hanging Sign (z)",
        "cherry_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        JUNGLE_WALL_HANGING_SIGN,
        "Jungle Wall Hanging Sign",
        "jungle_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        JUNGLE_WALL_HANGING_SIGN + 1,
        "Jungle Wall Hanging Sign (z)",
        "jungle_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_WALL_HANGING_SIGN,
        "Dark Oak Wall Hanging Sign",
        "dark_oak_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        DARK_OAK_WALL_HANGING_SIGN + 1,
        "Dark Oak Wall Hanging Sign (z)",
        "dark_oak_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_WALL_HANGING_SIGN,
        "Crimson Wall Hanging Sign",
        "crimson_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_WALL_HANGING_SIGN + 1,
        "Crimson Wall Hanging Sign (z)",
        "crimson_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        WARPED_WALL_HANGING_SIGN,
        "Warped Wall Hanging Sign",
        "warped_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        WARPED_WALL_HANGING_SIGN + 1,
        "Warped Wall Hanging Sign (z)",
        "warped_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_WALL_HANGING_SIGN,
        "Mangrove Wall Hanging Sign",
        "mangrove_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        MANGROVE_WALL_HANGING_SIGN + 1,
        "Mangrove Wall Hanging Sign (z)",
        "mangrove_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_WALL_HANGING_SIGN,
        "Bamboo Wall Hanging Sign",
        "bamboo_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_WALL_HANGING_SIGN + 1,
        "Bamboo Wall Hanging Sign (z)",
        "bamboo_wall_hanging_sign",
        1.0,
        false,
        false,
    ),
    block(LEVER, "Lever", "lever", 0.5, false, false),
    block(LEVER + 1, "Lever (off south)", "lever", 0.5, false, false),
    block(LEVER + 2, "Lever (off west)", "lever", 0.5, false, false),
    block(LEVER + 3, "Lever (off east)", "lever", 0.5, false, false),
    block(LEVER + 4, "Lever (on north)", "lever", 0.5, false, false),
    block(LEVER + 5, "Lever (on south)", "lever", 0.5, false, false),
    block(LEVER + 6, "Lever (on west)", "lever", 0.5, false, false),
    block(LEVER + 7, "Lever (on east)", "lever", 0.5, false, false),
    block(
        STONE_PRESSURE_PLATE,
        "Stone Pressure Plate",
        "stone_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(IRON_DOOR, "Iron Door", "iron_door", 5.0, true, false),
    block(
        IRON_DOOR + 1,
        "Iron Door (lower closed south)",
        "iron_door",
        5.0,
        true,
        false,
    ),
    block(
        IRON_DOOR + 2,
        "Iron Door (lower closed west)",
        "iron_door",
        5.0,
        true,
        false,
    ),
    block(
        IRON_DOOR + 3,
        "Iron Door (lower closed east)",
        "iron_door",
        5.0,
        true,
        false,
    ),
    block(
        IRON_DOOR + 4,
        "Iron Door (lower open north)",
        "iron_door",
        5.0,
        false,
        false,
    ),
    block(
        IRON_DOOR + 5,
        "Iron Door (lower open south)",
        "iron_door",
        5.0,
        false,
        false,
    ),
    block(
        IRON_DOOR + 6,
        "Iron Door (lower open west)",
        "iron_door",
        5.0,
        false,
        false,
    ),
    block(
        IRON_DOOR + 7,
        "Iron Door (lower open east)",
        "iron_door",
        5.0,
        false,
        false,
    ),
    block(
        IRON_DOOR + 8,
        "Iron Door (upper closed north)",
        "iron_door",
        5.0,
        true,
        false,
    ),
    block(
        IRON_DOOR + 9,
        "Iron Door (upper closed south)",
        "iron_door",
        5.0,
        true,
        false,
    ),
    block(
        IRON_DOOR + 10,
        "Iron Door (upper closed west)",
        "iron_door",
        5.0,
        true,
        false,
    ),
    block(
        IRON_DOOR + 11,
        "Iron Door (upper closed east)",
        "iron_door",
        5.0,
        true,
        false,
    ),
    block(
        IRON_DOOR + 12,
        "Iron Door (upper open north)",
        "iron_door",
        5.0,
        false,
        false,
    ),
    block(
        IRON_DOOR + 13,
        "Iron Door (upper open south)",
        "iron_door",
        5.0,
        false,
        false,
    ),
    block(
        IRON_DOOR + 14,
        "Iron Door (upper open west)",
        "iron_door",
        5.0,
        false,
        false,
    ),
    block(
        IRON_DOOR + 15,
        "Iron Door (upper open east)",
        "iron_door",
        5.0,
        false,
        false,
    ),
    block(
        OAK_PRESSURE_PLATE,
        "Oak Pressure Plate",
        "oak_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        SPRUCE_PRESSURE_PLATE,
        "Spruce Pressure Plate",
        "spruce_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        BIRCH_PRESSURE_PLATE,
        "Birch Pressure Plate",
        "birch_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        JUNGLE_PRESSURE_PLATE,
        "Jungle Pressure Plate",
        "jungle_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        ACACIA_PRESSURE_PLATE,
        "Acacia Pressure Plate",
        "acacia_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        CHERRY_PRESSURE_PLATE,
        "Cherry Pressure Plate",
        "cherry_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        DARK_OAK_PRESSURE_PLATE,
        "Dark Oak Pressure Plate",
        "dark_oak_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        MANGROVE_PRESSURE_PLATE,
        "Mangrove Pressure Plate",
        "mangrove_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        BAMBOO_PRESSURE_PLATE,
        "Bamboo Pressure Plate",
        "bamboo_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        REDSTONE_ORE,
        "Redstone Ore",
        "redstone_ore",
        3.0,
        true,
        true,
    ),
    block(
        DEEPSLATE_REDSTONE_ORE,
        "Deepslate Redstone Ore",
        "deepslate_redstone_ore",
        4.5,
        true,
        true,
    ),
    BlockDef {
        light_emission: 7,
        ..block(
            REDSTONE_TORCH,
            "Redstone Torch",
            "redstone_torch",
            0.05,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 7,
        ..block(
            REDSTONE_WALL_TORCH,
            "Redstone Wall Torch",
            "redstone_wall_torch",
            0.05,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 7,
        ..block(
            REDSTONE_WALL_TORCH + 1,
            "Redstone Wall Torch (south)",
            "redstone_wall_torch",
            0.05,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 7,
        ..block(
            REDSTONE_WALL_TORCH + 2,
            "Redstone Wall Torch (west)",
            "redstone_wall_torch",
            0.05,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 7,
        ..block(
            REDSTONE_WALL_TORCH + 3,
            "Redstone Wall Torch (east)",
            "redstone_wall_torch",
            0.05,
            false,
            false,
        )
    },
    block(
        STONE_BUTTON,
        "Stone Button",
        "stone_button",
        0.5,
        false,
        false,
    ),
    block(
        STONE_BUTTON + 1,
        "Stone Button (south)",
        "stone_button",
        0.5,
        false,
        false,
    ),
    block(
        STONE_BUTTON + 2,
        "Stone Button (west)",
        "stone_button",
        0.5,
        false,
        false,
    ),
    block(
        STONE_BUTTON + 3,
        "Stone Button (east)",
        "stone_button",
        0.5,
        false,
        false,
    ),
    block(SNOW_LAYER, "Snow Layer", "snow_layer", 0.1, false, false),
    block(CACTUS, "Cactus", "cactus", 0.4, true, false),
    block(SUGAR_CANE, "Sugar Cane", "sugar_cane", 0.05, false, false),
    block(JUKEBOX, "Jukebox", "jukebox", 2.0, true, true),
    block(OAK_FENCE, "Oak Fence", "oak_fence", 2.0, true, false),
    block(NETHERRACK, "Netherrack", "netherrack", 0.4, true, true),
    block(SOUL_SAND, "Soul Sand", "soul_sand", 0.5, true, true),
    block(SOUL_SOIL, "Soul Soil", "soul_soil", 0.5, true, true),
    BlockDef {
        light_emission: 10,
        ..block(SOUL_TORCH, "Soul Torch", "soul_torch", 0.05, false, false)
    },
    BlockDef {
        light_emission: 10,
        ..block(
            SOUL_WALL_TORCH,
            "Soul Wall Torch",
            "soul_wall_torch",
            0.05,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 10,
        ..block(
            SOUL_WALL_TORCH + 1,
            "Soul Wall Torch (south)",
            "soul_wall_torch",
            0.05,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 10,
        ..block(
            SOUL_WALL_TORCH + 2,
            "Soul Wall Torch (west)",
            "soul_wall_torch",
            0.05,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 10,
        ..block(
            SOUL_WALL_TORCH + 3,
            "Soul Wall Torch (east)",
            "soul_wall_torch",
            0.05,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 15,
        ..block(GLOWSTONE, "Glowstone", "glowstone", 0.3, true, true)
    },
    BlockDef {
        light_emission: 11,
        ..block(
            NETHER_PORTAL,
            "Nether Portal",
            "nether_portal",
            0.5,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 11,
        ..block(
            NETHER_PORTAL + 1,
            "Nether Portal (z)",
            "nether_portal",
            0.5,
            false,
            false,
        )
    },
    block(
        CARVED_PUMPKIN,
        "Carved Pumpkin",
        "carved_pumpkin",
        1.0,
        true,
        true,
    ),
    block(
        CARVED_PUMPKIN + 1,
        "Carved Pumpkin (south)",
        "carved_pumpkin",
        1.0,
        true,
        true,
    ),
    block(
        CARVED_PUMPKIN + 2,
        "Carved Pumpkin (west)",
        "carved_pumpkin",
        1.0,
        true,
        true,
    ),
    block(
        CARVED_PUMPKIN + 3,
        "Carved Pumpkin (east)",
        "carved_pumpkin",
        1.0,
        true,
        true,
    ),
    BlockDef {
        light_emission: 15,
        ..block(
            JACK_O_LANTERN,
            "Jack o'Lantern",
            "jack_o_lantern",
            1.0,
            true,
            true,
        )
    },
    BlockDef {
        light_emission: 15,
        ..block(
            JACK_O_LANTERN + 1,
            "Jack o'Lantern (south)",
            "jack_o_lantern",
            1.0,
            true,
            true,
        )
    },
    BlockDef {
        light_emission: 15,
        ..block(
            JACK_O_LANTERN + 2,
            "Jack o'Lantern (west)",
            "jack_o_lantern",
            1.0,
            true,
            true,
        )
    },
    BlockDef {
        light_emission: 15,
        ..block(
            JACK_O_LANTERN + 3,
            "Jack o'Lantern (east)",
            "jack_o_lantern",
            1.0,
            true,
            true,
        )
    },
    block(CAKE, "Cake", "cake", 0.5, true, false),
    block(REPEATER, "Repeater", "repeater", 0.05, false, false),
    block(
        REPEATER + 1,
        "Repeater (south)",
        "repeater",
        0.05,
        false,
        false,
    ),
    block(
        REPEATER + 2,
        "Repeater (west)",
        "repeater",
        0.05,
        false,
        false,
    ),
    block(
        REPEATER + 3,
        "Repeater (east)",
        "repeater",
        0.05,
        false,
        false,
    ),
    block(
        OAK_TRAPDOOR,
        "Oak Trapdoor",
        "oak_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        OAK_TRAPDOOR + 1,
        "Oak Trapdoor (open north)",
        "oak_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        OAK_TRAPDOOR + 2,
        "Oak Trapdoor (open south)",
        "oak_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        OAK_TRAPDOOR + 3,
        "Oak Trapdoor (open west)",
        "oak_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        OAK_TRAPDOOR + 4,
        "Oak Trapdoor (open east)",
        "oak_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_TRAPDOOR,
        "Spruce Trapdoor",
        "spruce_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        SPRUCE_TRAPDOOR + 1,
        "Spruce Trapdoor (open north)",
        "spruce_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_TRAPDOOR + 2,
        "Spruce Trapdoor (open south)",
        "spruce_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_TRAPDOOR + 3,
        "Spruce Trapdoor (open west)",
        "spruce_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_TRAPDOOR + 4,
        "Spruce Trapdoor (open east)",
        "spruce_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_TRAPDOOR,
        "Birch Trapdoor",
        "birch_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        BIRCH_TRAPDOOR + 1,
        "Birch Trapdoor (open north)",
        "birch_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_TRAPDOOR + 2,
        "Birch Trapdoor (open south)",
        "birch_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_TRAPDOOR + 3,
        "Birch Trapdoor (open west)",
        "birch_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_TRAPDOOR + 4,
        "Birch Trapdoor (open east)",
        "birch_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_TRAPDOOR,
        "Jungle Trapdoor",
        "jungle_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        JUNGLE_TRAPDOOR + 1,
        "Jungle Trapdoor (open north)",
        "jungle_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_TRAPDOOR + 2,
        "Jungle Trapdoor (open south)",
        "jungle_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_TRAPDOOR + 3,
        "Jungle Trapdoor (open west)",
        "jungle_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_TRAPDOOR + 4,
        "Jungle Trapdoor (open east)",
        "jungle_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_TRAPDOOR,
        "Acacia Trapdoor",
        "acacia_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        ACACIA_TRAPDOOR + 1,
        "Acacia Trapdoor (open north)",
        "acacia_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_TRAPDOOR + 2,
        "Acacia Trapdoor (open south)",
        "acacia_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_TRAPDOOR + 3,
        "Acacia Trapdoor (open west)",
        "acacia_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_TRAPDOOR + 4,
        "Acacia Trapdoor (open east)",
        "acacia_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_TRAPDOOR,
        "Cherry Trapdoor",
        "cherry_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        CHERRY_TRAPDOOR + 1,
        "Cherry Trapdoor (open north)",
        "cherry_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_TRAPDOOR + 2,
        "Cherry Trapdoor (open south)",
        "cherry_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_TRAPDOOR + 3,
        "Cherry Trapdoor (open west)",
        "cherry_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_TRAPDOOR + 4,
        "Cherry Trapdoor (open east)",
        "cherry_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_TRAPDOOR,
        "Dark Oak Trapdoor",
        "dark_oak_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        DARK_OAK_TRAPDOOR + 1,
        "Dark Oak Trapdoor (open north)",
        "dark_oak_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_TRAPDOOR + 2,
        "Dark Oak Trapdoor (open south)",
        "dark_oak_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_TRAPDOOR + 3,
        "Dark Oak Trapdoor (open west)",
        "dark_oak_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_TRAPDOOR + 4,
        "Dark Oak Trapdoor (open east)",
        "dark_oak_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_TRAPDOOR,
        "Mangrove Trapdoor",
        "mangrove_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        MANGROVE_TRAPDOOR + 1,
        "Mangrove Trapdoor (open north)",
        "mangrove_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_TRAPDOOR + 2,
        "Mangrove Trapdoor (open south)",
        "mangrove_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_TRAPDOOR + 3,
        "Mangrove Trapdoor (open west)",
        "mangrove_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_TRAPDOOR + 4,
        "Mangrove Trapdoor (open east)",
        "mangrove_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_TRAPDOOR,
        "Bamboo Trapdoor",
        "bamboo_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        BAMBOO_TRAPDOOR + 1,
        "Bamboo Trapdoor (open north)",
        "bamboo_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_TRAPDOOR + 2,
        "Bamboo Trapdoor (open south)",
        "bamboo_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_TRAPDOOR + 3,
        "Bamboo Trapdoor (open west)",
        "bamboo_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_TRAPDOOR + 4,
        "Bamboo Trapdoor (open east)",
        "bamboo_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        CHISELED_STONE_BRICKS,
        "Chiseled Stone Bricks",
        "chiseled_stone_bricks",
        1.5,
        true,
        true,
    ),
    block(
        INFESTED_STONE,
        "Infested Stone",
        "infested_stone",
        0.75,
        true,
        true,
    ),
    block(
        INFESTED_COBBLESTONE,
        "Infested Cobblestone",
        "infested_cobblestone",
        0.75,
        true,
        true,
    ),
    block(
        INFESTED_STONE_BRICKS,
        "Infested Stone Bricks",
        "infested_stone_bricks",
        0.75,
        true,
        true,
    ),
    block(
        INFESTED_MOSSY_STONE_BRICKS,
        "Infested Mossy Stone Bricks",
        "infested_mossy_stone_bricks",
        0.75,
        true,
        true,
    ),
    block(
        INFESTED_CRACKED_STONE_BRICKS,
        "Infested Cracked Stone Bricks",
        "infested_cracked_stone_bricks",
        0.75,
        true,
        true,
    ),
    block(
        INFESTED_CHISELED_STONE_BRICKS,
        "Infested Chiseled Stone Bricks",
        "infested_chiseled_stone_bricks",
        0.75,
        true,
        true,
    ),
    block(
        BROWN_MUSHROOM_BLOCK,
        "Brown Mushroom Block",
        "brown_mushroom_block",
        0.2,
        true,
        true,
    ),
    block(
        RED_MUSHROOM_BLOCK,
        "Red Mushroom Block",
        "red_mushroom_block",
        0.2,
        true,
        true,
    ),
    block(
        MUSHROOM_STEM,
        "Mushroom Stem",
        "mushroom_stem",
        0.2,
        true,
        true,
    ),
    block(IRON_BARS, "Iron Bars", "iron_bars", 5.0, true, false),
    block(CHAIN, "Chain", "chain", 5.0, false, false),
    block(GLASS_PANE, "Glass Pane", "glass_pane", 0.3, true, false),
];

/// Растения рисуются двумя скрещёнными плоскостями и не мешают ходьбе.
#[must_use]
pub fn is_plant(id: BlockId) -> bool {
    is_sprite(id)
}

/// Куда смотрит «лицо» блока (или от какой стены отходит факел).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Facing {
    /// Минус Z.
    North,
    /// Плюс Z.
    South,
    /// Минус X.
    West,
    /// Плюс X.
    East,
    Up,
    Down,
}

impl Facing {
    pub const ALL: [Self; 6] = [
        Self::North,
        Self::South,
        Self::West,
        Self::East,
        Self::Up,
        Self::Down,
    ];

    /// Ось (0 = X, 1 = Y, 2 = Z) и знак направления.
    #[must_use]
    pub const fn axis_positive(self) -> (usize, bool) {
        match self {
            Self::North => (2, false),
            Self::South => (2, true),
            Self::West => (0, false),
            Self::East => (0, true),
            Self::Up => (1, true),
            Self::Down => (1, false),
        }
    }

    #[must_use]
    pub const fn from_axis(axis: usize, positive: bool) -> Self {
        match (axis, positive) {
            (0, false) => Self::West,
            (0, true) => Self::East,
            (1, false) => Self::Down,
            (1, true) => Self::Up,
            (_, false) => Self::North,
            (_, true) => Self::South,
        }
    }

    #[must_use]
    pub const fn opposite(self) -> Self {
        let (axis, positive) = self.axis_positive();
        Self::from_axis(axis, !positive)
    }

    /// Единичный сдвиг клетки в этом направлении.
    #[must_use]
    pub const fn offset(self) -> [i32; 3] {
        let (axis, positive) = self.axis_positive();
        let mut offset = [0; 3];
        offset[axis] = if positive { 1 } else { -1 };
        offset
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::North => "север",
            Self::South => "юг",
            Self::West => "запад",
            Self::East => "восток",
            Self::Up => "вверх",
            Self::Down => "вниз",
        }
    }

    /// Направление по единичному сдвигу; `None`, если сдвиг не единичный.
    #[must_use]
    pub fn from_offset(offset: [i32; 3]) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|facing| facing.offset() == offset)
    }
}

/// Блоки с шестью направлениями: раздатчик и два поршня (север — основной).
const FACING_FAMILIES: [[BlockId; 6]; 3] = [
    [
        DISPENSER,
        DISPENSER_SOUTH,
        DISPENSER_WEST,
        DISPENSER_EAST,
        DISPENSER_UP,
        DISPENSER_DOWN,
    ],
    [
        PISTON,
        PISTON_SOUTH,
        PISTON_WEST,
        PISTON_EAST,
        PISTON_UP,
        PISTON_DOWN,
    ],
    [
        STICKY_PISTON,
        STICKY_PISTON_SOUTH,
        STICKY_PISTON_WEST,
        STICKY_PISTON_EAST,
        STICKY_PISTON_UP,
        STICKY_PISTON_DOWN,
    ],
];
/// Настенный факел: четыре стороны, `Facing` — от стены наружу.
const WALL_TORCHES: [BlockId; 4] = [
    WALL_TORCH,
    WALL_TORCH_SOUTH,
    WALL_TORCH_WEST,
    WALL_TORCH_EAST,
];
/// Прямые рельсы: вдоль Z (основной блок) и вдоль X.
const RAILS: [[BlockId; 2]; 3] = [
    [POWERED_RAIL, POWERED_RAIL_X],
    [DETECTOR_RAIL, DETECTOR_RAIL_X],
    [RAIL, RAIL + 1],
];

/// Семейство состояний блока: (первый ID, номер варианта, вид).
#[must_use]
pub fn family_of(id: BlockId) -> Option<(BlockId, u8, FamilyKind)> {
    FAMILIES
        .iter()
        .find(|(first, len, _)| id >= *first && id < *first + BlockId::from(*len))
        .map(|(first, _, kind)| (*first, (id - *first) as u8, *kind))
}

/// Основной блок семейства направлений: тот самый, что лежит в инвентаре.
#[must_use]
pub fn base_block(id: BlockId) -> BlockId {
    for family in FACING_FAMILIES {
        if family.contains(&id) {
            return family[0];
        }
    }
    if WALL_TORCHES.contains(&id) {
        return WALL_TORCH;
    }
    for rails in RAILS {
        if rails.contains(&id) {
            return rails[0];
        }
    }
    family_of(id).map_or(id, |(first, _, _)| first)
}

/// Направление блока семейства; у остальных блоков `None`.
#[must_use]
pub fn facing_of(id: BlockId) -> Option<Facing> {
    for family in FACING_FAMILIES {
        if let Some(index) = family.iter().position(|member| *member == id) {
            return Some(Facing::ALL[index]);
        }
    }
    if let Some(index) = WALL_TORCHES.iter().position(|member| *member == id) {
        return Some(Facing::ALL[index]);
    }
    match family_of(id) {
        Some((_, index, FamilyKind::H4)) => Some(Facing::ALL[usize::from(index)]),
        _ => None,
    }
}

/// Вариант блока для направления. У настенного факела вертикальные направления
/// невозможны и заменяются на север; у блоков вне семейств ID не меняется.
#[must_use]
pub fn facing_variant(id: BlockId, facing: Facing) -> BlockId {
    let base = base_block(id);
    for family in FACING_FAMILIES {
        if family[0] == base {
            return family[Facing::ALL.iter().position(|f| *f == facing).unwrap_or(0)];
        }
    }
    if base == WALL_TORCH {
        let index = Facing::ALL
            .iter()
            .position(|f| *f == facing)
            .filter(|index| *index < 4)
            .unwrap_or(0);
        return WALL_TORCHES[index];
    }
    if let Some((first, _, FamilyKind::H4)) = family_of(base) {
        let index = Facing::ALL
            .iter()
            .position(|f| *f == facing)
            .filter(|index| *index < 4)
            .unwrap_or(0);
        return first + index as BlockId;
    }
    id
}

/// Номер варианта H4-семейства для стороны света (север, юг, запад, восток);
/// вертикальные направления заменяются на север.
#[must_use]
pub fn facing_variant_index(first: BlockId, facing: Facing) -> BlockId {
    let index = Facing::ALL
        .iter()
        .position(|f| *f == facing)
        .filter(|index| *index < 4)
        .unwrap_or(0);
    first + index as BlockId
}

/// Рельсы: вариант вдоль оси X или Z.
#[must_use]
pub fn rail_variant(id: BlockId, along_x: bool) -> BlockId {
    let base = base_block(id);
    for rails in RAILS {
        if rails[0] == base {
            return rails[usize::from(along_x)];
        }
    }
    id
}

#[must_use]
pub fn is_rail(id: BlockId) -> bool {
    RAILS.iter().any(|rails| rails.contains(&id))
}

#[must_use]
pub fn is_rail_x(id: BlockId) -> bool {
    RAILS.iter().any(|rails| rails[1] == id)
}

#[must_use]
pub fn is_wall_torch(id: BlockId) -> bool {
    WALL_TORCHES.contains(&id) || matches!(base_block(id), REDSTONE_WALL_TORCH | SOUL_WALL_TORCH)
}

/// Блок, из которого получается выпадающий предмет: без направления и состояния,
/// а настенный факел и головка поршня дают обычный факел и поршень.
#[must_use]
pub fn drop_base(id: BlockId) -> BlockId {
    match base_block(unlit(id)) {
        WALL_TORCH => TORCH,
        REDSTONE_WALL_TORCH => REDSTONE_TORCH,
        SOUL_WALL_TORCH => SOUL_TORCH,
        PISTON_HEAD => PISTON,
        other => other,
    }
}

/// Растение, факел или огонь: две скрещённые плоскости, без столкновений.
#[must_use]
pub fn is_cross(id: BlockId) -> bool {
    (POPPY..=TALL_GRASS).contains(&id)
        || (PLANT2_FIRST..=PLANT2_LAST).contains(&id)
        || matches!(id, WHEAT | SUGAR_CANE | REDSTONE_TORCH | SOUL_TORCH)
        || is_wall_torch(id)
}

/// Плоский рисунок на полу (рельсы).
#[must_use]
pub fn is_flat(id: BlockId) -> bool {
    is_rail(id) || id == REDSTONE_WIRE
}

/// Блок, который рисуется спрайтом и не мешает ходьбе.
#[must_use]
pub fn is_sprite(id: BlockId) -> bool {
    is_cross(id) || is_flat(id)
}

/// Кровать: низкий блок цвета одеяла.
#[must_use]
pub fn is_bed(id: BlockId) -> bool {
    (WHITE_BED..=BLACK_BED).contains(&id)
}

/// Огонь: жжёт тело игрока.
#[must_use]
pub fn is_fire(id: BlockId) -> bool {
    id == FIRE || id == SOUL_FIRE
}

/// Невидимый технический блок.
#[must_use]
pub fn is_invisible(id: BlockId) -> bool {
    id == MOVING_PISTON
}

/// Нижняя половина клетки: плиты, кровати, головка поршня.
#[must_use]
pub fn is_half(id: BlockId) -> bool {
    is_slab(id) || (WHITE_BED..=PISTON_HEAD).contains(&id)
}

/// Что получится, если ударить бревно или древесину топором ПКМ.
#[must_use]
pub fn stripped_variant(id: BlockId) -> Option<BlockId> {
    Some(match id {
        WOOD => STRIPPED_OAK_LOG,
        SPRUCE_LOG => STRIPPED_SPRUCE_LOG,
        BIRCH_LOG => STRIPPED_BIRCH_LOG,
        JUNGLE_LOG => STRIPPED_JUNGLE_LOG,
        ACACIA_LOG => STRIPPED_ACACIA_LOG,
        CHERRY_LOG => STRIPPED_CHERRY_LOG,
        DARK_OAK_LOG => STRIPPED_DARK_OAK_LOG,
        MANGROVE_LOG => STRIPPED_MANGROVE_LOG,
        BAMBOO_BLOCK => STRIPPED_BAMBOO_BLOCK,
        OAK_WOOD => STRIPPED_OAK_WOOD,
        SPRUCE_WOOD => STRIPPED_SPRUCE_WOOD,
        BIRCH_WOOD => STRIPPED_BIRCH_WOOD,
        JUNGLE_WOOD => STRIPPED_JUNGLE_WOOD,
        ACACIA_WOOD => STRIPPED_ACACIA_WOOD,
        CHERRY_WOOD => STRIPPED_CHERRY_WOOD,
        DARK_OAK_WOOD => STRIPPED_DARK_OAK_WOOD,
        MANGROVE_WOOD => STRIPPED_MANGROVE_WOOD,
        _ => return None,
    })
}

/// Насколько охотно блок горит, 0…100. Камень, руды, земля и жидкости не горят.
#[must_use]
pub fn flammability(id: BlockId) -> u8 {
    if is_leaves(id) || (WOOL_ORANGE..=WOOL_BLACK).contains(&id) || id == WOOL || id == HAY_BALE {
        return 60;
    }
    if is_cross(id) {
        // Растения горят, факелы и огонь — нет.
        return if matches!(
            id,
            TORCH
                | FIRE
                | SOUL_FIRE
                | WALL_TORCH
                | WALL_TORCH_SOUTH
                | WALL_TORCH_WEST
                | WALL_TORCH_EAST
        ) {
            0
        } else {
            60
        };
    }
    if is_bed(id) {
        return 40;
    }
    match id {
        PLANKS
        | WOOD
        | BIRCH_LOG
        | BIRCH_PLANKS
        | PINE_LOG
        | PINE_PLANKS
        | ACACIA_LOG
        | ACACIA_PLANKS
        | JUNGLE_LOG
        | JUNGLE_PLANKS
        | DARK_OAK_LOG
        | DARK_OAK_PLANKS
        | MANGROVE_LOG
        | MANGROVE_PLANKS
        | CHERRY_LOG
        | CHERRY_PLANKS
        | SPRUCE_LOG
        | SPRUCE_PLANKS
        | BAMBOO_BLOCK
        | BAMBOO_PLANKS
        | BAMBOO_MOSAIC
        | OAK_WOOD
        | SPRUCE_WOOD
        | BIRCH_WOOD
        | JUNGLE_WOOD
        | ACACIA_WOOD
        | CHERRY_WOOD
        | DARK_OAK_WOOD
        | MANGROVE_WOOD
        | STRIPPED_OAK_LOG
        | STRIPPED_SPRUCE_LOG
        | STRIPPED_BIRCH_LOG
        | STRIPPED_JUNGLE_LOG
        | STRIPPED_ACACIA_LOG
        | STRIPPED_CHERRY_LOG
        | STRIPPED_DARK_OAK_LOG
        | STRIPPED_MANGROVE_LOG
        | STRIPPED_BAMBOO_BLOCK
        | STRIPPED_OAK_WOOD
        | STRIPPED_SPRUCE_WOOD
        | STRIPPED_BIRCH_WOOD
        | STRIPPED_JUNGLE_WOOD
        | STRIPPED_ACACIA_WOOD
        | STRIPPED_CHERRY_WOOD
        | STRIPPED_DARK_OAK_WOOD
        | STRIPPED_MANGROVE_WOOD
        | BOOKSHELF
        | CHISELED_BOOKSHELF
        | NOTE_BLOCK
        | CRAFTING_TABLE
        | CHEST
        | BARREL
        | OAK_SLAB
        | BIRCH_SLAB
        | PINE_SLAB
        | MANGROVE_ROOTS
        | MUDDY_MANGROVE_ROOTS => 30,
        _ => 0,
    }
}

/// Любая вода: источник или течение.
#[must_use]
pub fn is_water(id: BlockId) -> bool {
    id == WATER || (FLOW_WATER_1..=FLOW_WATER_7).contains(&id)
}

/// Любая лава: источник или течение.
#[must_use]
pub fn is_lava(id: BlockId) -> bool {
    (LAVA..=FLOW_LAVA_3).contains(&id)
}

/// Жидкий блок: вода или лава.
#[must_use]
pub fn is_fluid(id: BlockId) -> bool {
    is_water(id) || is_lava(id)
}

/// Плита занимает нижнюю половину клетки.
#[must_use]
pub fn is_slab(id: BlockId) -> bool {
    (OAK_SLAB..=DARK_BRICK_SLAB).contains(&id)
}

/// Блок без состояния «горит»: в таком виде он выпадает и сохраняется.
#[must_use]
pub fn unlit(id: BlockId) -> BlockId {
    match id {
        FURNACE_LIT => FURNACE,
        SMOKER_LIT => SMOKER,
        BLAST_FURNACE_LIT => BLAST_FURNACE,
        other => other,
    }
}

/// Листва: куб с вырезанной по альфе текстурой, поэтому не непрозрачна.
#[must_use]
pub fn is_leaves(id: BlockId) -> bool {
    matches!(
        id,
        LEAVES
            | BIRCH_LEAVES
            | PINE_LEAVES
            | ACACIA_LEAVES
            | JUNGLE_LEAVES
            | DARK_OAK_LEAVES
            | MANGROVE_LEAVES
            | CHERRY_LEAVES
            | SPRUCE_LEAVES
            | AZALEA_LEAVES
            | FLOWERING_AZALEA_LEAVES
    )
}

#[must_use]
pub fn is_solid(id: BlockId) -> bool {
    BLOCKS
        .get(usize::from(id))
        .is_some_and(|block| block.collision)
}
#[must_use]
pub fn is_opaque(id: BlockId) -> bool {
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
        assert_eq!(BLOCKS.len(), 604);
        for (offset, name) in expected.iter().enumerate() {
            let id = 17 + offset;
            assert_eq!(BLOCKS[id].id as usize, id);
            assert_eq!(BLOCKS[id].name, *name);
            assert!(is_solid(id as BlockId) && is_opaque(id as BlockId));
        }
    }

    #[test]
    fn chest_extends_registry_without_changing_foundation_ids() {
        assert_eq!(BLOCKS.len(), 604);
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
        assert!(!is_solid(BlockId::MAX));
        assert!(!is_opaque(BlockId::MAX));
    }
}
