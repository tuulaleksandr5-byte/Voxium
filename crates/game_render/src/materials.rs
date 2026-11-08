use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use game_core::{AIR, BLOCKS, GLASS, LEAVES, WATER};

const TEXTURE_SIZE: u32 = 16;

/// Creates original, deterministic 16×16 pixel textures and per-block materials.
///
/// Handles are indexed by the stable block ID, including a transparent air
/// placeholder. Nearest filtering and repeat addressing preserve texture scale
/// across greedy quads. No external assets or GPU/window initialization is needed.
#[must_use]
pub fn create_materials(
    images: &mut Assets<Image>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Handle<StandardMaterial>> {
    (0..BLOCKS.len() as u8)
        .map(|id| {
            let mut pixels = Vec::with_capacity((TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize);
            for y in 0..TEXTURE_SIZE {
                for x in 0..TEXTURE_SIZE {
                    pixels.extend(pixel(id, x, y));
                }
            }
            let mut image = Image::new(
                Extent3d {
                    width: TEXTURE_SIZE,
                    height: TEXTURE_SIZE,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                pixels,
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::default(),
            );
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                address_mode_w: ImageAddressMode::Repeat,
                ..ImageSamplerDescriptor::nearest()
            });
            materials.add(StandardMaterial {
                base_color_texture: Some(images.add(image)),
                perceptual_roughness: if id == WATER || id == GLASS {
                    0.22
                } else {
                    0.93
                },
                reflectance: if id == WATER || id == GLASS {
                    0.35
                } else {
                    0.15
                },
                alpha_mode: match id {
                    AIR | GLASS | WATER => AlphaMode::Blend,
                    LEAVES => AlphaMode::Mask(0.5),
                    _ => AlphaMode::Opaque,
                },
                ..default()
            })
        })
        .collect()
}

fn pixel(id: u8, x: u32, y: u32) -> [u8; 4] {
    let noise = noise_at(id, x, y);
    let fine = (noise % 19) as i16 - 9;
    match id {
        AIR => [0; 4],
        // Mossy layered turf: uneven green tips over warm earthy lower bands.
        1 => {
            let grass_depth = 5 + noise_at(id, x, 0) % 4;
            if y < grass_depth {
                shade([92, 145, 61], fine + if y < 2 { 12 } else { 0 }, 255)
            } else {
                shade(
                    [112, 91, 54],
                    fine + if y == grass_depth { -14 } else { 0 },
                    255,
                )
            }
        }
        2 => shade(
            [129, 92, 59],
            fine * 2 + if noise % 29 < 3 { -23 } else { 0 },
            255,
        ),
        3 => {
            let vein = if (x + y * 3).is_multiple_of(13) {
                16
            } else {
                0
            };
            shade([125, 132, 137], fine + vein, 255)
        }
        4 => shade(
            [65, 72, 85],
            fine + if y.is_multiple_of(5) { -13 } else { 0 },
            255,
        ),
        5 => shade(
            [222, 206, 151],
            fine / 2 + if (x + y * 2) % 11 < 2 { 7 } else { 0 },
            255,
        ),
        6 => {
            let pebble = (noise_at(id, x / 3, y / 3) % 43) as i16 - 21;
            shade([135, 128, 116], pebble + fine / 2, 255)
        }
        7 => {
            let groove = match x % 7 {
                0 => -36,
                1 => -14,
                5 => 18,
                _ => 0,
            };
            let knot = if (x as i32 - 9).abs() + (y as i32 - 9).abs() < 3 {
                -17
            } else {
                0
            };
            shade([137, 99, 57], fine / 2 + groove + knot, 255)
        }
        LEAVES => {
            let cluster = (noise_at(id, x / 3, y / 3) % 29) as i16 - 14;
            shade(
                [72, 121, 54],
                fine + cluster,
                if noise % 13 < 2 { 0 } else { 255 },
            )
        }
        9 => {
            let board = y / 4;
            let seam = y.is_multiple_of(4) || (x + board * 5).is_multiple_of(16);
            let grain = if (x * 3 + y) % 11 < 2 { -10 } else { 0 };
            shade(
                [181, 135, 79],
                fine / 2 + grain + if seam { -39 } else { 0 },
                255,
            )
        }
        10 => {
            let row = y / 5;
            let shifted_x = (x + row * 3) % 8;
            let mortar = y.is_multiple_of(5) || shifted_x == 0;
            let stone = (noise_at(id, (x + row * 3) / 8, row) % 25) as i16 - 12;
            shade(
                [132, 136, 131],
                fine / 2 + stone + if mortar { -38 } else { 0 },
                255,
            )
        }
        GLASS => {
            if x == 0 || y == 0 || x == 15 || y == 15 {
                [177, 211, 206, 175]
            } else if (x + y) % 15 < 2 {
                [225, 243, 239, 104]
            } else {
                [165, 204, 202, 36]
            }
        }
        WATER => {
            let ripple = if (x + (y / 3) * 5) % 13 < 4 { 15 } else { 0 };
            shade([62, 126, 173], fine / 3 + ripple, 153)
        }
        13 => shade(
            [49, 48, 55],
            fine * 2 + if noise % 31 < 5 { 22 } else { 0 },
            255,
        ),
        14 => shade(
            [229, 237, 237],
            fine / 2 + if (x + y * 3) % 19 < 2 { -12 } else { 0 },
            255,
        ),
        15 => shade(
            [198, 177, 125],
            fine / 2 + if y.is_multiple_of(5) { -19 } else { 0 },
            255,
        ),
        _ => [255, 0, 255, 255],
    }
}

fn shade(base: [u8; 3], amount: i16, alpha: u8) -> [u8; 4] {
    let [r, g, b] = base.map(|channel| (i16::from(channel) + amount).clamp(0, 255) as u8);
    [r, g, b, alpha]
}

fn noise_at(id: u8, x: u32, y: u32) -> u32 {
    let mut hash = x.wrapping_mul(0x8da6_b343)
        ^ y.wrapping_mul(0xd816_3841)
        ^ u32::from(id).wrapping_mul(0xcb1a_b31f);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0x85eb_ca6b);
    hash ^ (hash >> 16)
}
