//! Lossless in-memory assembly of the supplied block texture PNGs.

use image::{GenericImage, RgbaImage};
use std::io::Cursor;

const TILE_SIZE: u32 = 128;
const ATLAS_WIDTH: u32 = TILE_SIZE * 6;
const SOURCE_HEIGHT: u32 = TILE_SIZE * 3;
const ATLAS_HEIGHT: u32 = TILE_SIZE * 6;
const SOURCE_TILE_COUNT: usize = 17;

/// Layer order for the fifteen additional 128×128 PNGs (layers 18…32).
pub const ADDITIONAL_TILE_NAMES: [&str; 15] = [
    "gold_ore",
    "coal_ore",
    "diamond_ore",
    "gold_block",
    "diamond_block",
    "iron_ore",
    "iron_block",
    "crafting_table_top",
    "crafting_table_front",
    "crafting_table_side",
    "crafting_table_bottom",
    "furnace_front",
    "furnace_front_on",
    "furnace_side",
    "furnace_top",
];
const ADDITIONAL_TILES: [&[u8]; 15] = [
    include_bytes!("../../../assets/textures/blocks/128x128/gold_ore.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/coal_ore.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/diamond_ore.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/gold_block.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/diamond_block.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/iron_ore.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/iron_block.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_top.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_front.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_side.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_bottom.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/furnace_front.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/furnace_front_on.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/furnace_side.png"),
    include_bytes!("../../../assets/textures/blocks/128x128/furnace_top.png"),
];

/// Decoded original block artwork, shared by world materials and hotbar icons.
///
/// Assembly copies decoded RGBA pixels without resizing, flipping, repainting,
/// or re-encoding the source PNG files. The internal 6×6 atlas appends the
/// embedded additional tiles at layers 18…32. Tile assembly leaves layer 17
/// transparent; whole-atlas loading preserves all eighteen source tiles.
/// Layers 33…35 remain transparent.
#[derive(Clone, Debug)]
pub struct BlockAtlas {
    pub(crate) rgba: RgbaImage,
}

impl BlockAtlas {
    /// Decode an original 768×384 PNG with six columns and three rows.
    pub fn from_png(bytes: &[u8]) -> Result<Self, String> {
        Self::from_legacy_rgba(&decode_png(bytes, (ATLAS_WIDTH, SOURCE_HEIGHT), "atlas")?)
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
        Self { rgba }.with_additional_tiles(&ADDITIONAL_TILES)
    }

    pub(crate) fn from_legacy_rgba(source: &RgbaImage) -> Result<Self, String> {
        let mut rgba = RgbaImage::new(ATLAS_WIDTH, ATLAS_HEIGHT);
        rgba.copy_from(source, 0, 0).map_err(|e| e.to_string())?;
        Self { rgba }.with_additional_tiles(&ADDITIONAL_TILES)
    }

    /// Replace layers 18…32 with exactly fifteen supplied 128×128 PNGs.
    /// Source layers 0…17 are preserved, and all PNG pixels are copied exactly.
    /// The returned atlas can be shared by GPU materials and UI icon baking.
    pub fn with_additional_tiles(mut self, tiles: &[&[u8]]) -> Result<Self, String> {
        if tiles.len() != ADDITIONAL_TILE_NAMES.len() {
            return Err(format!(
                "expected 15 additional block texture PNGs, got {}",
                tiles.len()
            ));
        }
        for (offset, bytes) in tiles.iter().enumerate() {
            self.replace_additional_tile(offset, bytes)?;
        }
        Ok(self)
    }

    /// Apply a decoded square texture to an atlas layer, preserving other layers.
    pub fn replace_layer_rgba(&mut self, layer: usize, rgba: &RgbaImage) -> Result<(), String> {
        if layer >= 36 || rgba.dimensions() != (TILE_SIZE, TILE_SIZE) {
            return Err("expected 128×128 RGBA texture for a valid atlas layer".into());
        }
        self.rgba
            .copy_from(
                rgba,
                (layer as u32 % 6) * TILE_SIZE,
                (layer as u32 / 6) * TILE_SIZE,
            )
            .map_err(|error| error.to_string())
    }

