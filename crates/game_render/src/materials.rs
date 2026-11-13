use crate::{BlockAtlas, MATERIAL_COUNT};
use bevy::{
    asset::{RenderAssetUsages, embedded_asset},
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{
        AsBindGroup, Extent3d, TextureDataOrder, TextureDimension, TextureFormat,
        TextureViewDescriptor, TextureViewDimension,
    },
    shader::ShaderRef,
};

pub const TEXTURE_SIZE: u32 = 128;
const ATLAS_WIDTH: u32 = TEXTURE_SIZE * 6;
const ATLAS_HEIGHT: u32 = TEXTURE_SIZE * 6;

pub type VoxelMaterial = ExtendedMaterial<StandardMaterial, TileArray>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct TileArray {
    #[texture(100, dimension = "2d_array")]
    #[sampler(101)]
    pub image: Handle<Image>,
    #[uniform(102)]
    pub layer: u32,
}
impl MaterialExtension for TileArray {
    fn fragment_shader() -> ShaderRef {
        "embedded://game_render/shaders/voxel.wesl".into()
    }
    fn prepass_fragment_shader() -> ShaderRef {
        "embedded://game_render/shaders/voxel_prepass.wesl".into()
    }
}

pub struct VoxelRenderPlugin;
impl Plugin for VoxelRenderPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shaders/voxel.wesl");
        embedded_asset!(app, "shaders/voxel_prepass.wesl");
        app.add_plugins(MaterialPlugin::<VoxelMaterial>::default());
    }
}

/// Decode only the specified 768×384 PNG; never silently resize or flip the art.
pub fn atlas_png(bytes: &[u8]) -> Result<Image, String> {
    atlas_array(BlockAtlas::from_png(bytes)?.rgba.as_raw())
}

/// Row-major RGBA atlas -> layer-major GPU image, with an independent mip chain per tile.
/// UVs stay unbounded: hardware repeat works inside a layer, including anisotropic taps.
pub fn atlas_array(rgba: &[u8]) -> Result<Image, String> {
    // Keep the original public 18-layer RGBA input working alongside the
    // internal 36-layer atlas. Legacy source pixels are copied without changes.
    if rgba.len() == (ATLAS_WIDTH * TEXTURE_SIZE * 3 * 4) as usize {
        let source = image::RgbaImage::from_raw(ATLAS_WIDTH, TEXTURE_SIZE * 3, rgba.to_vec())
            .expect("legacy RGBA length checked");
        return atlas_array(BlockAtlas::from_legacy_rgba(&source)?.rgba.as_raw());
    }
    if rgba.len() != (ATLAS_WIDTH * ATLAS_HEIGHT * 4) as usize {
        return Err("atlas RGBA byte count does not match 768×384 or 768×768".into());
    }
    let mut data = Vec::new();
    for layer in 0..MATERIAL_COUNT {
        let mut tile = Vec::with_capacity((TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize);
        for y in 0..TEXTURE_SIZE as usize {
            let start = (((layer / 6) * TEXTURE_SIZE as usize + y) * ATLAS_WIDTH as usize
                + (layer % 6) * TEXTURE_SIZE as usize)
                * 4;
            tile.extend_from_slice(&rgba[start..start + TEXTURE_SIZE as usize * 4]);
        }
        append_mips(&mut data, tile, TEXTURE_SIZE as usize);
    }
    let mut image = Image::new(
        Extent3d {
            width: TEXTURE_SIZE,
            height: TEXTURE_SIZE,
            depth_or_array_layers: MATERIAL_COUNT as u32,
        },
        TextureDimension::D2,
        // Image::new checks the base-level length; assign mip data afterwards.
        vec![0; (TEXTURE_SIZE * TEXTURE_SIZE * MATERIAL_COUNT as u32 * 4) as usize],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.data = Some(data);
    image.data_order = TextureDataOrder::LayerMajor;
    image.texture_descriptor.mip_level_count = 8;
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::D2Array),
        ..default()
    });
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 8,
        ..default()
    });
    Ok(image)
}

