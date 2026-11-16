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
    /// Калитка: закрыта/открыта × четыре стороны.
    Gate,
    /// Шесть направлений: север, юг, запад, восток, вверх, вниз.
    H6,
}

/// Семейства блоков с состоянием: (первый ID, число вариантов, вид).
pub const FAMILIES: [(BlockId, u8, FamilyKind); 212] = [
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
    (610, 8, FamilyKind::Gate),
    (618, 4, FamilyKind::H4),
    (622, 4, FamilyKind::H4),
    (626, 4, FamilyKind::H4),
    (634, 4, FamilyKind::H4),
    (651, 4, FamilyKind::H4),
    (657, 4, FamilyKind::H4),
    (662, 4, FamilyKind::H4),
    (666, 4, FamilyKind::H4),
    (670, 4, FamilyKind::H4),
    (708, 4, FamilyKind::H4),
    (712, 4, FamilyKind::H4),
    (716, 4, FamilyKind::H4),
    (720, 4, FamilyKind::H4),
    (724, 4, FamilyKind::H4),
    (728, 4, FamilyKind::H4),
    (732, 4, FamilyKind::H4),
    (736, 4, FamilyKind::H4),
    (740, 4, FamilyKind::H4),
    (745, 4, FamilyKind::H4),
    (750, 4, FamilyKind::H4),
    (755, 4, FamilyKind::H4),
    (760, 4, FamilyKind::H4),
    (765, 4, FamilyKind::H4),
    (770, 4, FamilyKind::H4),
    (775, 4, FamilyKind::H4),
    (779, 2, FamilyKind::AX2),
    (781, 2, FamilyKind::AX2),
    (783, 2, FamilyKind::AX2),
    (788, 4, FamilyKind::H4),
    (798, 4, FamilyKind::H4),
    (802, 2, FamilyKind::Rail),
    (804, 6, FamilyKind::H6),
    (826, 4, FamilyKind::H4),
    (830, 4, FamilyKind::H4),
    (834, 4, FamilyKind::H4),
    (838, 4, FamilyKind::H4),
    (842, 4, FamilyKind::H4),
    (846, 4, FamilyKind::H4),
    (853, 5, FamilyKind::Trap),
    (861, 4, FamilyKind::H4),
    (865, 4, FamilyKind::H4),
    (869, 4, FamilyKind::H4),
    (899, 4, FamilyKind::H4),
    (903, 4, FamilyKind::H4),
    (907, 4, FamilyKind::H4),
    (911, 4, FamilyKind::H4),
    (915, 4, FamilyKind::H4),
    (919, 4, FamilyKind::H4),
    (923, 4, FamilyKind::H4),
    (927, 4, FamilyKind::H4),
    (931, 4, FamilyKind::H4),
    (935, 4, FamilyKind::H4),
    (939, 4, FamilyKind::H4),
    (943, 4, FamilyKind::H4),
    (947, 4, FamilyKind::H4),
    (951, 4, FamilyKind::H4),
    (955, 4, FamilyKind::H4),
    (959, 4, FamilyKind::H4),
    (963, 4, FamilyKind::H4),
    (967, 4, FamilyKind::H4),
    (971, 4, FamilyKind::H4),
    (975, 4, FamilyKind::H4),
    (979, 4, FamilyKind::H4),
    (983, 4, FamilyKind::H4),
    (987, 4, FamilyKind::H4),
    (991, 4, FamilyKind::H4),
    (995, 4, FamilyKind::H4),
    (999, 4, FamilyKind::H4),
    (1003, 4, FamilyKind::H4),
    (1007, 4, FamilyKind::H4),
    (1011, 4, FamilyKind::H4),
    (1015, 4, FamilyKind::H4),
    (1019, 4, FamilyKind::H4),
    (1023, 4, FamilyKind::H4),
    (1029, 4, FamilyKind::H4),
    (1052, 8, FamilyKind::Gate),
    (1060, 8, FamilyKind::Gate),
    (1068, 8, FamilyKind::Gate),
    (1076, 8, FamilyKind::Gate),
    (1084, 8, FamilyKind::Gate),
    (1092, 8, FamilyKind::Gate),
    (1100, 8, FamilyKind::Gate),
    (1108, 8, FamilyKind::Gate),
    (1124, 16, FamilyKind::Door),
    (1140, 16, FamilyKind::Door),
    (1156, 16, FamilyKind::Door),
    (1172, 16, FamilyKind::Door),
    (1188, 16, FamilyKind::Door),
    (1204, 16, FamilyKind::Door),
    (1220, 16, FamilyKind::Door),
    (1236, 16, FamilyKind::Door),
    (1257, 4, FamilyKind::H4),
    (1275, 6, FamilyKind::H6),
    (1365, 4, FamilyKind::H4),
    (1369, 4, FamilyKind::H4),
    (1373, 4, FamilyKind::H4),
    (1377, 4, FamilyKind::H4),
    (1381, 4, FamilyKind::H4),
    (1385, 4, FamilyKind::H4),
    (1389, 4, FamilyKind::H4),
    (1393, 4, FamilyKind::H4),
    (1397, 4, FamilyKind::H4),
    (1401, 4, FamilyKind::H4),
    (1414, 4, FamilyKind::H4),
    (1418, 4, FamilyKind::H4),
    (1422, 4, FamilyKind::H4),
    (1426, 4, FamilyKind::H4),
    (1430, 4, FamilyKind::H4),
    (1434, 4, FamilyKind::H4),
    (1438, 4, FamilyKind::H4),
    (1442, 4, FamilyKind::H4),
    (1446, 4, FamilyKind::H4),
    (1450, 4, FamilyKind::H4),
    (1454, 4, FamilyKind::H4),
    (1458, 4, FamilyKind::H4),
    (1462, 4, FamilyKind::H4),
    (1466, 4, FamilyKind::H4),
    (1537, 5, FamilyKind::Trap),
    (1542, 5, FamilyKind::Trap),
    (1547, 8, FamilyKind::Gate),
    (1555, 8, FamilyKind::Gate),
    (1563, 4, FamilyKind::H4),
    (1567, 4, FamilyKind::H4),
    (1571, 4, FamilyKind::H4),
    (1575, 4, FamilyKind::H4),
    (1579, 16, FamilyKind::Door),
    (1595, 16, FamilyKind::Door),
    (1611, 4, FamilyKind::H4),
    (1615, 4, FamilyKind::H4),
    (1619, 4, FamilyKind::H4),
    (1623, 4, FamilyKind::H4),
    (1645, 4, FamilyKind::H4),
    (1656, 4, FamilyKind::H4),
    (1662, 4, FamilyKind::H4),
    (1668, 4, FamilyKind::H4),
    (1731, 4, FamilyKind::H4),
    (1735, 4, FamilyKind::H4),
    (1739, 4, FamilyKind::H4),
    (1743, 4, FamilyKind::H4),
    (1759, 4, FamilyKind::H4),
    (1763, 4, FamilyKind::H4),
    (1767, 4, FamilyKind::H4),
    (1771, 4, FamilyKind::H4),
    (1795, 4, FamilyKind::H4),
    (1802, 4, FamilyKind::H4),
    (1809, 4, FamilyKind::H4),
    (1816, 4, FamilyKind::H4),
    (1841, 4, FamilyKind::H4),
];

