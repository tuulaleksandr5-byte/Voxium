use game_core::BLOCKS;
use serde::Deserialize;

/// Слои 0…35 — исходный атлас 6×6, слои 36…215 — дополнительные пиксельные тайлы.
pub const MATERIAL_COUNT: usize = 582;
/// Строк в атласе: по шесть слоёв в строке.
pub const ATLAS_ROWS: u32 = (MATERIAL_COUNT / 6) as u32;
/// Слой листвы дуба в исходном атласе.
pub const OAK_LEAVES_LAYER: usize = 9;
pub const GLASS_LAYER: usize = 12;
pub const WATER_LAYER: usize = 13;
pub const BIRCH_LEAVES_LAYER: usize = 41;
pub const PINE_LEAVES_LAYER: usize = 111;
/// Слои растений: первый и последний.
pub const PLANT_LAYERS: std::ops::RangeInclusive<usize> = 118..=122;

/// Слой с вырезанными текселями: листва (обычные кубы) и растения.
#[must_use]
pub fn is_cutout_layer(layer: usize) -> bool {
    matches!(
        layer,
        OAK_LEAVES_LAYER | BIRCH_LEAVES_LAYER | PINE_LEAVES_LAYER
    ) || PLANT_LAYERS.contains(&layer)
        || LEAF2_LAYERS.contains(&layer)
        || PLANT2_LAYERS.contains(&layer)
        || CUTOUT3_LAYERS.contains(&layer)
        || CUTOUT4_LAYERS.contains(&layer)
        || CUTOUT5_LAYERS.contains(&layer)
        || CUTOUT6_LAYERS.contains(&layer)
        || CUTOUT7_LAYERS.contains(&layer)
        || matches!(layer, 125 | 129 | 133 | 137 | 141)
}

/// Листва и корни этой партии: кубы с вырезанной по альфе текстурой.
pub const LEAF2_LAYERS: std::ops::RangeInclusive<usize> = 204..=208;
/// Растения, факелы, огонь и рельсы этой партии.
/// Растения, лестницы, рельсы, цепи и прутья части 2.
pub const CUTOUT3_LAYERS: std::ops::RangeInclusive<usize> = 322..=332;
/// Растения, лианы, стебли и растяжка части 3.
pub const CUTOUT4_LAYERS: std::ops::RangeInclusive<usize> = 371..=383;
/// Рельсы, барьер, свет и высокие цветы части 4.
pub const CUTOUT5_LAYERS: std::ops::RangeInclusive<usize> = 419..=426;
/// Ростки, кувшинница и пустой блок части 5.
pub const CUTOUT6_LAYERS: std::ops::RangeInclusive<usize> = 439..=443;
/// Кораллы, ламинария, пузыри и подмостки части 6.
pub const CUTOUT7_LAYERS: std::ops::RangeInclusive<usize> = 523..=548;
pub const PLANT2_LAYERS: std::ops::RangeInclusive<usize> = 209..=239;

/// Слои растений: рисуются плоскостями, видны с обеих сторон.
#[must_use]
pub fn is_double_sided_layer(layer: usize) -> bool {
    PLANT_LAYERS.contains(&layer)
        || PLANT2_LAYERS.contains(&layer)
        || CUTOUT3_LAYERS.contains(&layer)
        || CUTOUT4_LAYERS.contains(&layer)
        || CUTOUT5_LAYERS.contains(&layer)
        || CUTOUT6_LAYERS.contains(&layer)
        || CUTOUT7_LAYERS.contains(&layer)
}

/// Слои цветного стекла.
pub const STAINED_GLASS_LAYERS: std::ops::RangeInclusive<usize> = 149..=164;

/// Полупрозрачный слой: стекло, вода, лёд.
#[must_use]
pub fn is_blend_layer(layer: usize) -> bool {
    matches!(layer, GLASS_LAYER | WATER_LAYER | ICE_LAYER) || STAINED_GLASS_LAYERS.contains(&layer)
}
/// Слой лавы: светится и не бывает прозрачным.
pub const LAVA_LAYER: usize = 203;
pub const LAMP_LAYER: usize = 45;
pub const ICE_LAYER: usize = 46;
pub const GRASS_TOP: u16 = 0;
pub const GRASS_BOTTOM: u16 = 2;
pub const WOOD_END: u16 = 8;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct FaceTextures {
    pub top: u16,
    pub side: u16,
    pub bottom: u16,
    /// Лицевая грань; по умолчанию на отрицательной стороне Z.
    #[serde(default)]
    pub front: Option<u16>,
    /// Ось (0 = X, 1 = Y, 2 = Z) и знак стороны, на которой лежит `front`.
    #[serde(default)]
    pub face: Option<(u8, bool)>,
    /// Текстура противоположной грани (у поршня — основание).
    #[serde(default)]
    pub back: Option<u16>,
    /// Дополнительный тайл формы: цепи подвесных табличек, ручка рычага, факел повторителя.
    #[serde(default)]
    pub aux: Option<u16>,
}

