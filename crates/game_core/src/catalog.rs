//! Справочник всех блоков игры. Таблицы в `docs/BLOCKS.md` и `docs/BLOCKS.csv`
//! создаёт пример `block_list` из реестров, поэтому не расходятся с кодом:
//! тест падает, если файлы устарели.
//!
//! Обновить: `cargo run -p game_core --example block_list`.

use crate::{
    BLOCKS, BlockDef,
    content::{BLOCK_NAMES, ContentRegistry},
    fluids::{FluidKind, fluid_of},
    is_fluid, is_plant, is_slab,
    mining::{MiningConfig, ToolKind},
    unlit,
};

/// Виды блоков в порядке сводной таблицы.
const KINDS: [&str; 12] = [
    "куб",
    "прозрачный куб",
    "листва",
    "плита",
    "растение",
    "жидкость",
    "функциональный",
    "низкий блок",
    "плоский",
    "технический",
    "особая форма",
    "пустота",
];

fn tier_name(tier: u8) -> &'static str {
    match tier {
        0 => "любая",
        1 => "деревянная",
        2 => "каменная",
        3 => "железная",
        _ => "алмазная",
    }
}

fn tool_name(tool: ToolKind) -> &'static str {
    match tool {
        ToolKind::Hand => "рука",
        ToolKind::Pickaxe => "кирка",
        ToolKind::Axe => "топор",
        ToolKind::Shovel => "лопата",
        ToolKind::Hoe => "мотыга",
    }
}

fn kind_of(id: crate::BlockId, def: &BlockDef) -> &'static str {
    if id == crate::AIR {
        "пустота"
    } else if is_fluid(id) {
        "жидкость"
    } else if crate::is_invisible(id) {
        "технический"
    } else if crate::is_flat(id) {
        "плоский"
    } else if is_plant(id) {
        "растение"
    } else if is_slab(id) {
        "плита"
    } else if crate::is_half(id) {
        "низкий блок"
    } else if crate::is_leaves(id) {
        "листва"
    } else if matches!(
        crate::base_block(id),
        crate::CHEST
            | crate::BARREL
            | crate::VAULT
            | crate::STONECUTTER
            | crate::CRAFTING_TABLE
            | crate::DISPENSER
    ) || crate::smelting::FurnaceKind::of_block(id).is_some()
    {
        "функциональный"
    } else if crate::models::shape_of(id).is_some() {
        "особая форма"
    } else if !def.opacity {
        "прозрачный куб"
    } else {
        "куб"
    }
}

fn function_note(id: crate::BlockId) -> &'static str {
    use crate::models::Shape;
    match crate::models::shape_of(id) {
        Some(Shape::Door | Shape::Trapdoor | Shape::Gate) => return "ПКМ: открыть и закрыть",
        Some(Shape::Lever) => return "ПКМ: переключить",
        Some(Shape::Ladder) => return "лазание: пробел или движение вперёд",
        Some(Shape::Stairs) => return "ступени смотрят на игрока при установке",
        Some(Shape::Fence | Shape::Pane | Shape::Wall) => return "соединяется с соседями",
        _ => {}
    }
    if id == crate::NETHER_PORTAL {
        return "декоративный, светится";
    }
    if id >= crate::PART2_FIRST && BLOCKS[usize::from(id)].light_emission > 0 {
        return "светится";
    }
    #[allow(clippy::match_overlapping_arm)]
    match crate::base_block(id) {
        crate::CHEST | crate::BARREL => "хранилище на 27 ячеек",
        crate::VAULT => "общее хранилище: все такие блоки открывают одно",
        crate::CRAFTING_TABLE => "крафт 3×3",
        crate::STONECUTTER => "резка камня один к одному",
        crate::FURNACE | crate::FURNACE_LIT => "печь: плавка и жарка",
        crate::SMOKER | crate::SMOKER_LIT => "коптильня: еда вдвое быстрее",
        crate::BLAST_FURNACE | crate::BLAST_FURNACE_LIT => "плавильня: руда вдвое быстрее",
        crate::DISPENSER => "хранилище на 9 ячеек",
        crate::WHITE_BED..=crate::BLACK_BED => "ПКМ: точка возрождения, ночь пропускается",
        crate::COBWEB => "замедляет движение",
        crate::FIRE | crate::SOUL_FIRE => "жжёт",
        crate::TORCH | crate::WALL_TORCH => "светится",
        crate::MOVING_PISTON | crate::PISTON_HEAD => "технический блок",
        crate::TNT => "декоративный",
        crate::NOTE_BLOCK | crate::PISTON | crate::STICKY_PISTON => "декоративный, без редстоуна",
        crate::LAMP => "светится",
        crate::LAVA => "жжёт, течёт на 3 клетки",
        crate::WATER => "течёт на 7 клеток, плавание",
        _ => "",
    }
}

