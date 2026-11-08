//! CPU-baked hotbar cubes using only pixels from the supplied original atlas.

use crate::{BlockAtlas, BlockTextures, MATERIAL_COUNT};
use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::Image,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use game_core::{AIR, BLOCKS};
use image::RgbaImage;

const ICON_SIZE: u32 = 128;
const TILE_SIZE: u32 = 128;
const SAMPLES: u32 = 4;
type Point = [f32; 2];

// Shared vertices keep all three faces watertight. Coordinates use image axes.
const A: Point = [64., 8.];
const B: Point = [12., 34.];
const C: Point = [64., 60.];
const D: Point = [116., 34.];
const E: Point = [12., 94.];
const F: Point = [64., 120.];
const G: Point = [116., 94.];
const EDGES: [(Point, Point); 9] = [
    (A, B),
    (A, D),
    (B, C),
    (C, D),
    (B, E),
    (C, F),
    (D, G),
    (E, F),
    (F, G),
];

/// Bake one reusable UI image per stable block ID, including transparent AIR.
///
/// The input must be the original 768×384 PNG (6×3 tiles of 128 pixels).
/// Top and side layers come from `textures`; the hidden bottom is never drawn.
/// Left/right RGB channels are multiplied by exactly 0.85/0.70, respectively.
/// Source alpha is retained, including for glass and water. Bilinear sampling
/// clamps within each tile, and 4× supersampling antialiases the cube outline.
/// Call once when loading the atlas, add the results to `Assets<Image>`, and
/// reuse the resulting handles whenever the hotbar selection changes.
pub fn bake_block_icons(atlas_png: &[u8], textures: &BlockTextures) -> Result<Vec<Image>, String> {
    bake_atlas_icons(&BlockAtlas::from_png(atlas_png)?, textures)
}

/// Bake reusable icons directly from the same decoded atlas as world materials.
/// Accepts either an original whole atlas or the original seventeen tile PNGs
/// assembled by `BlockAtlas::from_tiles`, without re-encoding any source image.
pub fn bake_atlas_icons(
    atlas: &BlockAtlas,
    textures: &BlockTextures,
) -> Result<Vec<Image>, String> {
    if textures.0.iter().any(|faces| {
        [faces.top, faces.side, faces.bottom]
            .into_iter()
            .any(|layer| layer as usize >= MATERIAL_COUNT)
    }) {
        return Err("icon texture layer must be in 0..18".into());
    }
    Ok((0..BLOCKS.len())
        .map(|block| {
            let pixels = if block == AIR as usize {
                vec![0; (ICON_SIZE * ICON_SIZE * 4) as usize]
            } else {
                bake_cube(&atlas.rgba, textures, block as u8)
            };
            let mut image = Image::new(
                Extent3d {
                    width: ICON_SIZE,
                    height: ICON_SIZE,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                pixels,
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::default(),
            );
            image.sampler = ImageSampler::linear();
            image
        })
        .collect())
}

fn bake_cube(atlas: &RgbaImage, textures: &BlockTextures, block: u8) -> Vec<u8> {
    let mut output = vec![0; (ICON_SIZE * ICON_SIZE * 4) as usize];
    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            let mut sum = [0.; 4];
            for sy in 0..SAMPLES {
                for sx in 0..SAMPLES {
                    let point = [
                        x as f32 + (sx as f32 + 0.5) / SAMPLES as f32,
                        y as f32 + (sy as f32 + 0.5) / SAMPLES as f32,
                    ];
                    let face = face_uv(point, A, D, B)
                        .map(|uv| (uv, textures.layer(block, 1, true), 1.))
                        .or_else(|| {
                            face_uv(point, B, C, E)
                                .map(|uv| (uv, textures.layer(block, 2, true), 0.85))
                        })
                        .or_else(|| {
                            face_uv(point, C, D, F)
                                .map(|uv| (uv, textures.layer(block, 0, true), 0.70))
                        });
                    let Some((uv, layer, shade)) = face else {
                        continue;
                    };
                    let mut color = sample_tile(atlas, layer, uv);
                    let outline = EDGES.iter().any(|&(start, end)| {
                        segment_distance_squared(point, start, end) < 0.65 * 0.65
                    });
                    for channel in &mut color[..3] {
                        *channel = if outline { 24. } else { *channel * shade };
                    }
                    // Accumulate premultiplied color so transparent texels and
                    // partially covered outline pixels cannot create dark halos.
                    for channel in 0..3 {
                        sum[channel] += color[channel] * color[3];
                    }
                    sum[3] += color[3];
                }
            }
            let pixel = &mut output[((y * ICON_SIZE + x) * 4) as usize..][..4];
            if sum[3] > 0. {
                for channel in 0..3 {
                    pixel[channel] = (sum[channel] / sum[3]).round() as u8;
                }
                pixel[3] = (sum[3] / (SAMPLES * SAMPLES) as f32).round() as u8;
            }
        }
    }
    output
}

