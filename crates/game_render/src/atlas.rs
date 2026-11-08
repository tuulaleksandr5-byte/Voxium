//! Lossless in-memory assembly of the supplied block texture PNGs.

use image::{GenericImage, RgbaImage};
use std::io::Cursor;

const TILE_SIZE: u32 = 128;
const ATLAS_WIDTH: u32 = TILE_SIZE * 6;
const ATLAS_HEIGHT: u32 = TILE_SIZE * 3;
const SOURCE_TILE_COUNT: usize = 17;

/// Decoded original block artwork, shared by world materials and hotbar icons.
///
/// Assembly copies decoded RGBA pixels without resizing, flipping, repainting,
/// or re-encoding the source PNG files. Tile assembly leaves the eighteenth
/// tile transparent; whole-atlas loading preserves all eighteen source tiles.
#[derive(Clone, Debug)]
pub struct BlockAtlas {
    pub(crate) rgba: RgbaImage,
}

impl BlockAtlas {
    /// Decode an original 768×384 PNG with six columns and three rows.
    pub fn from_png(bytes: &[u8]) -> Result<Self, String> {
        Ok(Self {
            rgba: decode_png(bytes, (ATLAS_WIDTH, ATLAS_HEIGHT), "atlas")?,
        })
    }

    /// Assemble exactly seventeen original 128×128 PNGs in this layer order:
    /// grass_top, grass_side, dirt, stone, deepslate, sand, gravel, oak_log,
    /// oak_log_top, oak_leaves, oak_planks, cobblestone, glass, water, snow,
    /// sandstone, sandstone_top. The unused eighteenth layer stays transparent.
    pub fn from_tiles(tiles: &[&[u8]]) -> Result<Self, String> {
        if tiles.len() != SOURCE_TILE_COUNT {
            return Err(format!(
                "expected {SOURCE_TILE_COUNT} block texture PNGs, got {}",
                tiles.len()
            ));
        }
        let mut rgba = RgbaImage::new(ATLAS_WIDTH, ATLAS_HEIGHT);
        for (layer, bytes) in tiles.iter().enumerate() {
            let tile = decode_png(bytes, (TILE_SIZE, TILE_SIZE), &format!("tile {layer}"))?;
            rgba.copy_from(
                &tile,
                (layer as u32 % 6) * TILE_SIZE,
                (layer as u32 / 6) * TILE_SIZE,
            )
            .map_err(|e| format!("cannot copy tile {layer}: {e}"))?;
        }
        Ok(Self { rgba })
    }
}