#[derive(Clone, Debug)]
pub struct BlockTextures(pub [FaceTextures; BLOCKS.len()]);

impl Default for BlockTextures {
    fn default() -> Self {
        Self::from_ron(include_str!("../../../assets/blocks.ron"))
            .expect("embedded block textures must be valid")
    }
}

impl BlockTextures {
    pub fn from_ron(source: &str) -> Result<Self, String> {
        let mut faces: Vec<FaceTextures> = ron::from_str(source).map_err(|e| e.to_string())?;
        // Preserve legacy assignments, filling only appended IDs from defaults.
        if matches!(faces.len(), 16 | 17) {
            let defaults: Vec<FaceTextures> =
                ron::from_str(include_str!("../../../assets/blocks.ron"))
                    .map_err(|e| format!("invalid embedded mapping: {e}"))?;
            faces.extend_from_slice(&defaults[faces.len()..]);
        }
        if faces.len() != BLOCKS.len() {
            return Err(format!(
                "expected {} block entries, got {}",
                BLOCKS.len(),
                faces.len()
            ));
        }
        if faces.iter().any(|f| {
            [
                Some(f.top),
                Some(f.side),
                Some(f.bottom),
                f.front,
                f.back,
                f.aux,
            ]
            .into_iter()
            .flatten()
            .any(|i| i as usize >= MATERIAL_COUNT)
                || f.face.is_some_and(|(axis, _)| axis > 2)
        }) {
            return Err(format!("texture layer must be in 0..{MATERIAL_COUNT}"));
        }
        Ok(Self(faces.try_into().expect("length checked")))
    }

    pub fn layer(&self, block: game_core::BlockId, axis: usize, positive: bool) -> u16 {
        let faces = self.0[block as usize];
        if let Some(front) = faces.front {
            let (front_axis, front_positive) = faces.face.unwrap_or((2, false));
            if axis == usize::from(front_axis) {
                if positive == front_positive {
                    return front;
                }
                if let Some(back) = faces.back {
                    return back;
                }
            }
        }
        if axis != 1 {
            faces.side
        } else if positive {
            faces.top
        } else {
            faces.bottom
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_mappings_preserve_assignments_and_append_embedded_defaults() {
        let defaults = BlockTextures::default();
        for count in [16, 17] {
            let entries = vec!["(top: 6, side: 5, bottom: 4)"; count];
            let parsed = BlockTextures::from_ron(&format!("[{}]", entries.join(","))).unwrap();
            assert!(
                parsed.0[..count]
                    .iter()
                    .all(|f| f.top == 6 && f.side == 5 && f.bottom == 4)
            );
            assert_eq!(parsed.0[count..], defaults.0[count..]);
        }
        for count in [0, 15, 18, 25, 27] {
            let source = format!(
                "[{}]",
                vec!["(top: 0, side: 0, bottom: 0)"; count].join(",")
            );
            assert!(
                BlockTextures::from_ron(&source).is_err(),
                "accepted {count} entries"
            );
        }
    }

    #[test]
    fn new_blocks_map_to_supplied_layers_and_only_negative_z_is_front() {
        let textures = BlockTextures::default();
        for (block, layer) in [
            (17, 19),
            (18, 23),
            (19, 18),
            (20, 20),
            (21, 24),
            (22, 21),
            (23, 22),
        ] {
            for axis in 0..3 {
                for positive in [false, true] {
                    assert_eq!(textures.layer(block, axis, positive), layer);
                }
            }
        }
        for (block, top, side, bottom, front) in [(24, 25, 27, 28, 26), (25, 32, 31, 32, 29)] {
            assert_eq!(textures.layer(block, 1, true), top);
            assert_eq!(textures.layer(block, 1, false), bottom);
            assert_eq!(textures.layer(block, 2, false), front);
            for (axis, positive) in [(0, false), (0, true), (2, true)] {
                assert_eq!(textures.layer(block, axis, positive), side);
            }
        }
    }

    #[test]
    fn ron_assigns_grass_log_and_sandstone_faces() {
        let f = BlockTextures::default();
        for (block, expected) in [(1, [0, 1, 2]), (7, [8, 7, 8]), (15, [16, 15, 16])] {
            assert_eq!(
                [
                    f.layer(block, 1, true),
                    f.layer(block, 0, true),
                    f.layer(block, 1, false)
                ],
                expected
            );
        }
    }
    #[test]
    fn rejects_missing_blocks_and_out_of_range_layers() {
        assert!(BlockTextures::from_ron("[]").is_err());
        let source = include_str!("../../../assets/blocks.ron")
            .replace("top: 0", &format!("top: {MATERIAL_COUNT}"));
        assert!(BlockTextures::from_ron(&source).is_err());
        let source = include_str!("../../../assets/blocks.ron")
            .replace("Some(26)", &format!("Some({MATERIAL_COUNT})"));
        assert!(BlockTextures::from_ron(&source).is_err());
    }
}
