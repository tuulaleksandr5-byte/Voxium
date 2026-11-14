use game_core::BLOCKS;
use serde::Deserialize;

/// Слои 0…35 — исходный атлас 6×6, слои 36…71 — дополнительные пиксельные тайлы.
pub const MATERIAL_COUNT: usize = 72;
/// Слой листвы дуба в исходном атласе.
pub const OAK_LEAVES_LAYER: usize = 9;
pub const GLASS_LAYER: usize = 12;
pub const WATER_LAYER: usize = 13;
pub const BIRCH_LEAVES_LAYER: usize = 41;
pub const LAMP_LAYER: usize = 45;
pub const ICE_LAYER: usize = 46;
pub const GRASS_TOP: u8 = 0;
pub const GRASS_BOTTOM: u8 = 2;
pub const WOOD_END: u8 = 8;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub struct FaceTextures {
    pub top: u8,
    pub side: u8,
    pub bottom: u8,
    /// Optional fixed front on the negative-Z world face.
    #[serde(default)]
    pub front: Option<u8>,
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
            [Some(f.top), Some(f.side), Some(f.bottom), f.front]
                .into_iter()
                .flatten()
                .any(|i| i as usize >= MATERIAL_COUNT)
        }) {
            return Err("texture layer must be in 0..72".into());
        }
        Ok(Self(faces.try_into().expect("length checked")))
    }

    pub fn layer(&self, block: u8, axis: usize, positive: bool) -> u8 {
        let faces = self.0[block as usize];
        if axis == 2 && !positive {
            faces.front.unwrap_or(faces.side)
        } else if axis != 1 {
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
        let source = include_str!("../../../assets/blocks.ron").replace("top: 0", "top: 72");
        assert!(BlockTextures::from_ron(&source).is_err());
        let source = include_str!("../../../assets/blocks.ron").replace("Some(26)", "Some(72)");
        assert!(BlockTextures::from_ron(&source).is_err());
    }
}