fn append_mips(data: &mut Vec<u8>, mut level: Vec<u8>, mut width: usize) {
    loop {
        data.extend_from_slice(&level);
        if width == 1 {
            break;
        }
        let next_width = width / 2;
        let mut next = vec![0; next_width * next_width * 4];
        for y in 0..next_width {
            for x in 0..next_width {
                let mut sum = [0.; 4];
                for dy in 0..2 {
                    for dx in 0..2 {
                        let p = &level[((y * 2 + dy) * width + x * 2 + dx) * 4..][..4];
                        let a = f32::from(p[3]) / 255.;
                        for c in 0..3 {
                            sum[c] += srgb_to_linear(f32::from(p[c]) / 255.) * a;
                        }
                        sum[3] += a;
                    }
                }
                let dst = &mut next[(y * next_width + x) * 4..][..4];
                for c in 0..3 {
                    dst[c] = if sum[3] > 0. {
                        (linear_to_srgb(sum[c] / sum[3]) * 255.).round() as u8
                    } else {
                        0
                    };
                }
                dst[3] = (sum[3] * 255. / 4.).round() as u8;
            }
        }
        level = next;
        width = next_width;
    }
}
fn srgb_to_linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}
fn linear_to_srgb(v: f32) -> f32 {
    if v <= 0.0031308 {
        v * 12.92
    } else {
        1.055 * v.powf(1. / 2.4) - 0.055
    }
}

pub fn create_materials(
    images: &mut Assets<Image>,
    materials: &mut Assets<VoxelMaterial>,
    atlas: &[u8],
) -> Result<Vec<Handle<VoxelMaterial>>, String> {
    create_materials_from_atlas(images, materials, &BlockAtlas::from_png(atlas)?)
}