pub const PART2_FIRST: BlockId = 321;
pub const PART2_LAST: BlockId = 603;
pub const ATTACHED_PUMPKIN_STEM: BlockId = 604;
pub const ATTACHED_MELON_STEM: BlockId = 605;
pub const PUMPKIN_STEM: BlockId = 606;
pub const MELON_STEM: BlockId = 607;
pub const VINE: BlockId = 608;
pub const GLOW_LICHEN: BlockId = 609;
pub const OAK_FENCE_GATE: BlockId = 610;
pub const BRICK_STAIRS: BlockId = 618;
pub const STONE_BRICK_STAIRS: BlockId = 622;
pub const MUD_BRICK_STAIRS: BlockId = 626;
pub const MYCELIUM: BlockId = 630;
pub const LILY_PAD: BlockId = 631;
pub const NETHER_BRICKS: BlockId = 632;
pub const NETHER_BRICK_FENCE: BlockId = 633;
pub const NETHER_BRICK_STAIRS: BlockId = 634;
pub const NETHER_WART: BlockId = 638;
pub const ENCHANTING_TABLE: BlockId = 639;
pub const BREWING_STAND: BlockId = 640;
pub const CAULDRON: BlockId = 641;
pub const WATER_CAULDRON: BlockId = 642;
pub const LAVA_CAULDRON: BlockId = 643;
pub const POWDER_SNOW_CAULDRON: BlockId = 644;
pub const END_PORTAL: BlockId = 645;
pub const END_PORTAL_FRAME: BlockId = 646;
pub const END_STONE: BlockId = 647;
pub const DRAGON_EGG: BlockId = 648;
pub const REDSTONE_LAMP: BlockId = 649;
pub const COCOA: BlockId = 650;
pub const SANDSTONE_STAIRS: BlockId = 651;
pub const DEEPSLATE_EMERALD_ORE: BlockId = 655;
pub const ENDER_CHEST: BlockId = 656;
pub const TRIPWIRE_HOOK: BlockId = 657;
pub const TRIPWIRE: BlockId = 661;
pub const SPRUCE_STAIRS: BlockId = 662;
pub const BIRCH_STAIRS: BlockId = 666;
pub const JUNGLE_STAIRS: BlockId = 670;
pub const COMMAND_BLOCK: BlockId = 674;
pub const BEACON: BlockId = 675;
pub const COBBLESTONE_WALL: BlockId = 676;
pub const MOSSY_COBBLESTONE_WALL: BlockId = 677;
pub const FLOWER_POT: BlockId = 678;
pub const POTTED_TORCHFLOWER: BlockId = 679;
pub const POTTED_OAK_SAPLING: BlockId = 680;
pub const POTTED_SPRUCE_SAPLING: BlockId = 681;
pub const POTTED_BIRCH_SAPLING: BlockId = 682;
pub const POTTED_JUNGLE_SAPLING: BlockId = 683;
pub const POTTED_ACACIA_SAPLING: BlockId = 684;
pub const POTTED_CHERRY_SAPLING: BlockId = 685;
pub const POTTED_DARK_OAK_SAPLING: BlockId = 686;
pub const POTTED_MANGROVE_PROPAGULE: BlockId = 687;
pub const POTTED_FERN: BlockId = 688;
pub const POTTED_DANDELION: BlockId = 689;
pub const POTTED_POPPY: BlockId = 690;
pub const POTTED_BLUE_ORCHID: BlockId = 691;
pub const POTTED_ALLIUM: BlockId = 692;
pub const POTTED_AZURE_BLUET: BlockId = 693;
pub const POTTED_RED_TULIP: BlockId = 694;
pub const POTTED_ORANGE_TULIP: BlockId = 695;
pub const POTTED_WHITE_TULIP: BlockId = 696;
pub const POTTED_PINK_TULIP: BlockId = 697;
pub const POTTED_OXEYE_DAISY: BlockId = 698;
pub const POTTED_CORNFLOWER: BlockId = 699;
pub const POTTED_LILY_OF_THE_VALLEY: BlockId = 700;
pub const POTTED_WITHER_ROSE: BlockId = 701;
pub const POTTED_RED_MUSHROOM: BlockId = 702;
pub const POTTED_BROWN_MUSHROOM: BlockId = 703;
pub const POTTED_DEAD_BUSH: BlockId = 704;
pub const POTTED_CACTUS: BlockId = 705;
pub const CARROTS: BlockId = 706;
pub const POTATOES: BlockId = 707;
pub const OAK_BUTTON: BlockId = 708;
pub const SPRUCE_BUTTON: BlockId = 712;
pub const BIRCH_BUTTON: BlockId = 716;
pub const JUNGLE_BUTTON: BlockId = 720;
pub const ACACIA_BUTTON: BlockId = 724;
pub const CHERRY_BUTTON: BlockId = 728;
pub const DARK_OAK_BUTTON: BlockId = 732;
pub const MANGROVE_BUTTON: BlockId = 736;
pub const BAMBOO_BUTTON: BlockId = 740;
pub const SKELETON_SKULL: BlockId = 744;
pub const SKELETON_WALL_SKULL: BlockId = 745;
pub const WITHER_SKELETON_SKULL: BlockId = 749;
pub const WITHER_SKELETON_WALL_SKULL: BlockId = 750;
pub const ZOMBIE_HEAD: BlockId = 754;
pub const ZOMBIE_WALL_HEAD: BlockId = 755;
pub const PLAYER_HEAD: BlockId = 759;
pub const PLAYER_WALL_HEAD: BlockId = 760;
pub const CREEPER_HEAD: BlockId = 764;
pub const CREEPER_WALL_HEAD: BlockId = 765;
pub const DRAGON_HEAD: BlockId = 769;
pub const DRAGON_WALL_HEAD: BlockId = 770;
pub const PIGLIN_HEAD: BlockId = 774;
pub const PIGLIN_WALL_HEAD: BlockId = 775;
pub const ANVIL: BlockId = 779;
pub const CHIPPED_ANVIL: BlockId = 781;
pub const DAMAGED_ANVIL: BlockId = 783;
pub const TRAPPED_CHEST: BlockId = 785;
pub const LIGHT_WEIGHTED_PRESSURE_PLATE: BlockId = 786;
pub const HEAVY_WEIGHTED_PRESSURE_PLATE: BlockId = 787;
pub const COMPARATOR: BlockId = 788;
pub const DAYLIGHT_DETECTOR: BlockId = 792;
pub const REDSTONE_BLOCK: BlockId = 793;
pub const NETHER_QUARTZ_ORE: BlockId = 794;
pub const HOPPER: BlockId = 795;
pub const CHISELED_QUARTZ_BLOCK: BlockId = 796;
pub const PART3_FIRST: BlockId = 604;
pub const PART3_LAST: BlockId = 796;
pub const QUARTZ_PILLAR: BlockId = 797;
pub const QUARTZ_STAIRS: BlockId = 798;
pub const ACTIVATOR_RAIL: BlockId = 802;
pub const DROPPER: BlockId = 804;
pub const WHITE_STAINED_GLASS_PANE: BlockId = 810;
pub const ORANGE_STAINED_GLASS_PANE: BlockId = 811;
pub const MAGENTA_STAINED_GLASS_PANE: BlockId = 812;
pub const LIGHT_BLUE_STAINED_GLASS_PANE: BlockId = 813;
pub const YELLOW_STAINED_GLASS_PANE: BlockId = 814;
pub const LIME_STAINED_GLASS_PANE: BlockId = 815;
pub const PINK_STAINED_GLASS_PANE: BlockId = 816;
pub const GRAY_STAINED_GLASS_PANE: BlockId = 817;
pub const LIGHT_GRAY_STAINED_GLASS_PANE: BlockId = 818;
pub const CYAN_STAINED_GLASS_PANE: BlockId = 819;
pub const PURPLE_STAINED_GLASS_PANE: BlockId = 820;
pub const BLUE_STAINED_GLASS_PANE: BlockId = 821;
pub const BROWN_STAINED_GLASS_PANE: BlockId = 822;
pub const GREEN_STAINED_GLASS_PANE: BlockId = 823;
pub const RED_STAINED_GLASS_PANE: BlockId = 824;
pub const BLACK_STAINED_GLASS_PANE: BlockId = 825;
pub const ACACIA_STAIRS: BlockId = 826;
pub const CHERRY_STAIRS: BlockId = 830;
pub const DARK_OAK_STAIRS: BlockId = 834;
pub const MANGROVE_STAIRS: BlockId = 838;
pub const BAMBOO_STAIRS: BlockId = 842;
pub const BAMBOO_MOSAIC_STAIRS: BlockId = 846;
pub const SLIME_BLOCK: BlockId = 850;
pub const BARRIER: BlockId = 851;
pub const LIGHT: BlockId = 852;
pub const IRON_TRAPDOOR: BlockId = 853;
pub const PRISMARINE: BlockId = 858;
pub const PRISMARINE_BRICKS: BlockId = 859;
pub const DARK_PRISMARINE: BlockId = 860;
pub const PRISMARINE_STAIRS: BlockId = 861;
pub const PRISMARINE_BRICK_STAIRS: BlockId = 865;
pub const DARK_PRISMARINE_STAIRS: BlockId = 869;
pub const PRISMARINE_SLAB: BlockId = 873;
pub const PRISMARINE_BRICK_SLAB: BlockId = 874;
pub const DARK_PRISMARINE_SLAB: BlockId = 875;
pub const SEA_LANTERN: BlockId = 876;
pub const WHITE_CARPET: BlockId = 877;
pub const ORANGE_CARPET: BlockId = 878;
pub const MAGENTA_CARPET: BlockId = 879;
pub const LIGHT_BLUE_CARPET: BlockId = 880;
pub const YELLOW_CARPET: BlockId = 881;
pub const LIME_CARPET: BlockId = 882;
pub const PINK_CARPET: BlockId = 883;
pub const GRAY_CARPET: BlockId = 884;
pub const LIGHT_GRAY_CARPET: BlockId = 885;
pub const CYAN_CARPET: BlockId = 886;
pub const PURPLE_CARPET: BlockId = 887;
pub const BLUE_CARPET: BlockId = 888;
pub const BROWN_CARPET: BlockId = 889;
pub const GREEN_CARPET: BlockId = 890;
pub const RED_CARPET: BlockId = 891;
pub const BLACK_CARPET: BlockId = 892;
pub const PACKED_ICE: BlockId = 893;
pub const SUNFLOWER: BlockId = 894;
pub const LILAC: BlockId = 895;
pub const ROSE_BUSH: BlockId = 896;
pub const PEONY: BlockId = 897;
pub const LARGE_FERN: BlockId = 898;
pub const WHITE_BANNER: BlockId = 899;
pub const ORANGE_BANNER: BlockId = 903;
pub const MAGENTA_BANNER: BlockId = 907;
pub const LIGHT_BLUE_BANNER: BlockId = 911;
pub const YELLOW_BANNER: BlockId = 915;
pub const LIME_BANNER: BlockId = 919;
pub const PINK_BANNER: BlockId = 923;
pub const GRAY_BANNER: BlockId = 927;
pub const LIGHT_GRAY_BANNER: BlockId = 931;
pub const CYAN_BANNER: BlockId = 935;
pub const PURPLE_BANNER: BlockId = 939;
pub const BLUE_BANNER: BlockId = 943;
pub const BROWN_BANNER: BlockId = 947;
pub const GREEN_BANNER: BlockId = 951;
pub const RED_BANNER: BlockId = 955;
pub const BLACK_BANNER: BlockId = 959;
pub const WHITE_WALL_BANNER: BlockId = 963;
pub const ORANGE_WALL_BANNER: BlockId = 967;
pub const MAGENTA_WALL_BANNER: BlockId = 971;
pub const LIGHT_BLUE_WALL_BANNER: BlockId = 975;
pub const YELLOW_WALL_BANNER: BlockId = 979;
pub const LIME_WALL_BANNER: BlockId = 983;
pub const PINK_WALL_BANNER: BlockId = 987;
pub const GRAY_WALL_BANNER: BlockId = 991;
pub const LIGHT_GRAY_WALL_BANNER: BlockId = 995;
pub const CYAN_WALL_BANNER: BlockId = 999;
pub const PURPLE_WALL_BANNER: BlockId = 1003;
pub const BLUE_WALL_BANNER: BlockId = 1007;
pub const BROWN_WALL_BANNER: BlockId = 1011;
pub const GREEN_WALL_BANNER: BlockId = 1015;
pub const RED_WALL_BANNER: BlockId = 1019;
pub const BLACK_WALL_BANNER: BlockId = 1023;
pub const CHISELED_RED_SANDSTONE: BlockId = 1027;
pub const CUT_RED_SANDSTONE: BlockId = 1028;
pub const RED_SANDSTONE_STAIRS: BlockId = 1029;
pub const SPRUCE_SLAB: BlockId = 1033;
pub const JUNGLE_SLAB: BlockId = 1034;
pub const ACACIA_SLAB: BlockId = 1035;
pub const CHERRY_SLAB: BlockId = 1036;
pub const DARK_OAK_SLAB: BlockId = 1037;
pub const MANGROVE_SLAB: BlockId = 1038;
pub const BAMBOO_SLAB: BlockId = 1039;
pub const BAMBOO_MOSAIC_SLAB: BlockId = 1040;
pub const PART4_FIRST: BlockId = 797;
pub const PART4_LAST: BlockId = 1040;
pub const CUT_SANDSTONE_SLAB: BlockId = 1041;
pub const PETRIFIED_OAK_SLAB: BlockId = 1042;
pub const MUD_BRICK_SLAB: BlockId = 1043;
pub const NETHER_BRICK_SLAB: BlockId = 1044;
pub const QUARTZ_SLAB: BlockId = 1045;
pub const RED_SANDSTONE_SLAB: BlockId = 1046;
pub const CUT_RED_SANDSTONE_SLAB: BlockId = 1047;
pub const PURPUR_SLAB: BlockId = 1048;
pub const SMOOTH_SANDSTONE: BlockId = 1049;
pub const SMOOTH_QUARTZ: BlockId = 1050;
pub const SMOOTH_RED_SANDSTONE: BlockId = 1051;
pub const SPRUCE_FENCE_GATE: BlockId = 1052;
pub const BIRCH_FENCE_GATE: BlockId = 1060;
pub const JUNGLE_FENCE_GATE: BlockId = 1068;
pub const ACACIA_FENCE_GATE: BlockId = 1076;
pub const CHERRY_FENCE_GATE: BlockId = 1084;
pub const DARK_OAK_FENCE_GATE: BlockId = 1092;
pub const MANGROVE_FENCE_GATE: BlockId = 1100;
pub const BAMBOO_FENCE_GATE: BlockId = 1108;
pub const SPRUCE_FENCE: BlockId = 1116;
pub const BIRCH_FENCE: BlockId = 1117;
pub const JUNGLE_FENCE: BlockId = 1118;
pub const ACACIA_FENCE: BlockId = 1119;
pub const CHERRY_FENCE: BlockId = 1120;
pub const DARK_OAK_FENCE: BlockId = 1121;
pub const MANGROVE_FENCE: BlockId = 1122;
pub const BAMBOO_FENCE: BlockId = 1123;
pub const SPRUCE_DOOR: BlockId = 1124;
pub const BIRCH_DOOR: BlockId = 1140;
pub const JUNGLE_DOOR: BlockId = 1156;
pub const ACACIA_DOOR: BlockId = 1172;
pub const CHERRY_DOOR: BlockId = 1188;
pub const DARK_OAK_DOOR: BlockId = 1204;
pub const MANGROVE_DOOR: BlockId = 1220;
pub const BAMBOO_DOOR: BlockId = 1236;
pub const END_ROD: BlockId = 1252;
pub const CHORUS_PLANT: BlockId = 1253;
pub const CHORUS_FLOWER: BlockId = 1254;
pub const PURPUR_BLOCK: BlockId = 1255;
pub const PURPUR_PILLAR: BlockId = 1256;
pub const PURPUR_STAIRS: BlockId = 1257;
pub const END_STONE_BRICKS: BlockId = 1261;
pub const TORCHFLOWER_CROP: BlockId = 1262;
pub const PITCHER_CROP: BlockId = 1263;
pub const PITCHER_PLANT: BlockId = 1264;
pub const BEETROOTS: BlockId = 1265;
pub const DIRT_PATH: BlockId = 1266;
pub const END_GATEWAY: BlockId = 1267;
pub const REPEATING_COMMAND_BLOCK: BlockId = 1268;
pub const CHAIN_COMMAND_BLOCK: BlockId = 1269;
pub const FROSTED_ICE: BlockId = 1270;
pub const MAGMA_BLOCK: BlockId = 1271;
pub const NETHER_WART_BLOCK: BlockId = 1272;
pub const RED_NETHER_BRICKS: BlockId = 1273;
pub const STRUCTURE_VOID: BlockId = 1274;
pub const OBSERVER: BlockId = 1275;
pub const SHULKER_BOX: BlockId = 1281;
pub const WHITE_SHULKER_BOX: BlockId = 1282;
pub const ORANGE_SHULKER_BOX: BlockId = 1283;
pub const MAGENTA_SHULKER_BOX: BlockId = 1284;
pub const LIGHT_BLUE_SHULKER_BOX: BlockId = 1285;
pub const YELLOW_SHULKER_BOX: BlockId = 1286;
pub const LIME_SHULKER_BOX: BlockId = 1287;
pub const PINK_SHULKER_BOX: BlockId = 1288;
pub const GRAY_SHULKER_BOX: BlockId = 1289;
pub const LIGHT_GRAY_SHULKER_BOX: BlockId = 1290;
pub const CYAN_SHULKER_BOX: BlockId = 1291;
pub const PURPLE_SHULKER_BOX: BlockId = 1292;
pub const BLUE_SHULKER_BOX: BlockId = 1293;
pub const BROWN_SHULKER_BOX: BlockId = 1294;
pub const GREEN_SHULKER_BOX: BlockId = 1295;
pub const RED_SHULKER_BOX: BlockId = 1296;
pub const BLACK_SHULKER_BOX: BlockId = 1297;
pub const WHITE_GLAZED_TERRACOTTA: BlockId = 1298;
pub const ORANGE_GLAZED_TERRACOTTA: BlockId = 1299;
pub const MAGENTA_GLAZED_TERRACOTTA: BlockId = 1300;
pub const LIGHT_BLUE_GLAZED_TERRACOTTA: BlockId = 1301;
pub const YELLOW_GLAZED_TERRACOTTA: BlockId = 1302;
pub const LIME_GLAZED_TERRACOTTA: BlockId = 1303;
pub const PINK_GLAZED_TERRACOTTA: BlockId = 1304;
pub const GRAY_GLAZED_TERRACOTTA: BlockId = 1305;
pub const LIGHT_GRAY_GLAZED_TERRACOTTA: BlockId = 1306;
pub const CYAN_GLAZED_TERRACOTTA: BlockId = 1307;
pub const PURPLE_GLAZED_TERRACOTTA: BlockId = 1308;
pub const BLUE_GLAZED_TERRACOTTA: BlockId = 1309;
pub const BROWN_GLAZED_TERRACOTTA: BlockId = 1310;
pub const GREEN_GLAZED_TERRACOTTA: BlockId = 1311;
pub const RED_GLAZED_TERRACOTTA: BlockId = 1312;
pub const BLACK_GLAZED_TERRACOTTA: BlockId = 1313;
pub const WHITE_CONCRETE_POWDER: BlockId = 1314;
pub const ORANGE_CONCRETE_POWDER: BlockId = 1315;
pub const MAGENTA_CONCRETE_POWDER: BlockId = 1316;
pub const LIGHT_BLUE_CONCRETE_POWDER: BlockId = 1317;
pub const YELLOW_CONCRETE_POWDER: BlockId = 1318;
pub const LIME_CONCRETE_POWDER: BlockId = 1319;
pub const PINK_CONCRETE_POWDER: BlockId = 1320;
pub const GRAY_CONCRETE_POWDER: BlockId = 1321;
pub const LIGHT_GRAY_CONCRETE_POWDER: BlockId = 1322;
pub const CYAN_CONCRETE_POWDER: BlockId = 1323;
pub const PURPLE_CONCRETE_POWDER: BlockId = 1324;
pub const BLUE_CONCRETE_POWDER: BlockId = 1325;
pub const BROWN_CONCRETE_POWDER: BlockId = 1326;
pub const GREEN_CONCRETE_POWDER: BlockId = 1327;
pub const RED_CONCRETE_POWDER: BlockId = 1328;
pub const BLACK_CONCRETE_POWDER: BlockId = 1329;
pub const PART5_FIRST: BlockId = 1041;
pub const PART5_LAST: BlockId = 1329;
pub const KELP: BlockId = 1330;
pub const KELP_PLANT: BlockId = 1331;
pub const DRIED_KELP_BLOCK: BlockId = 1332;
pub const TURTLE_EGG: BlockId = 1333;
pub const SNIFFER_EGG: BlockId = 1334;
pub const DEAD_TUBE_CORAL_BLOCK: BlockId = 1335;
pub const DEAD_BRAIN_CORAL_BLOCK: BlockId = 1336;
pub const DEAD_BUBBLE_CORAL_BLOCK: BlockId = 1337;
pub const DEAD_FIRE_CORAL_BLOCK: BlockId = 1338;
pub const DEAD_HORN_CORAL_BLOCK: BlockId = 1339;
pub const TUBE_CORAL_BLOCK: BlockId = 1340;
pub const BRAIN_CORAL_BLOCK: BlockId = 1341;
pub const BUBBLE_CORAL_BLOCK: BlockId = 1342;
pub const FIRE_CORAL_BLOCK: BlockId = 1343;
pub const HORN_CORAL_BLOCK: BlockId = 1344;
pub const DEAD_TUBE_CORAL: BlockId = 1345;
pub const DEAD_BRAIN_CORAL: BlockId = 1346;
pub const DEAD_BUBBLE_CORAL: BlockId = 1347;
pub const DEAD_FIRE_CORAL: BlockId = 1348;
pub const DEAD_HORN_CORAL: BlockId = 1349;
pub const TUBE_CORAL: BlockId = 1350;
pub const BRAIN_CORAL: BlockId = 1351;
pub const BUBBLE_CORAL: BlockId = 1352;
pub const FIRE_CORAL: BlockId = 1353;
pub const HORN_CORAL: BlockId = 1354;
pub const DEAD_TUBE_CORAL_FAN: BlockId = 1355;
pub const DEAD_BRAIN_CORAL_FAN: BlockId = 1356;
pub const DEAD_BUBBLE_CORAL_FAN: BlockId = 1357;
pub const DEAD_FIRE_CORAL_FAN: BlockId = 1358;
pub const DEAD_HORN_CORAL_FAN: BlockId = 1359;
pub const TUBE_CORAL_FAN: BlockId = 1360;
pub const BRAIN_CORAL_FAN: BlockId = 1361;
pub const BUBBLE_CORAL_FAN: BlockId = 1362;
pub const FIRE_CORAL_FAN: BlockId = 1363;
pub const HORN_CORAL_FAN: BlockId = 1364;
pub const DEAD_TUBE_CORAL_WALL_FAN: BlockId = 1365;
pub const DEAD_BRAIN_CORAL_WALL_FAN: BlockId = 1369;
pub const DEAD_BUBBLE_CORAL_WALL_FAN: BlockId = 1373;
pub const DEAD_FIRE_CORAL_WALL_FAN: BlockId = 1377;
pub const DEAD_HORN_CORAL_WALL_FAN: BlockId = 1381;
pub const TUBE_CORAL_WALL_FAN: BlockId = 1385;
pub const BRAIN_CORAL_WALL_FAN: BlockId = 1389;
pub const BUBBLE_CORAL_WALL_FAN: BlockId = 1393;
pub const FIRE_CORAL_WALL_FAN: BlockId = 1397;
pub const HORN_CORAL_WALL_FAN: BlockId = 1401;
pub const SEA_PICKLE: BlockId = 1405;
pub const BLUE_ICE: BlockId = 1406;
pub const CONDUIT: BlockId = 1407;
pub const BAMBOO_SAPLING: BlockId = 1408;
pub const BAMBOO: BlockId = 1409;
pub const POTTED_BAMBOO: BlockId = 1410;
pub const VOID_AIR: BlockId = 1411;
pub const CAVE_AIR: BlockId = 1412;
pub const BUBBLE_COLUMN: BlockId = 1413;
pub const POLISHED_GRANITE_STAIRS: BlockId = 1414;
pub const SMOOTH_RED_SANDSTONE_STAIRS: BlockId = 1418;
pub const MOSSY_STONE_BRICK_STAIRS: BlockId = 1422;
pub const POLISHED_DIORITE_STAIRS: BlockId = 1426;
pub const MOSSY_COBBLESTONE_STAIRS: BlockId = 1430;
pub const END_STONE_BRICK_STAIRS: BlockId = 1434;
pub const STONE_STAIRS: BlockId = 1438;
pub const SMOOTH_SANDSTONE_STAIRS: BlockId = 1442;
pub const SMOOTH_QUARTZ_STAIRS: BlockId = 1446;
pub const GRANITE_STAIRS: BlockId = 1450;
pub const ANDESITE_STAIRS: BlockId = 1454;
pub const RED_NETHER_BRICK_STAIRS: BlockId = 1458;
pub const POLISHED_ANDESITE_STAIRS: BlockId = 1462;
pub const DIORITE_STAIRS: BlockId = 1466;
pub const POLISHED_GRANITE_SLAB: BlockId = 1470;
pub const SMOOTH_RED_SANDSTONE_SLAB: BlockId = 1471;
pub const MOSSY_STONE_BRICK_SLAB: BlockId = 1472;
pub const POLISHED_DIORITE_SLAB: BlockId = 1473;
pub const MOSSY_COBBLESTONE_SLAB: BlockId = 1474;
pub const END_STONE_BRICK_SLAB: BlockId = 1475;
pub const SMOOTH_SANDSTONE_SLAB: BlockId = 1476;
pub const SMOOTH_QUARTZ_SLAB: BlockId = 1477;
pub const ANDESITE_SLAB: BlockId = 1478;
pub const RED_NETHER_BRICK_SLAB: BlockId = 1479;
pub const POLISHED_ANDESITE_SLAB: BlockId = 1480;
pub const DIORITE_SLAB: BlockId = 1481;
pub const BRICK_WALL: BlockId = 1482;
pub const PRISMARINE_WALL: BlockId = 1483;
pub const RED_SANDSTONE_WALL: BlockId = 1484;
pub const MOSSY_STONE_BRICK_WALL: BlockId = 1485;
pub const GRANITE_WALL: BlockId = 1486;
pub const STONE_BRICK_WALL: BlockId = 1487;
pub const MUD_BRICK_WALL: BlockId = 1488;
pub const NETHER_BRICK_WALL: BlockId = 1489;
pub const ANDESITE_WALL: BlockId = 1490;
pub const RED_NETHER_BRICK_WALL: BlockId = 1491;
pub const SANDSTONE_WALL: BlockId = 1492;
pub const END_STONE_BRICK_WALL: BlockId = 1493;
pub const DIORITE_WALL: BlockId = 1494;
pub const SCAFFOLDING: BlockId = 1495;
pub const LOOM: BlockId = 1496;
pub const CARTOGRAPHY_TABLE: BlockId = 1497;
pub const FLETCHING_TABLE: BlockId = 1498;
pub const GRINDSTONE: BlockId = 1499;
pub const LECTERN: BlockId = 1500;
pub const SMITHING_TABLE: BlockId = 1501;
pub const BELL: BlockId = 1502;
pub const LANTERN: BlockId = 1503;
pub const SOUL_LANTERN: BlockId = 1504;
pub const CAMPFIRE: BlockId = 1505;
pub const PART6_FIRST: BlockId = 1330;
pub const PART6_LAST: BlockId = 1505;
pub const SOUL_CAMPFIRE: BlockId = 1506;
pub const SWEET_BERRY_BUSH: BlockId = 1507;
pub const WARPED_STEM: BlockId = 1508;
pub const STRIPPED_WARPED_STEM: BlockId = 1509;
pub const WARPED_HYPHAE: BlockId = 1510;
pub const STRIPPED_WARPED_HYPHAE: BlockId = 1511;
pub const WARPED_NYLIUM: BlockId = 1512;
pub const WARPED_FUNGUS: BlockId = 1513;
pub const WARPED_WART_BLOCK: BlockId = 1514;
pub const WARPED_ROOTS: BlockId = 1515;
pub const NETHER_SPROUTS: BlockId = 1516;
pub const CRIMSON_STEM: BlockId = 1517;
pub const STRIPPED_CRIMSON_STEM: BlockId = 1518;
pub const CRIMSON_HYPHAE: BlockId = 1519;
pub const STRIPPED_CRIMSON_HYPHAE: BlockId = 1520;
pub const CRIMSON_NYLIUM: BlockId = 1521;
pub const CRIMSON_FUNGUS: BlockId = 1522;
pub const SHROOMLIGHT: BlockId = 1523;
pub const WEEPING_VINES: BlockId = 1524;
pub const WEEPING_VINES_PLANT: BlockId = 1525;
pub const TWISTING_VINES: BlockId = 1526;
pub const TWISTING_VINES_PLANT: BlockId = 1527;
pub const CRIMSON_ROOTS: BlockId = 1528;
pub const CRIMSON_PLANKS: BlockId = 1529;
pub const WARPED_PLANKS: BlockId = 1530;
pub const CRIMSON_SLAB: BlockId = 1531;
pub const WARPED_SLAB: BlockId = 1532;
pub const CRIMSON_PRESSURE_PLATE: BlockId = 1533;
pub const WARPED_PRESSURE_PLATE: BlockId = 1534;
pub const CRIMSON_FENCE: BlockId = 1535;
pub const WARPED_FENCE: BlockId = 1536;
pub const CRIMSON_TRAPDOOR: BlockId = 1537;
pub const WARPED_TRAPDOOR: BlockId = 1542;
pub const CRIMSON_FENCE_GATE: BlockId = 1547;
pub const WARPED_FENCE_GATE: BlockId = 1555;
pub const CRIMSON_STAIRS: BlockId = 1563;
pub const WARPED_STAIRS: BlockId = 1567;
pub const CRIMSON_BUTTON: BlockId = 1571;
pub const WARPED_BUTTON: BlockId = 1575;
pub const CRIMSON_DOOR: BlockId = 1579;
pub const WARPED_DOOR: BlockId = 1595;
pub const CRIMSON_SIGN: BlockId = 1611;
pub const WARPED_SIGN: BlockId = 1615;
pub const CRIMSON_WALL_SIGN: BlockId = 1619;
pub const WARPED_WALL_SIGN: BlockId = 1623;
pub const STRUCTURE_BLOCK: BlockId = 1627;
pub const JIGSAW: BlockId = 1628;
pub const COMPOSTER: BlockId = 1629;
pub const TARGET: BlockId = 1630;
pub const BEE_NEST: BlockId = 1631;
pub const BEEHIVE: BlockId = 1632;
pub const HONEY_BLOCK: BlockId = 1633;
pub const HONEYCOMB_BLOCK: BlockId = 1634;
pub const NETHERITE_BLOCK: BlockId = 1635;
pub const ANCIENT_DEBRIS: BlockId = 1636;
pub const CRYING_OBSIDIAN: BlockId = 1637;
pub const RESPAWN_ANCHOR: BlockId = 1638;
pub const POTTED_CRIMSON_FUNGUS: BlockId = 1639;
pub const POTTED_WARPED_FUNGUS: BlockId = 1640;
pub const POTTED_CRIMSON_ROOTS: BlockId = 1641;
pub const POTTED_WARPED_ROOTS: BlockId = 1642;
pub const LODESTONE: BlockId = 1643;
pub const BLACKSTONE: BlockId = 1644;
pub const BLACKSTONE_STAIRS: BlockId = 1645;
pub const BLACKSTONE_WALL: BlockId = 1649;
pub const BLACKSTONE_SLAB: BlockId = 1650;
pub const POLISHED_BLACKSTONE: BlockId = 1651;
pub const POLISHED_BLACKSTONE_BRICKS: BlockId = 1652;
pub const CRACKED_POLISHED_BLACKSTONE_BRICKS: BlockId = 1653;
pub const CHISELED_POLISHED_BLACKSTONE: BlockId = 1654;
pub const POLISHED_BLACKSTONE_BRICK_SLAB: BlockId = 1655;
pub const POLISHED_BLACKSTONE_BRICK_STAIRS: BlockId = 1656;
pub const POLISHED_BLACKSTONE_BRICK_WALL: BlockId = 1660;
pub const GILDED_BLACKSTONE: BlockId = 1661;
pub const POLISHED_BLACKSTONE_STAIRS: BlockId = 1662;
pub const POLISHED_BLACKSTONE_SLAB: BlockId = 1666;
pub const POLISHED_BLACKSTONE_PRESSURE_PLATE: BlockId = 1667;
pub const POLISHED_BLACKSTONE_BUTTON: BlockId = 1668;
pub const POLISHED_BLACKSTONE_WALL: BlockId = 1672;
pub const CHISELED_NETHER_BRICKS: BlockId = 1673;
pub const CRACKED_NETHER_BRICKS: BlockId = 1674;
pub const QUARTZ_BRICKS: BlockId = 1675;
pub const CANDLE: BlockId = 1676;
pub const WHITE_CANDLE: BlockId = 1677;
pub const ORANGE_CANDLE: BlockId = 1678;
pub const MAGENTA_CANDLE: BlockId = 1679;
pub const LIGHT_BLUE_CANDLE: BlockId = 1680;
pub const YELLOW_CANDLE: BlockId = 1681;
pub const LIME_CANDLE: BlockId = 1682;
pub const PINK_CANDLE: BlockId = 1683;
pub const GRAY_CANDLE: BlockId = 1684;
pub const LIGHT_GRAY_CANDLE: BlockId = 1685;
pub const CYAN_CANDLE: BlockId = 1686;
pub const PURPLE_CANDLE: BlockId = 1687;
pub const BLUE_CANDLE: BlockId = 1688;
pub const BROWN_CANDLE: BlockId = 1689;
pub const GREEN_CANDLE: BlockId = 1690;
pub const RED_CANDLE: BlockId = 1691;
pub const BLACK_CANDLE: BlockId = 1692;
pub const CANDLE_CAKE: BlockId = 1693;
pub const WHITE_CANDLE_CAKE: BlockId = 1694;
pub const ORANGE_CANDLE_CAKE: BlockId = 1695;
pub const MAGENTA_CANDLE_CAKE: BlockId = 1696;
pub const LIGHT_BLUE_CANDLE_CAKE: BlockId = 1697;
pub const YELLOW_CANDLE_CAKE: BlockId = 1698;
pub const LIME_CANDLE_CAKE: BlockId = 1699;
pub const PINK_CANDLE_CAKE: BlockId = 1700;
pub const GRAY_CANDLE_CAKE: BlockId = 1701;
pub const LIGHT_GRAY_CANDLE_CAKE: BlockId = 1702;
pub const CYAN_CANDLE_CAKE: BlockId = 1703;
pub const PURPLE_CANDLE_CAKE: BlockId = 1704;
pub const BLUE_CANDLE_CAKE: BlockId = 1705;
pub const BROWN_CANDLE_CAKE: BlockId = 1706;
pub const GREEN_CANDLE_CAKE: BlockId = 1707;
pub const RED_CANDLE_CAKE: BlockId = 1708;
pub const BLACK_CANDLE_CAKE: BlockId = 1709;
pub const BUDDING_AMETHYST: BlockId = 1710;
pub const AMETHYST_CLUSTER: BlockId = 1711;
pub const LARGE_AMETHYST_BUD: BlockId = 1712;
pub const MEDIUM_AMETHYST_BUD: BlockId = 1713;
pub const SMALL_AMETHYST_BUD: BlockId = 1714;
pub const TINTED_GLASS: BlockId = 1715;
pub const POWDER_SNOW: BlockId = 1716;
pub const SCULK_SENSOR: BlockId = 1717;
pub const CALIBRATED_SCULK_SENSOR: BlockId = 1718;
pub const SCULK: BlockId = 1719;
pub const SCULK_VEIN: BlockId = 1720;
pub const SCULK_CATALYST: BlockId = 1721;
pub const SCULK_SHRIEKER: BlockId = 1722;
pub const OXIDIZED_COPPER: BlockId = 1723;
pub const WEATHERED_COPPER: BlockId = 1724;
pub const EXPOSED_COPPER: BlockId = 1725;
pub const DEEPSLATE_COPPER_ORE: BlockId = 1726;
pub const OXIDIZED_CUT_COPPER: BlockId = 1727;
pub const WEATHERED_CUT_COPPER: BlockId = 1728;
pub const EXPOSED_CUT_COPPER: BlockId = 1729;
pub const CUT_COPPER: BlockId = 1730;
pub const OXIDIZED_CUT_COPPER_STAIRS: BlockId = 1731;
pub const WEATHERED_CUT_COPPER_STAIRS: BlockId = 1735;
pub const EXPOSED_CUT_COPPER_STAIRS: BlockId = 1739;
pub const CUT_COPPER_STAIRS: BlockId = 1743;
pub const OXIDIZED_CUT_COPPER_SLAB: BlockId = 1747;
pub const WEATHERED_CUT_COPPER_SLAB: BlockId = 1748;
pub const EXPOSED_CUT_COPPER_SLAB: BlockId = 1749;
pub const CUT_COPPER_SLAB: BlockId = 1750;
pub const WAXED_COPPER_BLOCK: BlockId = 1751;
pub const WAXED_WEATHERED_COPPER: BlockId = 1752;
pub const WAXED_EXPOSED_COPPER: BlockId = 1753;
pub const WAXED_OXIDIZED_COPPER: BlockId = 1754;
pub const WAXED_OXIDIZED_CUT_COPPER: BlockId = 1755;
pub const WAXED_WEATHERED_CUT_COPPER: BlockId = 1756;
pub const WAXED_EXPOSED_CUT_COPPER: BlockId = 1757;
pub const WAXED_CUT_COPPER: BlockId = 1758;
pub const WAXED_OXIDIZED_CUT_COPPER_STAIRS: BlockId = 1759;
pub const WAXED_WEATHERED_CUT_COPPER_STAIRS: BlockId = 1763;
pub const WAXED_EXPOSED_CUT_COPPER_STAIRS: BlockId = 1767;
pub const WAXED_CUT_COPPER_STAIRS: BlockId = 1771;
pub const WAXED_OXIDIZED_CUT_COPPER_SLAB: BlockId = 1775;
pub const WAXED_WEATHERED_CUT_COPPER_SLAB: BlockId = 1776;
pub const WAXED_EXPOSED_CUT_COPPER_SLAB: BlockId = 1777;
pub const WAXED_CUT_COPPER_SLAB: BlockId = 1778;
pub const LIGHTNING_ROD: BlockId = 1779;
pub const POINTED_DRIPSTONE: BlockId = 1780;
pub const DRIPSTONE_BLOCK: BlockId = 1781;
pub const CAVE_VINES: BlockId = 1782;
pub const CAVE_VINES_PLANT: BlockId = 1783;
pub const SPORE_BLOSSOM: BlockId = 1784;
pub const AZALEA: BlockId = 1785;
pub const FLOWERING_AZALEA: BlockId = 1786;
pub const MOSS_CARPET: BlockId = 1787;
pub const PINK_PETALS: BlockId = 1788;
pub const BIG_DRIPLEAF: BlockId = 1789;
pub const BIG_DRIPLEAF_STEM: BlockId = 1790;
pub const SMALL_DRIPLEAF: BlockId = 1791;
pub const HANGING_ROOTS: BlockId = 1792;
pub const ROOTED_DIRT: BlockId = 1793;
pub const COBBLED_DEEPSLATE: BlockId = 1794;
pub const COBBLED_DEEPSLATE_STAIRS: BlockId = 1795;
pub const COBBLED_DEEPSLATE_SLAB: BlockId = 1799;
pub const COBBLED_DEEPSLATE_WALL: BlockId = 1800;
pub const POLISHED_DEEPSLATE: BlockId = 1801;
pub const POLISHED_DEEPSLATE_STAIRS: BlockId = 1802;
pub const POLISHED_DEEPSLATE_SLAB: BlockId = 1806;
pub const POLISHED_DEEPSLATE_WALL: BlockId = 1807;
pub const DEEPSLATE_TILES: BlockId = 1808;
pub const DEEPSLATE_TILE_STAIRS: BlockId = 1809;
pub const DEEPSLATE_TILE_SLAB: BlockId = 1813;
pub const DEEPSLATE_TILE_WALL: BlockId = 1814;
pub const DEEPSLATE_BRICKS: BlockId = 1815;
pub const DEEPSLATE_BRICK_STAIRS: BlockId = 1816;
pub const DEEPSLATE_BRICK_SLAB: BlockId = 1820;
pub const DEEPSLATE_BRICK_WALL: BlockId = 1821;
pub const CHISELED_DEEPSLATE: BlockId = 1822;
pub const CRACKED_DEEPSLATE_BRICKS: BlockId = 1823;
pub const CRACKED_DEEPSLATE_TILES: BlockId = 1824;
pub const INFESTED_DEEPSLATE: BlockId = 1825;
pub const SMOOTH_BASALT: BlockId = 1826;
pub const RAW_IRON_BLOCK: BlockId = 1827;
pub const RAW_COPPER_BLOCK: BlockId = 1828;
pub const RAW_GOLD_BLOCK: BlockId = 1829;
pub const POTTED_AZALEA_BUSH: BlockId = 1830;
pub const POTTED_FLOWERING_AZALEA_BUSH: BlockId = 1831;
pub const OCHRE_FROGLIGHT: BlockId = 1832;
pub const VERDANT_FROGLIGHT: BlockId = 1833;
pub const PEARLESCENT_FROGLIGHT: BlockId = 1834;
pub const FROGSPAWN: BlockId = 1835;
pub const REINFORCED_DEEPSLATE: BlockId = 1836;
pub const DECORATED_POT: BlockId = 1837;
pub const PART78_FIRST: BlockId = 1506;
pub const PART78_LAST: BlockId = 1837;
pub const REDSTONE_WIRE_ON: BlockId = 1838;
pub const REDSTONE_LAMP_LIT: BlockId = 1839;
pub const REDSTONE_TORCH_OFF: BlockId = 1840;
pub const REDSTONE_WALL_TORCH_OFF: BlockId = 1841;
pub const WHEAT_0: BlockId = 1845;
pub const WHEAT_1: BlockId = 1846;
pub const WHEAT_2: BlockId = 1847;
pub const CARROTS_0: BlockId = 1848;
pub const CARROTS_1: BlockId = 1849;
pub const CARROTS_2: BlockId = 1850;
pub const POTATOES_0: BlockId = 1851;
pub const POTATOES_1: BlockId = 1852;
pub const POTATOES_2: BlockId = 1853;
pub const BEETROOTS_0: BlockId = 1854;
pub const BEETROOTS_1: BlockId = 1855;
pub const BEETROOTS_2: BlockId = 1856;
pub const FARMLAND_MOIST: BlockId = 1857;
pub const BAMBOO_SMALL: BlockId = 1858;
pub const BAMBOO_LARGE: BlockId = 1859;
pub const KELP_SPROUT: BlockId = 1860;
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
pub const BLOCKS: [BlockDef; 1861] = [
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
    block(
        ATTACHED_PUMPKIN_STEM,
        "Attached Pumpkin Stem",
        "attached_pumpkin_stem",
        0.05,
        false,
        false,
    ),
    block(
        ATTACHED_MELON_STEM,
        "Attached Melon Stem",
        "attached_melon_stem",
        0.05,
        false,
        false,
    ),
    block(
        PUMPKIN_STEM,
        "Pumpkin Stem",
        "pumpkin_stem",
        0.05,
        false,
        false,
    ),
    block(MELON_STEM, "Melon Stem", "melon_stem", 0.05, false, false),
    block(VINE, "Vine", "vine", 0.05, false, false),
    BlockDef {
        light_emission: 7,
        ..block(
            GLOW_LICHEN,
            "Glow Lichen",
            "glow_lichen",
            0.05,
            false,
            false,
        )
    },
    block(
        OAK_FENCE_GATE,
        "Oak Fence Gate",
        "oak_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        OAK_FENCE_GATE + 1,
        "Oak Fence Gate (closed south)",
        "oak_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        OAK_FENCE_GATE + 2,
        "Oak Fence Gate (closed west)",
        "oak_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        OAK_FENCE_GATE + 3,
        "Oak Fence Gate (closed east)",
        "oak_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        OAK_FENCE_GATE + 4,
        "Oak Fence Gate (open north)",
        "oak_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        OAK_FENCE_GATE + 5,
        "Oak Fence Gate (open south)",
        "oak_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        OAK_FENCE_GATE + 6,
        "Oak Fence Gate (open west)",
        "oak_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        OAK_FENCE_GATE + 7,
        "Oak Fence Gate (open east)",
        "oak_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        BRICK_STAIRS,
        "Brick Stairs",
        "brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BRICK_STAIRS + 1,
        "Brick Stairs (south)",
        "brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BRICK_STAIRS + 2,
        "Brick Stairs (west)",
        "brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BRICK_STAIRS + 3,
        "Brick Stairs (east)",
        "brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        STONE_BRICK_STAIRS,
        "Stone Brick Stairs",
        "stone_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        STONE_BRICK_STAIRS + 1,
        "Stone Brick Stairs (south)",
        "stone_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        STONE_BRICK_STAIRS + 2,
        "Stone Brick Stairs (west)",
        "stone_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        STONE_BRICK_STAIRS + 3,
        "Stone Brick Stairs (east)",
        "stone_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        MUD_BRICK_STAIRS,
        "Mud Brick Stairs",
        "mud_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        MUD_BRICK_STAIRS + 1,
        "Mud Brick Stairs (south)",
        "mud_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        MUD_BRICK_STAIRS + 2,
        "Mud Brick Stairs (west)",
        "mud_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        MUD_BRICK_STAIRS + 3,
        "Mud Brick Stairs (east)",
        "mud_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(MYCELIUM, "Mycelium", "mycelium", 0.6, true, true),
    block(LILY_PAD, "Lily Pad", "lily_pad", 0.05, false, false),
    block(
        NETHER_BRICKS,
        "Nether Bricks",
        "nether_bricks",
        2.0,
        true,
        true,
    ),
    block(
        NETHER_BRICK_FENCE,
        "Nether Brick Fence",
        "nether_brick_fence",
        2.0,
        true,
        false,
    ),
    block(
        NETHER_BRICK_STAIRS,
        "Nether Brick Stairs",
        "nether_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        NETHER_BRICK_STAIRS + 1,
        "Nether Brick Stairs (south)",
        "nether_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        NETHER_BRICK_STAIRS + 2,
        "Nether Brick Stairs (west)",
        "nether_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        NETHER_BRICK_STAIRS + 3,
        "Nether Brick Stairs (east)",
        "nether_brick_stairs",
        2.0,
        true,
        false,
    ),
    block(
        NETHER_WART,
        "Nether Wart",
        "nether_wart",
        0.05,
        false,
        false,
    ),
    block(
        ENCHANTING_TABLE,
        "Enchanting Table",
        "enchanting_table",
        5.0,
        true,
        false,
    ),
    BlockDef {
        light_emission: 1,
        ..block(
            BREWING_STAND,
            "Brewing Stand",
            "brewing_stand",
            0.5,
            true,
            false,
        )
    },
    block(CAULDRON, "Cauldron", "cauldron", 2.0, true, false),
    block(
        WATER_CAULDRON,
        "Water Cauldron",
        "water_cauldron",
        2.0,
        true,
        false,
    ),
    BlockDef {
        light_emission: 15,
        ..block(
            LAVA_CAULDRON,
            "Lava Cauldron",
            "lava_cauldron",
            2.0,
            true,
            false,
        )
    },
    block(
        POWDER_SNOW_CAULDRON,
        "Powder Snow Cauldron",
        "powder_snow_cauldron",
        2.0,
        true,
        false,
    ),
    BlockDef {
        light_emission: 15,
        ..block(END_PORTAL, "End Portal", "end_portal", 0.5, false, false)
    },
    BlockDef {
        light_emission: 1,
        ..block(
            END_PORTAL_FRAME,
            "End Portal Frame",
            "end_portal_frame",
            5.0,
            true,
            false,
        )
    },
    block(END_STONE, "End Stone", "end_stone", 3.0, true, true),
    BlockDef {
        light_emission: 1,
        ..block(DRAGON_EGG, "Dragon Egg", "dragon_egg", 3.0, true, false)
    },
    BlockDef {
        light_emission: 15,
        ..block(
            REDSTONE_LAMP,
            "Redstone Lamp",
            "redstone_lamp",
            0.3,
            true,
            true,
        )
    },
    block(COCOA, "Cocoa", "cocoa", 0.2, false, false),
    block(
        SANDSTONE_STAIRS,
        "Sandstone Stairs",
        "sandstone_stairs",
        0.8,
        true,
        false,
    ),
    block(
        SANDSTONE_STAIRS + 1,
        "Sandstone Stairs (south)",
        "sandstone_stairs",
        0.8,
        true,
        false,
    ),
    block(
        SANDSTONE_STAIRS + 2,
        "Sandstone Stairs (west)",
        "sandstone_stairs",
        0.8,
        true,
        false,
    ),
    block(
        SANDSTONE_STAIRS + 3,
        "Sandstone Stairs (east)",
        "sandstone_stairs",
        0.8,
        true,
        false,
    ),
    block(
        DEEPSLATE_EMERALD_ORE,
        "Deepslate Emerald Ore",
        "deepslate_emerald_ore",
        4.5,
        true,
        true,
    ),
    BlockDef {
        light_emission: 7,
        ..block(ENDER_CHEST, "Ender Chest", "ender_chest", 5.0, true, false)
    },
    block(
        TRIPWIRE_HOOK,
        "Tripwire Hook",
        "tripwire_hook",
        0.05,
        false,
        false,
    ),
    block(
        TRIPWIRE_HOOK + 1,
        "Tripwire Hook (south)",
        "tripwire_hook",
        0.05,
        false,
        false,
    ),
    block(
        TRIPWIRE_HOOK + 2,
        "Tripwire Hook (west)",
        "tripwire_hook",
        0.05,
        false,
        false,
    ),
    block(
        TRIPWIRE_HOOK + 3,
        "Tripwire Hook (east)",
        "tripwire_hook",
        0.05,
        false,
        false,
    ),
    block(TRIPWIRE, "Tripwire", "tripwire", 0.05, false, false),
    block(
        SPRUCE_STAIRS,
        "Spruce Stairs",
        "spruce_stairs",
        2.0,
        true,
        false,
    ),
    block(
        SPRUCE_STAIRS + 1,
        "Spruce Stairs (south)",
        "spruce_stairs",
        2.0,
        true,
        false,
    ),
    block(
        SPRUCE_STAIRS + 2,
        "Spruce Stairs (west)",
        "spruce_stairs",
        2.0,
        true,
        false,
    ),
    block(
        SPRUCE_STAIRS + 3,
        "Spruce Stairs (east)",
        "spruce_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BIRCH_STAIRS,
        "Birch Stairs",
        "birch_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BIRCH_STAIRS + 1,
        "Birch Stairs (south)",
        "birch_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BIRCH_STAIRS + 2,
        "Birch Stairs (west)",
        "birch_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BIRCH_STAIRS + 3,
        "Birch Stairs (east)",
        "birch_stairs",
        2.0,
        true,
        false,
    ),
    block(
        JUNGLE_STAIRS,
        "Jungle Stairs",
        "jungle_stairs",
        2.0,
        true,
        false,
    ),
    block(
        JUNGLE_STAIRS + 1,
        "Jungle Stairs (south)",
        "jungle_stairs",
        2.0,
        true,
        false,
    ),
    block(
        JUNGLE_STAIRS + 2,
        "Jungle Stairs (west)",
        "jungle_stairs",
        2.0,
        true,
        false,
    ),
    block(
        JUNGLE_STAIRS + 3,
        "Jungle Stairs (east)",
        "jungle_stairs",
        2.0,
        true,
        false,
    ),
    block(
        COMMAND_BLOCK,
        "Command Block",
        "command_block",
        5.0,
        true,
        true,
    ),
    BlockDef {
        light_emission: 15,
        ..block(BEACON, "Beacon", "beacon", 3.0, true, false)
    },
    block(
        COBBLESTONE_WALL,
        "Cobblestone Wall",
        "cobblestone_wall",
        2.0,
        true,
        false,
    ),
    block(
        MOSSY_COBBLESTONE_WALL,
        "Mossy Cobblestone Wall",
        "mossy_cobblestone_wall",
        2.0,
        true,
        false,
    ),
    block(FLOWER_POT, "Flower Pot", "flower_pot", 0.0, true, false),
    block(
        POTTED_TORCHFLOWER,
        "Potted Torchflower",
        "potted_torchflower",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_OAK_SAPLING,
        "Potted Oak Sapling",
        "potted_oak_sapling",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_SPRUCE_SAPLING,
        "Potted Spruce Sapling",
        "potted_spruce_sapling",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_BIRCH_SAPLING,
        "Potted Birch Sapling",
        "potted_birch_sapling",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_JUNGLE_SAPLING,
        "Potted Jungle Sapling",
        "potted_jungle_sapling",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_ACACIA_SAPLING,
        "Potted Acacia Sapling",
        "potted_acacia_sapling",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_CHERRY_SAPLING,
        "Potted Cherry Sapling",
        "potted_cherry_sapling",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_DARK_OAK_SAPLING,
        "Potted Dark Oak Sapling",
        "potted_dark_oak_sapling",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_MANGROVE_PROPAGULE,
        "Potted Mangrove Propagule",
        "potted_mangrove_propagule",
        0.0,
        true,
        false,
    ),
    block(POTTED_FERN, "Potted Fern", "potted_fern", 0.0, true, false),
    block(
        POTTED_DANDELION,
        "Potted Dandelion",
        "potted_dandelion",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_POPPY,
        "Potted Poppy",
        "potted_poppy",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_BLUE_ORCHID,
        "Potted Blue Orchid",
        "potted_blue_orchid",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_ALLIUM,
        "Potted Allium",
        "potted_allium",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_AZURE_BLUET,
        "Potted Azure Bluet",
        "potted_azure_bluet",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_RED_TULIP,
        "Potted Red Tulip",
        "potted_red_tulip",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_ORANGE_TULIP,
        "Potted Orange Tulip",
        "potted_orange_tulip",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_WHITE_TULIP,
        "Potted White Tulip",
        "potted_white_tulip",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_PINK_TULIP,
        "Potted Pink Tulip",
        "potted_pink_tulip",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_OXEYE_DAISY,
        "Potted Oxeye Daisy",
        "potted_oxeye_daisy",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_CORNFLOWER,
        "Potted Cornflower",
        "potted_cornflower",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_LILY_OF_THE_VALLEY,
        "Potted Lily Of The Valley",
        "potted_lily_of_the_valley",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_WITHER_ROSE,
        "Potted Wither Rose",
        "potted_wither_rose",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_RED_MUSHROOM,
        "Potted Red Mushroom",
        "potted_red_mushroom",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_BROWN_MUSHROOM,
        "Potted Brown Mushroom",
        "potted_brown_mushroom",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_DEAD_BUSH,
        "Potted Dead Bush",
        "potted_dead_bush",
        0.0,
        true,
        false,
    ),
    block(
        POTTED_CACTUS,
        "Potted Cactus",
        "potted_cactus",
        0.0,
        true,
        false,
    ),
    block(CARROTS, "Carrots", "carrots", 0.05, false, false),
    block(POTATOES, "Potatoes", "potatoes", 0.05, false, false),
    block(OAK_BUTTON, "Oak Button", "oak_button", 0.5, false, false),
    block(
        OAK_BUTTON + 1,
        "Oak Button (south)",
        "oak_button",
        0.5,
        false,
        false,
    ),
    block(
        OAK_BUTTON + 2,
        "Oak Button (west)",
        "oak_button",
        0.5,
        false,
        false,
    ),
    block(
        OAK_BUTTON + 3,
        "Oak Button (east)",
        "oak_button",
        0.5,
        false,
        false,
    ),
    block(
        SPRUCE_BUTTON,
        "Spruce Button",
        "spruce_button",
        0.5,
        false,
        false,
    ),
    block(
        SPRUCE_BUTTON + 1,
        "Spruce Button (south)",
        "spruce_button",
        0.5,
        false,
        false,
    ),
    block(
        SPRUCE_BUTTON + 2,
        "Spruce Button (west)",
        "spruce_button",
        0.5,
        false,
        false,
    ),
    block(
        SPRUCE_BUTTON + 3,
        "Spruce Button (east)",
        "spruce_button",
        0.5,
        false,
        false,
    ),
    block(
        BIRCH_BUTTON,
        "Birch Button",
        "birch_button",
        0.5,
        false,
        false,
    ),
    block(
        BIRCH_BUTTON + 1,
        "Birch Button (south)",
        "birch_button",
        0.5,
        false,
        false,
    ),
    block(
        BIRCH_BUTTON + 2,
        "Birch Button (west)",
        "birch_button",
        0.5,
        false,
        false,
    ),
    block(
        BIRCH_BUTTON + 3,
        "Birch Button (east)",
        "birch_button",
        0.5,
        false,
        false,
    ),
    block(
        JUNGLE_BUTTON,
        "Jungle Button",
        "jungle_button",
        0.5,
        false,
        false,
    ),
    block(
        JUNGLE_BUTTON + 1,
        "Jungle Button (south)",
        "jungle_button",
        0.5,
        false,
        false,
    ),
    block(
        JUNGLE_BUTTON + 2,
        "Jungle Button (west)",
        "jungle_button",
        0.5,
        false,
        false,
    ),
    block(
        JUNGLE_BUTTON + 3,
        "Jungle Button (east)",
        "jungle_button",
        0.5,
        false,
        false,
    ),
    block(
        ACACIA_BUTTON,
        "Acacia Button",
        "acacia_button",
        0.5,
        false,
        false,
    ),
    block(
        ACACIA_BUTTON + 1,
        "Acacia Button (south)",
        "acacia_button",
        0.5,
        false,
        false,
    ),
    block(
        ACACIA_BUTTON + 2,
        "Acacia Button (west)",
        "acacia_button",
        0.5,
        false,
        false,
    ),
    block(
        ACACIA_BUTTON + 3,
        "Acacia Button (east)",
        "acacia_button",
        0.5,
        false,
        false,
    ),
    block(
        CHERRY_BUTTON,
        "Cherry Button",
        "cherry_button",
        0.5,
        false,
        false,
    ),
    block(
        CHERRY_BUTTON + 1,
        "Cherry Button (south)",
        "cherry_button",
        0.5,
        false,
        false,
    ),
    block(
        CHERRY_BUTTON + 2,
        "Cherry Button (west)",
        "cherry_button",
        0.5,
        false,
        false,
    ),
    block(
        CHERRY_BUTTON + 3,
        "Cherry Button (east)",
        "cherry_button",
        0.5,
        false,
        false,
    ),
    block(
        DARK_OAK_BUTTON,
        "Dark Oak Button",
        "dark_oak_button",
        0.5,
        false,
        false,
    ),
    block(
        DARK_OAK_BUTTON + 1,
        "Dark Oak Button (south)",
        "dark_oak_button",
        0.5,
        false,
        false,
    ),
    block(
        DARK_OAK_BUTTON + 2,
        "Dark Oak Button (west)",
        "dark_oak_button",
        0.5,
        false,
        false,
    ),
    block(
        DARK_OAK_BUTTON + 3,
        "Dark Oak Button (east)",
        "dark_oak_button",
        0.5,
        false,
        false,
    ),
    block(
        MANGROVE_BUTTON,
        "Mangrove Button",
        "mangrove_button",
        0.5,
        false,
        false,
    ),
    block(
        MANGROVE_BUTTON + 1,
        "Mangrove Button (south)",
        "mangrove_button",
        0.5,
        false,
        false,
    ),
    block(
        MANGROVE_BUTTON + 2,
        "Mangrove Button (west)",
        "mangrove_button",
        0.5,
        false,
        false,
    ),
    block(
        MANGROVE_BUTTON + 3,
        "Mangrove Button (east)",
        "mangrove_button",
        0.5,
        false,
        false,
    ),
    block(
        BAMBOO_BUTTON,
        "Bamboo Button",
        "bamboo_button",
        0.5,
        false,
        false,
    ),
    block(
        BAMBOO_BUTTON + 1,
        "Bamboo Button (south)",
        "bamboo_button",
        0.5,
        false,
        false,
    ),
    block(
        BAMBOO_BUTTON + 2,
        "Bamboo Button (west)",
        "bamboo_button",
        0.5,
        false,
        false,
    ),
    block(
        BAMBOO_BUTTON + 3,
        "Bamboo Button (east)",
        "bamboo_button",
        0.5,
        false,
        false,
    ),
    block(
        SKELETON_SKULL,
        "Skeleton Skull",
        "skeleton_skull",
        1.0,
        true,
        false,
    ),
    block(
        SKELETON_WALL_SKULL,
        "Skeleton Wall Skull",
        "skeleton_wall_skull",
        1.0,
        true,
        false,
    ),
    block(
        SKELETON_WALL_SKULL + 1,
        "Skeleton Wall Skull (south)",
        "skeleton_wall_skull",
        1.0,
        true,
        false,
    ),
    block(
        SKELETON_WALL_SKULL + 2,
        "Skeleton Wall Skull (west)",
        "skeleton_wall_skull",
        1.0,
        true,
        false,
    ),
    block(
        SKELETON_WALL_SKULL + 3,
        "Skeleton Wall Skull (east)",
        "skeleton_wall_skull",
        1.0,
        true,
        false,
    ),
    block(
        WITHER_SKELETON_SKULL,
        "Wither Skeleton Skull",
        "wither_skeleton_skull",
        1.0,
        true,
        false,
    ),
    block(
        WITHER_SKELETON_WALL_SKULL,
        "Wither Skeleton Wall Skull",
        "wither_skeleton_wall_skull",
        1.0,
        true,
        false,
    ),
    block(
        WITHER_SKELETON_WALL_SKULL + 1,
        "Wither Skeleton Wall Skull (south)",
        "wither_skeleton_wall_skull",
        1.0,
        true,
        false,
    ),
    block(
        WITHER_SKELETON_WALL_SKULL + 2,
        "Wither Skeleton Wall Skull (west)",
        "wither_skeleton_wall_skull",
        1.0,
        true,
        false,
    ),
    block(
        WITHER_SKELETON_WALL_SKULL + 3,
        "Wither Skeleton Wall Skull (east)",
        "wither_skeleton_wall_skull",
        1.0,
        true,
        false,
    ),
    block(ZOMBIE_HEAD, "Zombie Head", "zombie_head", 1.0, true, false),
    block(
        ZOMBIE_WALL_HEAD,
        "Zombie Wall Head",
        "zombie_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        ZOMBIE_WALL_HEAD + 1,
        "Zombie Wall Head (south)",
        "zombie_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        ZOMBIE_WALL_HEAD + 2,
        "Zombie Wall Head (west)",
        "zombie_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        ZOMBIE_WALL_HEAD + 3,
        "Zombie Wall Head (east)",
        "zombie_wall_head",
        1.0,
        true,
        false,
    ),
    block(PLAYER_HEAD, "Player Head", "player_head", 1.0, true, false),
    block(
        PLAYER_WALL_HEAD,
        "Player Wall Head",
        "player_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        PLAYER_WALL_HEAD + 1,
        "Player Wall Head (south)",
        "player_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        PLAYER_WALL_HEAD + 2,
        "Player Wall Head (west)",
        "player_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        PLAYER_WALL_HEAD + 3,
        "Player Wall Head (east)",
        "player_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        CREEPER_HEAD,
        "Creeper Head",
        "creeper_head",
        1.0,
        true,
        false,
    ),
    block(
        CREEPER_WALL_HEAD,
        "Creeper Wall Head",
        "creeper_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        CREEPER_WALL_HEAD + 1,
        "Creeper Wall Head (south)",
        "creeper_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        CREEPER_WALL_HEAD + 2,
        "Creeper Wall Head (west)",
        "creeper_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        CREEPER_WALL_HEAD + 3,
        "Creeper Wall Head (east)",
        "creeper_wall_head",
        1.0,
        true,
        false,
    ),
    block(DRAGON_HEAD, "Dragon Head", "dragon_head", 1.0, true, false),
    block(
        DRAGON_WALL_HEAD,
        "Dragon Wall Head",
        "dragon_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        DRAGON_WALL_HEAD + 1,
        "Dragon Wall Head (south)",
        "dragon_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        DRAGON_WALL_HEAD + 2,
        "Dragon Wall Head (west)",
        "dragon_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        DRAGON_WALL_HEAD + 3,
        "Dragon Wall Head (east)",
        "dragon_wall_head",
        1.0,
        true,
        false,
    ),
    block(PIGLIN_HEAD, "Piglin Head", "piglin_head", 1.0, true, false),
    block(
        PIGLIN_WALL_HEAD,
        "Piglin Wall Head",
        "piglin_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        PIGLIN_WALL_HEAD + 1,
        "Piglin Wall Head (south)",
        "piglin_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        PIGLIN_WALL_HEAD + 2,
        "Piglin Wall Head (west)",
        "piglin_wall_head",
        1.0,
        true,
        false,
    ),
    block(
        PIGLIN_WALL_HEAD + 3,
        "Piglin Wall Head (east)",
        "piglin_wall_head",
        1.0,
        true,
        false,
    ),
    block(ANVIL, "Anvil", "anvil", 5.0, true, false),
    block(ANVIL + 1, "Anvil (z)", "anvil", 5.0, true, false),
    block(
        CHIPPED_ANVIL,
        "Chipped Anvil",
        "chipped_anvil",
        5.0,
        true,
        false,
    ),
    block(
        CHIPPED_ANVIL + 1,
        "Chipped Anvil (z)",
        "chipped_anvil",
        5.0,
        true,
        false,
    ),
    block(
        DAMAGED_ANVIL,
        "Damaged Anvil",
        "damaged_anvil",
        5.0,
        true,
        false,
    ),
    block(
        DAMAGED_ANVIL + 1,
        "Damaged Anvil (z)",
        "damaged_anvil",
        5.0,
        true,
        false,
    ),
    block(
        TRAPPED_CHEST,
        "Trapped Chest",
        "trapped_chest",
        2.5,
        true,
        true,
    ),
    block(
        LIGHT_WEIGHTED_PRESSURE_PLATE,
        "Light Weighted Pressure Plate",
        "light_weighted_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        HEAVY_WEIGHTED_PRESSURE_PLATE,
        "Heavy Weighted Pressure Plate",
        "heavy_weighted_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(COMPARATOR, "Comparator", "comparator", 0.05, false, false),
    block(
        COMPARATOR + 1,
        "Comparator (south)",
        "comparator",
        0.05,
        false,
        false,
    ),
    block(
        COMPARATOR + 2,
        "Comparator (west)",
        "comparator",
        0.05,
        false,
        false,
    ),
    block(
        COMPARATOR + 3,
        "Comparator (east)",
        "comparator",
        0.05,
        false,
        false,
    ),
    block(
        DAYLIGHT_DETECTOR,
        "Daylight Detector",
        "daylight_detector",
        0.2,
        true,
        false,
    ),
    block(
        REDSTONE_BLOCK,
        "Block of Redstone",
        "redstone_block",
        5.0,
        true,
        true,
    ),
    block(
        NETHER_QUARTZ_ORE,
        "Nether Quartz Ore",
        "nether_quartz_ore",
        3.0,
        true,
        true,
    ),
    block(HOPPER, "Hopper", "hopper", 3.0, true, false),
    block(
        CHISELED_QUARTZ_BLOCK,
        "Chiseled Quartz Block",
        "chiseled_quartz_block",
        0.8,
        true,
        true,
    ),
    block(
        QUARTZ_PILLAR,
        "Quartz Pillar",
        "quartz_pillar",
        0.8,
        true,
        true,
    ),
    block(
        QUARTZ_STAIRS,
        "Quartz Stairs",
        "quartz_stairs",
        0.8,
        true,
        false,
    ),
    block(
        QUARTZ_STAIRS + 1,
        "Quartz Stairs (south)",
        "quartz_stairs",
        0.8,
        true,
        false,
    ),
    block(
        QUARTZ_STAIRS + 2,
        "Quartz Stairs (west)",
        "quartz_stairs",
        0.8,
        true,
        false,
    ),
    block(
        QUARTZ_STAIRS + 3,
        "Quartz Stairs (east)",
        "quartz_stairs",
        0.8,
        true,
        false,
    ),
    block(
        ACTIVATOR_RAIL,
        "Activator Rail",
        "activator_rail",
        0.7,
        false,
        false,
    ),
    block(
        ACTIVATOR_RAIL + 1,
        "Activator Rail (x)",
        "activator_rail",
        0.7,
        false,
        false,
    ),
    block(DROPPER, "Dropper", "dropper", 3.5, true, true),
    block(DROPPER + 1, "Dropper (south)", "dropper", 3.5, true, true),
    block(DROPPER + 2, "Dropper (west)", "dropper", 3.5, true, true),
    block(DROPPER + 3, "Dropper (east)", "dropper", 3.5, true, true),
    block(DROPPER + 4, "Dropper (up)", "dropper", 3.5, true, true),
    block(DROPPER + 5, "Dropper (down)", "dropper", 3.5, true, true),
    block(
        WHITE_STAINED_GLASS_PANE,
        "White Stained Glass Pane",
        "white_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        ORANGE_STAINED_GLASS_PANE,
        "Orange Stained Glass Pane",
        "orange_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        MAGENTA_STAINED_GLASS_PANE,
        "Magenta Stained Glass Pane",
        "magenta_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        LIGHT_BLUE_STAINED_GLASS_PANE,
        "Light Blue Stained Glass Pane",
        "light_blue_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        YELLOW_STAINED_GLASS_PANE,
        "Yellow Stained Glass Pane",
        "yellow_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        LIME_STAINED_GLASS_PANE,
        "Lime Stained Glass Pane",
        "lime_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        PINK_STAINED_GLASS_PANE,
        "Pink Stained Glass Pane",
        "pink_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        GRAY_STAINED_GLASS_PANE,
        "Gray Stained Glass Pane",
        "gray_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        LIGHT_GRAY_STAINED_GLASS_PANE,
        "Light Gray Stained Glass Pane",
        "light_gray_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        CYAN_STAINED_GLASS_PANE,
        "Cyan Stained Glass Pane",
        "cyan_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        PURPLE_STAINED_GLASS_PANE,
        "Purple Stained Glass Pane",
        "purple_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        BLUE_STAINED_GLASS_PANE,
        "Blue Stained Glass Pane",
        "blue_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        BROWN_STAINED_GLASS_PANE,
        "Brown Stained Glass Pane",
        "brown_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        GREEN_STAINED_GLASS_PANE,
        "Green Stained Glass Pane",
        "green_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        RED_STAINED_GLASS_PANE,
        "Red Stained Glass Pane",
        "red_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        BLACK_STAINED_GLASS_PANE,
        "Black Stained Glass Pane",
        "black_stained_glass_pane",
        0.3,
        true,
        false,
    ),
    block(
        ACACIA_STAIRS,
        "Acacia Stairs",
        "acacia_stairs",
        2.0,
        true,
        false,
    ),
    block(
        ACACIA_STAIRS + 1,
        "Acacia Stairs (south)",
        "acacia_stairs",
        2.0,
        true,
        false,
    ),
    block(
        ACACIA_STAIRS + 2,
        "Acacia Stairs (west)",
        "acacia_stairs",
        2.0,
        true,
        false,
    ),
    block(
        ACACIA_STAIRS + 3,
        "Acacia Stairs (east)",
        "acacia_stairs",
        2.0,
        true,
        false,
    ),
    block(
        CHERRY_STAIRS,
        "Cherry Stairs",
        "cherry_stairs",
        2.0,
        true,
        false,
    ),
    block(
        CHERRY_STAIRS + 1,
        "Cherry Stairs (south)",
        "cherry_stairs",
        2.0,
        true,
        false,
    ),
    block(
        CHERRY_STAIRS + 2,
        "Cherry Stairs (west)",
        "cherry_stairs",
        2.0,
        true,
        false,
    ),
    block(
        CHERRY_STAIRS + 3,
        "Cherry Stairs (east)",
        "cherry_stairs",
        2.0,
        true,
        false,
    ),
    block(
        DARK_OAK_STAIRS,
        "Dark Oak Stairs",
        "dark_oak_stairs",
        2.0,
        true,
        false,
    ),
    block(
        DARK_OAK_STAIRS + 1,
        "Dark Oak Stairs (south)",
        "dark_oak_stairs",
        2.0,
        true,
        false,
    ),
    block(
        DARK_OAK_STAIRS + 2,
        "Dark Oak Stairs (west)",
        "dark_oak_stairs",
        2.0,
        true,
        false,
    ),
    block(
        DARK_OAK_STAIRS + 3,
        "Dark Oak Stairs (east)",
        "dark_oak_stairs",
        2.0,
        true,
        false,
    ),
    block(
        MANGROVE_STAIRS,
        "Mangrove Stairs",
        "mangrove_stairs",
        2.0,
        true,
        false,
    ),
    block(
        MANGROVE_STAIRS + 1,
        "Mangrove Stairs (south)",
        "mangrove_stairs",
        2.0,
        true,
        false,
    ),
    block(
        MANGROVE_STAIRS + 2,
        "Mangrove Stairs (west)",
        "mangrove_stairs",
        2.0,
        true,
        false,
    ),
    block(
        MANGROVE_STAIRS + 3,
        "Mangrove Stairs (east)",
        "mangrove_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_STAIRS,
        "Bamboo Stairs",
        "bamboo_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_STAIRS + 1,
        "Bamboo Stairs (south)",
        "bamboo_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_STAIRS + 2,
        "Bamboo Stairs (west)",
        "bamboo_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_STAIRS + 3,
        "Bamboo Stairs (east)",
        "bamboo_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_MOSAIC_STAIRS,
        "Bamboo Mosaic Stairs",
        "bamboo_mosaic_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_MOSAIC_STAIRS + 1,
        "Bamboo Mosaic Stairs (south)",
        "bamboo_mosaic_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_MOSAIC_STAIRS + 2,
        "Bamboo Mosaic Stairs (west)",
        "bamboo_mosaic_stairs",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_MOSAIC_STAIRS + 3,
        "Bamboo Mosaic Stairs (east)",
        "bamboo_mosaic_stairs",
        2.0,
        true,
        false,
    ),
    block(SLIME_BLOCK, "Slime Block", "slime_block", 0.2, true, true),
    block(BARRIER, "Barrier", "barrier", 0.05, true, false),
    BlockDef {
        light_emission: 15,
        ..block(LIGHT, "Light", "light", 0.05, false, false)
    },
    block(
        IRON_TRAPDOOR,
        "Iron Trapdoor",
        "iron_trapdoor",
        5.0,
        true,
        false,
    ),
    block(
        IRON_TRAPDOOR + 1,
        "Iron Trapdoor (open north)",
        "iron_trapdoor",
        5.0,
        false,
        false,
    ),
    block(
        IRON_TRAPDOOR + 2,
        "Iron Trapdoor (open south)",
        "iron_trapdoor",
        5.0,
        false,
        false,
    ),
    block(
        IRON_TRAPDOOR + 3,
        "Iron Trapdoor (open west)",
        "iron_trapdoor",
        5.0,
        false,
        false,
    ),
    block(
        IRON_TRAPDOOR + 4,
        "Iron Trapdoor (open east)",
        "iron_trapdoor",
        5.0,
        false,
        false,
    ),
    block(PRISMARINE, "Prismarine", "prismarine", 1.5, true, true),
    block(
        PRISMARINE_BRICKS,
        "Prismarine Bricks",
        "prismarine_bricks",
        1.5,
        true,
        true,
    ),
    block(
        DARK_PRISMARINE,
        "Dark Prismarine",
        "dark_prismarine",
        1.5,
        true,
        true,
    ),
    block(
        PRISMARINE_STAIRS,
        "Prismarine Stairs",
        "prismarine_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PRISMARINE_STAIRS + 1,
        "Prismarine Stairs (south)",
        "prismarine_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PRISMARINE_STAIRS + 2,
        "Prismarine Stairs (west)",
        "prismarine_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PRISMARINE_STAIRS + 3,
        "Prismarine Stairs (east)",
        "prismarine_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PRISMARINE_BRICK_STAIRS,
        "Prismarine Brick Stairs",
        "prismarine_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PRISMARINE_BRICK_STAIRS + 1,
        "Prismarine Brick Stairs (south)",
        "prismarine_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PRISMARINE_BRICK_STAIRS + 2,
        "Prismarine Brick Stairs (west)",
        "prismarine_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PRISMARINE_BRICK_STAIRS + 3,
        "Prismarine Brick Stairs (east)",
        "prismarine_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        DARK_PRISMARINE_STAIRS,
        "Dark Prismarine Stairs",
        "dark_prismarine_stairs",
        1.5,
        true,
        false,
    ),
    block(
        DARK_PRISMARINE_STAIRS + 1,
        "Dark Prismarine Stairs (south)",
        "dark_prismarine_stairs",
        1.5,
        true,
        false,
    ),
    block(
        DARK_PRISMARINE_STAIRS + 2,
        "Dark Prismarine Stairs (west)",
        "dark_prismarine_stairs",
        1.5,
        true,
        false,
    ),
    block(
        DARK_PRISMARINE_STAIRS + 3,
        "Dark Prismarine Stairs (east)",
        "dark_prismarine_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PRISMARINE_SLAB,
        "Prismarine Slab",
        "prismarine_slab",
        1.5,
        true,
        false,
    ),
    block(
        PRISMARINE_BRICK_SLAB,
        "Prismarine Brick Slab",
        "prismarine_brick_slab",
        1.5,
        true,
        false,
    ),
    block(
        DARK_PRISMARINE_SLAB,
        "Dark Prismarine Slab",
        "dark_prismarine_slab",
        1.5,
        true,
        false,
    ),
    BlockDef {
        light_emission: 15,
        ..block(SEA_LANTERN, "Sea Lantern", "sea_lantern", 0.3, true, true)
    },
    block(
        WHITE_CARPET,
        "White Carpet",
        "white_carpet",
        0.1,
        false,
        false,
    ),
    block(
        ORANGE_CARPET,
        "Orange Carpet",
        "orange_carpet",
        0.1,
        false,
        false,
    ),
    block(
        MAGENTA_CARPET,
        "Magenta Carpet",
        "magenta_carpet",
        0.1,
        false,
        false,
    ),
    block(
        LIGHT_BLUE_CARPET,
        "Light Blue Carpet",
        "light_blue_carpet",
        0.1,
        false,
        false,
    ),
    block(
        YELLOW_CARPET,
        "Yellow Carpet",
        "yellow_carpet",
        0.1,
        false,
        false,
    ),
    block(LIME_CARPET, "Lime Carpet", "lime_carpet", 0.1, false, false),
    block(PINK_CARPET, "Pink Carpet", "pink_carpet", 0.1, false, false),
    block(GRAY_CARPET, "Gray Carpet", "gray_carpet", 0.1, false, false),
    block(
        LIGHT_GRAY_CARPET,
        "Light Gray Carpet",
        "light_gray_carpet",
        0.1,
        false,
        false,
    ),
    block(CYAN_CARPET, "Cyan Carpet", "cyan_carpet", 0.1, false, false),
    block(
        PURPLE_CARPET,
        "Purple Carpet",
        "purple_carpet",
        0.1,
        false,
        false,
    ),
    block(BLUE_CARPET, "Blue Carpet", "blue_carpet", 0.1, false, false),
    block(
        BROWN_CARPET,
        "Brown Carpet",
        "brown_carpet",
        0.1,
        false,
        false,
    ),
    block(
        GREEN_CARPET,
        "Green Carpet",
        "green_carpet",
        0.1,
        false,
        false,
    ),
    block(RED_CARPET, "Red Carpet", "red_carpet", 0.1, false, false),
    block(
        BLACK_CARPET,
        "Black Carpet",
        "black_carpet",
        0.1,
        false,
        false,
    ),
    block(PACKED_ICE, "Packed Ice", "packed_ice", 0.5, true, true),
    block(SUNFLOWER, "Sunflower", "sunflower", 0.05, false, false),
    block(LILAC, "Lilac", "lilac", 0.05, false, false),
    block(ROSE_BUSH, "Rose Bush", "rose_bush", 0.05, false, false),
    block(PEONY, "Peony", "peony", 0.05, false, false),
    block(LARGE_FERN, "Large Fern", "large_fern", 0.05, false, false),
    block(
        WHITE_BANNER,
        "White Banner",
        "white_banner",
        1.0,
        false,
        false,
    ),
    block(
        WHITE_BANNER + 1,
        "White Banner (south)",
        "white_banner",
        1.0,
        false,
        false,
    ),
    block(
        WHITE_BANNER + 2,
        "White Banner (west)",
        "white_banner",
        1.0,
        false,
        false,
    ),
    block(
        WHITE_BANNER + 3,
        "White Banner (east)",
        "white_banner",
        1.0,
        false,
        false,
    ),
    block(
        ORANGE_BANNER,
        "Orange Banner",
        "orange_banner",
        1.0,
        false,
        false,
    ),
    block(
        ORANGE_BANNER + 1,
        "Orange Banner (south)",
        "orange_banner",
        1.0,
        false,
        false,
    ),
    block(
        ORANGE_BANNER + 2,
        "Orange Banner (west)",
        "orange_banner",
        1.0,
        false,
        false,
    ),
    block(
        ORANGE_BANNER + 3,
        "Orange Banner (east)",
        "orange_banner",
        1.0,
        false,
        false,
    ),
    block(
        MAGENTA_BANNER,
        "Magenta Banner",
        "magenta_banner",
        1.0,
        false,
        false,
    ),
    block(
        MAGENTA_BANNER + 1,
        "Magenta Banner (south)",
        "magenta_banner",
        1.0,
        false,
        false,
    ),
    block(
        MAGENTA_BANNER + 2,
        "Magenta Banner (west)",
        "magenta_banner",
        1.0,
        false,
        false,
    ),
    block(
        MAGENTA_BANNER + 3,
        "Magenta Banner (east)",
        "magenta_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_BLUE_BANNER,
        "Light Blue Banner",
        "light_blue_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_BLUE_BANNER + 1,
        "Light Blue Banner (south)",
        "light_blue_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_BLUE_BANNER + 2,
        "Light Blue Banner (west)",
        "light_blue_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_BLUE_BANNER + 3,
        "Light Blue Banner (east)",
        "light_blue_banner",
        1.0,
        false,
        false,
    ),
    block(
        YELLOW_BANNER,
        "Yellow Banner",
        "yellow_banner",
        1.0,
        false,
        false,
    ),
    block(
        YELLOW_BANNER + 1,
        "Yellow Banner (south)",
        "yellow_banner",
        1.0,
        false,
        false,
    ),
    block(
        YELLOW_BANNER + 2,
        "Yellow Banner (west)",
        "yellow_banner",
        1.0,
        false,
        false,
    ),
    block(
        YELLOW_BANNER + 3,
        "Yellow Banner (east)",
        "yellow_banner",
        1.0,
        false,
        false,
    ),
    block(LIME_BANNER, "Lime Banner", "lime_banner", 1.0, false, false),
    block(
        LIME_BANNER + 1,
        "Lime Banner (south)",
        "lime_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIME_BANNER + 2,
        "Lime Banner (west)",
        "lime_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIME_BANNER + 3,
        "Lime Banner (east)",
        "lime_banner",
        1.0,
        false,
        false,
    ),
    block(PINK_BANNER, "Pink Banner", "pink_banner", 1.0, false, false),
    block(
        PINK_BANNER + 1,
        "Pink Banner (south)",
        "pink_banner",
        1.0,
        false,
        false,
    ),
    block(
        PINK_BANNER + 2,
        "Pink Banner (west)",
        "pink_banner",
        1.0,
        false,
        false,
    ),
    block(
        PINK_BANNER + 3,
        "Pink Banner (east)",
        "pink_banner",
        1.0,
        false,
        false,
    ),
    block(GRAY_BANNER, "Gray Banner", "gray_banner", 1.0, false, false),
    block(
        GRAY_BANNER + 1,
        "Gray Banner (south)",
        "gray_banner",
        1.0,
        false,
        false,
    ),
    block(
        GRAY_BANNER + 2,
        "Gray Banner (west)",
        "gray_banner",
        1.0,
        false,
        false,
    ),
    block(
        GRAY_BANNER + 3,
        "Gray Banner (east)",
        "gray_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_GRAY_BANNER,
        "Light Gray Banner",
        "light_gray_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_GRAY_BANNER + 1,
        "Light Gray Banner (south)",
        "light_gray_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_GRAY_BANNER + 2,
        "Light Gray Banner (west)",
        "light_gray_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_GRAY_BANNER + 3,
        "Light Gray Banner (east)",
        "light_gray_banner",
        1.0,
        false,
        false,
    ),
    block(CYAN_BANNER, "Cyan Banner", "cyan_banner", 1.0, false, false),
    block(
        CYAN_BANNER + 1,
        "Cyan Banner (south)",
        "cyan_banner",
        1.0,
        false,
        false,
    ),
    block(
        CYAN_BANNER + 2,
        "Cyan Banner (west)",
        "cyan_banner",
        1.0,
        false,
        false,
    ),
    block(
        CYAN_BANNER + 3,
        "Cyan Banner (east)",
        "cyan_banner",
        1.0,
        false,
        false,
    ),
    block(
        PURPLE_BANNER,
        "Purple Banner",
        "purple_banner",
        1.0,
        false,
        false,
    ),
    block(
        PURPLE_BANNER + 1,
        "Purple Banner (south)",
        "purple_banner",
        1.0,
        false,
        false,
    ),
    block(
        PURPLE_BANNER + 2,
        "Purple Banner (west)",
        "purple_banner",
        1.0,
        false,
        false,
    ),
    block(
        PURPLE_BANNER + 3,
        "Purple Banner (east)",
        "purple_banner",
        1.0,
        false,
        false,
    ),
    block(BLUE_BANNER, "Blue Banner", "blue_banner", 1.0, false, false),
    block(
        BLUE_BANNER + 1,
        "Blue Banner (south)",
        "blue_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLUE_BANNER + 2,
        "Blue Banner (west)",
        "blue_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLUE_BANNER + 3,
        "Blue Banner (east)",
        "blue_banner",
        1.0,
        false,
        false,
    ),
    block(
        BROWN_BANNER,
        "Brown Banner",
        "brown_banner",
        1.0,
        false,
        false,
    ),
    block(
        BROWN_BANNER + 1,
        "Brown Banner (south)",
        "brown_banner",
        1.0,
        false,
        false,
    ),
    block(
        BROWN_BANNER + 2,
        "Brown Banner (west)",
        "brown_banner",
        1.0,
        false,
        false,
    ),
    block(
        BROWN_BANNER + 3,
        "Brown Banner (east)",
        "brown_banner",
        1.0,
        false,
        false,
    ),
    block(
        GREEN_BANNER,
        "Green Banner",
        "green_banner",
        1.0,
        false,
        false,
    ),
    block(
        GREEN_BANNER + 1,
        "Green Banner (south)",
        "green_banner",
        1.0,
        false,
        false,
    ),
    block(
        GREEN_BANNER + 2,
        "Green Banner (west)",
        "green_banner",
        1.0,
        false,
        false,
    ),
    block(
        GREEN_BANNER + 3,
        "Green Banner (east)",
        "green_banner",
        1.0,
        false,
        false,
    ),
    block(RED_BANNER, "Red Banner", "red_banner", 1.0, false, false),
    block(
        RED_BANNER + 1,
        "Red Banner (south)",
        "red_banner",
        1.0,
        false,
        false,
    ),
    block(
        RED_BANNER + 2,
        "Red Banner (west)",
        "red_banner",
        1.0,
        false,
        false,
    ),
    block(
        RED_BANNER + 3,
        "Red Banner (east)",
        "red_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLACK_BANNER,
        "Black Banner",
        "black_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLACK_BANNER + 1,
        "Black Banner (south)",
        "black_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLACK_BANNER + 2,
        "Black Banner (west)",
        "black_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLACK_BANNER + 3,
        "Black Banner (east)",
        "black_banner",
        1.0,
        false,
        false,
    ),
    block(
        WHITE_WALL_BANNER,
        "White Wall Banner",
        "white_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        WHITE_WALL_BANNER + 1,
        "White Wall Banner (south)",
        "white_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        WHITE_WALL_BANNER + 2,
        "White Wall Banner (west)",
        "white_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        WHITE_WALL_BANNER + 3,
        "White Wall Banner (east)",
        "white_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        ORANGE_WALL_BANNER,
        "Orange Wall Banner",
        "orange_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        ORANGE_WALL_BANNER + 1,
        "Orange Wall Banner (south)",
        "orange_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        ORANGE_WALL_BANNER + 2,
        "Orange Wall Banner (west)",
        "orange_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        ORANGE_WALL_BANNER + 3,
        "Orange Wall Banner (east)",
        "orange_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        MAGENTA_WALL_BANNER,
        "Magenta Wall Banner",
        "magenta_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        MAGENTA_WALL_BANNER + 1,
        "Magenta Wall Banner (south)",
        "magenta_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        MAGENTA_WALL_BANNER + 2,
        "Magenta Wall Banner (west)",
        "magenta_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        MAGENTA_WALL_BANNER + 3,
        "Magenta Wall Banner (east)",
        "magenta_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_BLUE_WALL_BANNER,
        "Light Blue Wall Banner",
        "light_blue_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_BLUE_WALL_BANNER + 1,
        "Light Blue Wall Banner (south)",
        "light_blue_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_BLUE_WALL_BANNER + 2,
        "Light Blue Wall Banner (west)",
        "light_blue_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_BLUE_WALL_BANNER + 3,
        "Light Blue Wall Banner (east)",
        "light_blue_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        YELLOW_WALL_BANNER,
        "Yellow Wall Banner",
        "yellow_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        YELLOW_WALL_BANNER + 1,
        "Yellow Wall Banner (south)",
        "yellow_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        YELLOW_WALL_BANNER + 2,
        "Yellow Wall Banner (west)",
        "yellow_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        YELLOW_WALL_BANNER + 3,
        "Yellow Wall Banner (east)",
        "yellow_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIME_WALL_BANNER,
        "Lime Wall Banner",
        "lime_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIME_WALL_BANNER + 1,
        "Lime Wall Banner (south)",
        "lime_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIME_WALL_BANNER + 2,
        "Lime Wall Banner (west)",
        "lime_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIME_WALL_BANNER + 3,
        "Lime Wall Banner (east)",
        "lime_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        PINK_WALL_BANNER,
        "Pink Wall Banner",
        "pink_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        PINK_WALL_BANNER + 1,
        "Pink Wall Banner (south)",
        "pink_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        PINK_WALL_BANNER + 2,
        "Pink Wall Banner (west)",
        "pink_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        PINK_WALL_BANNER + 3,
        "Pink Wall Banner (east)",
        "pink_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        GRAY_WALL_BANNER,
        "Gray Wall Banner",
        "gray_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        GRAY_WALL_BANNER + 1,
        "Gray Wall Banner (south)",
        "gray_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        GRAY_WALL_BANNER + 2,
        "Gray Wall Banner (west)",
        "gray_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        GRAY_WALL_BANNER + 3,
        "Gray Wall Banner (east)",
        "gray_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_GRAY_WALL_BANNER,
        "Light Gray Wall Banner",
        "light_gray_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_GRAY_WALL_BANNER + 1,
        "Light Gray Wall Banner (south)",
        "light_gray_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_GRAY_WALL_BANNER + 2,
        "Light Gray Wall Banner (west)",
        "light_gray_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        LIGHT_GRAY_WALL_BANNER + 3,
        "Light Gray Wall Banner (east)",
        "light_gray_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        CYAN_WALL_BANNER,
        "Cyan Wall Banner",
        "cyan_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        CYAN_WALL_BANNER + 1,
        "Cyan Wall Banner (south)",
        "cyan_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        CYAN_WALL_BANNER + 2,
        "Cyan Wall Banner (west)",
        "cyan_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        CYAN_WALL_BANNER + 3,
        "Cyan Wall Banner (east)",
        "cyan_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        PURPLE_WALL_BANNER,
        "Purple Wall Banner",
        "purple_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        PURPLE_WALL_BANNER + 1,
        "Purple Wall Banner (south)",
        "purple_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        PURPLE_WALL_BANNER + 2,
        "Purple Wall Banner (west)",
        "purple_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        PURPLE_WALL_BANNER + 3,
        "Purple Wall Banner (east)",
        "purple_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLUE_WALL_BANNER,
        "Blue Wall Banner",
        "blue_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLUE_WALL_BANNER + 1,
        "Blue Wall Banner (south)",
        "blue_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLUE_WALL_BANNER + 2,
        "Blue Wall Banner (west)",
        "blue_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLUE_WALL_BANNER + 3,
        "Blue Wall Banner (east)",
        "blue_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BROWN_WALL_BANNER,
        "Brown Wall Banner",
        "brown_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BROWN_WALL_BANNER + 1,
        "Brown Wall Banner (south)",
        "brown_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BROWN_WALL_BANNER + 2,
        "Brown Wall Banner (west)",
        "brown_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BROWN_WALL_BANNER + 3,
        "Brown Wall Banner (east)",
        "brown_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        GREEN_WALL_BANNER,
        "Green Wall Banner",
        "green_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        GREEN_WALL_BANNER + 1,
        "Green Wall Banner (south)",
        "green_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        GREEN_WALL_BANNER + 2,
        "Green Wall Banner (west)",
        "green_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        GREEN_WALL_BANNER + 3,
        "Green Wall Banner (east)",
        "green_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        RED_WALL_BANNER,
        "Red Wall Banner",
        "red_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        RED_WALL_BANNER + 1,
        "Red Wall Banner (south)",
        "red_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        RED_WALL_BANNER + 2,
        "Red Wall Banner (west)",
        "red_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        RED_WALL_BANNER + 3,
        "Red Wall Banner (east)",
        "red_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLACK_WALL_BANNER,
        "Black Wall Banner",
        "black_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLACK_WALL_BANNER + 1,
        "Black Wall Banner (south)",
        "black_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLACK_WALL_BANNER + 2,
        "Black Wall Banner (west)",
        "black_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        BLACK_WALL_BANNER + 3,
        "Black Wall Banner (east)",
        "black_wall_banner",
        1.0,
        false,
        false,
    ),
    block(
        CHISELED_RED_SANDSTONE,
        "Chiseled Red Sandstone",
        "chiseled_red_sandstone",
        0.8,
        true,
        true,
    ),
    block(
        CUT_RED_SANDSTONE,
        "Cut Red Sandstone",
        "cut_red_sandstone",
        0.8,
        true,
        true,
    ),
    block(
        RED_SANDSTONE_STAIRS,
        "Red Sandstone Stairs",
        "red_sandstone_stairs",
        0.8,
        true,
        false,
    ),
    block(
        RED_SANDSTONE_STAIRS + 1,
        "Red Sandstone Stairs (south)",
        "red_sandstone_stairs",
        0.8,
        true,
        false,
    ),
    block(
        RED_SANDSTONE_STAIRS + 2,
        "Red Sandstone Stairs (west)",
        "red_sandstone_stairs",
        0.8,
        true,
        false,
    ),
    block(
        RED_SANDSTONE_STAIRS + 3,
        "Red Sandstone Stairs (east)",
        "red_sandstone_stairs",
        0.8,
        true,
        false,
    ),
    block(SPRUCE_SLAB, "Spruce Slab", "spruce_slab", 2.0, true, false),
    block(JUNGLE_SLAB, "Jungle Slab", "jungle_slab", 2.0, true, false),
    block(ACACIA_SLAB, "Acacia Slab", "acacia_slab", 2.0, true, false),
    block(CHERRY_SLAB, "Cherry Slab", "cherry_slab", 2.0, true, false),
    block(
        DARK_OAK_SLAB,
        "Dark Oak Slab",
        "dark_oak_slab",
        2.0,
        true,
        false,
    ),
    block(
        MANGROVE_SLAB,
        "Mangrove Slab",
        "mangrove_slab",
        2.0,
        true,
        false,
    ),
    block(BAMBOO_SLAB, "Bamboo Slab", "bamboo_slab", 2.0, true, false),
    block(
        BAMBOO_MOSAIC_SLAB,
        "Bamboo Mosaic Slab",
        "bamboo_mosaic_slab",
        2.0,
        true,
        false,
    ),
    block(
        CUT_SANDSTONE_SLAB,
        "Cut Sandstone Slab",
        "cut_sandstone_slab",
        2.0,
        true,
        false,
    ),
    block(
        PETRIFIED_OAK_SLAB,
        "Petrified Oak Slab",
        "petrified_oak_slab",
        2.0,
        true,
        false,
    ),
    block(
        MUD_BRICK_SLAB,
        "Mud Brick Slab",
        "mud_brick_slab",
        2.0,
        true,
        false,
    ),
    block(
        NETHER_BRICK_SLAB,
        "Nether Brick Slab",
        "nether_brick_slab",
        2.0,
        true,
        false,
    ),
    block(QUARTZ_SLAB, "Quartz Slab", "quartz_slab", 2.0, true, false),
    block(
        RED_SANDSTONE_SLAB,
        "Red Sandstone Slab",
        "red_sandstone_slab",
        2.0,
        true,
        false,
    ),
    block(
        CUT_RED_SANDSTONE_SLAB,
        "Cut Red Sandstone Slab",
        "cut_red_sandstone_slab",
        2.0,
        true,
        false,
    ),
    block(PURPUR_SLAB, "Purpur Slab", "purpur_slab", 2.0, true, false),
    block(
        SMOOTH_SANDSTONE,
        "Smooth Sandstone",
        "smooth_sandstone",
        2.0,
        true,
        true,
    ),
    block(
        SMOOTH_QUARTZ,
        "Smooth Quartz",
        "smooth_quartz",
        2.0,
        true,
        true,
    ),
    block(
        SMOOTH_RED_SANDSTONE,
        "Smooth Red Sandstone",
        "smooth_red_sandstone",
        2.0,
        true,
        true,
    ),
    block(
        SPRUCE_FENCE_GATE,
        "Spruce Fence Gate",
        "spruce_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        SPRUCE_FENCE_GATE + 1,
        "Spruce Fence Gate (closed south)",
        "spruce_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        SPRUCE_FENCE_GATE + 2,
        "Spruce Fence Gate (closed west)",
        "spruce_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        SPRUCE_FENCE_GATE + 3,
        "Spruce Fence Gate (closed east)",
        "spruce_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        SPRUCE_FENCE_GATE + 4,
        "Spruce Fence Gate (open north)",
        "spruce_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        SPRUCE_FENCE_GATE + 5,
        "Spruce Fence Gate (open south)",
        "spruce_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        SPRUCE_FENCE_GATE + 6,
        "Spruce Fence Gate (open west)",
        "spruce_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        SPRUCE_FENCE_GATE + 7,
        "Spruce Fence Gate (open east)",
        "spruce_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        BIRCH_FENCE_GATE,
        "Birch Fence Gate",
        "birch_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        BIRCH_FENCE_GATE + 1,
        "Birch Fence Gate (closed south)",
        "birch_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        BIRCH_FENCE_GATE + 2,
        "Birch Fence Gate (closed west)",
        "birch_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        BIRCH_FENCE_GATE + 3,
        "Birch Fence Gate (closed east)",
        "birch_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        BIRCH_FENCE_GATE + 4,
        "Birch Fence Gate (open north)",
        "birch_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        BIRCH_FENCE_GATE + 5,
        "Birch Fence Gate (open south)",
        "birch_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        BIRCH_FENCE_GATE + 6,
        "Birch Fence Gate (open west)",
        "birch_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        BIRCH_FENCE_GATE + 7,
        "Birch Fence Gate (open east)",
        "birch_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        JUNGLE_FENCE_GATE,
        "Jungle Fence Gate",
        "jungle_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        JUNGLE_FENCE_GATE + 1,
        "Jungle Fence Gate (closed south)",
        "jungle_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        JUNGLE_FENCE_GATE + 2,
        "Jungle Fence Gate (closed west)",
        "jungle_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        JUNGLE_FENCE_GATE + 3,
        "Jungle Fence Gate (closed east)",
        "jungle_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        JUNGLE_FENCE_GATE + 4,
        "Jungle Fence Gate (open north)",
        "jungle_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        JUNGLE_FENCE_GATE + 5,
        "Jungle Fence Gate (open south)",
        "jungle_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        JUNGLE_FENCE_GATE + 6,
        "Jungle Fence Gate (open west)",
        "jungle_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        JUNGLE_FENCE_GATE + 7,
        "Jungle Fence Gate (open east)",
        "jungle_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        ACACIA_FENCE_GATE,
        "Acacia Fence Gate",
        "acacia_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        ACACIA_FENCE_GATE + 1,
        "Acacia Fence Gate (closed south)",
        "acacia_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        ACACIA_FENCE_GATE + 2,
        "Acacia Fence Gate (closed west)",
        "acacia_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        ACACIA_FENCE_GATE + 3,
        "Acacia Fence Gate (closed east)",
        "acacia_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        ACACIA_FENCE_GATE + 4,
        "Acacia Fence Gate (open north)",
        "acacia_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        ACACIA_FENCE_GATE + 5,
        "Acacia Fence Gate (open south)",
        "acacia_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        ACACIA_FENCE_GATE + 6,
        "Acacia Fence Gate (open west)",
        "acacia_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        ACACIA_FENCE_GATE + 7,
        "Acacia Fence Gate (open east)",
        "acacia_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        CHERRY_FENCE_GATE,
        "Cherry Fence Gate",
        "cherry_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        CHERRY_FENCE_GATE + 1,
        "Cherry Fence Gate (closed south)",
        "cherry_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        CHERRY_FENCE_GATE + 2,
        "Cherry Fence Gate (closed west)",
        "cherry_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        CHERRY_FENCE_GATE + 3,
        "Cherry Fence Gate (closed east)",
        "cherry_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        CHERRY_FENCE_GATE + 4,
        "Cherry Fence Gate (open north)",
        "cherry_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        CHERRY_FENCE_GATE + 5,
        "Cherry Fence Gate (open south)",
        "cherry_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        CHERRY_FENCE_GATE + 6,
        "Cherry Fence Gate (open west)",
        "cherry_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        CHERRY_FENCE_GATE + 7,
        "Cherry Fence Gate (open east)",
        "cherry_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        DARK_OAK_FENCE_GATE,
        "Dark Oak Fence Gate",
        "dark_oak_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        DARK_OAK_FENCE_GATE + 1,
        "Dark Oak Fence Gate (closed south)",
        "dark_oak_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        DARK_OAK_FENCE_GATE + 2,
        "Dark Oak Fence Gate (closed west)",
        "dark_oak_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        DARK_OAK_FENCE_GATE + 3,
        "Dark Oak Fence Gate (closed east)",
        "dark_oak_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        DARK_OAK_FENCE_GATE + 4,
        "Dark Oak Fence Gate (open north)",
        "dark_oak_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        DARK_OAK_FENCE_GATE + 5,
        "Dark Oak Fence Gate (open south)",
        "dark_oak_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        DARK_OAK_FENCE_GATE + 6,
        "Dark Oak Fence Gate (open west)",
        "dark_oak_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        DARK_OAK_FENCE_GATE + 7,
        "Dark Oak Fence Gate (open east)",
        "dark_oak_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        MANGROVE_FENCE_GATE,
        "Mangrove Fence Gate",
        "mangrove_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        MANGROVE_FENCE_GATE + 1,
        "Mangrove Fence Gate (closed south)",
        "mangrove_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        MANGROVE_FENCE_GATE + 2,
        "Mangrove Fence Gate (closed west)",
        "mangrove_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        MANGROVE_FENCE_GATE + 3,
        "Mangrove Fence Gate (closed east)",
        "mangrove_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        MANGROVE_FENCE_GATE + 4,
        "Mangrove Fence Gate (open north)",
        "mangrove_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        MANGROVE_FENCE_GATE + 5,
        "Mangrove Fence Gate (open south)",
        "mangrove_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        MANGROVE_FENCE_GATE + 6,
        "Mangrove Fence Gate (open west)",
        "mangrove_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        MANGROVE_FENCE_GATE + 7,
        "Mangrove Fence Gate (open east)",
        "mangrove_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        BAMBOO_FENCE_GATE,
        "Bamboo Fence Gate",
        "bamboo_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_FENCE_GATE + 1,
        "Bamboo Fence Gate (closed south)",
        "bamboo_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_FENCE_GATE + 2,
        "Bamboo Fence Gate (closed west)",
        "bamboo_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_FENCE_GATE + 3,
        "Bamboo Fence Gate (closed east)",
        "bamboo_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_FENCE_GATE + 4,
        "Bamboo Fence Gate (open north)",
        "bamboo_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        BAMBOO_FENCE_GATE + 5,
        "Bamboo Fence Gate (open south)",
        "bamboo_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        BAMBOO_FENCE_GATE + 6,
        "Bamboo Fence Gate (open west)",
        "bamboo_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        BAMBOO_FENCE_GATE + 7,
        "Bamboo Fence Gate (open east)",
        "bamboo_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        SPRUCE_FENCE,
        "Spruce Fence",
        "spruce_fence",
        2.0,
        true,
        false,
    ),
    block(BIRCH_FENCE, "Birch Fence", "birch_fence", 2.0, true, false),
    block(
        JUNGLE_FENCE,
        "Jungle Fence",
        "jungle_fence",
        2.0,
        true,
        false,
    ),
    block(
        ACACIA_FENCE,
        "Acacia Fence",
        "acacia_fence",
        2.0,
        true,
        false,
    ),
    block(
        CHERRY_FENCE,
        "Cherry Fence",
        "cherry_fence",
        2.0,
        true,
        false,
    ),
    block(
        DARK_OAK_FENCE,
        "Dark Oak Fence",
        "dark_oak_fence",
        2.0,
        true,
        false,
    ),
    block(
        MANGROVE_FENCE,
        "Mangrove Fence",
        "mangrove_fence",
        2.0,
        true,
        false,
    ),
    block(
        BAMBOO_FENCE,
        "Bamboo Fence",
        "bamboo_fence",
        2.0,
        true,
        false,
    ),
    block(SPRUCE_DOOR, "Spruce Door", "spruce_door", 3.0, true, false),
    block(
        SPRUCE_DOOR + 1,
        "Spruce Door (lower closed south)",
        "spruce_door",
        3.0,
        true,
        false,
    ),
    block(
        SPRUCE_DOOR + 2,
        "Spruce Door (lower closed west)",
        "spruce_door",
        3.0,
        true,
        false,
    ),
    block(
        SPRUCE_DOOR + 3,
        "Spruce Door (lower closed east)",
        "spruce_door",
        3.0,
        true,
        false,
    ),
    block(
        SPRUCE_DOOR + 4,
        "Spruce Door (lower open north)",
        "spruce_door",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_DOOR + 5,
        "Spruce Door (lower open south)",
        "spruce_door",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_DOOR + 6,
        "Spruce Door (lower open west)",
        "spruce_door",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_DOOR + 7,
        "Spruce Door (lower open east)",
        "spruce_door",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_DOOR + 8,
        "Spruce Door (upper closed north)",
        "spruce_door",
        3.0,
        true,
        false,
    ),
    block(
        SPRUCE_DOOR + 9,
        "Spruce Door (upper closed south)",
        "spruce_door",
        3.0,
        true,
        false,
    ),
    block(
        SPRUCE_DOOR + 10,
        "Spruce Door (upper closed west)",
        "spruce_door",
        3.0,
        true,
        false,
    ),
    block(
        SPRUCE_DOOR + 11,
        "Spruce Door (upper closed east)",
        "spruce_door",
        3.0,
        true,
        false,
    ),
    block(
        SPRUCE_DOOR + 12,
        "Spruce Door (upper open north)",
        "spruce_door",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_DOOR + 13,
        "Spruce Door (upper open south)",
        "spruce_door",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_DOOR + 14,
        "Spruce Door (upper open west)",
        "spruce_door",
        3.0,
        false,
        false,
    ),
    block(
        SPRUCE_DOOR + 15,
        "Spruce Door (upper open east)",
        "spruce_door",
        3.0,
        false,
        false,
    ),
    block(BIRCH_DOOR, "Birch Door", "birch_door", 3.0, true, false),
    block(
        BIRCH_DOOR + 1,
        "Birch Door (lower closed south)",
        "birch_door",
        3.0,
        true,
        false,
    ),
    block(
        BIRCH_DOOR + 2,
        "Birch Door (lower closed west)",
        "birch_door",
        3.0,
        true,
        false,
    ),
    block(
        BIRCH_DOOR + 3,
        "Birch Door (lower closed east)",
        "birch_door",
        3.0,
        true,
        false,
    ),
    block(
        BIRCH_DOOR + 4,
        "Birch Door (lower open north)",
        "birch_door",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_DOOR + 5,
        "Birch Door (lower open south)",
        "birch_door",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_DOOR + 6,
        "Birch Door (lower open west)",
        "birch_door",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_DOOR + 7,
        "Birch Door (lower open east)",
        "birch_door",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_DOOR + 8,
        "Birch Door (upper closed north)",
        "birch_door",
        3.0,
        true,
        false,
    ),
    block(
        BIRCH_DOOR + 9,
        "Birch Door (upper closed south)",
        "birch_door",
        3.0,
        true,
        false,
    ),
    block(
        BIRCH_DOOR + 10,
        "Birch Door (upper closed west)",
        "birch_door",
        3.0,
        true,
        false,
    ),
    block(
        BIRCH_DOOR + 11,
        "Birch Door (upper closed east)",
        "birch_door",
        3.0,
        true,
        false,
    ),
    block(
        BIRCH_DOOR + 12,
        "Birch Door (upper open north)",
        "birch_door",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_DOOR + 13,
        "Birch Door (upper open south)",
        "birch_door",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_DOOR + 14,
        "Birch Door (upper open west)",
        "birch_door",
        3.0,
        false,
        false,
    ),
    block(
        BIRCH_DOOR + 15,
        "Birch Door (upper open east)",
        "birch_door",
        3.0,
        false,
        false,
    ),
    block(JUNGLE_DOOR, "Jungle Door", "jungle_door", 3.0, true, false),
    block(
        JUNGLE_DOOR + 1,
        "Jungle Door (lower closed south)",
        "jungle_door",
        3.0,
        true,
        false,
    ),
    block(
        JUNGLE_DOOR + 2,
        "Jungle Door (lower closed west)",
        "jungle_door",
        3.0,
        true,
        false,
    ),
    block(
        JUNGLE_DOOR + 3,
        "Jungle Door (lower closed east)",
        "jungle_door",
        3.0,
        true,
        false,
    ),
    block(
        JUNGLE_DOOR + 4,
        "Jungle Door (lower open north)",
        "jungle_door",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_DOOR + 5,
        "Jungle Door (lower open south)",
        "jungle_door",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_DOOR + 6,
        "Jungle Door (lower open west)",
        "jungle_door",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_DOOR + 7,
        "Jungle Door (lower open east)",
        "jungle_door",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_DOOR + 8,
        "Jungle Door (upper closed north)",
        "jungle_door",
        3.0,
        true,
        false,
    ),
    block(
        JUNGLE_DOOR + 9,
        "Jungle Door (upper closed south)",
        "jungle_door",
        3.0,
        true,
        false,
    ),
    block(
        JUNGLE_DOOR + 10,
        "Jungle Door (upper closed west)",
        "jungle_door",
        3.0,
        true,
        false,
    ),
    block(
        JUNGLE_DOOR + 11,
        "Jungle Door (upper closed east)",
        "jungle_door",
        3.0,
        true,
        false,
    ),
    block(
        JUNGLE_DOOR + 12,
        "Jungle Door (upper open north)",
        "jungle_door",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_DOOR + 13,
        "Jungle Door (upper open south)",
        "jungle_door",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_DOOR + 14,
        "Jungle Door (upper open west)",
        "jungle_door",
        3.0,
        false,
        false,
    ),
    block(
        JUNGLE_DOOR + 15,
        "Jungle Door (upper open east)",
        "jungle_door",
        3.0,
        false,
        false,
    ),
    block(ACACIA_DOOR, "Acacia Door", "acacia_door", 3.0, true, false),
    block(
        ACACIA_DOOR + 1,
        "Acacia Door (lower closed south)",
        "acacia_door",
        3.0,
        true,
        false,
    ),
    block(
        ACACIA_DOOR + 2,
        "Acacia Door (lower closed west)",
        "acacia_door",
        3.0,
        true,
        false,
    ),
    block(
        ACACIA_DOOR + 3,
        "Acacia Door (lower closed east)",
        "acacia_door",
        3.0,
        true,
        false,
    ),
    block(
        ACACIA_DOOR + 4,
        "Acacia Door (lower open north)",
        "acacia_door",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_DOOR + 5,
        "Acacia Door (lower open south)",
        "acacia_door",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_DOOR + 6,
        "Acacia Door (lower open west)",
        "acacia_door",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_DOOR + 7,
        "Acacia Door (lower open east)",
        "acacia_door",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_DOOR + 8,
        "Acacia Door (upper closed north)",
        "acacia_door",
        3.0,
        true,
        false,
    ),
    block(
        ACACIA_DOOR + 9,
        "Acacia Door (upper closed south)",
        "acacia_door",
        3.0,
        true,
        false,
    ),
    block(
        ACACIA_DOOR + 10,
        "Acacia Door (upper closed west)",
        "acacia_door",
        3.0,
        true,
        false,
    ),
    block(
        ACACIA_DOOR + 11,
        "Acacia Door (upper closed east)",
        "acacia_door",
        3.0,
        true,
        false,
    ),
    block(
        ACACIA_DOOR + 12,
        "Acacia Door (upper open north)",
        "acacia_door",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_DOOR + 13,
        "Acacia Door (upper open south)",
        "acacia_door",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_DOOR + 14,
        "Acacia Door (upper open west)",
        "acacia_door",
        3.0,
        false,
        false,
    ),
    block(
        ACACIA_DOOR + 15,
        "Acacia Door (upper open east)",
        "acacia_door",
        3.0,
        false,
        false,
    ),
    block(CHERRY_DOOR, "Cherry Door", "cherry_door", 3.0, true, false),
    block(
        CHERRY_DOOR + 1,
        "Cherry Door (lower closed south)",
        "cherry_door",
        3.0,
        true,
        false,
    ),
    block(
        CHERRY_DOOR + 2,
        "Cherry Door (lower closed west)",
        "cherry_door",
        3.0,
        true,
        false,
    ),
    block(
        CHERRY_DOOR + 3,
        "Cherry Door (lower closed east)",
        "cherry_door",
        3.0,
        true,
        false,
    ),
    block(
        CHERRY_DOOR + 4,
        "Cherry Door (lower open north)",
        "cherry_door",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_DOOR + 5,
        "Cherry Door (lower open south)",
        "cherry_door",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_DOOR + 6,
        "Cherry Door (lower open west)",
        "cherry_door",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_DOOR + 7,
        "Cherry Door (lower open east)",
        "cherry_door",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_DOOR + 8,
        "Cherry Door (upper closed north)",
        "cherry_door",
        3.0,
        true,
        false,
    ),
    block(
        CHERRY_DOOR + 9,
        "Cherry Door (upper closed south)",
        "cherry_door",
        3.0,
        true,
        false,
    ),
    block(
        CHERRY_DOOR + 10,
        "Cherry Door (upper closed west)",
        "cherry_door",
        3.0,
        true,
        false,
    ),
    block(
        CHERRY_DOOR + 11,
        "Cherry Door (upper closed east)",
        "cherry_door",
        3.0,
        true,
        false,
    ),
    block(
        CHERRY_DOOR + 12,
        "Cherry Door (upper open north)",
        "cherry_door",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_DOOR + 13,
        "Cherry Door (upper open south)",
        "cherry_door",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_DOOR + 14,
        "Cherry Door (upper open west)",
        "cherry_door",
        3.0,
        false,
        false,
    ),
    block(
        CHERRY_DOOR + 15,
        "Cherry Door (upper open east)",
        "cherry_door",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_DOOR,
        "Dark Oak Door",
        "dark_oak_door",
        3.0,
        true,
        false,
    ),
    block(
        DARK_OAK_DOOR + 1,
        "Dark Oak Door (lower closed south)",
        "dark_oak_door",
        3.0,
        true,
        false,
    ),
    block(
        DARK_OAK_DOOR + 2,
        "Dark Oak Door (lower closed west)",
        "dark_oak_door",
        3.0,
        true,
        false,
    ),
    block(
        DARK_OAK_DOOR + 3,
        "Dark Oak Door (lower closed east)",
        "dark_oak_door",
        3.0,
        true,
        false,
    ),
    block(
        DARK_OAK_DOOR + 4,
        "Dark Oak Door (lower open north)",
        "dark_oak_door",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_DOOR + 5,
        "Dark Oak Door (lower open south)",
        "dark_oak_door",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_DOOR + 6,
        "Dark Oak Door (lower open west)",
        "dark_oak_door",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_DOOR + 7,
        "Dark Oak Door (lower open east)",
        "dark_oak_door",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_DOOR + 8,
        "Dark Oak Door (upper closed north)",
        "dark_oak_door",
        3.0,
        true,
        false,
    ),
    block(
        DARK_OAK_DOOR + 9,
        "Dark Oak Door (upper closed south)",
        "dark_oak_door",
        3.0,
        true,
        false,
    ),
    block(
        DARK_OAK_DOOR + 10,
        "Dark Oak Door (upper closed west)",
        "dark_oak_door",
        3.0,
        true,
        false,
    ),
    block(
        DARK_OAK_DOOR + 11,
        "Dark Oak Door (upper closed east)",
        "dark_oak_door",
        3.0,
        true,
        false,
    ),
    block(
        DARK_OAK_DOOR + 12,
        "Dark Oak Door (upper open north)",
        "dark_oak_door",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_DOOR + 13,
        "Dark Oak Door (upper open south)",
        "dark_oak_door",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_DOOR + 14,
        "Dark Oak Door (upper open west)",
        "dark_oak_door",
        3.0,
        false,
        false,
    ),
    block(
        DARK_OAK_DOOR + 15,
        "Dark Oak Door (upper open east)",
        "dark_oak_door",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_DOOR,
        "Mangrove Door",
        "mangrove_door",
        3.0,
        true,
        false,
    ),
    block(
        MANGROVE_DOOR + 1,
        "Mangrove Door (lower closed south)",
        "mangrove_door",
        3.0,
        true,
        false,
    ),
    block(
        MANGROVE_DOOR + 2,
        "Mangrove Door (lower closed west)",
        "mangrove_door",
        3.0,
        true,
        false,
    ),
    block(
        MANGROVE_DOOR + 3,
        "Mangrove Door (lower closed east)",
        "mangrove_door",
        3.0,
        true,
        false,
    ),
    block(
        MANGROVE_DOOR + 4,
        "Mangrove Door (lower open north)",
        "mangrove_door",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_DOOR + 5,
        "Mangrove Door (lower open south)",
        "mangrove_door",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_DOOR + 6,
        "Mangrove Door (lower open west)",
        "mangrove_door",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_DOOR + 7,
        "Mangrove Door (lower open east)",
        "mangrove_door",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_DOOR + 8,
        "Mangrove Door (upper closed north)",
        "mangrove_door",
        3.0,
        true,
        false,
    ),
    block(
        MANGROVE_DOOR + 9,
        "Mangrove Door (upper closed south)",
        "mangrove_door",
        3.0,
        true,
        false,
    ),
    block(
        MANGROVE_DOOR + 10,
        "Mangrove Door (upper closed west)",
        "mangrove_door",
        3.0,
        true,
        false,
    ),
    block(
        MANGROVE_DOOR + 11,
        "Mangrove Door (upper closed east)",
        "mangrove_door",
        3.0,
        true,
        false,
    ),
    block(
        MANGROVE_DOOR + 12,
        "Mangrove Door (upper open north)",
        "mangrove_door",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_DOOR + 13,
        "Mangrove Door (upper open south)",
        "mangrove_door",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_DOOR + 14,
        "Mangrove Door (upper open west)",
        "mangrove_door",
        3.0,
        false,
        false,
    ),
    block(
        MANGROVE_DOOR + 15,
        "Mangrove Door (upper open east)",
        "mangrove_door",
        3.0,
        false,
        false,
    ),
    block(BAMBOO_DOOR, "Bamboo Door", "bamboo_door", 3.0, true, false),
    block(
        BAMBOO_DOOR + 1,
        "Bamboo Door (lower closed south)",
        "bamboo_door",
        3.0,
        true,
        false,
    ),
    block(
        BAMBOO_DOOR + 2,
        "Bamboo Door (lower closed west)",
        "bamboo_door",
        3.0,
        true,
        false,
    ),
    block(
        BAMBOO_DOOR + 3,
        "Bamboo Door (lower closed east)",
        "bamboo_door",
        3.0,
        true,
        false,
    ),
    block(
        BAMBOO_DOOR + 4,
        "Bamboo Door (lower open north)",
        "bamboo_door",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_DOOR + 5,
        "Bamboo Door (lower open south)",
        "bamboo_door",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_DOOR + 6,
        "Bamboo Door (lower open west)",
        "bamboo_door",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_DOOR + 7,
        "Bamboo Door (lower open east)",
        "bamboo_door",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_DOOR + 8,
        "Bamboo Door (upper closed north)",
        "bamboo_door",
        3.0,
        true,
        false,
    ),
    block(
        BAMBOO_DOOR + 9,
        "Bamboo Door (upper closed south)",
        "bamboo_door",
        3.0,
        true,
        false,
    ),
    block(
        BAMBOO_DOOR + 10,
        "Bamboo Door (upper closed west)",
        "bamboo_door",
        3.0,
        true,
        false,
    ),
    block(
        BAMBOO_DOOR + 11,
        "Bamboo Door (upper closed east)",
        "bamboo_door",
        3.0,
        true,
        false,
    ),
    block(
        BAMBOO_DOOR + 12,
        "Bamboo Door (upper open north)",
        "bamboo_door",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_DOOR + 13,
        "Bamboo Door (upper open south)",
        "bamboo_door",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_DOOR + 14,
        "Bamboo Door (upper open west)",
        "bamboo_door",
        3.0,
        false,
        false,
    ),
    block(
        BAMBOO_DOOR + 15,
        "Bamboo Door (upper open east)",
        "bamboo_door",
        3.0,
        false,
        false,
    ),
    BlockDef {
        light_emission: 14,
        ..block(END_ROD, "End Rod", "end_rod", 0.05, true, false)
    },
    block(
        CHORUS_PLANT,
        "Chorus Plant",
        "chorus_plant",
        0.4,
        true,
        false,
    ),
    block(
        CHORUS_FLOWER,
        "Chorus Flower",
        "chorus_flower",
        0.4,
        true,
        false,
    ),
    block(
        PURPUR_BLOCK,
        "Purpur Block",
        "purpur_block",
        1.5,
        true,
        true,
    ),
    block(
        PURPUR_PILLAR,
        "Purpur Pillar",
        "purpur_pillar",
        1.5,
        true,
        true,
    ),
    block(
        PURPUR_STAIRS,
        "Purpur Stairs",
        "purpur_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PURPUR_STAIRS + 1,
        "Purpur Stairs (south)",
        "purpur_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PURPUR_STAIRS + 2,
        "Purpur Stairs (west)",
        "purpur_stairs",
        1.5,
        true,
        false,
    ),
    block(
        PURPUR_STAIRS + 3,
        "Purpur Stairs (east)",
        "purpur_stairs",
        1.5,
        true,
        false,
    ),
    block(
        END_STONE_BRICKS,
        "End Stone Bricks",
        "end_stone_bricks",
        3.0,
        true,
        true,
    ),
    block(
        TORCHFLOWER_CROP,
        "Torchflower Crop",
        "torchflower_crop",
        0.05,
        false,
        false,
    ),
    block(
        PITCHER_CROP,
        "Pitcher Crop",
        "pitcher_crop",
        0.05,
        false,
        false,
    ),
    block(
        PITCHER_PLANT,
        "Pitcher Plant",
        "pitcher_plant",
        0.05,
        false,
        false,
    ),
    block(BEETROOTS, "Beetroots", "beetroots", 0.05, false, false),
    block(DIRT_PATH, "Dirt Path", "dirt_path", 0.65, true, false),
    BlockDef {
        light_emission: 15,
        ..block(END_GATEWAY, "End Gateway", "end_gateway", 0.5, false, false)
    },
    block(
        REPEATING_COMMAND_BLOCK,
        "Repeating Command Block",
        "repeating_command_block",
        5.0,
        true,
        true,
    ),
    block(
        CHAIN_COMMAND_BLOCK,
        "Chain Command Block",
        "chain_command_block",
        5.0,
        true,
        true,
    ),
    block(FROSTED_ICE, "Frosted Ice", "frosted_ice", 0.5, true, false),
    BlockDef {
        light_emission: 3,
        ..block(MAGMA_BLOCK, "Magma Block", "magma_block", 0.5, true, true)
    },
    block(
        NETHER_WART_BLOCK,
        "Nether Wart Block",
        "nether_wart_block",
        1.0,
        true,
        true,
    ),
    block(
        RED_NETHER_BRICKS,
        "Red Nether Bricks",
        "red_nether_bricks",
        2.0,
        true,
        true,
    ),
    block(
        STRUCTURE_VOID,
        "Structure Void",
        "structure_void",
        0.05,
        false,
        false,
    ),
    block(OBSERVER, "Observer", "observer", 3.0, true, true),
    block(
        OBSERVER + 1,
        "Observer (south)",
        "observer",
        3.0,
        true,
        true,
    ),
    block(OBSERVER + 2, "Observer (west)", "observer", 3.0, true, true),
    block(OBSERVER + 3, "Observer (east)", "observer", 3.0, true, true),
    block(OBSERVER + 4, "Observer (up)", "observer", 3.0, true, true),
    block(OBSERVER + 5, "Observer (down)", "observer", 3.0, true, true),
    block(SHULKER_BOX, "Shulker Box", "shulker_box", 2.0, true, true),
    block(
        WHITE_SHULKER_BOX,
        "White Shulker Box",
        "white_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        ORANGE_SHULKER_BOX,
        "Orange Shulker Box",
        "orange_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        MAGENTA_SHULKER_BOX,
        "Magenta Shulker Box",
        "magenta_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        LIGHT_BLUE_SHULKER_BOX,
        "Light Blue Shulker Box",
        "light_blue_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        YELLOW_SHULKER_BOX,
        "Yellow Shulker Box",
        "yellow_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        LIME_SHULKER_BOX,
        "Lime Shulker Box",
        "lime_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        PINK_SHULKER_BOX,
        "Pink Shulker Box",
        "pink_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        GRAY_SHULKER_BOX,
        "Gray Shulker Box",
        "gray_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        LIGHT_GRAY_SHULKER_BOX,
        "Light Gray Shulker Box",
        "light_gray_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        CYAN_SHULKER_BOX,
        "Cyan Shulker Box",
        "cyan_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        PURPLE_SHULKER_BOX,
        "Purple Shulker Box",
        "purple_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        BLUE_SHULKER_BOX,
        "Blue Shulker Box",
        "blue_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        BROWN_SHULKER_BOX,
        "Brown Shulker Box",
        "brown_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        GREEN_SHULKER_BOX,
        "Green Shulker Box",
        "green_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        RED_SHULKER_BOX,
        "Red Shulker Box",
        "red_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        BLACK_SHULKER_BOX,
        "Black Shulker Box",
        "black_shulker_box",
        2.0,
        true,
        true,
    ),
    block(
        WHITE_GLAZED_TERRACOTTA,
        "White Glazed Terracotta",
        "white_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        ORANGE_GLAZED_TERRACOTTA,
        "Orange Glazed Terracotta",
        "orange_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        MAGENTA_GLAZED_TERRACOTTA,
        "Magenta Glazed Terracotta",
        "magenta_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        LIGHT_BLUE_GLAZED_TERRACOTTA,
        "Light Blue Glazed Terracotta",
        "light_blue_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        YELLOW_GLAZED_TERRACOTTA,
        "Yellow Glazed Terracotta",
        "yellow_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        LIME_GLAZED_TERRACOTTA,
        "Lime Glazed Terracotta",
        "lime_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        PINK_GLAZED_TERRACOTTA,
        "Pink Glazed Terracotta",
        "pink_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        GRAY_GLAZED_TERRACOTTA,
        "Gray Glazed Terracotta",
        "gray_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        LIGHT_GRAY_GLAZED_TERRACOTTA,
        "Light Gray Glazed Terracotta",
        "light_gray_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        CYAN_GLAZED_TERRACOTTA,
        "Cyan Glazed Terracotta",
        "cyan_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        PURPLE_GLAZED_TERRACOTTA,
        "Purple Glazed Terracotta",
        "purple_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        BLUE_GLAZED_TERRACOTTA,
        "Blue Glazed Terracotta",
        "blue_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        BROWN_GLAZED_TERRACOTTA,
        "Brown Glazed Terracotta",
        "brown_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        GREEN_GLAZED_TERRACOTTA,
        "Green Glazed Terracotta",
        "green_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        RED_GLAZED_TERRACOTTA,
        "Red Glazed Terracotta",
        "red_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        BLACK_GLAZED_TERRACOTTA,
        "Black Glazed Terracotta",
        "black_glazed_terracotta",
        1.4,
        true,
        true,
    ),
    block(
        WHITE_CONCRETE_POWDER,
        "White Concrete Powder",
        "white_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        ORANGE_CONCRETE_POWDER,
        "Orange Concrete Powder",
        "orange_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        MAGENTA_CONCRETE_POWDER,
        "Magenta Concrete Powder",
        "magenta_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        LIGHT_BLUE_CONCRETE_POWDER,
        "Light Blue Concrete Powder",
        "light_blue_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        YELLOW_CONCRETE_POWDER,
        "Yellow Concrete Powder",
        "yellow_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        LIME_CONCRETE_POWDER,
        "Lime Concrete Powder",
        "lime_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        PINK_CONCRETE_POWDER,
        "Pink Concrete Powder",
        "pink_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        GRAY_CONCRETE_POWDER,
        "Gray Concrete Powder",
        "gray_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        LIGHT_GRAY_CONCRETE_POWDER,
        "Light Gray Concrete Powder",
        "light_gray_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        CYAN_CONCRETE_POWDER,
        "Cyan Concrete Powder",
        "cyan_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        PURPLE_CONCRETE_POWDER,
        "Purple Concrete Powder",
        "purple_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        BLUE_CONCRETE_POWDER,
        "Blue Concrete Powder",
        "blue_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        BROWN_CONCRETE_POWDER,
        "Brown Concrete Powder",
        "brown_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        GREEN_CONCRETE_POWDER,
        "Green Concrete Powder",
        "green_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        RED_CONCRETE_POWDER,
        "Red Concrete Powder",
        "red_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(
        BLACK_CONCRETE_POWDER,
        "Black Concrete Powder",
        "black_concrete_powder",
        0.5,
        true,
        true,
    ),
    block(KELP, "Kelp", "kelp", 0.05, false, false),
    block(KELP_PLANT, "Kelp Plant", "kelp_plant", 0.05, false, false),
    block(
        DRIED_KELP_BLOCK,
        "Dried Kelp Block",
        "dried_kelp_block",
        0.5,
        true,
        true,
    ),
    block(TURTLE_EGG, "Turtle Egg", "turtle_egg", 0.5, true, false),
    block(SNIFFER_EGG, "Sniffer Egg", "sniffer_egg", 0.5, true, false),
    block(
        DEAD_TUBE_CORAL_BLOCK,
        "Dead Tube Coral Block",
        "dead_tube_coral_block",
        1.5,
        true,
        true,
    ),
    block(
        DEAD_BRAIN_CORAL_BLOCK,
        "Dead Brain Coral Block",
        "dead_brain_coral_block",
        1.5,
        true,
        true,
    ),
    block(
        DEAD_BUBBLE_CORAL_BLOCK,
        "Dead Bubble Coral Block",
        "dead_bubble_coral_block",
        1.5,
        true,
        true,
    ),
    block(
        DEAD_FIRE_CORAL_BLOCK,
        "Dead Fire Coral Block",
        "dead_fire_coral_block",
        1.5,
        true,
        true,
    ),
    block(
        DEAD_HORN_CORAL_BLOCK,
        "Dead Horn Coral Block",
        "dead_horn_coral_block",
        1.5,
        true,
        true,
    ),
    block(
        TUBE_CORAL_BLOCK,
        "Tube Coral Block",
        "tube_coral_block",
        1.5,
        true,
        true,
    ),
    block(
        BRAIN_CORAL_BLOCK,
        "Brain Coral Block",
        "brain_coral_block",
        1.5,
        true,
        true,
    ),
    block(
        BUBBLE_CORAL_BLOCK,
        "Bubble Coral Block",
        "bubble_coral_block",
        1.5,
        true,
        true,
    ),
    block(
        FIRE_CORAL_BLOCK,
        "Fire Coral Block",
        "fire_coral_block",
        1.5,
        true,
        true,
    ),
    block(
        HORN_CORAL_BLOCK,
        "Horn Coral Block",
        "horn_coral_block",
        1.5,
        true,
        true,
    ),
    block(
        DEAD_TUBE_CORAL,
        "Dead Tube Coral",
        "dead_tube_coral",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BRAIN_CORAL,
        "Dead Brain Coral",
        "dead_brain_coral",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BUBBLE_CORAL,
        "Dead Bubble Coral",
        "dead_bubble_coral",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_FIRE_CORAL,
        "Dead Fire Coral",
        "dead_fire_coral",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_HORN_CORAL,
        "Dead Horn Coral",
        "dead_horn_coral",
        0.05,
        false,
        false,
    ),
    block(TUBE_CORAL, "Tube Coral", "tube_coral", 0.05, false, false),
    block(
        BRAIN_CORAL,
        "Brain Coral",
        "brain_coral",
        0.05,
        false,
        false,
    ),
    block(
        BUBBLE_CORAL,
        "Bubble Coral",
        "bubble_coral",
        0.05,
        false,
        false,
    ),
    block(FIRE_CORAL, "Fire Coral", "fire_coral", 0.05, false, false),
    block(HORN_CORAL, "Horn Coral", "horn_coral", 0.05, false, false),
    block(
        DEAD_TUBE_CORAL_FAN,
        "Dead Tube Coral Fan",
        "dead_tube_coral_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BRAIN_CORAL_FAN,
        "Dead Brain Coral Fan",
        "dead_brain_coral_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BUBBLE_CORAL_FAN,
        "Dead Bubble Coral Fan",
        "dead_bubble_coral_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_FIRE_CORAL_FAN,
        "Dead Fire Coral Fan",
        "dead_fire_coral_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_HORN_CORAL_FAN,
        "Dead Horn Coral Fan",
        "dead_horn_coral_fan",
        0.05,
        false,
        false,
    ),
    block(
        TUBE_CORAL_FAN,
        "Tube Coral Fan",
        "tube_coral_fan",
        0.05,
        false,
        false,
    ),
    block(
        BRAIN_CORAL_FAN,
        "Brain Coral Fan",
        "brain_coral_fan",
        0.05,
        false,
        false,
    ),
    block(
        BUBBLE_CORAL_FAN,
        "Bubble Coral Fan",
        "bubble_coral_fan",
        0.05,
        false,
        false,
    ),
    block(
        FIRE_CORAL_FAN,
        "Fire Coral Fan",
        "fire_coral_fan",
        0.05,
        false,
        false,
    ),
    block(
        HORN_CORAL_FAN,
        "Horn Coral Fan",
        "horn_coral_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_TUBE_CORAL_WALL_FAN,
        "Dead Tube Coral Wall Fan",
        "dead_tube_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_TUBE_CORAL_WALL_FAN + 1,
        "Dead Tube Coral Wall Fan (south)",
        "dead_tube_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_TUBE_CORAL_WALL_FAN + 2,
        "Dead Tube Coral Wall Fan (west)",
        "dead_tube_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_TUBE_CORAL_WALL_FAN + 3,
        "Dead Tube Coral Wall Fan (east)",
        "dead_tube_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BRAIN_CORAL_WALL_FAN,
        "Dead Brain Coral Wall Fan",
        "dead_brain_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BRAIN_CORAL_WALL_FAN + 1,
        "Dead Brain Coral Wall Fan (south)",
        "dead_brain_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BRAIN_CORAL_WALL_FAN + 2,
        "Dead Brain Coral Wall Fan (west)",
        "dead_brain_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BRAIN_CORAL_WALL_FAN + 3,
        "Dead Brain Coral Wall Fan (east)",
        "dead_brain_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BUBBLE_CORAL_WALL_FAN,
        "Dead Bubble Coral Wall Fan",
        "dead_bubble_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BUBBLE_CORAL_WALL_FAN + 1,
        "Dead Bubble Coral Wall Fan (south)",
        "dead_bubble_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BUBBLE_CORAL_WALL_FAN + 2,
        "Dead Bubble Coral Wall Fan (west)",
        "dead_bubble_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_BUBBLE_CORAL_WALL_FAN + 3,
        "Dead Bubble Coral Wall Fan (east)",
        "dead_bubble_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_FIRE_CORAL_WALL_FAN,
        "Dead Fire Coral Wall Fan",
        "dead_fire_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_FIRE_CORAL_WALL_FAN + 1,
        "Dead Fire Coral Wall Fan (south)",
        "dead_fire_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_FIRE_CORAL_WALL_FAN + 2,
        "Dead Fire Coral Wall Fan (west)",
        "dead_fire_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_FIRE_CORAL_WALL_FAN + 3,
        "Dead Fire Coral Wall Fan (east)",
        "dead_fire_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_HORN_CORAL_WALL_FAN,
        "Dead Horn Coral Wall Fan",
        "dead_horn_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_HORN_CORAL_WALL_FAN + 1,
        "Dead Horn Coral Wall Fan (south)",
        "dead_horn_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_HORN_CORAL_WALL_FAN + 2,
        "Dead Horn Coral Wall Fan (west)",
        "dead_horn_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        DEAD_HORN_CORAL_WALL_FAN + 3,
        "Dead Horn Coral Wall Fan (east)",
        "dead_horn_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        TUBE_CORAL_WALL_FAN,
        "Tube Coral Wall Fan",
        "tube_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        TUBE_CORAL_WALL_FAN + 1,
        "Tube Coral Wall Fan (south)",
        "tube_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        TUBE_CORAL_WALL_FAN + 2,
        "Tube Coral Wall Fan (west)",
        "tube_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        TUBE_CORAL_WALL_FAN + 3,
        "Tube Coral Wall Fan (east)",
        "tube_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        BRAIN_CORAL_WALL_FAN,
        "Brain Coral Wall Fan",
        "brain_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        BRAIN_CORAL_WALL_FAN + 1,
        "Brain Coral Wall Fan (south)",
        "brain_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        BRAIN_CORAL_WALL_FAN + 2,
        "Brain Coral Wall Fan (west)",
        "brain_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        BRAIN_CORAL_WALL_FAN + 3,
        "Brain Coral Wall Fan (east)",
        "brain_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        BUBBLE_CORAL_WALL_FAN,
        "Bubble Coral Wall Fan",
        "bubble_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        BUBBLE_CORAL_WALL_FAN + 1,
        "Bubble Coral Wall Fan (south)",
        "bubble_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        BUBBLE_CORAL_WALL_FAN + 2,
        "Bubble Coral Wall Fan (west)",
        "bubble_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        BUBBLE_CORAL_WALL_FAN + 3,
        "Bubble Coral Wall Fan (east)",
        "bubble_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        FIRE_CORAL_WALL_FAN,
        "Fire Coral Wall Fan",
        "fire_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        FIRE_CORAL_WALL_FAN + 1,
        "Fire Coral Wall Fan (south)",
        "fire_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        FIRE_CORAL_WALL_FAN + 2,
        "Fire Coral Wall Fan (west)",
        "fire_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        FIRE_CORAL_WALL_FAN + 3,
        "Fire Coral Wall Fan (east)",
        "fire_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        HORN_CORAL_WALL_FAN,
        "Horn Coral Wall Fan",
        "horn_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        HORN_CORAL_WALL_FAN + 1,
        "Horn Coral Wall Fan (south)",
        "horn_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        HORN_CORAL_WALL_FAN + 2,
        "Horn Coral Wall Fan (west)",
        "horn_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(
        HORN_CORAL_WALL_FAN + 3,
        "Horn Coral Wall Fan (east)",
        "horn_coral_wall_fan",
        0.05,
        false,
        false,
    ),
    block(SEA_PICKLE, "Sea Pickle", "sea_pickle", 0.05, false, false),
    block(BLUE_ICE, "Blue Ice", "blue_ice", 2.8, true, true),
    BlockDef {
        light_emission: 15,
        ..block(CONDUIT, "Conduit", "conduit", 3.0, true, false)
    },
    block(
        BAMBOO_SAPLING,
        "Bamboo Sapling",
        "bamboo_sapling",
        0.05,
        false,
        false,
    ),
    block(BAMBOO, "Bamboo", "bamboo", 1.0, false, false),
    block(
        POTTED_BAMBOO,
        "Potted Bamboo",
        "potted_bamboo",
        0.05,
        true,
        false,
    ),
    block(VOID_AIR, "Void Air", "void_air", 0.05, false, false),
    block(CAVE_AIR, "Cave Air", "cave_air", 0.05, false, false),
    block(
        BUBBLE_COLUMN,
        "Bubble Column",
        "bubble_column",
        0.05,
        false,
        false,
    ),
    block(
        POLISHED_GRANITE_STAIRS,
        "Polished Granite Stairs",
        "polished_granite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_GRANITE_STAIRS + 1,
        "Polished Granite Stairs (south)",
        "polished_granite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_GRANITE_STAIRS + 2,
        "Polished Granite Stairs (west)",
        "polished_granite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_GRANITE_STAIRS + 3,
        "Polished Granite Stairs (east)",
        "polished_granite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_RED_SANDSTONE_STAIRS,
        "Smooth Red Sandstone Stairs",
        "smooth_red_sandstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_RED_SANDSTONE_STAIRS + 1,
        "Smooth Red Sandstone Stairs (south)",
        "smooth_red_sandstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_RED_SANDSTONE_STAIRS + 2,
        "Smooth Red Sandstone Stairs (west)",
        "smooth_red_sandstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_RED_SANDSTONE_STAIRS + 3,
        "Smooth Red Sandstone Stairs (east)",
        "smooth_red_sandstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        MOSSY_STONE_BRICK_STAIRS,
        "Mossy Stone Brick Stairs",
        "mossy_stone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        MOSSY_STONE_BRICK_STAIRS + 1,
        "Mossy Stone Brick Stairs (south)",
        "mossy_stone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        MOSSY_STONE_BRICK_STAIRS + 2,
        "Mossy Stone Brick Stairs (west)",
        "mossy_stone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        MOSSY_STONE_BRICK_STAIRS + 3,
        "Mossy Stone Brick Stairs (east)",
        "mossy_stone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_DIORITE_STAIRS,
        "Polished Diorite Stairs",
        "polished_diorite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_DIORITE_STAIRS + 1,
        "Polished Diorite Stairs (south)",
        "polished_diorite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_DIORITE_STAIRS + 2,
        "Polished Diorite Stairs (west)",
        "polished_diorite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_DIORITE_STAIRS + 3,
        "Polished Diorite Stairs (east)",
        "polished_diorite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        MOSSY_COBBLESTONE_STAIRS,
        "Mossy Cobblestone Stairs",
        "mossy_cobblestone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        MOSSY_COBBLESTONE_STAIRS + 1,
        "Mossy Cobblestone Stairs (south)",
        "mossy_cobblestone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        MOSSY_COBBLESTONE_STAIRS + 2,
        "Mossy Cobblestone Stairs (west)",
        "mossy_cobblestone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        MOSSY_COBBLESTONE_STAIRS + 3,
        "Mossy Cobblestone Stairs (east)",
        "mossy_cobblestone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        END_STONE_BRICK_STAIRS,
        "End Stone Brick Stairs",
        "end_stone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        END_STONE_BRICK_STAIRS + 1,
        "End Stone Brick Stairs (south)",
        "end_stone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        END_STONE_BRICK_STAIRS + 2,
        "End Stone Brick Stairs (west)",
        "end_stone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        END_STONE_BRICK_STAIRS + 3,
        "End Stone Brick Stairs (east)",
        "end_stone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        STONE_STAIRS,
        "Stone Stairs",
        "stone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        STONE_STAIRS + 1,
        "Stone Stairs (south)",
        "stone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        STONE_STAIRS + 2,
        "Stone Stairs (west)",
        "stone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        STONE_STAIRS + 3,
        "Stone Stairs (east)",
        "stone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_SANDSTONE_STAIRS,
        "Smooth Sandstone Stairs",
        "smooth_sandstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_SANDSTONE_STAIRS + 1,
        "Smooth Sandstone Stairs (south)",
        "smooth_sandstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_SANDSTONE_STAIRS + 2,
        "Smooth Sandstone Stairs (west)",
        "smooth_sandstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_SANDSTONE_STAIRS + 3,
        "Smooth Sandstone Stairs (east)",
        "smooth_sandstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_QUARTZ_STAIRS,
        "Smooth Quartz Stairs",
        "smooth_quartz_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_QUARTZ_STAIRS + 1,
        "Smooth Quartz Stairs (south)",
        "smooth_quartz_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_QUARTZ_STAIRS + 2,
        "Smooth Quartz Stairs (west)",
        "smooth_quartz_stairs",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_QUARTZ_STAIRS + 3,
        "Smooth Quartz Stairs (east)",
        "smooth_quartz_stairs",
        1.5,
        true,
        false,
    ),
    block(
        GRANITE_STAIRS,
        "Granite Stairs",
        "granite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        GRANITE_STAIRS + 1,
        "Granite Stairs (south)",
        "granite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        GRANITE_STAIRS + 2,
        "Granite Stairs (west)",
        "granite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        GRANITE_STAIRS + 3,
        "Granite Stairs (east)",
        "granite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        ANDESITE_STAIRS,
        "Andesite Stairs",
        "andesite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        ANDESITE_STAIRS + 1,
        "Andesite Stairs (south)",
        "andesite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        ANDESITE_STAIRS + 2,
        "Andesite Stairs (west)",
        "andesite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        ANDESITE_STAIRS + 3,
        "Andesite Stairs (east)",
        "andesite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        RED_NETHER_BRICK_STAIRS,
        "Red Nether Brick Stairs",
        "red_nether_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        RED_NETHER_BRICK_STAIRS + 1,
        "Red Nether Brick Stairs (south)",
        "red_nether_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        RED_NETHER_BRICK_STAIRS + 2,
        "Red Nether Brick Stairs (west)",
        "red_nether_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        RED_NETHER_BRICK_STAIRS + 3,
        "Red Nether Brick Stairs (east)",
        "red_nether_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_ANDESITE_STAIRS,
        "Polished Andesite Stairs",
        "polished_andesite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_ANDESITE_STAIRS + 1,
        "Polished Andesite Stairs (south)",
        "polished_andesite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_ANDESITE_STAIRS + 2,
        "Polished Andesite Stairs (west)",
        "polished_andesite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_ANDESITE_STAIRS + 3,
        "Polished Andesite Stairs (east)",
        "polished_andesite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        DIORITE_STAIRS,
        "Diorite Stairs",
        "diorite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        DIORITE_STAIRS + 1,
        "Diorite Stairs (south)",
        "diorite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        DIORITE_STAIRS + 2,
        "Diorite Stairs (west)",
        "diorite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        DIORITE_STAIRS + 3,
        "Diorite Stairs (east)",
        "diorite_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_GRANITE_SLAB,
        "Polished Granite Slab",
        "polished_granite_slab",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_RED_SANDSTONE_SLAB,
        "Smooth Red Sandstone Slab",
        "smooth_red_sandstone_slab",
        1.5,
        true,
        false,
    ),
    block(
        MOSSY_STONE_BRICK_SLAB,
        "Mossy Stone Brick Slab",
        "mossy_stone_brick_slab",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_DIORITE_SLAB,
        "Polished Diorite Slab",
        "polished_diorite_slab",
        1.5,
        true,
        false,
    ),
    block(
        MOSSY_COBBLESTONE_SLAB,
        "Mossy Cobblestone Slab",
        "mossy_cobblestone_slab",
        1.5,
        true,
        false,
    ),
    block(
        END_STONE_BRICK_SLAB,
        "End Stone Brick Slab",
        "end_stone_brick_slab",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_SANDSTONE_SLAB,
        "Smooth Sandstone Slab",
        "smooth_sandstone_slab",
        1.5,
        true,
        false,
    ),
    block(
        SMOOTH_QUARTZ_SLAB,
        "Smooth Quartz Slab",
        "smooth_quartz_slab",
        1.5,
        true,
        false,
    ),
    block(
        ANDESITE_SLAB,
        "Andesite Slab",
        "andesite_slab",
        1.5,
        true,
        false,
    ),
    block(
        RED_NETHER_BRICK_SLAB,
        "Red Nether Brick Slab",
        "red_nether_brick_slab",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_ANDESITE_SLAB,
        "Polished Andesite Slab",
        "polished_andesite_slab",
        1.5,
        true,
        false,
    ),
    block(
        DIORITE_SLAB,
        "Diorite Slab",
        "diorite_slab",
        1.5,
        true,
        false,
    ),
    block(BRICK_WALL, "Brick Wall", "brick_wall", 2.0, true, false),
    block(
        PRISMARINE_WALL,
        "Prismarine Wall",
        "prismarine_wall",
        2.0,
        true,
        false,
    ),
    block(
        RED_SANDSTONE_WALL,
        "Red Sandstone Wall",
        "red_sandstone_wall",
        2.0,
        true,
        false,
    ),
    block(
        MOSSY_STONE_BRICK_WALL,
        "Mossy Stone Brick Wall",
        "mossy_stone_brick_wall",
        2.0,
        true,
        false,
    ),
    block(
        GRANITE_WALL,
        "Granite Wall",
        "granite_wall",
        2.0,
        true,
        false,
    ),
    block(
        STONE_BRICK_WALL,
        "Stone Brick Wall",
        "stone_brick_wall",
        2.0,
        true,
        false,
    ),
    block(
        MUD_BRICK_WALL,
        "Mud Brick Wall",
        "mud_brick_wall",
        2.0,
        true,
        false,
    ),
    block(
        NETHER_BRICK_WALL,
        "Nether Brick Wall",
        "nether_brick_wall",
        2.0,
        true,
        false,
    ),
    block(
        ANDESITE_WALL,
        "Andesite Wall",
        "andesite_wall",
        2.0,
        true,
        false,
    ),
    block(
        RED_NETHER_BRICK_WALL,
        "Red Nether Brick Wall",
        "red_nether_brick_wall",
        2.0,
        true,
        false,
    ),
    block(
        SANDSTONE_WALL,
        "Sandstone Wall",
        "sandstone_wall",
        2.0,
        true,
        false,
    ),
    block(
        END_STONE_BRICK_WALL,
        "End Stone Brick Wall",
        "end_stone_brick_wall",
        2.0,
        true,
        false,
    ),
    block(
        DIORITE_WALL,
        "Diorite Wall",
        "diorite_wall",
        2.0,
        true,
        false,
    ),
    block(SCAFFOLDING, "Scaffolding", "scaffolding", 0.05, true, false),
    block(LOOM, "Loom", "loom", 2.5, true, true),
    block(
        CARTOGRAPHY_TABLE,
        "Cartography Table",
        "cartography_table",
        2.5,
        true,
        true,
    ),
    block(
        FLETCHING_TABLE,
        "Fletching Table",
        "fletching_table",
        2.5,
        true,
        true,
    ),
    block(GRINDSTONE, "Grindstone", "grindstone", 2.0, true, false),
    block(LECTERN, "Lectern", "lectern", 2.5, true, false),
    block(
        SMITHING_TABLE,
        "Smithing Table",
        "smithing_table",
        2.5,
        true,
        true,
    ),
    block(BELL, "Bell", "bell", 5.0, true, false),
    BlockDef {
        light_emission: 15,
        ..block(LANTERN, "Lantern", "lantern", 3.5, true, false)
    },
    BlockDef {
        light_emission: 10,
        ..block(
            SOUL_LANTERN,
            "Soul Lantern",
            "soul_lantern",
            3.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 15,
        ..block(CAMPFIRE, "Campfire", "campfire", 2.0, true, false)
    },
    BlockDef {
        light_emission: 10,
        ..block(
            SOUL_CAMPFIRE,
            "Soul Campfire",
            "soul_campfire",
            2.0,
            true,
            false,
        )
    },
    block(
        SWEET_BERRY_BUSH,
        "Sweet Berry Bush",
        "sweet_berry_bush",
        0.05,
        false,
        false,
    ),
    block(WARPED_STEM, "Warped Stem", "warped_stem", 2.0, true, true),
    block(
        STRIPPED_WARPED_STEM,
        "Stripped Warped Stem",
        "stripped_warped_stem",
        2.0,
        true,
        true,
    ),
    block(
        WARPED_HYPHAE,
        "Warped Hyphae",
        "warped_hyphae",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_WARPED_HYPHAE,
        "Stripped Warped Hyphae",
        "stripped_warped_hyphae",
        2.0,
        true,
        true,
    ),
    block(
        WARPED_NYLIUM,
        "Warped Nylium",
        "warped_nylium",
        0.4,
        true,
        true,
    ),
    block(
        WARPED_FUNGUS,
        "Warped Fungus",
        "warped_fungus",
        0.05,
        false,
        false,
    ),
    block(
        WARPED_WART_BLOCK,
        "Warped Wart Block",
        "warped_wart_block",
        1.0,
        true,
        true,
    ),
    block(
        WARPED_ROOTS,
        "Warped Roots",
        "warped_roots",
        0.05,
        false,
        false,
    ),
    block(
        NETHER_SPROUTS,
        "Nether Sprouts",
        "nether_sprouts",
        0.05,
        false,
        false,
    ),
    block(
        CRIMSON_STEM,
        "Crimson Stem",
        "crimson_stem",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_CRIMSON_STEM,
        "Stripped Crimson Stem",
        "stripped_crimson_stem",
        2.0,
        true,
        true,
    ),
    block(
        CRIMSON_HYPHAE,
        "Crimson Hyphae",
        "crimson_hyphae",
        2.0,
        true,
        true,
    ),
    block(
        STRIPPED_CRIMSON_HYPHAE,
        "Stripped Crimson Hyphae",
        "stripped_crimson_hyphae",
        2.0,
        true,
        true,
    ),
    block(
        CRIMSON_NYLIUM,
        "Crimson Nylium",
        "crimson_nylium",
        0.4,
        true,
        true,
    ),
    block(
        CRIMSON_FUNGUS,
        "Crimson Fungus",
        "crimson_fungus",
        0.05,
        false,
        false,
    ),
    BlockDef {
        light_emission: 15,
        ..block(SHROOMLIGHT, "Shroomlight", "shroomlight", 1.0, true, true)
    },
    block(
        WEEPING_VINES,
        "Weeping Vines",
        "weeping_vines",
        0.05,
        false,
        false,
    ),
    block(
        WEEPING_VINES_PLANT,
        "Weeping Vines Plant",
        "weeping_vines_plant",
        0.05,
        false,
        false,
    ),
    block(
        TWISTING_VINES,
        "Twisting Vines",
        "twisting_vines",
        0.05,
        false,
        false,
    ),
    block(
        TWISTING_VINES_PLANT,
        "Twisting Vines Plant",
        "twisting_vines_plant",
        0.05,
        false,
        false,
    ),
    block(
        CRIMSON_ROOTS,
        "Crimson Roots",
        "crimson_roots",
        0.05,
        false,
        false,
    ),
    block(
        CRIMSON_PLANKS,
        "Crimson Planks",
        "crimson_planks",
        2.0,
        true,
        true,
    ),
    block(
        WARPED_PLANKS,
        "Warped Planks",
        "warped_planks",
        2.0,
        true,
        true,
    ),
    block(
        CRIMSON_SLAB,
        "Crimson Slab",
        "crimson_slab",
        2.0,
        true,
        false,
    ),
    block(WARPED_SLAB, "Warped Slab", "warped_slab", 2.0, true, false),
    block(
        CRIMSON_PRESSURE_PLATE,
        "Crimson Pressure Plate",
        "crimson_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        WARPED_PRESSURE_PLATE,
        "Warped Pressure Plate",
        "warped_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        CRIMSON_FENCE,
        "Crimson Fence",
        "crimson_fence",
        2.0,
        true,
        false,
    ),
    block(
        WARPED_FENCE,
        "Warped Fence",
        "warped_fence",
        2.0,
        true,
        false,
    ),
    block(
        CRIMSON_TRAPDOOR,
        "Crimson Trapdoor",
        "crimson_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        CRIMSON_TRAPDOOR + 1,
        "Crimson Trapdoor (open north)",
        "crimson_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_TRAPDOOR + 2,
        "Crimson Trapdoor (open south)",
        "crimson_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_TRAPDOOR + 3,
        "Crimson Trapdoor (open west)",
        "crimson_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_TRAPDOOR + 4,
        "Crimson Trapdoor (open east)",
        "crimson_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_TRAPDOOR,
        "Warped Trapdoor",
        "warped_trapdoor",
        3.0,
        true,
        false,
    ),
    block(
        WARPED_TRAPDOOR + 1,
        "Warped Trapdoor (open north)",
        "warped_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_TRAPDOOR + 2,
        "Warped Trapdoor (open south)",
        "warped_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_TRAPDOOR + 3,
        "Warped Trapdoor (open west)",
        "warped_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_TRAPDOOR + 4,
        "Warped Trapdoor (open east)",
        "warped_trapdoor",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_FENCE_GATE,
        "Crimson Fence Gate",
        "crimson_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        CRIMSON_FENCE_GATE + 1,
        "Crimson Fence Gate (closed south)",
        "crimson_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        CRIMSON_FENCE_GATE + 2,
        "Crimson Fence Gate (closed west)",
        "crimson_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        CRIMSON_FENCE_GATE + 3,
        "Crimson Fence Gate (closed east)",
        "crimson_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        CRIMSON_FENCE_GATE + 4,
        "Crimson Fence Gate (open north)",
        "crimson_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        CRIMSON_FENCE_GATE + 5,
        "Crimson Fence Gate (open south)",
        "crimson_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        CRIMSON_FENCE_GATE + 6,
        "Crimson Fence Gate (open west)",
        "crimson_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        CRIMSON_FENCE_GATE + 7,
        "Crimson Fence Gate (open east)",
        "crimson_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        WARPED_FENCE_GATE,
        "Warped Fence Gate",
        "warped_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        WARPED_FENCE_GATE + 1,
        "Warped Fence Gate (closed south)",
        "warped_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        WARPED_FENCE_GATE + 2,
        "Warped Fence Gate (closed west)",
        "warped_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        WARPED_FENCE_GATE + 3,
        "Warped Fence Gate (closed east)",
        "warped_fence_gate",
        2.0,
        true,
        false,
    ),
    block(
        WARPED_FENCE_GATE + 4,
        "Warped Fence Gate (open north)",
        "warped_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        WARPED_FENCE_GATE + 5,
        "Warped Fence Gate (open south)",
        "warped_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        WARPED_FENCE_GATE + 6,
        "Warped Fence Gate (open west)",
        "warped_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        WARPED_FENCE_GATE + 7,
        "Warped Fence Gate (open east)",
        "warped_fence_gate",
        2.0,
        false,
        false,
    ),
    block(
        CRIMSON_STAIRS,
        "Crimson Stairs",
        "crimson_stairs",
        2.0,
        true,
        false,
    ),
    block(
        CRIMSON_STAIRS + 1,
        "Crimson Stairs (south)",
        "crimson_stairs",
        2.0,
        true,
        false,
    ),
    block(
        CRIMSON_STAIRS + 2,
        "Crimson Stairs (west)",
        "crimson_stairs",
        2.0,
        true,
        false,
    ),
    block(
        CRIMSON_STAIRS + 3,
        "Crimson Stairs (east)",
        "crimson_stairs",
        2.0,
        true,
        false,
    ),
    block(
        WARPED_STAIRS,
        "Warped Stairs",
        "warped_stairs",
        2.0,
        true,
        false,
    ),
    block(
        WARPED_STAIRS + 1,
        "Warped Stairs (south)",
        "warped_stairs",
        2.0,
        true,
        false,
    ),
    block(
        WARPED_STAIRS + 2,
        "Warped Stairs (west)",
        "warped_stairs",
        2.0,
        true,
        false,
    ),
    block(
        WARPED_STAIRS + 3,
        "Warped Stairs (east)",
        "warped_stairs",
        2.0,
        true,
        false,
    ),
    block(
        CRIMSON_BUTTON,
        "Crimson Button",
        "crimson_button",
        0.5,
        false,
        false,
    ),
    block(
        CRIMSON_BUTTON + 1,
        "Crimson Button (south)",
        "crimson_button",
        0.5,
        false,
        false,
    ),
    block(
        CRIMSON_BUTTON + 2,
        "Crimson Button (west)",
        "crimson_button",
        0.5,
        false,
        false,
    ),
    block(
        CRIMSON_BUTTON + 3,
        "Crimson Button (east)",
        "crimson_button",
        0.5,
        false,
        false,
    ),
    block(
        WARPED_BUTTON,
        "Warped Button",
        "warped_button",
        0.5,
        false,
        false,
    ),
    block(
        WARPED_BUTTON + 1,
        "Warped Button (south)",
        "warped_button",
        0.5,
        false,
        false,
    ),
    block(
        WARPED_BUTTON + 2,
        "Warped Button (west)",
        "warped_button",
        0.5,
        false,
        false,
    ),
    block(
        WARPED_BUTTON + 3,
        "Warped Button (east)",
        "warped_button",
        0.5,
        false,
        false,
    ),
    block(
        CRIMSON_DOOR,
        "Crimson Door",
        "crimson_door",
        3.0,
        true,
        false,
    ),
    block(
        CRIMSON_DOOR + 1,
        "Crimson Door (lower closed south)",
        "crimson_door",
        3.0,
        true,
        false,
    ),
    block(
        CRIMSON_DOOR + 2,
        "Crimson Door (lower closed west)",
        "crimson_door",
        3.0,
        true,
        false,
    ),
    block(
        CRIMSON_DOOR + 3,
        "Crimson Door (lower closed east)",
        "crimson_door",
        3.0,
        true,
        false,
    ),
    block(
        CRIMSON_DOOR + 4,
        "Crimson Door (lower open north)",
        "crimson_door",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_DOOR + 5,
        "Crimson Door (lower open south)",
        "crimson_door",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_DOOR + 6,
        "Crimson Door (lower open west)",
        "crimson_door",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_DOOR + 7,
        "Crimson Door (lower open east)",
        "crimson_door",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_DOOR + 8,
        "Crimson Door (upper closed north)",
        "crimson_door",
        3.0,
        true,
        false,
    ),
    block(
        CRIMSON_DOOR + 9,
        "Crimson Door (upper closed south)",
        "crimson_door",
        3.0,
        true,
        false,
    ),
    block(
        CRIMSON_DOOR + 10,
        "Crimson Door (upper closed west)",
        "crimson_door",
        3.0,
        true,
        false,
    ),
    block(
        CRIMSON_DOOR + 11,
        "Crimson Door (upper closed east)",
        "crimson_door",
        3.0,
        true,
        false,
    ),
    block(
        CRIMSON_DOOR + 12,
        "Crimson Door (upper open north)",
        "crimson_door",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_DOOR + 13,
        "Crimson Door (upper open south)",
        "crimson_door",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_DOOR + 14,
        "Crimson Door (upper open west)",
        "crimson_door",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_DOOR + 15,
        "Crimson Door (upper open east)",
        "crimson_door",
        3.0,
        false,
        false,
    ),
    block(WARPED_DOOR, "Warped Door", "warped_door", 3.0, true, false),
    block(
        WARPED_DOOR + 1,
        "Warped Door (lower closed south)",
        "warped_door",
        3.0,
        true,
        false,
    ),
    block(
        WARPED_DOOR + 2,
        "Warped Door (lower closed west)",
        "warped_door",
        3.0,
        true,
        false,
    ),
    block(
        WARPED_DOOR + 3,
        "Warped Door (lower closed east)",
        "warped_door",
        3.0,
        true,
        false,
    ),
    block(
        WARPED_DOOR + 4,
        "Warped Door (lower open north)",
        "warped_door",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_DOOR + 5,
        "Warped Door (lower open south)",
        "warped_door",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_DOOR + 6,
        "Warped Door (lower open west)",
        "warped_door",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_DOOR + 7,
        "Warped Door (lower open east)",
        "warped_door",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_DOOR + 8,
        "Warped Door (upper closed north)",
        "warped_door",
        3.0,
        true,
        false,
    ),
    block(
        WARPED_DOOR + 9,
        "Warped Door (upper closed south)",
        "warped_door",
        3.0,
        true,
        false,
    ),
    block(
        WARPED_DOOR + 10,
        "Warped Door (upper closed west)",
        "warped_door",
        3.0,
        true,
        false,
    ),
    block(
        WARPED_DOOR + 11,
        "Warped Door (upper closed east)",
        "warped_door",
        3.0,
        true,
        false,
    ),
    block(
        WARPED_DOOR + 12,
        "Warped Door (upper open north)",
        "warped_door",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_DOOR + 13,
        "Warped Door (upper open south)",
        "warped_door",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_DOOR + 14,
        "Warped Door (upper open west)",
        "warped_door",
        3.0,
        false,
        false,
    ),
    block(
        WARPED_DOOR + 15,
        "Warped Door (upper open east)",
        "warped_door",
        3.0,
        false,
        false,
    ),
    block(
        CRIMSON_SIGN,
        "Crimson Sign",
        "crimson_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_SIGN + 1,
        "Crimson Sign (south)",
        "crimson_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_SIGN + 2,
        "Crimson Sign (west)",
        "crimson_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_SIGN + 3,
        "Crimson Sign (east)",
        "crimson_sign",
        1.0,
        false,
        false,
    ),
    block(WARPED_SIGN, "Warped Sign", "warped_sign", 1.0, false, false),
    block(
        WARPED_SIGN + 1,
        "Warped Sign (south)",
        "warped_sign",
        1.0,
        false,
        false,
    ),
    block(
        WARPED_SIGN + 2,
        "Warped Sign (west)",
        "warped_sign",
        1.0,
        false,
        false,
    ),
    block(
        WARPED_SIGN + 3,
        "Warped Sign (east)",
        "warped_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_WALL_SIGN,
        "Crimson Wall Sign",
        "crimson_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_WALL_SIGN + 1,
        "Crimson Wall Sign (south)",
        "crimson_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_WALL_SIGN + 2,
        "Crimson Wall Sign (west)",
        "crimson_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        CRIMSON_WALL_SIGN + 3,
        "Crimson Wall Sign (east)",
        "crimson_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        WARPED_WALL_SIGN,
        "Warped Wall Sign",
        "warped_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        WARPED_WALL_SIGN + 1,
        "Warped Wall Sign (south)",
        "warped_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        WARPED_WALL_SIGN + 2,
        "Warped Wall Sign (west)",
        "warped_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        WARPED_WALL_SIGN + 3,
        "Warped Wall Sign (east)",
        "warped_wall_sign",
        1.0,
        false,
        false,
    ),
    block(
        STRUCTURE_BLOCK,
        "Structure Block",
        "structure_block",
        5.0,
        true,
        true,
    ),
    block(JIGSAW, "Jigsaw", "jigsaw", 5.0, true, true),
    block(COMPOSTER, "Composter", "composter", 0.6, true, false),
    block(TARGET, "Target", "target", 0.5, true, true),
    block(BEE_NEST, "Bee Nest", "bee_nest", 0.3, true, true),
    block(BEEHIVE, "Beehive", "beehive", 0.6, true, true),
    block(HONEY_BLOCK, "Honey Block", "honey_block", 0.5, true, false),
    block(
        HONEYCOMB_BLOCK,
        "Honeycomb Block",
        "honeycomb_block",
        0.6,
        true,
        true,
    ),
    block(
        NETHERITE_BLOCK,
        "Netherite Block",
        "netherite_block",
        50.0,
        true,
        true,
    ),
    block(
        ANCIENT_DEBRIS,
        "Ancient Debris",
        "ancient_debris",
        30.0,
        true,
        true,
    ),
    BlockDef {
        light_emission: 10,
        ..block(
            CRYING_OBSIDIAN,
            "Crying Obsidian",
            "crying_obsidian",
            50.0,
            true,
            true,
        )
    },
    block(
        RESPAWN_ANCHOR,
        "Respawn Anchor",
        "respawn_anchor",
        50.0,
        true,
        true,
    ),
    block(
        POTTED_CRIMSON_FUNGUS,
        "Potted Crimson Fungus",
        "potted_crimson_fungus",
        0.05,
        true,
        false,
    ),
    block(
        POTTED_WARPED_FUNGUS,
        "Potted Warped Fungus",
        "potted_warped_fungus",
        0.05,
        true,
        false,
    ),
    block(
        POTTED_CRIMSON_ROOTS,
        "Potted Crimson Roots",
        "potted_crimson_roots",
        0.05,
        true,
        false,
    ),
    block(
        POTTED_WARPED_ROOTS,
        "Potted Warped Roots",
        "potted_warped_roots",
        0.05,
        true,
        false,
    ),
    block(LODESTONE, "Lodestone", "lodestone", 3.5, true, true),
    block(BLACKSTONE, "Blackstone", "blackstone", 1.5, true, true),
    block(
        BLACKSTONE_STAIRS,
        "Blackstone Stairs",
        "blackstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        BLACKSTONE_STAIRS + 1,
        "Blackstone Stairs (south)",
        "blackstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        BLACKSTONE_STAIRS + 2,
        "Blackstone Stairs (west)",
        "blackstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        BLACKSTONE_STAIRS + 3,
        "Blackstone Stairs (east)",
        "blackstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        BLACKSTONE_WALL,
        "Blackstone Wall",
        "blackstone_wall",
        2.0,
        true,
        false,
    ),
    block(
        BLACKSTONE_SLAB,
        "Blackstone Slab",
        "blackstone_slab",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE,
        "Polished Blackstone",
        "polished_blackstone",
        2.0,
        true,
        true,
    ),
    block(
        POLISHED_BLACKSTONE_BRICKS,
        "Polished Blackstone Bricks",
        "polished_blackstone_bricks",
        1.5,
        true,
        true,
    ),
    block(
        CRACKED_POLISHED_BLACKSTONE_BRICKS,
        "Cracked Polished Blackstone Bricks",
        "cracked_polished_blackstone_bricks",
        1.5,
        true,
        true,
    ),
    block(
        CHISELED_POLISHED_BLACKSTONE,
        "Chiseled Polished Blackstone",
        "chiseled_polished_blackstone",
        1.5,
        true,
        true,
    ),
    block(
        POLISHED_BLACKSTONE_BRICK_SLAB,
        "Polished Blackstone Brick Slab",
        "polished_blackstone_brick_slab",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_BRICK_STAIRS,
        "Polished Blackstone Brick Stairs",
        "polished_blackstone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_BRICK_STAIRS + 1,
        "Polished Blackstone Brick Stairs (south)",
        "polished_blackstone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_BRICK_STAIRS + 2,
        "Polished Blackstone Brick Stairs (west)",
        "polished_blackstone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_BRICK_STAIRS + 3,
        "Polished Blackstone Brick Stairs (east)",
        "polished_blackstone_brick_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_BRICK_WALL,
        "Polished Blackstone Brick Wall",
        "polished_blackstone_brick_wall",
        2.0,
        true,
        false,
    ),
    block(
        GILDED_BLACKSTONE,
        "Gilded Blackstone",
        "gilded_blackstone",
        1.5,
        true,
        true,
    ),
    block(
        POLISHED_BLACKSTONE_STAIRS,
        "Polished Blackstone Stairs",
        "polished_blackstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_STAIRS + 1,
        "Polished Blackstone Stairs (south)",
        "polished_blackstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_STAIRS + 2,
        "Polished Blackstone Stairs (west)",
        "polished_blackstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_STAIRS + 3,
        "Polished Blackstone Stairs (east)",
        "polished_blackstone_stairs",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_SLAB,
        "Polished Blackstone Slab",
        "polished_blackstone_slab",
        1.5,
        true,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_PRESSURE_PLATE,
        "Polished Blackstone Pressure Plate",
        "polished_blackstone_pressure_plate",
        0.5,
        false,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_BUTTON,
        "Polished Blackstone Button",
        "polished_blackstone_button",
        0.5,
        false,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_BUTTON + 1,
        "Polished Blackstone Button (south)",
        "polished_blackstone_button",
        0.5,
        false,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_BUTTON + 2,
        "Polished Blackstone Button (west)",
        "polished_blackstone_button",
        0.5,
        false,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_BUTTON + 3,
        "Polished Blackstone Button (east)",
        "polished_blackstone_button",
        0.5,
        false,
        false,
    ),
    block(
        POLISHED_BLACKSTONE_WALL,
        "Polished Blackstone Wall",
        "polished_blackstone_wall",
        2.0,
        true,
        false,
    ),
    block(
        CHISELED_NETHER_BRICKS,
        "Chiseled Nether Bricks",
        "chiseled_nether_bricks",
        2.0,
        true,
        true,
    ),
    block(
        CRACKED_NETHER_BRICKS,
        "Cracked Nether Bricks",
        "cracked_nether_bricks",
        2.0,
        true,
        true,
    ),
    block(
        QUARTZ_BRICKS,
        "Quartz Bricks",
        "quartz_bricks",
        0.8,
        true,
        true,
    ),
    BlockDef {
        light_emission: 3,
        ..block(CANDLE, "Candle", "candle", 0.1, false, false)
    },
    BlockDef {
        light_emission: 3,
        ..block(
            WHITE_CANDLE,
            "White Candle",
            "white_candle",
            0.1,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            ORANGE_CANDLE,
            "Orange Candle",
            "orange_candle",
            0.1,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            MAGENTA_CANDLE,
            "Magenta Candle",
            "magenta_candle",
            0.1,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            LIGHT_BLUE_CANDLE,
            "Light Blue Candle",
            "light_blue_candle",
            0.1,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            YELLOW_CANDLE,
            "Yellow Candle",
            "yellow_candle",
            0.1,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(LIME_CANDLE, "Lime Candle", "lime_candle", 0.1, false, false)
    },
    BlockDef {
        light_emission: 3,
        ..block(PINK_CANDLE, "Pink Candle", "pink_candle", 0.1, false, false)
    },
    BlockDef {
        light_emission: 3,
        ..block(GRAY_CANDLE, "Gray Candle", "gray_candle", 0.1, false, false)
    },
    BlockDef {
        light_emission: 3,
        ..block(
            LIGHT_GRAY_CANDLE,
            "Light Gray Candle",
            "light_gray_candle",
            0.1,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(CYAN_CANDLE, "Cyan Candle", "cyan_candle", 0.1, false, false)
    },
    BlockDef {
        light_emission: 3,
        ..block(
            PURPLE_CANDLE,
            "Purple Candle",
            "purple_candle",
            0.1,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(BLUE_CANDLE, "Blue Candle", "blue_candle", 0.1, false, false)
    },
    BlockDef {
        light_emission: 3,
        ..block(
            BROWN_CANDLE,
            "Brown Candle",
            "brown_candle",
            0.1,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            GREEN_CANDLE,
            "Green Candle",
            "green_candle",
            0.1,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(RED_CANDLE, "Red Candle", "red_candle", 0.1, false, false)
    },
    BlockDef {
        light_emission: 3,
        ..block(
            BLACK_CANDLE,
            "Black Candle",
            "black_candle",
            0.1,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(CANDLE_CAKE, "Candle Cake", "candle_cake", 0.5, true, false)
    },
    BlockDef {
        light_emission: 3,
        ..block(
            WHITE_CANDLE_CAKE,
            "White Candle Cake",
            "white_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            ORANGE_CANDLE_CAKE,
            "Orange Candle Cake",
            "orange_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            MAGENTA_CANDLE_CAKE,
            "Magenta Candle Cake",
            "magenta_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            LIGHT_BLUE_CANDLE_CAKE,
            "Light Blue Candle Cake",
            "light_blue_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            YELLOW_CANDLE_CAKE,
            "Yellow Candle Cake",
            "yellow_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            LIME_CANDLE_CAKE,
            "Lime Candle Cake",
            "lime_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            PINK_CANDLE_CAKE,
            "Pink Candle Cake",
            "pink_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            GRAY_CANDLE_CAKE,
            "Gray Candle Cake",
            "gray_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            LIGHT_GRAY_CANDLE_CAKE,
            "Light Gray Candle Cake",
            "light_gray_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            CYAN_CANDLE_CAKE,
            "Cyan Candle Cake",
            "cyan_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            PURPLE_CANDLE_CAKE,
            "Purple Candle Cake",
            "purple_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            BLUE_CANDLE_CAKE,
            "Blue Candle Cake",
            "blue_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            BROWN_CANDLE_CAKE,
            "Brown Candle Cake",
            "brown_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            GREEN_CANDLE_CAKE,
            "Green Candle Cake",
            "green_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            RED_CANDLE_CAKE,
            "Red Candle Cake",
            "red_candle_cake",
            0.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 3,
        ..block(
            BLACK_CANDLE_CAKE,
            "Black Candle Cake",
            "black_candle_cake",
            0.5,
            true,
            false,
        )
    },
    block(
        BUDDING_AMETHYST,
        "Budding Amethyst",
        "budding_amethyst",
        1.5,
        true,
        true,
    ),
    BlockDef {
        light_emission: 5,
        ..block(
            AMETHYST_CLUSTER,
            "Amethyst Cluster",
            "amethyst_cluster",
            1.5,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 4,
        ..block(
            LARGE_AMETHYST_BUD,
            "Large Amethyst Bud",
            "large_amethyst_bud",
            1.5,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 2,
        ..block(
            MEDIUM_AMETHYST_BUD,
            "Medium Amethyst Bud",
            "medium_amethyst_bud",
            1.5,
            false,
            false,
        )
    },
    BlockDef {
        light_emission: 1,
        ..block(
            SMALL_AMETHYST_BUD,
            "Small Amethyst Bud",
            "small_amethyst_bud",
            1.5,
            false,
            false,
        )
    },
    block(
        TINTED_GLASS,
        "Tinted Glass",
        "tinted_glass",
        0.3,
        true,
        false,
    ),
    block(
        POWDER_SNOW,
        "Powder Snow",
        "powder_snow",
        0.25,
        false,
        false,
    ),
    BlockDef {
        light_emission: 1,
        ..block(
            SCULK_SENSOR,
            "Sculk Sensor",
            "sculk_sensor",
            1.5,
            true,
            false,
        )
    },
    BlockDef {
        light_emission: 1,
        ..block(
            CALIBRATED_SCULK_SENSOR,
            "Calibrated Sculk Sensor",
            "calibrated_sculk_sensor",
            1.5,
            true,
            false,
        )
    },
    block(SCULK, "Sculk", "sculk", 0.6, true, true),
    block(SCULK_VEIN, "Sculk Vein", "sculk_vein", 0.2, false, false),
    BlockDef {
        light_emission: 6,
        ..block(
            SCULK_CATALYST,
            "Sculk Catalyst",
            "sculk_catalyst",
            3.0,
            true,
            true,
        )
    },
    block(
        SCULK_SHRIEKER,
        "Sculk Shrieker",
        "sculk_shrieker",
        3.0,
        true,
        true,
    ),
    block(
        OXIDIZED_COPPER,
        "Oxidized Copper",
        "oxidized_copper",
        3.0,
        true,
        true,
    ),
    block(
        WEATHERED_COPPER,
        "Weathered Copper",
        "weathered_copper",
        3.0,
        true,
        true,
    ),
    block(
        EXPOSED_COPPER,
        "Exposed Copper",
        "exposed_copper",
        3.0,
        true,
        true,
    ),
    block(
        DEEPSLATE_COPPER_ORE,
        "Deepslate Copper Ore",
        "deepslate_copper_ore",
        4.5,
        true,
        true,
    ),
    block(
        OXIDIZED_CUT_COPPER,
        "Oxidized Cut Copper",
        "oxidized_cut_copper",
        3.0,
        true,
        true,
    ),
    block(
        WEATHERED_CUT_COPPER,
        "Weathered Cut Copper",
        "weathered_cut_copper",
        3.0,
        true,
        true,
    ),
    block(
        EXPOSED_CUT_COPPER,
        "Exposed Cut Copper",
        "exposed_cut_copper",
        3.0,
        true,
        true,
    ),
    block(CUT_COPPER, "Cut Copper", "cut_copper", 3.0, true, true),
    block(
        OXIDIZED_CUT_COPPER_STAIRS,
        "Oxidized Cut Copper Stairs",
        "oxidized_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        OXIDIZED_CUT_COPPER_STAIRS + 1,
        "Oxidized Cut Copper Stairs (south)",
        "oxidized_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        OXIDIZED_CUT_COPPER_STAIRS + 2,
        "Oxidized Cut Copper Stairs (west)",
        "oxidized_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        OXIDIZED_CUT_COPPER_STAIRS + 3,
        "Oxidized Cut Copper Stairs (east)",
        "oxidized_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WEATHERED_CUT_COPPER_STAIRS,
        "Weathered Cut Copper Stairs",
        "weathered_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WEATHERED_CUT_COPPER_STAIRS + 1,
        "Weathered Cut Copper Stairs (south)",
        "weathered_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WEATHERED_CUT_COPPER_STAIRS + 2,
        "Weathered Cut Copper Stairs (west)",
        "weathered_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WEATHERED_CUT_COPPER_STAIRS + 3,
        "Weathered Cut Copper Stairs (east)",
        "weathered_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        EXPOSED_CUT_COPPER_STAIRS,
        "Exposed Cut Copper Stairs",
        "exposed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        EXPOSED_CUT_COPPER_STAIRS + 1,
        "Exposed Cut Copper Stairs (south)",
        "exposed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        EXPOSED_CUT_COPPER_STAIRS + 2,
        "Exposed Cut Copper Stairs (west)",
        "exposed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        EXPOSED_CUT_COPPER_STAIRS + 3,
        "Exposed Cut Copper Stairs (east)",
        "exposed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        CUT_COPPER_STAIRS,
        "Cut Copper Stairs",
        "cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        CUT_COPPER_STAIRS + 1,
        "Cut Copper Stairs (south)",
        "cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        CUT_COPPER_STAIRS + 2,
        "Cut Copper Stairs (west)",
        "cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        CUT_COPPER_STAIRS + 3,
        "Cut Copper Stairs (east)",
        "cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        OXIDIZED_CUT_COPPER_SLAB,
        "Oxidized Cut Copper Slab",
        "oxidized_cut_copper_slab",
        3.0,
        true,
        false,
    ),
    block(
        WEATHERED_CUT_COPPER_SLAB,
        "Weathered Cut Copper Slab",
        "weathered_cut_copper_slab",
        3.0,
        true,
        false,
    ),
    block(
        EXPOSED_CUT_COPPER_SLAB,
        "Exposed Cut Copper Slab",
        "exposed_cut_copper_slab",
        3.0,
        true,
        false,
    ),
    block(
        CUT_COPPER_SLAB,
        "Cut Copper Slab",
        "cut_copper_slab",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_COPPER_BLOCK,
        "Waxed Copper Block",
        "waxed_copper_block",
        3.0,
        true,
        true,
    ),
    block(
        WAXED_WEATHERED_COPPER,
        "Waxed Weathered Copper",
        "waxed_weathered_copper",
        3.0,
        true,
        true,
    ),
    block(
        WAXED_EXPOSED_COPPER,
        "Waxed Exposed Copper",
        "waxed_exposed_copper",
        3.0,
        true,
        true,
    ),
    block(
        WAXED_OXIDIZED_COPPER,
        "Waxed Oxidized Copper",
        "waxed_oxidized_copper",
        3.0,
        true,
        true,
    ),
    block(
        WAXED_OXIDIZED_CUT_COPPER,
        "Waxed Oxidized Cut Copper",
        "waxed_oxidized_cut_copper",
        3.0,
        true,
        true,
    ),
    block(
        WAXED_WEATHERED_CUT_COPPER,
        "Waxed Weathered Cut Copper",
        "waxed_weathered_cut_copper",
        3.0,
        true,
        true,
    ),
    block(
        WAXED_EXPOSED_CUT_COPPER,
        "Waxed Exposed Cut Copper",
        "waxed_exposed_cut_copper",
        3.0,
        true,
        true,
    ),
    block(
        WAXED_CUT_COPPER,
        "Waxed Cut Copper",
        "waxed_cut_copper",
        3.0,
        true,
        true,
    ),
    block(
        WAXED_OXIDIZED_CUT_COPPER_STAIRS,
        "Waxed Oxidized Cut Copper Stairs",
        "waxed_oxidized_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_OXIDIZED_CUT_COPPER_STAIRS + 1,
        "Waxed Oxidized Cut Copper Stairs (south)",
        "waxed_oxidized_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_OXIDIZED_CUT_COPPER_STAIRS + 2,
        "Waxed Oxidized Cut Copper Stairs (west)",
        "waxed_oxidized_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_OXIDIZED_CUT_COPPER_STAIRS + 3,
        "Waxed Oxidized Cut Copper Stairs (east)",
        "waxed_oxidized_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_WEATHERED_CUT_COPPER_STAIRS,
        "Waxed Weathered Cut Copper Stairs",
        "waxed_weathered_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_WEATHERED_CUT_COPPER_STAIRS + 1,
        "Waxed Weathered Cut Copper Stairs (south)",
        "waxed_weathered_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_WEATHERED_CUT_COPPER_STAIRS + 2,
        "Waxed Weathered Cut Copper Stairs (west)",
        "waxed_weathered_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_WEATHERED_CUT_COPPER_STAIRS + 3,
        "Waxed Weathered Cut Copper Stairs (east)",
        "waxed_weathered_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_EXPOSED_CUT_COPPER_STAIRS,
        "Waxed Exposed Cut Copper Stairs",
        "waxed_exposed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_EXPOSED_CUT_COPPER_STAIRS + 1,
        "Waxed Exposed Cut Copper Stairs (south)",
        "waxed_exposed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_EXPOSED_CUT_COPPER_STAIRS + 2,
        "Waxed Exposed Cut Copper Stairs (west)",
        "waxed_exposed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_EXPOSED_CUT_COPPER_STAIRS + 3,
        "Waxed Exposed Cut Copper Stairs (east)",
        "waxed_exposed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_CUT_COPPER_STAIRS,
        "Waxed Cut Copper Stairs",
        "waxed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_CUT_COPPER_STAIRS + 1,
        "Waxed Cut Copper Stairs (south)",
        "waxed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_CUT_COPPER_STAIRS + 2,
        "Waxed Cut Copper Stairs (west)",
        "waxed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_CUT_COPPER_STAIRS + 3,
        "Waxed Cut Copper Stairs (east)",
        "waxed_cut_copper_stairs",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_OXIDIZED_CUT_COPPER_SLAB,
        "Waxed Oxidized Cut Copper Slab",
        "waxed_oxidized_cut_copper_slab",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_WEATHERED_CUT_COPPER_SLAB,
        "Waxed Weathered Cut Copper Slab",
        "waxed_weathered_cut_copper_slab",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_EXPOSED_CUT_COPPER_SLAB,
        "Waxed Exposed Cut Copper Slab",
        "waxed_exposed_cut_copper_slab",
        3.0,
        true,
        false,
    ),
    block(
        WAXED_CUT_COPPER_SLAB,
        "Waxed Cut Copper Slab",
        "waxed_cut_copper_slab",
        3.0,
        true,
        false,
    ),
    block(
        LIGHTNING_ROD,
        "Lightning Rod",
        "lightning_rod",
        3.0,
        true,
        false,
    ),
    block(
        POINTED_DRIPSTONE,
        "Pointed Dripstone",
        "pointed_dripstone",
        1.5,
        false,
        false,
    ),
    block(
        DRIPSTONE_BLOCK,
        "Dripstone Block",
        "dripstone_block",
        1.5,
        true,
        true,
    ),
    BlockDef {
        light_emission: 14,
        ..block(CAVE_VINES, "Cave Vines", "cave_vines", 0.05, false, false)
    },
    BlockDef {
        light_emission: 14,
        ..block(
            CAVE_VINES_PLANT,
            "Cave Vines Plant",
            "cave_vines_plant",
            0.05,
            false,
            false,
        )
    },
    block(
        SPORE_BLOSSOM,
        "Spore Blossom",
        "spore_blossom",
        0.05,
        false,
        false,
    ),
    block(AZALEA, "Azalea", "azalea", 0.05, false, false),
    block(
        FLOWERING_AZALEA,
        "Flowering Azalea",
        "flowering_azalea",
        0.05,
        false,
        false,
    ),
    block(MOSS_CARPET, "Moss Carpet", "moss_carpet", 0.1, false, false),
    block(
        PINK_PETALS,
        "Pink Petals",
        "pink_petals",
        0.05,
        false,
        false,
    ),
    block(
        BIG_DRIPLEAF,
        "Big Dripleaf",
        "big_dripleaf",
        0.1,
        false,
        false,
    ),
    block(
        BIG_DRIPLEAF_STEM,
        "Big Dripleaf Stem",
        "big_dripleaf_stem",
        0.05,
        false,
        false,
    ),
    block(
        SMALL_DRIPLEAF,
        "Small Dripleaf",
        "small_dripleaf",
        0.05,
        false,
        false,
    ),
    block(
        HANGING_ROOTS,
        "Hanging Roots",
        "hanging_roots",
        0.05,
        false,
        false,
    ),
    block(ROOTED_DIRT, "Rooted Dirt", "rooted_dirt", 0.5, true, true),
    block(
        COBBLED_DEEPSLATE,
        "Cobbled Deepslate",
        "cobbled_deepslate",
        3.5,
        true,
        true,
    ),
    block(
        COBBLED_DEEPSLATE_STAIRS,
        "Cobbled Deepslate Stairs",
        "cobbled_deepslate_stairs",
        3.5,
        true,
        false,
    ),
    block(
        COBBLED_DEEPSLATE_STAIRS + 1,
        "Cobbled Deepslate Stairs (south)",
        "cobbled_deepslate_stairs",
        3.5,
        true,
        false,
    ),
    block(
        COBBLED_DEEPSLATE_STAIRS + 2,
        "Cobbled Deepslate Stairs (west)",
        "cobbled_deepslate_stairs",
        3.5,
        true,
        false,
    ),
    block(
        COBBLED_DEEPSLATE_STAIRS + 3,
        "Cobbled Deepslate Stairs (east)",
        "cobbled_deepslate_stairs",
        3.5,
        true,
        false,
    ),
    block(
        COBBLED_DEEPSLATE_SLAB,
        "Cobbled Deepslate Slab",
        "cobbled_deepslate_slab",
        3.5,
        true,
        false,
    ),
    block(
        COBBLED_DEEPSLATE_WALL,
        "Cobbled Deepslate Wall",
        "cobbled_deepslate_wall",
        3.5,
        true,
        false,
    ),
    block(
        POLISHED_DEEPSLATE,
        "Polished Deepslate",
        "polished_deepslate",
        3.5,
        true,
        true,
    ),
    block(
        POLISHED_DEEPSLATE_STAIRS,
        "Polished Deepslate Stairs",
        "polished_deepslate_stairs",
        3.5,
        true,
        false,
    ),
    block(
        POLISHED_DEEPSLATE_STAIRS + 1,
        "Polished Deepslate Stairs (south)",
        "polished_deepslate_stairs",
        3.5,
        true,
        false,
    ),
    block(
        POLISHED_DEEPSLATE_STAIRS + 2,
        "Polished Deepslate Stairs (west)",
        "polished_deepslate_stairs",
        3.5,
        true,
        false,
    ),
    block(
        POLISHED_DEEPSLATE_STAIRS + 3,
        "Polished Deepslate Stairs (east)",
        "polished_deepslate_stairs",
        3.5,
        true,
        false,
    ),
    block(
        POLISHED_DEEPSLATE_SLAB,
        "Polished Deepslate Slab",
        "polished_deepslate_slab",
        3.5,
        true,
        false,
    ),
    block(
        POLISHED_DEEPSLATE_WALL,
        "Polished Deepslate Wall",
        "polished_deepslate_wall",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_TILES,
        "Deepslate Tiles",
        "deepslate_tiles",
        3.5,
        true,
        true,
    ),
    block(
        DEEPSLATE_TILE_STAIRS,
        "Deepslate Tile Stairs",
        "deepslate_tile_stairs",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_TILE_STAIRS + 1,
        "Deepslate Tile Stairs (south)",
        "deepslate_tile_stairs",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_TILE_STAIRS + 2,
        "Deepslate Tile Stairs (west)",
        "deepslate_tile_stairs",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_TILE_STAIRS + 3,
        "Deepslate Tile Stairs (east)",
        "deepslate_tile_stairs",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_TILE_SLAB,
        "Deepslate Tile Slab",
        "deepslate_tile_slab",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_TILE_WALL,
        "Deepslate Tile Wall",
        "deepslate_tile_wall",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_BRICKS,
        "Deepslate Bricks",
        "deepslate_bricks",
        3.5,
        true,
        true,
    ),
    block(
        DEEPSLATE_BRICK_STAIRS,
        "Deepslate Brick Stairs",
        "deepslate_brick_stairs",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_BRICK_STAIRS + 1,
        "Deepslate Brick Stairs (south)",
        "deepslate_brick_stairs",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_BRICK_STAIRS + 2,
        "Deepslate Brick Stairs (west)",
        "deepslate_brick_stairs",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_BRICK_STAIRS + 3,
        "Deepslate Brick Stairs (east)",
        "deepslate_brick_stairs",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_BRICK_SLAB,
        "Deepslate Brick Slab",
        "deepslate_brick_slab",
        3.5,
        true,
        false,
    ),
    block(
        DEEPSLATE_BRICK_WALL,
        "Deepslate Brick Wall",
        "deepslate_brick_wall",
        3.5,
        true,
        false,
    ),
    block(
        CHISELED_DEEPSLATE,
        "Chiseled Deepslate",
        "chiseled_deepslate",
        3.5,
        true,
        true,
    ),
    block(
        CRACKED_DEEPSLATE_BRICKS,
        "Cracked Deepslate Bricks",
        "cracked_deepslate_bricks",
        3.5,
        true,
        true,
    ),
    block(
        CRACKED_DEEPSLATE_TILES,
        "Cracked Deepslate Tiles",
        "cracked_deepslate_tiles",
        3.5,
        true,
        true,
    ),
    block(
        INFESTED_DEEPSLATE,
        "Infested Deepslate",
        "infested_deepslate",
        1.5,
        true,
        true,
    ),
    block(
        SMOOTH_BASALT,
        "Smooth Basalt",
        "smooth_basalt",
        1.25,
        true,
        true,
    ),
    block(
        RAW_IRON_BLOCK,
        "Raw Iron Block",
        "raw_iron_block",
        5.0,
        true,
        true,
    ),
    block(
        RAW_COPPER_BLOCK,
        "Raw Copper Block",
        "raw_copper_block",
        5.0,
        true,
        true,
    ),
    block(
        RAW_GOLD_BLOCK,
        "Raw Gold Block",
        "raw_gold_block",
        5.0,
        true,
        true,
    ),
    block(
        POTTED_AZALEA_BUSH,
        "Potted Azalea Bush",
        "potted_azalea_bush",
        0.05,
        true,
        false,
    ),
    block(
        POTTED_FLOWERING_AZALEA_BUSH,
        "Potted Flowering Azalea Bush",
        "potted_flowering_azalea_bush",
        0.05,
        true,
        false,
    ),
    BlockDef {
        light_emission: 15,
        ..block(
            OCHRE_FROGLIGHT,
            "Ochre Froglight",
            "ochre_froglight",
            0.3,
            true,
            true,
        )
    },
    BlockDef {
        light_emission: 15,
        ..block(
            VERDANT_FROGLIGHT,
            "Verdant Froglight",
            "verdant_froglight",
            0.3,
            true,
            true,
        )
    },
    BlockDef {
        light_emission: 15,
        ..block(
            PEARLESCENT_FROGLIGHT,
            "Pearlescent Froglight",
            "pearlescent_froglight",
            0.3,
            true,
            true,
        )
    },
    block(FROGSPAWN, "Frogspawn", "frogspawn", 0.05, false, false),
    block(
        REINFORCED_DEEPSLATE,
        "Reinforced Deepslate",
        "reinforced_deepslate",
        55.0,
        true,
        true,
    ),
    block(
        DECORATED_POT,
        "Decorated Pot",
        "decorated_pot",
        0.05,
        true,
        false,
    ),
    block(
        REDSTONE_WIRE_ON,
        "Redstone Wire (on)",
        "redstone_wire_on",
        0.05,
        false,
        false,
    ),
    BlockDef {
        light_emission: 15,
        ..block(
            REDSTONE_LAMP_LIT,
            "Redstone Lamp (lit)",
            "redstone_lamp_lit",
            0.3,
            true,
            true,
        )
    },
    block(
        REDSTONE_TORCH_OFF,
        "Redstone Torch (off)",
        "redstone_torch_off",
        0.05,
        false,
        false,
    ),
    block(
        REDSTONE_WALL_TORCH_OFF,
        "Redstone Wall Torch (off)",
        "redstone_wall_torch_off",
        0.05,
        false,
        false,
    ),
    block(
        REDSTONE_WALL_TORCH_OFF + 1,
        "Redstone Wall Torch (off) (south)",
        "redstone_wall_torch_off",
        0.05,
        false,
        false,
    ),
    block(
        REDSTONE_WALL_TORCH_OFF + 2,
        "Redstone Wall Torch (off) (west)",
        "redstone_wall_torch_off",
        0.05,
        false,
        false,
    ),
    block(
        REDSTONE_WALL_TORCH_OFF + 3,
        "Redstone Wall Torch (off) (east)",
        "redstone_wall_torch_off",
        0.05,
        false,
        false,
    ),
    block(
        WHEAT_0,
        "Wheat (stage 0)",
        "wheat_stage_0",
        0.05,
        false,
        false,
    ),
    block(
        WHEAT_1,
        "Wheat (stage 1)",
        "wheat_stage_1",
        0.05,
        false,
        false,
    ),
    block(
        WHEAT_2,
        "Wheat (stage 2)",
        "wheat_stage_2",
        0.05,
        false,
        false,
    ),
    block(
        CARROTS_0,
        "Carrots (stage 0)",
        "carrots_stage_0",
        0.05,
        false,
        false,
    ),
    block(
        CARROTS_1,
        "Carrots (stage 1)",
        "carrots_stage_1",
        0.05,
        false,
        false,
    ),
    block(
        CARROTS_2,
        "Carrots (stage 2)",
        "carrots_stage_2",
        0.05,
        false,
        false,
    ),
    block(
        POTATOES_0,
        "Potatoes (stage 0)",
        "potatoes_stage_0",
        0.05,
        false,
        false,
    ),
    block(
        POTATOES_1,
        "Potatoes (stage 1)",
        "potatoes_stage_1",
        0.05,
        false,
        false,
    ),
    block(
        POTATOES_2,
        "Potatoes (stage 2)",
        "potatoes_stage_2",
        0.05,
        false,
        false,
    ),
    block(
        BEETROOTS_0,
        "Beetroots (stage 0)",
        "beetroots_stage_0",
        0.05,
        false,
        false,
    ),
    block(
        BEETROOTS_1,
        "Beetroots (stage 1)",
        "beetroots_stage_1",
        0.05,
        false,
        false,
    ),
    block(
        BEETROOTS_2,
        "Beetroots (stage 2)",
        "beetroots_stage_2",
        0.05,
        false,
        false,
    ),
    block(
        FARMLAND_MOIST,
        "Farmland (moist)",
        "farmland_moist",
        0.6,
        true,
        true,
    ),
    block(
        BAMBOO_SMALL,
        "Bamboo (small leaves)",
        "bamboo_small",
        1.0,
        false,
        false,
    ),
    block(
        BAMBOO_LARGE,
        "Bamboo (large leaves)",
        "bamboo_large",
        1.0,
        false,
        false,
    ),
    block(
        KELP_SPROUT,
        "Kelp (sprout)",
        "kelp_sprout",
        0.05,
        false,
        false,
    ),
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
const RAILS: [[BlockId; 2]; 4] = [
    [ACTIVATOR_RAIL, ACTIVATOR_RAIL + 1],
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

/// Блоки, которых нет в итоговом списке игры. Их ID остаются занятыми, чтобы старые
/// миры читались, но блок всегда заменяется на соответствующий из списка.
pub const RETIRED: [(BlockId, BlockId); 17] = [
    (LAMP, REDSTONE_LAMP),
    (VAULT, BARREL),
    (MARBLE, CALCITE),
    (POLISHED_MARBLE, POLISHED_DIORITE),
    (SLATE, COBBLED_DEEPSLATE),
    (POLISHED_SLATE, POLISHED_DEEPSLATE),
    (LIMESTONE, SANDSTONE),
    (POLISHED_LIMESTONE, CUT_SANDSTONE),
    (PINE_LOG, SPRUCE_LOG),
    (PINE_LEAVES, SPRUCE_LEAVES),
    (PINE_PLANKS, SPRUCE_PLANKS),
    (SANDSTONE_BRICKS, SANDSTONE),
    (DARK_BRICKS, DEEPSLATE_BRICKS),
    (STONE_TILES, DEEPSLATE_TILES),
    (PINE_SLAB, SPRUCE_SLAB),
    (MARBLE_SLAB, DIORITE_SLAB),
    (DARK_BRICK_SLAB, DEEPSLATE_BRICK_SLAB),
];

/// Заменяет снятый с игры блок на блок из итогового списка; остальные не меняются.
#[must_use]
pub fn canonical(id: BlockId) -> BlockId {
    RETIRED
        .iter()
        .find(|(old, _)| *old == id)
        .map_or(id, |(_, new)| *new)
}

#[must_use]
pub fn is_retired(id: BlockId) -> bool {
    RETIRED.iter().any(|(old, _)| *old == id)
}

/// Шалкеровый ящик любого цвета: переносной сундук на 27 ячеек.
#[must_use]
pub fn is_shulker(id: BlockId) -> bool {
    (SHULKER_BOX..=BLACK_SHULKER_BOX).contains(&id)
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
        Some((_, index, FamilyKind::H4 | FamilyKind::H6)) => Some(Facing::ALL[usize::from(index)]),
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
    if let Some((first, _, FamilyKind::H6)) = family_of(base) {
        let index = Facing::ALL.iter().position(|f| *f == facing).unwrap_or(0);
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
    WALL_TORCHES.contains(&id)
        || matches!(
            base_block(id),
            REDSTONE_WALL_TORCH | SOUL_WALL_TORCH | REDSTONE_WALL_TORCH_OFF
        )
}

/// Блок, из которого получается выпадающий предмет: без направления и состояния,
/// а настенный факел и головка поршня дают обычный факел и поршень.
#[must_use]
pub fn drop_base(id: BlockId) -> BlockId {
    match base_block(unlit(id)) {
        WALL_TORCH => TORCH,
        REDSTONE_WALL_TORCH | REDSTONE_WALL_TORCH_OFF => REDSTONE_TORCH,
        SOUL_WALL_TORCH => SOUL_TORCH,
        PISTON_HEAD => PISTON,
        BAMBOO_SMALL | BAMBOO_LARGE => BAMBOO,
        KELP_SPROUT => KELP,
        other => other,
    }
}

/// Растение, факел или огонь: две скрещённые плоскости, без столкновений.
#[must_use]
pub fn is_cross(id: BlockId) -> bool {
    (POPPY..=TALL_GRASS).contains(&id)
        || (PLANT2_FIRST..=PLANT2_LAST).contains(&id)
        || matches!(
            id,
            WHEAT | SUGAR_CANE | REDSTONE_TORCH | SOUL_TORCH | BARRIER | LIGHT | REDSTONE_TORCH_OFF
        )
        || (SUNFLOWER..=LARGE_FERN).contains(&id)
        || (TORCHFLOWER_CROP..=BEETROOTS).contains(&id)
        || (WHEAT_0..=BEETROOTS_2).contains(&id)
        || id == STRUCTURE_VOID
        || crate::shape_table::is_cross(id)
        || matches!(
            id,
            KELP | KELP_PLANT
                | SEA_PICKLE
                | BAMBOO_SAPLING
                | BUBBLE_COLUMN
                | BAMBOO_SMALL
                | BAMBOO_LARGE
                | KELP_SPROUT
        )
        || (DEAD_TUBE_CORAL..=HORN_CORAL_FAN).contains(&id)
        || matches!(
            id,
            VINE | GLOW_LICHEN | NETHER_WART | COCOA | CARROTS | POTATOES
        )
        || (ATTACHED_PUMPKIN_STEM..=MELON_STEM).contains(&id)
        || is_wall_torch(id)
}

/// Плоский рисунок на полу (рельсы).
#[must_use]
pub fn is_flat(id: BlockId) -> bool {
    is_rail(id)
        || matches!(id, REDSTONE_WIRE | REDSTONE_WIRE_ON | LILY_PAD | TRIPWIRE)
        || crate::shape_table::is_flat(id)
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
    matches!(id, MOVING_PISTON | VOID_AIR | CAVE_AIR)
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
        WARPED_STEM => STRIPPED_WARPED_STEM,
        WARPED_HYPHAE => STRIPPED_WARPED_HYPHAE,
        CRIMSON_STEM => STRIPPED_CRIMSON_STEM,
        CRIMSON_HYPHAE => STRIPPED_CRIMSON_HYPHAE,
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
        || (PRISMARINE_SLAB..=DARK_PRISMARINE_SLAB).contains(&id)
        || (CUT_SANDSTONE_SLAB..=PURPUR_SLAB).contains(&id)
        || (POLISHED_GRANITE_SLAB..=DIORITE_SLAB).contains(&id)
        || crate::shape_table::is_slab(id)
        || (SPRUCE_SLAB..=BAMBOO_MOSAIC_SLAB).contains(&id)
}

/// Блок без состояния «горит»: в таком виде он выпадает и сохраняется.
#[must_use]
pub fn unlit(id: BlockId) -> BlockId {
    match id {
        FURNACE_LIT => FURNACE,
        SMOKER_LIT => SMOKER,
        BLAST_FURNACE_LIT => BLAST_FURNACE,
        REDSTONE_LAMP_LIT => REDSTONE_LAMP,
        REDSTONE_WIRE_ON => REDSTONE_WIRE,
        REDSTONE_TORCH_OFF => REDSTONE_TORCH,
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
        assert_eq!(BLOCKS.len(), 1861);
        for (offset, name) in expected.iter().enumerate() {
            let id = 17 + offset;
            assert_eq!(BLOCKS[id].id as usize, id);
            assert_eq!(BLOCKS[id].name, *name);
            assert!(is_solid(id as BlockId) && is_opaque(id as BlockId));
        }
    }

    #[test]
    fn chest_extends_registry_without_changing_foundation_ids() {
        assert_eq!(BLOCKS.len(), 1861);
        let chest = BLOCKS.get(16).expect("сундук зарегистрирован");
        assert_eq!(chest.name, "Chest");
        assert!(is_solid(16));
        assert!(is_opaque(16));
        assert_eq!(BLOCKS.get(15).unwrap().name, "Sandstone");
    }

    #[test]
    fn nether_stems_strip_and_fire_ignores_them() {
        assert_eq!(stripped_variant(WARPED_STEM), Some(STRIPPED_WARPED_STEM));
        assert_eq!(
            stripped_variant(CRIMSON_HYPHAE),
            Some(STRIPPED_CRIMSON_HYPHAE)
        );
        assert_eq!(flammability(WARPED_STEM), 0);
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