fn decode_png(bytes: &[u8], expected: (u32, u32), name: &str) -> Result<RgbaImage, String> {
    let dimensions = image::ImageReader::with_format(Cursor::new(bytes), image::ImageFormat::Png)
        .into_dimensions()
        .map_err(|e| format!("cannot read {name} PNG: {e}"))?;
    if dimensions != expected {
        return Err(format!(
            "{name} must be {}×{}, got {}×{}",
            expected.0, expected.1, dimensions.0, dimensions.1
        ));
    }
    image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .map_err(|e| format!("cannot decode {name} PNG: {e}"))
        .map(image::DynamicImage::into_rgba8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas_array;

    const ORIGINAL_TILES: [&[u8]; SOURCE_TILE_COUNT] = [
        include_bytes!("../../../assets/textures/grass_top.png"),
        include_bytes!("../../../assets/textures/grass_side.png"),
        include_bytes!("../../../assets/textures/dirt.png"),
        include_bytes!("../../../assets/textures/stone.png"),
        include_bytes!("../../../assets/textures/deepslate.png"),
        include_bytes!("../../../assets/textures/sand.png"),
        include_bytes!("../../../assets/textures/gravel.png"),
        include_bytes!("../../../assets/textures/oak_log.png"),
        include_bytes!("../../../assets/textures/oak_log_top.png"),
        include_bytes!("../../../assets/textures/oak_leaves.png"),
        include_bytes!("../../../assets/textures/oak_planks.png"),
        include_bytes!("../../../assets/textures/cobblestone.png"),
        include_bytes!("../../../assets/textures/glass.png"),
        include_bytes!("../../../assets/textures/water.png"),
        include_bytes!("../../../assets/textures/snow.png"),
        include_bytes!("../../../assets/textures/sandstone.png"),
        include_bytes!("../../../assets/textures/sandstone_top.png"),
    ];

    #[test]
    fn uploaded_tiles_preserve_every_rgba_pixel_in_atlas_and_gpu_base_layers() {
        let atlas = BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap();
        let gpu = atlas_array(atlas.rgba.as_raw()).unwrap();
        let gpu_bytes = gpu.data.as_ref().unwrap();
        let layer_stride = (0..8).map(|m| (128_usize >> m).pow(2) * 4).sum::<usize>();
        for (layer, bytes) in ORIGINAL_TILES.iter().enumerate() {
            let original = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
                .unwrap()
                .into_rgba8();
            assert_eq!(original.dimensions(), (TILE_SIZE, TILE_SIZE));
            for (x, y, pixel) in original.enumerate_pixels() {
                assert_eq!(
                    atlas.rgba.get_pixel(
                        (layer as u32 % 6) * TILE_SIZE + x,
                        (layer as u32 / 6) * TILE_SIZE + y
                    ),
                    pixel,
                    "original tile {layer} pixel {x},{y} changed"
                );
            }
            assert_eq!(
                &gpu_bytes[layer * layer_stride..][..original.as_raw().len()],
                original.as_raw(),
                "GPU base RGBA differs from original tile {layer}"
            );
        }
        assert!(
            gpu_bytes[SOURCE_TILE_COUNT * layer_stride..]
                .iter()
                .all(|&v| v == 0)
        );
    }

    #[test]
    fn uploaded_tiles_keep_rgba_in_each_isolated_mip_chain() {
        let atlas = BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap();
        let combined = atlas_array(atlas.rgba.as_raw()).unwrap();
        let combined_bytes = combined.data.as_ref().unwrap();
        let layer_stride = combined_bytes.len() / 18;
        for (layer, bytes) in ORIGINAL_TILES.iter().enumerate() {
            // Repeat just this source tile into every slot. Its mip chain must
            // match the chain built among the seventeen different real images,
            // so no filtering tap can bleed neighboring artwork or alpha.
            let isolated = BlockAtlas::from_tiles(&[*bytes; SOURCE_TILE_COUNT]).unwrap();
            let isolated_gpu = atlas_array(isolated.rgba.as_raw()).unwrap();
            assert_eq!(
                &combined_bytes[layer * layer_stride..][..layer_stride],
                &isolated_gpu.data.as_ref().unwrap()[..layer_stride],
                "RGBA mip chain for source tile {layer} mixes neighboring tiles"
            );
        }
    }

    #[test]
    fn tile_count_dimensions_and_invalid_pngs_are_rejected() {
        assert!(BlockAtlas::from_tiles(&[]).unwrap_err().contains("17"));
        assert!(BlockAtlas::from_tiles(&ORIGINAL_TILES[..16]).is_err());
        let mut too_many = ORIGINAL_TILES.to_vec();
        too_many.push(ORIGINAL_TILES[0]);
        assert!(BlockAtlas::from_tiles(&too_many).is_err());
        let mut wrong_size = Cursor::new(Vec::new());
        RgbaImage::new(128, 127)
            .write_to(&mut wrong_size, image::ImageFormat::Png)
            .unwrap();
        let mut tiles = ORIGINAL_TILES;
        tiles[6] = wrong_size.get_ref();
        let error = BlockAtlas::from_tiles(&tiles).unwrap_err();
        assert!(error.contains("tile 6") && error.contains("128×128"));
        tiles[6] = b"invalid PNG";
        assert!(
            BlockAtlas::from_tiles(&tiles)
                .unwrap_err()
                .contains("tile 6")
        );
    }
}