    /// Заменить один дополнительный PNG; индекс 0…14 соответствует слоям 18…32.
    /// Проверка индекса и полное декодирование предшествуют изменению атласа.
    /// При ошибке текущие пиксели, включая все исходные слои, сохраняются.
    pub fn replace_additional_tile(&mut self, offset: usize, bytes: &[u8]) -> Result<(), String> {
        let name = ADDITIONAL_TILE_NAMES
            .get(offset)
            .ok_or_else(|| format!("additional tile index must be in 0..15, got {offset}"))?;
        let tile = decode_png(bytes, (TILE_SIZE, TILE_SIZE), name)?;
        let layer = offset + 18;
        self.rgba
            .copy_from(
                &tile,
                (layer as u32 % 6) * TILE_SIZE,
                (layer as u32 / 6) * TILE_SIZE,
            )
            .map_err(|e| format!("cannot copy tile {layer}: {e}"))
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
    fn single_additional_replacement_rejects_invalid_input_without_mutating_any_layer() {
        let mut atlas = BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap();
        let before = atlas.rgba.clone();
        for offset in [15, 18, 36, usize::MAX] {
            assert!(
                atlas
                    .replace_additional_tile(offset, ADDITIONAL_TILES[0])
                    .is_err()
            );
            assert_eq!(atlas.rgba, before);
        }
        let mut wrong_size = Cursor::new(Vec::new());
        RgbaImage::new(128, 127)
            .write_to(&mut wrong_size, image::ImageFormat::Png)
            .unwrap();
        for offset in [0, 14] {
            for invalid in [b"invalid PNG".as_slice(), wrong_size.get_ref()] {
                assert!(atlas.replace_additional_tile(offset, invalid).is_err());
                assert_eq!(atlas.rgba, before);
            }
        }
    }

    #[test]
    fn legacy_whole_atlas_and_raw_rgba_preserve_all_eighteen_source_layers() {
        let source = RgbaImage::from_fn(768, 384, |x, y| {
            image::Rgba([
                (x % 256) as u8,
                (y % 256) as u8,
                ((x + y) % 256) as u8,
                ((x * 3 + y) % 256) as u8,
            ])
        });
        let mut png = Cursor::new(Vec::new());
        source.write_to(&mut png, image::ImageFormat::Png).unwrap();
        let atlas = BlockAtlas::from_png(png.get_ref()).unwrap();
        assert_eq!(
            image::imageops::crop_imm(&atlas.rgba, 0, 0, 768, 384).to_image(),
            source
        );
        let legacy_gpu = atlas_array(source.as_raw()).unwrap();
        let expanded_gpu = atlas_array(atlas.rgba.as_raw()).unwrap();
        assert_eq!(legacy_gpu.data, expanded_gpu.data);
        let data = expanded_gpu.data.unwrap();
        let stride = data.len() / 36;
        for layer in 0..18 {
            let expected =
                image::imageops::crop_imm(&source, (layer % 6) * 128, (layer / 6) * 128, 128, 128)
                    .to_image();
            assert_eq!(
                &data[layer as usize * stride..][..128 * 128 * 4],
                expected.as_raw(),
                "legacy layer {layer}"
            );
        }
    }

    #[test]
    fn supplied_additional_tiles_replace_defaults_without_touching_originals() {
        let atlas = BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap();
        let mut png = Cursor::new(Vec::new());
        let source = RgbaImage::from_fn(128, 128, |x, y| image::Rgba([x as u8, y as u8, 42, 100]));
        source.write_to(&mut png, image::ImageFormat::Png).unwrap();
        let updated = atlas
            .clone()
            .with_additional_tiles(&[png.get_ref().as_slice(); 15])
            .unwrap();
        assert_eq!(
            image::imageops::crop_imm(&updated.rgba, 0, 0, 768, 384).to_image(),
            image::imageops::crop_imm(&atlas.rgba, 0, 0, 768, 384).to_image()
        );
        for layer in 18..33 {
            assert_eq!(
                image::imageops::crop_imm(
                    &updated.rgba,
                    (layer % 6) * 128,
                    (layer / 6) * 128,
                    128,
                    128
                )
                .to_image(),
                source
            );
        }
        assert!(atlas.clone().with_additional_tiles(&[]).is_err());
        assert!(
            atlas
                .clone()
                .with_additional_tiles(&ADDITIONAL_TILES[..14])
                .is_err()
        );
        assert!(
            atlas
                .clone()
                .with_additional_tiles(&[ADDITIONAL_TILES[0]; 16])
                .is_err()
        );
        assert!(
            atlas
                .clone()
                .with_additional_tiles(&[b"invalid PNG".as_slice(); 15])
                .unwrap_err()
                .contains("gold_ore")
        );
        let mut wrong_size = Cursor::new(Vec::new());
        RgbaImage::new(127, 128)
            .write_to(&mut wrong_size, image::ImageFormat::Png)
            .unwrap();
        assert!(
            atlas
                .with_additional_tiles(&[wrong_size.get_ref().as_slice(); 15])
                .unwrap_err()
                .contains("128×128")
        );
    }

    #[test]
    fn legacy_apis_append_new_tiles_with_exact_rgba_and_empty_reserved_layers() {
        let mut source = Cursor::new(Vec::new());
        RgbaImage::from_pixel(768, 384, image::Rgba([23, 45, 67, 89]))
            .write_to(&mut source, image::ImageFormat::Png)
            .unwrap();
        let tiles: [&[u8]; 15] = [
            include_bytes!("../../../assets/textures/blocks/128x128/gold_ore.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/coal_ore.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/diamond_ore.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/gold_block.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/diamond_block.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/iron_ore.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/iron_block.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_top.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_front.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_side.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/crafting_table_bottom.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/furnace_front.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/furnace_front_on.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/furnace_side.png"),
            include_bytes!("../../../assets/textures/blocks/128x128/furnace_top.png"),
        ];
        for atlas in [
            BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap(),
            BlockAtlas::from_png(source.get_ref()).unwrap(),
        ] {
            assert_eq!(atlas.rgba.dimensions(), (768, 768));
            let gpu = atlas_array(atlas.rgba.as_raw()).unwrap();
            let data = gpu.data.unwrap();
            let stride = data.len() / 36;
            for (offset, png) in tiles.iter().enumerate() {
                let layer = offset + 18;
                let expected = image::load_from_memory(png).unwrap().into_rgba8();
                let actual = image::imageops::crop_imm(
                    &atlas.rgba,
                    (layer as u32 % 6) * 128,
                    (layer as u32 / 6) * 128,
                    128,
                    128,
                )
                .to_image();
                assert_eq!(actual, expected, "atlas layer {layer}");
                assert_eq!(
                    &data[layer * stride..][..128 * 128 * 4],
                    expected.as_raw(),
                    "GPU layer {layer}"
                );
            }
            for layer in [33, 34, 35] {
                assert!(
                    data[layer * stride..(layer + 1) * stride]
                        .iter()
                        .all(|&b| b == 0)
                );
            }
        }
    }

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
            gpu_bytes[SOURCE_TILE_COUNT * layer_stride..18 * layer_stride]
                .iter()
                .all(|&v| v == 0)
        );
    }

    #[test]
    fn uploaded_tiles_keep_rgba_in_each_isolated_mip_chain() {
        let atlas = BlockAtlas::from_tiles(&ORIGINAL_TILES).unwrap();
        let combined = atlas_array(atlas.rgba.as_raw()).unwrap();
        let combined_bytes = combined.data.as_ref().unwrap();
        let layer_stride = combined_bytes.len() / 36;
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
