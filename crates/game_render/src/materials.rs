use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use game_core::{AIR, GLASS, LEAVES, WATER};

pub const TEXTURE_SIZE: u32 = 64;
pub const GRASS_TOP: u8 = 16;
pub const GRASS_BOTTOM: u8 = 17;
pub const WOOD_END: u8 = 18;
pub const MATERIAL_COUNT: usize = 19;

/// Creates original, deterministic 64×64 pixel textures and per-block materials.
///
/// The first 16 handles are indexed by stable block ID, including a transparent air
/// placeholder; three extra render-only slots select turf, soil and log ends.
/// Nearest magnification, filtered mipmaps and repeat addressing preserve texture scale
/// across greedy quads. No external assets or GPU/window initialization is needed.
#[must_use]
pub fn create_materials(
    images: &mut Assets<Image>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Handle<StandardMaterial>> {
    (0..MATERIAL_COUNT as u8)
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
            image.data = Some(with_mipmaps(
                image.data.take().expect("RGBA pixels"),
                TEXTURE_SIZE as usize,
            ));
            image.texture_descriptor.mip_level_count = 7;
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                min_filter: ImageFilterMode::Linear,
                mipmap_filter: ImageFilterMode::Linear,
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
    let fine = (noise_at(id, x, y) % 11) as i16 - 5;
    let xf = x as f32;
    let yf = y as f32;
    let tau = std::f32::consts::TAU;
    match id {
        AIR => [0; 4],
        1 => {
            let depth = 12 + noise_at(1, x / 3, 0) % 9;
            if y < depth { turf(x, y) } else { soil(x, y) }
        }
        2 | GRASS_BOTTOM => soil(x, y),
        GRASS_TOP => turf(x, y),
        3 | 4 | 6 | 10 | 13 => {
            let scale = if id == 6 { 8. } else { 16. };
            let (edge, dx, dy, h) = cells(xf, yf, scale, id);
            let base = match id {
                3 => [119, 124, 132],
                4 => [52, 55, 72],
                6 => [131, 129, 119],
                10 => [106, 110, 102],
                _ => [36, 35, 44],
            };
            let seam = if edge < if id == 10 { 1.9 } else { 0.9 } {
                -35
            } else {
                0
            };
            let bevel = if edge < 2.7 && dy < 0. { 16 } else { 0 };
            shade(
                base,
                (h % 27) as i16 - 13 + fine + seam + bevel + ((-dx - dy) * 0.6) as i16,
                255,
            )
        }
        5 => {
            let wave = (yf / 64. * tau * 4. + (xf / 64. * tau * 2.).sin() * 0.8).sin();
            shade([219, 197, 144], fine / 2 + (wave * 10.) as i16, 255)
        }
        7 => {
            let knot = ((xf - 31.) / 5.).powi(2) + ((yf - 34.) / 12.).powi(2);
            let bend = (yf / 64. * tau).sin() * 1.3;
            let grain = (xf / 64. * tau * 7. + bend).sin();
            let rings = if knot < 3.2 {
                (knot.sqrt() * 6.).sin() * 14.
            } else {
                0.
            };
            shade(
                [119, 79, 41],
                fine / 2 + (grain * 20. + rings) as i16 - if knot < 0.3 { 30 } else { 0 },
                255,
            )
        }
        WOOD_END => {
            let dx = xf - 31.5;
            let dy = yf - 31.5;
            let a = dy.atan2(dx);
            let r = (dx * dx + dy * dy).sqrt() + (a * 7.).sin() * 1.1;
            let ring = (r / 4.1).fract();
            let crack = r > 13. && ((a - 0.55).abs() < 0.035 || (a + 2.3).abs() < 0.025);
            let bark = dx.abs().max(dy.abs()) > 28.;
            shade(
                if bark { [102, 63, 31] } else { [181, 136, 74] },
                fine / 2
                    + if crack {
                        -65
                    } else if ring < 0.21 {
                        -29
                    } else {
                        8
                    },
                255,
            )
        }
        LEAVES => {
            let (_, dx, dy, h) = cells(xf, yf, 8., 8);
            let shape = dx.abs() / 4.4 + dy.abs() / 3.5;
            let alpha = if shape > 1.38 { 0 } else { 255 };
            shade(
                [52, 106, 36],
                fine + (h % 31) as i16 - 12 + ((-dx - dy) * 3.) as i16,
                alpha,
            )
        }
        9 => {
            let board = y / 16;
            let local = y % 16;
            let join = (x + board * 21) % 64;
            let grain = (xf * 0.29 + (yf * 0.17).sin()).sin() * 4.;
            let seam = local < 2 || join < 2;
            shade(
                [157, 112, 62],
                fine / 2
                    + (grain as i16)
                    + if seam {
                        -43
                    } else if local == 2 {
                        17
                    } else {
                        0
                    },
                255,
            )
        }
        GLASS => {
            let rim = x < 2 || y < 2 || x > 61 || y > 61;
            let shine = (xf - yf - 18.).abs() < 1.6 || (xf - yf + 24.).abs() < 0.8;
            if rim {
                [143, 215, 227, 190]
            } else if shine {
                [212, 245, 250, 115]
            } else {
                [135, 208, 225, 28]
            }
        }
        WATER => {
            let (edge, dx, dy, h) = cells(xf, yf, 16., 12);
            shade(
                [19, 119, 173],
                fine / 3
                    + (h % 13) as i16
                    + if edge < 0.7 {
                        57
                    } else {
                        ((-dx - dy) * 0.8) as i16
                    },
                155,
            )
        }
        14 => shade(
            [227, 239, 246],
            fine / 2
                + if noise_at(14, x / 3, y / 3) % 23 < 2 {
                    -13
                } else {
                    0
                },
            255,
        ),
        15 => {
            let row = y / 16;
            let joint = (x + row * 24) % 32;
            shade(
                [205, 182, 128],
                fine / 2
                    + if y % 16 < 2 || joint < 2 {
                        -28
                    } else if y % 16 == 2 {
                        13
                    } else {
                        0
                    },
                255,
            )
        }
        _ => [255, 0, 255, 255],
    }
}
fn turf(x: u32, y: u32) -> [u8; 4] {
    let h = noise_at(16, x / 4, y / 8);
    let blade_x = (x % 4) as f32;
    let blade_y = (y % 8) as f32;
    let curve = 1.3 + ((blade_y * 0.55) + (h % 9) as f32).sin() * 0.7;
    let blade = (blade_x - curve).abs() < 1.1 * (1. - blade_y / 10.);
    let fine = (noise_at(16, x, y) % 13) as i16 - 6;
    shade(
        if blade { [111, 151, 48] } else { [65, 115, 36] },
        fine + (h % 23) as i16 - 9 + if blade && blade_x < curve { 14 } else { 0 },
        255,
    )
}
fn soil(x: u32, y: u32) -> [u8; 4] {
    let (edge, dx, dy, h) = cells(x as f32, y as f32, 8., 2);
    let fine = (noise_at(2, x, y) % 13) as i16 - 6;
    let pebble = h.is_multiple_of(5) && dx * dx + dy * dy < 3.5;
    shade(
        if pebble {
            [156, 146, 119]
        } else {
            [111, 77, 46]
        },
        fine + (h % 21) as i16 - 10 + if edge < 0.7 { -8 } else { 0 },
        255,
    )
}
// Periodic jittered Voronoi cells give individually shaded pebbles/rock facets.
fn cells(x: f32, y: f32, scale: f32, id: u8) -> (f32, f32, f32, u32) {
    let gx = (x / scale).floor() as i32;
    let gy = (y / scale).floor() as i32;
    let period = 64 / (scale as i32);
    let mut first = f32::INFINITY;
    let mut second = f32::INFINITY;
    let mut found = (0., 0., 0);
    for oy in -1..=1 {
        for ox in -1..=1 {
            let cx = gx + ox;
            let cy = gy + oy;
            let h = noise_at(
                id,
                cx.rem_euclid(period) as u32,
                cy.rem_euclid(period) as u32,
            );
            let px = (cx as f32 + 0.25 + (h % 101) as f32 / 200.) * scale;
            let py = (cy as f32 + 0.25 + ((h >> 8) % 101) as f32 / 200.) * scale;
            let dx = x - px;
            let dy = y - py;
            let d = dx * dx + dy * dy;
            if d < first {
                second = first;
                first = d;
                found = (dx, dy, h);
            } else if d < second {
                second = d;
            }
        }
    }
    (second.sqrt() - first.sqrt(), found.0, found.1, found.2)
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

// Keep crisp magnification while filtering distant tiles to avoid moiré.
fn with_mipmaps(base: Vec<u8>, mut width: usize) -> Vec<u8> {
    let mut data = base.clone();
    let mut level = base;
    while width > 1 {
        let next_width = width / 2;
        let mut next = vec![0; next_width * next_width * 4];
        for y in 0..next_width {
            for x in 0..next_width {
                for c in 0..4 {
                    let mut sum = 0_u16;
                    for dy in 0..2 {
                        for dx in 0..2 {
                            sum += u16::from(level[((y * 2 + dy) * width + x * 2 + dx) * 4 + c]);
                        }
                    }
                    next[(y * next_width + x) * 4 + c] = (sum / 4) as u8;
                }
            }
        }
        data.extend_from_slice(&next);
        level = next;
        width = next_width;
    }
    data
}