/// Подпись варианта блока внутри семейства: сторона, ось, состояние.
fn variant_label(id: crate::BlockId) -> String {
    use crate::FamilyKind;
    let side = |index: u8| ["север", "юг", "запад", "восток"][usize::from(index & 3)];
    if let Some(facing) = crate::facing_of(id) {
        return facing.label().to_string();
    }
    match crate::family_of(id) {
        Some((_, index, FamilyKind::H4 | FamilyKind::H6)) => side(index).to_string(),
        Some((_, index, FamilyKind::AX2)) => {
            format!("вдоль {}", if index == 0 { "X" } else { "Z" })
        }
        Some((_, index, FamilyKind::Lever)) => {
            format!(
                "{}, {}",
                if index >= 4 {
                    "включён"
                } else {
                    "выключен"
                },
                side(index)
            )
        }
        Some((_, index, FamilyKind::Door)) => format!(
            "{}, {}, {}",
            if index >= 8 { "верх" } else { "низ" },
            if (index / 4) % 2 == 1 {
                "открыта"
            } else {
                "закрыта"
            },
            side(index)
        ),
        Some((_, 0, FamilyKind::Trap)) => "закрыт".to_string(),
        Some((_, index, FamilyKind::Trap)) => format!("открыт, {}", side(index - 1)),
        Some((_, index, FamilyKind::Gate)) => format!(
            "{}, {}",
            if index >= 4 {
                "открыта"
            } else {
                "закрыта"
            },
            side(index)
        ),
        Some((_, _, FamilyKind::Rail)) | None => "вдоль X".to_string(),
    }
}

/// Русское название блока: по предмету или по описанию состояния.
fn display_name(id: crate::BlockId, registry: &ContentRegistry) -> String {
    if id == crate::AIR {
        return "Воздух".into();
    }
    if id == crate::BEDROCK {
        return "Коренная порода".into();
    }
    let base_id = crate::base_block(id);
    if base_id != id {
        let name = display_name(base_id, registry);
        return format!("{name} ({})", variant_label(id));
    }
    if let Some(item) = registry
        .item_for_block(id)
        .and_then(|item| registry.get(item))
    {
        return item.name.clone();
    }
    if let Some((kind, level)) = fluid_of(id) {
        let name = match kind {
            FluidKind::Water => "Вода",
            FluidKind::Lava => "Лава",
        };
        return format!("{name} (течение {level})");
    }
    match id {
        crate::WALL_TORCH => return "Настенный факел".into(),
        crate::PISTON_HEAD => return "Головка поршня".into(),
        crate::MOVING_PISTON => return "Движущийся поршень".into(),
        _ => {}
    }
    let base = unlit(id);
    if base != id
        && let Some(item) = registry
            .item_for_block(base)
            .and_then(|item| registry.get(item))
    {
        return format!("{} (горит)", item.name);
    }
    BLOCKS[usize::from(id)].name.into()
}

struct Row {
    id: crate::BlockId,
    name_id: &'static str,
    name: String,
    kind: &'static str,
    solid: bool,
    opaque: bool,
    light: u8,
    hardness: f32,
    tool: String,
    drop: String,
    note: &'static str,
}

fn rows() -> Vec<Row> {
    let registry = ContentRegistry::default();
    let mining = MiningConfig::default();
    BLOCKS
        .iter()
        .filter(|def| !crate::is_retired(def.id))
        .map(|def| {
            let id = def.id;
            let tool = match mining.rule_info(id) {
                Some((tool, tier)) => format!("{}, {}", tool_name(tool), tier_name(tier)),
                None if id == crate::BEDROCK => "нельзя сломать".into(),
                None => "—".into(),
            };
            let drop = registry
                .drop_for_block(id)
                .and_then(|item| registry.get(item))
                .map_or_else(|| "—".into(), |item| item.name.clone());
            Row {
                id,
                name_id: BLOCK_NAMES[usize::from(id)],
                name: display_name(id, &registry),
                kind: kind_of(id, def),
                solid: def.collision,
                opaque: def.opacity,
                light: def.light_emission,
                hardness: def.hardness,
                tool,
                drop,
                note: function_note(id),
            }
        })
        .collect()
}

