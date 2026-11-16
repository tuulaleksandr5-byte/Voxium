//! Творческий инвентарь как в оригинале: 11 вкладок + «Инвентарь выживания»,
//! поиск по названию и распределение всех предметов по вкладкам.

use crate::content::{ContentRegistry, ItemId, ItemKind};

/// Вкладка творческого меню.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tab {
    Building,
    Colored,
    Natural,
    Functional,
    Redstone,
    Search,
    Tools,
    Combat,
    Food,
    Ingredients,
    Eggs,
    Survival,
}

impl Tab {
    /// Все вкладки в порядке индексов: верхний ряд, поиск, нижний ряд, выживание.
    pub const ALL: [Tab; 12] = [
        Tab::Building,
        Tab::Colored,
        Tab::Natural,
        Tab::Functional,
        Tab::Redstone,
        Tab::Search,
        Tab::Tools,
        Tab::Combat,
        Tab::Food,
        Tab::Ingredients,
        Tab::Eggs,
        Tab::Survival,
    ];
    /// Верхний ряд: пять вкладок блоков и поиск.
    pub const TOP: [Tab; 6] = [
        Tab::Building,
        Tab::Colored,
        Tab::Natural,
        Tab::Functional,
        Tab::Redstone,
        Tab::Search,
    ];
    /// Нижний ряд: предметы и инвентарь выживания.
    pub const BOTTOM: [Tab; 6] = [
        Tab::Tools,
        Tab::Combat,
        Tab::Food,
        Tab::Ingredients,
        Tab::Eggs,
        Tab::Survival,
    ];

    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Tab::Building => "Строительные блоки",
            Tab::Colored => "Разноцветные блоки",
            Tab::Natural => "Природные блоки",
            Tab::Functional => "Функциональные блоки",
            Tab::Redstone => "Редстоун",
            Tab::Search => "Поиск предметов",
            Tab::Tools => "Инструменты и приспособления",
            Tab::Combat => "Боевые предметы",
            Tab::Food => "Еда и напитки",
            Tab::Ingredients => "Ингредиенты",
            Tab::Eggs => "Яйца призыва",
            Tab::Survival => "Инвентарь выживания",
        }
    }

    #[must_use]
    pub fn index(self) -> usize {
        Self::ALL.iter().position(|tab| *tab == self).unwrap_or(0)
    }

    #[must_use]
    pub fn from_index(index: usize) -> Self {
        Self::ALL[index.min(Self::ALL.len() - 1)]
    }
}

/// Столбцов и рядов в сетке творческого меню.
pub const GRID_COLUMNS: usize = 9;
pub const GRID_ROWS: usize = 5;

const COLORS: [&str; 16] = [
    "white",
    "orange",
    "magenta",
    "light_blue",
    "yellow",
    "lime",
    "pink",
    "gray",
    "light_gray",
    "cyan",
    "purple",
    "blue",
    "brown",
    "green",
    "red",
    "black",
];

fn colored_name(name: &str) -> bool {
    let colored_kind = |kind: &str| {
        name.ends_with(kind) && COLORS.iter().any(|c| name.starts_with(&format!("{c}_")))
    };
    [
        "_wool",
        "_carpet",
        "_concrete",
        "_concrete_powder",
        "_terracotta",
        "_glazed_terracotta",
        "_stained_glass",
        "_stained_glass_pane",
        "_banner",
        "_shulker_box",
        "_candle",
        "_bed",
    ]
    .iter()
    .any(|kind| colored_kind(kind))
        || matches!(name, "wool" | "terracotta" | "shulker_box" | "candle")
        || ["wool_", "concrete_", "glass_", "terracotta_"]
            .iter()
            .any(|p| name.strip_prefix(p).is_some_and(|c| COLORS.contains(&c)))
}

fn redstone_name(name: &str) -> bool {
    matches!(
        name,
        "redstone"
            | "redstone_torch"
            | "redstone_block"
            | "redstone_lamp"
            | "repeater"
            | "comparator"
            | "piston"
            | "sticky_piston"
            | "observer"
            | "dispenser"
            | "dropper"
            | "hopper"
            | "lever"
            | "tnt"
            | "note_block"
            | "target"
            | "tripwire_hook"
            | "daylight_detector"
            | "powered_rail"
            | "detector_rail"
            | "activator_rail"
            | "rail"
            | "lightning_rod"
            | "sculk_sensor"
            | "calibrated_sculk_sensor"
            | "bell"
    ) || name.ends_with("_pressure_plate")
        || name.ends_with("_button")
        || name.ends_with("_door")
        || name.ends_with("_trapdoor")
        || name.ends_with("_fence_gate")
}

