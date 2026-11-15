//! Камнерез: один блок сырья превращается в выбранный вариант без сетки крафта.

use crate::content::{ContentRegistry, ItemId};

/// (сырьё, варианты результата). Обмен всегда один к одному.
const CUTS: [(&str, &[&str]); 13] = [
    (
        "foundation:stone",
        &[
            "builder:stone_bricks",
            "builder:smooth_stone",
            "foundation:cobblestone",
            "palette:stone_tiles",
        ],
    ),
    (
        "foundation:cobblestone",
        &["foundation:gravel", "builder:mossy_cobblestone"],
    ),
    (
        "builder:stone_bricks",
        &["foundation:stone", "palette:cracked_stone_bricks"],
    ),
    (
        "builder:smooth_stone",
        &["builder:stone_bricks", "foundation:stone"],
    ),
    ("palette:granite", &["palette:polished_granite"]),
    ("palette:marble", &["palette:polished_marble"]),
    ("palette:basalt", &["palette:polished_basalt"]),
    ("palette:slate", &["palette:polished_slate"]),
    ("palette:limestone", &["palette:polished_limestone"]),
    (
        "foundation:sandstone",
        &[
            "foundation:sand",
            "palette:sandstone_bricks",
            "voxium:cut_sandstone",
        ],
    ),
    ("foundation:gravel", &["foundation:sand"]),
    ("foundation:deepslate", &["foundation:cobblestone"]),
    ("builder:bricks", &["builder:terracotta"]),
];

/// Что камнерез может сделать из предмета.
#[must_use]
pub fn options(input: ItemId, catalog: &ContentRegistry) -> Vec<ItemId> {
    let Some(definition) = catalog.get(input) else {
        return Vec::new();
    };
    CUTS.iter()
        .find(|(source, _)| *source == definition.id)
        .map(|(_, outputs)| {
            outputs
                .iter()
                .filter_map(|output| catalog.find(output))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_cut_refers_to_catalog_items_and_never_to_itself() {
        let catalog = ContentRegistry::default();
        for (source, outputs) in CUTS {
            let input = catalog.find(source).expect(source);
            let found = options(input, &catalog);
            assert_eq!(found.len(), outputs.len(), "{source}");
            assert!(!found.contains(&input));
        }
        let dirt = catalog.find("foundation:dirt").unwrap();
        assert!(options(dirt, &catalog).is_empty());
    }
}