fn yes(flag: bool) -> &'static str {
    if flag { "да" } else { "нет" }
}

/// Таблица в формате Markdown.
#[must_use]
pub fn markdown() -> String {
    let rows = rows();
    let count = |kind: &str| rows.iter().filter(|row| row.kind == kind).count();
    let mut out = String::from("# Все блоки игры\n\n");
    out.push_str(
        "Файл создаёт `cargo run -p game_core --example block_list`; не правьте его вручную. ",
    );
    out.push_str("То же в таблице для программ: [BLOCKS.csv](BLOCKS.csv).\n\n");
    out.push_str(&format!(
        "Всего типов блоков: **{}** (ID 0…{}).\n\n",
        rows.len(),
        rows.len() - 1
    ));
    out.push_str("| Тип | Сколько |\n|---|---|\n");
    for kind in KINDS {
        out.push_str(&format!("| {kind} | {} |\n", count(kind)));
    }
    out.push_str(
        "\nСтолбцы: «Тв.» — есть столкновение, «Непр.» — непрозрачен, «Свет» — яркость свечения, ",
    );
    out.push_str("«Прочн.» — твёрдость, «Инструмент» — чем добывать и с какого уровня. ");
    out.push_str(
        "Горящие печи и течения жидкостей — состояния, отдельных предметов у них нет.\n\n",
    );
    out.push_str("| ID | Идентификатор | Название | Вид | Тв. | Непр. | Свет | Прочн. | Инструмент | Выпадает | Примечание |\n");
    out.push_str("|---|---|---|---|---|---|---|---|---|---|---|\n");
    for row in &rows {
        out.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.id,
            row.name_id,
            row.name,
            row.kind,
            yes(row.solid),
            yes(row.opaque),
            row.light,
            row.hardness,
            row.tool,
            row.drop,
            row.note
        ));
    }
    out
}