fn functional_name(name: &str) -> bool {
    matches!(
        name,
        "crafting_table"
            | "furnace"
            | "blast_furnace"
            | "smoker"
            | "chest"
            | "trapped_chest"
            | "ender_chest"
            | "barrel"
            | "anvil"
            | "chipped_anvil"
            | "damaged_anvil"
            | "grindstone"
            | "stonecutter"
            | "loom"
            | "cartography_table"
            | "fletching_table"
            | "smithing_table"
            | "lectern"
            | "composter"
            | "cauldron"
            | "brewing_stand"
            | "enchanting_table"
            | "bookshelf"
            | "chiseled_bookshelf"
            | "jukebox"
            | "beacon"
            | "conduit"
            | "spawner"
            | "command_block"
            | "repeating_command_block"
            | "chain_command_block"
            | "structure_block"
            | "jigsaw"
            | "ladder"
            | "scaffolding"
            | "torch"
            | "soul_torch"
            | "lantern"
            | "soul_lantern"
            | "campfire"
            | "soul_campfire"
            | "end_rod"
            | "chain"
            | "flower_pot"
            | "beehive"
            | "bee_nest"
            | "decorated_pot"
            | "respawn_anchor"
            | "lodestone"
            | "sea_lantern"
            | "glowstone"
            | "shroomlight"
            | "jack_o_lantern"
            | "end_portal_frame"
            | "dragon_egg"
            | "barrier"
            | "light"
            | "bed"
            | "cake"
            | "lava_cauldron"
            | "water_cauldron"
            | "powder_snow_cauldron"
            | "vault"
            | "lamp"
            | "iron_bars"
            | "glass_pane"
    ) || name.ends_with("_sign")
        || name.ends_with("_hanging_sign")
        || name.ends_with("_skull")
        || name.ends_with("_head")
        || name.ends_with("_candle_cake")
}

fn natural_name(name: &str) -> bool {
    const SIMPLE: [&str; 11] = [
        "grass",
        "dirt",
        "coarse_dirt",
        "podzol",
        "rooted_dirt",
        "mud",
        "clay",
        "gravel",
        "sand",
        "red_sand",
        "snow",
    ];
    if SIMPLE.contains(&name) {
        return true;
    }
    name.ends_with("_ore")
        || name.ends_with("_log")
        || name.ends_with("_leaves")
        || name.ends_with("_sapling")
        || name.ends_with("_stem")
        || name.ends_with("_hyphae")
        || name.ends_with("_wood") && !name.contains("planks")
        || name.ends_with("_coral")
        || name.ends_with("_coral_block")
        || name.ends_with("_coral_fan")
        || name.ends_with("_wall_fan")
        || name.ends_with("_fungus")
        || name.ends_with("_roots")
        || name.ends_with("_vines")
        || name.ends_with("_vines_plant")
        || name.ends_with("_nylium")
        || name.ends_with("_tulip")
        || name.ends_with("_mushroom")
        || name.ends_with("_mushroom_block")
        || name.ends_with("_amethyst_bud")
        || name.ends_with("_dripleaf")
        || matches!(
            name,
            "stone"
                | "granite"
                | "diorite"
                | "andesite"
                | "deepslate"
                | "calcite"
                | "tuff"
                | "dripstone_block"
                | "pointed_dripstone"
                | "bedrock"
                | "water"
                | "lava"
                | "ice"
                | "packed_ice"
                | "blue_ice"
                | "snow_block"
                | "moss_block"
                | "moss_carpet"
                | "netherrack"
                | "soul_sand"
                | "soul_soil"
                | "basalt"
                | "smooth_basalt"
                | "blackstone"
                | "end_stone"
                | "obsidian"
                | "crying_obsidian"
                | "magma_block"
                | "glow_lichen"
                | "sculk"
                | "sculk_vein"
                | "sculk_catalyst"
                | "sculk_shrieker"
                | "amethyst_cluster"
                | "budding_amethyst"
                | "amethyst_block"
                | "sponge"
                | "wet_sponge"
                | "dead_bush"
                | "fern"
                | "tall_grass"
                | "large_fern"
                | "poppy"
                | "dandelion"
                | "cornflower"
                | "daisy"
                | "oxeye_daisy"
                | "blue_orchid"
                | "allium"
                | "azure_bluet"
                | "lily_of_the_valley"
                | "wither_rose"
                | "torchflower"
                | "sunflower"
                | "lilac"
                | "rose_bush"
                | "peony"
                | "pink_petals"
                | "lily_pad"
                | "vine"
                | "cactus"
                | "sugar_cane"
                | "bamboo"
                | "kelp"
                | "kelp_plant"
                | "sea_pickle"
                | "seagrass"
                | "tall_seagrass"
                | "pumpkin"
                | "melon"
                | "hay_bale"
                | "hay_block"
                | "cobweb"
                | "mycelium"
                | "dirt_path"
                | "farmland"
                | "azalea"
                | "flowering_azalea"
                | "mangrove_propagule"
                | "cave_vines"
                | "cave_vines_plant"
                | "spore_blossom"
                | "hanging_roots"
                | "nether_wart"
                | "nether_sprouts"
                | "wheat"
                | "carrots"
                | "potatoes"
                | "beetroots"
                | "turtle_egg"
                | "sniffer_egg"
                | "frogspawn"
                | "dried_kelp_block"
                | "weeping_vines"
                | "twisting_vines"
                | "snow_layer"
                | "powder_snow"
                | "bubble_column"
                | "grass_block"
        )
}