/// Create world materials from the same decoded original pixels used by icons.
pub fn create_materials_from_atlas(
    images: &mut Assets<Image>,
    materials: &mut Assets<VoxelMaterial>,
    atlas: &BlockAtlas,
) -> Result<Vec<Handle<VoxelMaterial>>, String> {
    let array = atlas_array(atlas.rgba.as_raw())?;
    let data = array.data.as_ref().expect("initialized texture array");
    let layer_stride = data.len() / MATERIAL_COUNT;
    let has_alpha: [bool; MATERIAL_COUNT] = std::array::from_fn(|layer| {
        data[layer * layer_stride..][..(TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize]
            .as_chunks::<4>()
            .0
            .iter()
            .any(|p| p[3] < 255)
    });
    let image = images.add(array);
    Ok((0..MATERIAL_COUNT)
        .map(|layer| {
            materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    // Preserve authored alpha. RGB glass/water get a fallback opacity.
                    base_color: Color::srgba(
                        1.,
                        1.,
                        1.,
                        match layer {
                            _ if has_alpha[layer] => 1.,
                            12 => 0.35,
                            13 => 0.65,
                            _ => 1.,
                        },
                    ),
                    perceptual_roughness: if matches!(layer, 12 | 13) { 0.28 } else { 0.92 },
                    reflectance: 0.12,
                    alpha_mode: match layer {
                        12 | 13 => AlphaMode::Blend,
                        9 => AlphaMode::Mask(0.45),
                        _ => AlphaMode::Opaque,
                    },
                    ..default()
                },
                extension: TileArray {
                    image: image.clone(),
                    layer: layer as u32,
                },
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn pixel_art_sampler_is_nearest_with_repeat_and_no_anisotropy() {
        let mut image = atlas_array(&vec![0; (ATLAS_WIDTH * ATLAS_HEIGHT * 4) as usize]).unwrap();
        configure_pixel_sampler(&mut image);
        let ImageSampler::Descriptor(sampler) = &image.sampler else {
            panic!("pixel-art mode must use a custom sampler");
        };
        assert_eq!(sampler.address_mode_u, ImageAddressMode::Repeat);
        assert_eq!(sampler.address_mode_v, ImageAddressMode::Repeat);
        assert_eq!(sampler.mag_filter, ImageFilterMode::Nearest);
        assert_eq!(sampler.min_filter, ImageFilterMode::Nearest);
        assert_eq!(sampler.mipmap_filter, ImageFilterMode::Nearest);
        assert_eq!(sampler.anisotropy_clamp, 1);
    }
    #[test]
    fn layers_and_all_mips_never_mix_neighboring_tiles() {
        let mut rgba = vec![0; (ATLAS_WIDTH * ATLAS_HEIGHT * 4) as usize];
        for y in 0..ATLAS_HEIGHT as usize {
            for x in 0..ATLAS_WIDTH as usize {
                let layer = (y / 128) * 6 + x / 128;
                rgba[(y * ATLAS_WIDTH as usize + x) * 4..][..4].copy_from_slice(&[
                    layer as u8 * 7,
                    30,
                    90,
                    255,
                ]);
            }
        }
        let image = atlas_array(&rgba).unwrap();
        let data = image.data.as_ref().unwrap();
        let layer_bytes = (0..8)
            .map(|mip| (128_usize >> mip).pow(2) * 4)
            .sum::<usize>();
        assert_eq!(data.len(), layer_bytes * MATERIAL_COUNT);
        for layer in 0..MATERIAL_COUNT {
            assert!(
                data[layer * layer_bytes..(layer + 1) * layer_bytes]
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .all(|p| *p == [layer as u8 * 7, 30, 90, 255])
            );
        }
        assert!(atlas_array(&[]).is_err());
    }
    #[test]
    fn mipmaps_filter_in_linear_light_with_premultiplied_alpha() {
        let mut data = Vec::new();
        append_mips(
            &mut data,
            vec![255, 255, 255, 255, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 0],
            2,
        );
        assert_eq!(&data[16..], &[188, 188, 188, 128]);
    }
    #[test]
    fn png_dimensions_are_checked_before_decoding_pixels() {
        let mut bytes = Cursor::new(Vec::new());
        image::RgbaImage::new(6, 3)
            .write_to(&mut bytes, image::ImageFormat::Png)
            .unwrap();
        assert!(atlas_png(bytes.get_ref()).unwrap_err().contains("768×384"));
        assert!(atlas_png(b"not a PNG").is_err());
    }

    #[test]
    fn png_decode_preserves_top_left_tile_and_row_order() {
        let mut rgba = image::RgbaImage::new(ATLAS_WIDTH, TEXTURE_SIZE * 3);
        for (x, y, pixel) in rgba.enumerate_pixels_mut() {
            *pixel = image::Rgba([
                ((y / 128) * 6 + x / 128) as u8 * 13,
                (y % 128) as u8,
                80,
                255,
            ]);
        }
        let mut bytes = Cursor::new(Vec::new());
        rgba.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
        let actual = atlas_png(bytes.get_ref()).unwrap();
        let data = actual.data.unwrap();
        let layer_bytes = (0..8).map(|m| (128_usize >> m).pow(2) * 4).sum::<usize>();
        for layer in 0..18 {
            assert_eq!(
                &data[layer * layer_bytes..][..4],
                &[layer as u8 * 13, 0, 80, 255]
            );
            assert_eq!(
                &data[layer * layer_bytes + 127 * 128 * 4..][..4],
                &[layer as u8 * 13, 127, 80, 255]
            );
        }
    }

    #[test]
    fn source_glass_alpha_is_not_reduced_a_second_time() {
        let mut pixels = image::RgbaImage::from_pixel(
            ATLAS_WIDTH,
            TEXTURE_SIZE * 3,
            image::Rgba([150, 200, 220, 255]),
        );
        // Layer 12 is the first tile in the third row.
        for y in 256..384 {
            for x in 0..128 {
                pixels.get_pixel_mut(x, y)[3] = 100;
            }
        }
        let mut bytes = Cursor::new(Vec::new());
        pixels
            .write_to(&mut bytes, image::ImageFormat::Png)
            .unwrap();
        let mut images = Assets::default();
        let mut materials = Assets::default();
        let handles = create_materials(&mut images, &mut materials, bytes.get_ref()).unwrap();
        assert_eq!(
            materials.get(&handles[12]).unwrap().base.base_color.alpha(),
            1.
        );
        assert_eq!(
            materials.get(&handles[13]).unwrap().base.base_color.alpha(),
            0.65
        );
    }
}