/// Та же таблица в CSV с разделителем `;` и BOM для Excel.
#[must_use]
pub fn csv() -> String {
    let mut out = String::from(
        "\u{feff}id;identifier;name;kind;solid;opaque;light;hardness;tool;drop;note\n",
    );
    for row in rows() {
        out.push_str(&format!(
            "{};{};{};{};{};{};{};{};{};{};{}\n",
            row.id,
            row.name_id,
            row.name,
            row.kind,
            yes(row.solid),
            yes(row.opaque),
            row.light,
            row.hardness,
            row.tool,
            row.drop,
            row.note
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_block_has_exactly_one_row_with_a_unique_identifier() {
        let rows = rows();
        assert_eq!(rows.len(), BLOCKS.len() - crate::RETIRED.len());
        let mut names = std::collections::HashSet::new();
        let mut previous = None;
        for row in &rows {
            assert!(previous.is_none_or(|p| row.id > p));
            assert!(!crate::is_retired(row.id));
            previous = Some(row.id);
            assert!(names.insert(row.name_id), "{}", row.name_id);
            assert!(!row.name.trim().is_empty());
        }
        let kinds: usize = KINDS
            .iter()
            .map(|kind| rows.iter().filter(|row| row.kind == *kind).count())
            .sum();
        assert_eq!(kinds, rows.len(), "у каждого блока есть вид");
    }

    /// Имя блока в игре → имя из итогового списка (старые имена палитры и цвета).
    fn list_name(short: &str) -> Vec<String> {
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
        let rest = |prefix: &str| short.strip_prefix(prefix).filter(|c| COLORS.contains(c));
        if short == "wool" {
            return vec!["white_wool".into()];
        }
        if let Some(c) = rest("wool_") {
            return vec![format!("{c}_wool")];
        }
        if let Some(c) = rest("concrete_") {
            return vec![format!("{c}_concrete")];
        }
        if let Some(c) = rest("glass_") {
            return vec![format!("{c}_stained_glass")];
        }
        if let Some(c) = rest("terracotta_") {
            return vec![format!("{c}_terracotta")];
        }
        match short {
            "daisy" => vec!["oxeye_daisy".into()],
            "hay_bale" => vec!["hay_block".into()],
            "snow" => vec!["snow_block".into()],
            "snow_layer" => vec!["snow".into()],
            "grass" => vec!["grass".into(), "grass_block".into()],
            other => vec![other.into()],
        }
    }

    #[test]
    fn the_game_has_exactly_the_blocks_of_the_final_list() {
        let list: std::collections::BTreeSet<&str> =
            include_str!("../../../docs/MASTER_BLOCKS_1.20.1.txt")
                .lines()
                .filter(|line| line.starts_with(|c: char| c.is_ascii_digit()))
                .filter_map(|line| line.split(" | ").nth(2))
                .map(str::trim)
                .collect();
        assert_eq!(list.len(), 1003);
        let mut game = std::collections::BTreeSet::new();
        for id in 0..BLOCKS.len() as crate::BlockId {
            let flowing =
                crate::fluids::fluid_of(id).is_some() && id != crate::WATER && id != crate::LAVA;
            if crate::is_retired(id)
                || crate::base_block(id) != id
                || flowing
                || crate::unlit(id) != id
                || matches!(id, crate::REDSTONE_WALL_TORCH_OFF)
                || (crate::WHEAT_0..=crate::KELP_SPROUT).contains(&id)
            {
                continue;
            }
            let name = crate::content::BLOCK_NAMES[usize::from(id)];
            for listed in list_name(name.split_once(':').map_or(name, |(_, short)| short)) {
                game.insert(listed);
            }
        }
        let extra: Vec<_> = game
            .iter()
            .filter(|name| !list.contains(name.as_str()))
            .collect();
        let missing: Vec<_> = list.iter().filter(|name| !game.contains(**name)).collect();
        assert!(extra.is_empty(), "лишние блоки: {extra:?}");
        assert!(missing.is_empty(), "нет блоков из списка: {missing:?}");
    }

    #[test]
    fn retired_blocks_are_replaced_everywhere_and_never_listed() {
        let mut world = crate::World::new(1);
        world.chunks.insert(
            [0, 0],
            std::sync::Arc::new(crate::Chunk {
                position: [0, 0],
                sections: Default::default(),
            }),
        );
        for (old, new) in crate::RETIRED {
            assert!(!crate::is_retired(new), "{old} → {new}");
            assert_eq!(crate::canonical(old), new);
            assert!(world.set([1, 100, 1], old));
            assert_eq!(world.block([1, 100, 1]), new);
            world.set([1, 100, 1], crate::AIR);
        }
        assert!(rows().iter().all(|row| !crate::is_retired(row.id)));
    }

    #[test]
    fn every_item_of_the_final_list_exists_in_the_game() {
        let list: Vec<&str> = include_str!("../../../docs/MASTER_ITEMS_1.20.1.txt")
            .lines()
            .filter(|line| line.starts_with(|c: char| c.is_ascii_digit()))
            .filter_map(|line| line.split(" | ").nth(2))
            .map(str::trim)
            .collect();
        assert_eq!(list.len(), 1255);
        let registry = ContentRegistry::default();
        let mut game = std::collections::BTreeSet::new();
        for (_, item) in registry.iter() {
            let short = item.id.split_once(':').map_or(item.id.as_str(), |(_, s)| s);
            for name in list_name(short) {
                game.insert(name);
            }
            // Прежние имена еды, красителей и лазурита.
            let renamed = match short {
                "raw_beef" => "beef",
                "raw_pork" => "porkchop",
                "cooked_pork" => "cooked_porkchop",
                "raw_mutton" => "mutton",
                "raw_chicken" => "chicken",
                "rotten_meat" => "rotten_flesh",
                "lapis" => "lapis_lazuli",
                other => match other.strip_prefix("dye_") {
                    Some(color) => {
                        game.insert(format!("{color}_dye"));
                        continue;
                    }
                    None => continue,
                },
            };
            game.insert(renamed.to_owned());
        }
        let missing: Vec<_> = list.iter().filter(|name| !game.contains(**name)).collect();
        assert!(missing.is_empty(), "нет предметов из списка: {missing:?}");
    }

    #[test]
    fn committed_tables_match_the_registries() {
        // При расхождении выполните: cargo run -p game_core --example block_list
        assert_eq!(
            markdown(),
            include_str!("../../../docs/BLOCKS.md"),
            "docs/BLOCKS.md устарел"
        );
        assert_eq!(
            csv(),
            include_str!("../../../docs/BLOCKS.csv"),
            "docs/BLOCKS.csv устарел"
        );
    }
}
