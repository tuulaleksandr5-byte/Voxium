use game_core::BLOCKS;
use serde::Deserialize;

pub const MATERIAL_COUNT: usize = 18;
pub const GRASS_TOP: u8 = 0;
pub const GRASS_BOTTOM: u8 = 2;
pub const WOOD_END: u8 = 8;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub struct FaceTextures {
    pub top: u8,
    pub side: u8,
    pub bottom: u8,
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
        // Старый пользовательский mapping сохраняет все шестнадцать назначений.
        if faces.len() == 16 && BLOCKS.len() == 17 {
            faces.push(FaceTextures {
                top: 10,
                side: 10,
                bottom: 10,
            });
        }
        if faces.len() != BLOCKS.len() {
            return Err(format!(
                "expected {} block entries, got {}",
                BLOCKS.len(),
                faces.len()
            ));
        }
        if faces.iter().any(|f| {
            [f.top, f.side, f.bottom]
                .into_iter()
                .any(|i| i as usize >= MATERIAL_COUNT)
        }) {
            return Err("texture layer must be in 0..18".into());
        }
        Ok(Self(faces.try_into().expect("length checked")))
    }

    pub fn layer(&self, block: u8, axis: usize, positive: bool) -> u8 {
        let faces = self.0[block as usize];
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
        let source = include_str!("../../../assets/blocks.ron").replace("top: 0", "top: 18");
        assert!(BlockTextures::from_ron(&source).is_err());
    }
}