/// Вкладка для предмета.
#[must_use]
pub fn tab_of(registry: &ContentRegistry, id: ItemId) -> Tab {
    let Some(item) = registry.get(id) else {
        return Tab::Ingredients;
    };
    let name = item.id.split_once(':').map_or(item.id.as_str(), |(_, n)| n);
    if name.ends_with("_spawn_egg") {
        return Tab::Eggs;
    }
    match &item.kind {
        ItemKind::Armor { .. } => Tab::Combat,
        ItemKind::Food => Tab::Food,
        ItemKind::Tool => {
            if name.ends_with("_sword") || matches!(name, "bow" | "crossbow" | "trident" | "shield")
            {
                Tab::Combat
            } else {
                Tab::Tools
            }
        }
        ItemKind::Material => {
            if name.ends_with("_bucket")
                && matches!(
                    name,
                    "water_bucket" | "lava_bucket" | "bucket" | "milk_bucket"
                )
                || name.ends_with("_boat")
                || name.ends_with("_raft")
                || name.ends_with("minecart")
                || matches!(
                    name,
                    "bucket"
                        | "map"
                        | "filled_map"
                        | "painting"
                        | "item_frame"
                        | "glow_item_frame"
                        | "armor_stand"
                        | "name_tag"
                        | "lead"
                        | "bundle"
                        | "firework_rocket"
                        | "ender_pearl"
                        | "snowball"
                        | "end_crystal"
                )
                || name.ends_with("_bucket")
            {
                Tab::Tools
            } else if matches!(
                name,
                "arrow"
                    | "spectral_arrow"
                    | "tipped_arrow"
                    | "totem_of_undying"
                    | "iron_horse_armor"
                    | "golden_horse_armor"
                    | "diamond_horse_armor"
                    | "leather_horse_armor"
            ) {
                Tab::Combat
            } else if matches!(
                name,
                "potion"
                    | "splash_potion"
                    | "lingering_potion"
                    | "honey_bottle"
                    | "experience_bottle"
                    | "dragon_breath"
            ) {
                Tab::Food
            } else {
                Tab::Ingredients
            }
        }
        ItemKind::Block { .. } => {
            if colored_name(name) {
                Tab::Colored
            } else if redstone_name(name) {
                Tab::Redstone
            } else if functional_name(name) {
                Tab::Functional
            } else if natural_name(name) {
                Tab::Natural
            } else {
                Tab::Building
            }
        }
    }
}

