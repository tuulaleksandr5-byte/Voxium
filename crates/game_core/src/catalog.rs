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
        Some((_, index, FamilyKind::H4)) => side(index).to_string(),
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
        assert_eq!(rows.len(), BLOCKS.len());
        let mut names = std::collections::HashSet::new();
        for (index, row) in rows.iter().enumerate() {
            assert_eq!(usize::from(row.id), index);
            assert!(names.insert(row.name_id), "{}", row.name_id);
            assert!(!row.name.trim().is_empty());
        }
        let kinds: usize = KINDS
            .iter()
            .map(|kind| rows.iter().filter(|row| row.kind == *kind).count())
            .sum();
        assert_eq!(kinds, rows.len(), "у каждого блока есть вид");
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