fn face_uv(point: Point, origin: Point, u_corner: Point, v_corner: Point) -> Option<Point> {
    let u = [u_corner[0] - origin[0], u_corner[1] - origin[1]];
    let v = [v_corner[0] - origin[0], v_corner[1] - origin[1]];
    let p = [point[0] - origin[0], point[1] - origin[1]];
    let determinant = u[0] * v[1] - u[1] * v[0];
    let uv = [
        (p[0] * v[1] - p[1] * v[0]) / determinant,
        (u[0] * p[1] - u[1] * p[0]) / determinant,
    ];
    uv.iter()
        .all(|&coordinate| (-0.00001..=1.00001).contains(&coordinate))
        .then_some(uv)
}

fn segment_distance_squared(point: Point, start: Point, end: Point) -> f32 {
    let direction = [end[0] - start[0], end[1] - start[1]];
    let delta = [point[0] - start[0], point[1] - start[1]];
    let t = ((delta[0] * direction[0] + delta[1] * direction[1])
        / (direction[0] * direction[0] + direction[1] * direction[1]))
        .clamp(0., 1.);
    (delta[0] - direction[0] * t).powi(2) + (delta[1] - direction[1] * t).powi(2)
}

/// Return straight RGBA, filtering premultiplied alpha to avoid color bleeding
/// from transparent source texels. Taps never enter neighboring atlas tiles.
fn sample_tile(atlas: &RgbaImage, layer: u8, uv: Point) -> [f32; 4] {
    let x = (uv[0] * TILE_SIZE as f32 - 0.5).clamp(0., (TILE_SIZE - 1) as f32);
    let y = (uv[1] * TILE_SIZE as f32 - 0.5).clamp(0., (TILE_SIZE - 1) as f32);
    let x0 = x.floor() as u32;
    let y0 = y.floor() as u32;
    let origin = [
        u32::from(layer % 6) * TILE_SIZE,
        u32::from(layer / 6) * TILE_SIZE,
    ];
    let mut sum = [0.; 4];
    for (dy, wy) in [(0, 1. - y.fract()), (1, y.fract())] {
        for (dx, wx) in [(0, 1. - x.fract()), (1, x.fract())] {
            let pixel = atlas.get_pixel(
                origin[0] + (x0 + dx).min(TILE_SIZE - 1),
                origin[1] + (y0 + dy).min(TILE_SIZE - 1),
            );
            let weight = wx * wy;
            for channel in 0..3 {
                sum[channel] += f32::from(pixel[channel]) * f32::from(pixel[3]) * weight;
            }
            sum[3] += f32::from(pixel[3]) * weight;
        }
    }
    if sum[3] > 0. {
        for channel in 0..3 {
            sum[channel] /= sum[3];
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FaceTextures;
    use std::io::Cursor;

    const ATLAS_WIDTH: u32 = TILE_SIZE * 6;
    const ATLAS_HEIGHT: u32 = TILE_SIZE * 3;

    // Deliberately simple test colors, never used as production block artwork.
    fn fixture(alpha: u8) -> (Vec<u8>, BlockTextures) {
        let mut atlas = RgbaImage::new(ATLAS_WIDTH, ATLAS_HEIGHT);
        for (x, y, pixel) in atlas.enumerate_pixels_mut() {
            let layer = (y / TILE_SIZE) * 6 + x / TILE_SIZE;
            *pixel = image::Rgba(match layer {
                0 => [240, 40, 20, alpha],
                7 => [40, 200, 100, alpha],
                _ => [10, 20, 250, alpha],
            });
        }
        let mut bytes = Cursor::new(Vec::new());
        atlas.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
        (
            bytes.into_inner(),
            BlockTextures(
                [FaceTextures {
                    top: 0,
                    side: 7,
                    bottom: 17,
                }; BLOCKS.len()],
            ),
        )
    }

    fn pixel(icon: &Image, x: usize, y: usize) -> &[u8] {
        &icon.data.as_ref().unwrap()[(y * ICON_SIZE as usize + x) * 4..][..4]
    }

    #[test]
    fn ron_faces_and_exact_side_shading_are_baked_in_block_id_order() {
        let (bytes, mut textures) = fixture(255);
        textures.0[7] = FaceTextures {
            top: 7,
            side: 0,
            bottom: 17,
        };
        let icons = bake_block_icons(&bytes, &textures).unwrap();
        assert_eq!(icons.len(), BLOCKS.len());
        assert_eq!(icons[1].texture_descriptor.size.width, ICON_SIZE);
        assert!(
            icons[AIR as usize]
                .data
                .as_ref()
                .unwrap()
                .iter()
                .all(|&b| b == 0)
        );
        assert_eq!(pixel(&icons[1], 64, 32), [240, 40, 20, 255]);
        assert_eq!(pixel(&icons[1], 40, 75), [34, 170, 85, 255]);
        assert_eq!(pixel(&icons[1], 88, 75), [28, 140, 70, 255]);
        assert_eq!(pixel(&icons[15], 64, 32), pixel(&icons[1], 64, 32));
        assert_eq!(pixel(&icons[7], 64, 32), [40, 200, 100, 255]);
        assert_eq!(pixel(&icons[7], 40, 75), [204, 34, 17, 255]);
        assert_eq!(pixel(&icons[7], 88, 75), [168, 28, 14, 255]);
        // The selected bottom tile is blue, which must not appear on the cube.
        assert!(
            !icons[1]
                .data
                .as_ref()
                .unwrap()
                .as_chunks::<4>()
                .0
                .iter()
                .any(|p| p[2] > p[1])
        );
    }

    #[test]
    fn source_alpha_and_transparent_background_survive_baking() {
        let (bytes, textures) = fixture(100);
        let icons = bake_block_icons(&bytes, &textures).unwrap();
        for id in [1, 11, 12] {
            assert_eq!(pixel(&icons[id], 64, 32), [240, 40, 20, 100]);
            assert_eq!(pixel(&icons[id], 40, 75), [34, 170, 85, 100]);
            assert_eq!(pixel(&icons[id], 0, 0), [0, 0, 0, 0]);
            assert_eq!(pixel(&icons[id], 127, 127), [0, 0, 0, 0]);
        }
        // A thin stroke has partial sample coverage in this pixel; alpha is
        // unchanged while the edge is darker than the neighboring right face.
        assert_eq!(pixel(&icons[1], 64, 80), [25, 53, 36, 100]);
    }

    #[test]
    fn opaque_faces_have_no_transparent_cracks_at_shared_edges() {
        let (bytes, textures) = fixture(255);
        let icons = bake_block_icons(&bytes, &textures).unwrap();
        // Check the full interior of the hexagonal silhouette, including all
        // three shared face boundaries and the center junction.
        let perimeter = [(A, B), (A, D), (B, E), (D, G), (E, F), (F, G)];
        for y in 0..ICON_SIZE {
            for x in 0..ICON_SIZE {
                let p = [x as f32 + 0.5, y as f32 + 0.5];
                let inside = face_uv(p, A, D, B).is_some()
                    || face_uv(p, B, C, E).is_some()
                    || face_uv(p, C, D, F).is_some();
                if inside
                    && perimeter
                        .iter()
                        .all(|&(a, b)| segment_distance_squared(p, a, b) > 1.)
                {
                    assert_eq!(
                        pixel(&icons[1], x as usize, y as usize)[3],
                        255,
                        "gap at {x},{y}"
                    );
                }
            }
        }
    }

    #[test]
    fn bilinear_sampling_clamps_tiles_and_ignores_hidden_rgb() {
        let mut atlas =
            RgbaImage::from_pixel(ATLAS_WIDTH, ATLAS_HEIGHT, image::Rgba([0, 0, 255, 255]));
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                atlas.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
            }
        }
        assert_eq!(sample_tile(&atlas, 0, [1., 1.]), [255., 0., 0., 255.]);
        atlas.put_pixel(0, 0, image::Rgba([0, 255, 0, 0]));
        assert_eq!(
            sample_tile(&atlas, 0, [1. / 128., 0.5 / 128.]),
            [255., 0., 0., 127.5]
        );
    }

    #[test]
    fn invalid_png_dimensions_and_mappings_are_rejected() {
        let mut tiny = Cursor::new(Vec::new());
        RgbaImage::new(6, 3)
            .write_to(&mut tiny, image::ImageFormat::Png)
            .unwrap();
        assert!(
            bake_block_icons(tiny.get_ref(), &BlockTextures::default())
                .unwrap_err()
                .contains("768×384")
        );
        assert!(bake_block_icons(b"not a PNG", &BlockTextures::default()).is_err());
        let (bytes, mut textures) = fixture(255);
        textures.0[1].side = 18;
        assert!(
            bake_block_icons(&bytes, &textures)
                .unwrap_err()
                .contains("0..18")
        );
    }
}