/// Предметы вкладки, а для поиска — всех вкладок, названия которых содержат запрос.
#[must_use]
pub fn items(registry: &ContentRegistry, tab: Tab, query: &str) -> Vec<ItemId> {
    let query = query.trim().to_lowercase();
    registry
        .iter()
        .filter(|(id, item)| match tab {
            Tab::Survival => false,
            Tab::Search => query.is_empty() || item.name.to_lowercase().contains(&query),
            other => tab_of(registry, *id) == other,
        })
        .map(|(id, _)| id)
        .collect()
}

/// Сколько рядов можно прокрутить в списке из `count` предметов.
#[must_use]
pub fn max_scroll(count: usize) -> usize {
    count.div_ceil(GRID_COLUMNS).saturating_sub(GRID_ROWS)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tab(registry: &ContentRegistry, name: &str) -> Tab {
        tab_of(
            registry,
            registry.find(name).unwrap_or_else(|| panic!("{name}")),
        )
    }

    #[test]
    fn every_item_lands_in_exactly_one_tab_and_all_tabs_are_populated() {
        let registry = ContentRegistry::default();
        let mut counts = std::collections::HashMap::new();
        for (id, _) in registry.iter() {
            *counts.entry(tab_of(&registry, id)).or_insert(0_usize) += 1;
        }
        let total: usize = counts.values().sum();
        assert_eq!(total, registry.len());
        for t in Tab::ALL {
            if t != Tab::Search && t != Tab::Survival {
                assert!(
                    counts.get(&t).copied().unwrap_or(0) >= 10,
                    "{t:?}: {:?}",
                    counts.get(&t)
                );
            }
        }
        let by_tab: usize = Tab::ALL
            .iter()
            .map(|t| items(&registry, *t, "").len())
            .sum();
        // Поиск без запроса показывает всё, «Выживание» — ничего.
        assert_eq!(by_tab, registry.len() * 2);
    }

    #[test]
    fn well_known_items_are_in_the_vanilla_tabs() {
        let registry = ContentRegistry::default();
        assert_eq!(tab(&registry, "foundation:stone"), Tab::Natural);
        assert_eq!(tab(&registry, "builder:bricks"), Tab::Building);
        assert_eq!(tab(&registry, "voxium:oak_stairs"), Tab::Building);
        assert_eq!(tab(&registry, "voxium:red_carpet"), Tab::Colored);
        assert_eq!(tab(&registry, "palette:wool_orange"), Tab::Colored);
        assert_eq!(tab(&registry, "voxium:lever"), Tab::Redstone);
        assert_eq!(tab(&registry, "voxium:oak_door"), Tab::Redstone);
        assert_eq!(tab(&registry, "survival:chest"), Tab::Functional);
        assert_eq!(tab(&registry, "voxium:oak_sign"), Tab::Functional);
        assert_eq!(tab(&registry, "survival:diamond_pickaxe"), Tab::Tools);
        assert_eq!(tab(&registry, "survival:diamond_sword"), Tab::Combat);
        assert_eq!(tab(&registry, "survival:diamond_helmet"), Tab::Combat);
        assert_eq!(tab(&registry, "voxium:apple"), Tab::Food);
        assert_eq!(tab(&registry, "voxium:cow_spawn_egg"), Tab::Eggs);
        assert_eq!(tab(&registry, "voxium:redstone"), Tab::Redstone);
        assert_eq!(tab(&registry, "voxium:gunpowder"), Tab::Ingredients);
        assert_eq!(tab(&registry, "voxium:oak_boat"), Tab::Tools);
    }

    #[test]
    fn search_matches_names_case_insensitively_and_scroll_is_bounded() {
        let registry = ContentRegistry::default();
        let found = items(&registry, Tab::Search, "АЛМАЗН");
        assert!(found.len() >= 5);
        for id in &found {
            assert!(
                registry
                    .get(*id)
                    .unwrap()
                    .name
                    .to_lowercase()
                    .contains("алмазн")
            );
        }
        assert!(items(&registry, Tab::Search, "zzzz-нет-такого").is_empty());
        assert_eq!(max_scroll(0), 0);
        assert_eq!(max_scroll(45), 0);
        assert_eq!(max_scroll(46), 1);
        assert_eq!(max_scroll(90), 5);
        assert_eq!(Tab::from_index(99), Tab::Survival);
        assert_eq!(Tab::Search.index(), 5);
    }
}
