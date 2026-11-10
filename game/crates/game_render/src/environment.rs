use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
    },
};

/// Small continuous cubemap gradient; every face evaluates the same world direction.
/// Skybox brightness supplies day/night modulation without regenerating GPU images.
pub fn gradient_sky() -> Image {
    const SIZE: u32 = 64;
    let mut pixels = Vec::with_capacity((SIZE * SIZE * 6 * 4) as usize);
    for face in 0..6 {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let u = 2. * (x as f32 + 0.5) / SIZE as f32 - 1.;
                let v = 2. * (y as f32 + 0.5) / SIZE as f32 - 1.;
                let direction = match face {
                    0 => Vec3::new(1., -v, -u),
                    1 => Vec3::new(-1., -v, u),
                    2 => Vec3::new(u, 1., v),
                    3 => Vec3::new(u, -1., -v),
                    4 => Vec3::new(u, -v, 1.),
                    _ => Vec3::new(-u, -v, -1.),
                }
                .normalize();
                let t = direction.y.max(0.).sqrt();
                let horizon = Vec3::new(0.72, 0.84, 0.90);
                let zenith = Vec3::new(0.24, 0.52, 0.83);
                let c = horizon.lerp(zenith, t);
                pixels.extend([
                    (c.x * 255.) as u8,
                    (c.y * 255.) as u8,
                    (c.z * 255.) as u8,
                    255,
                ]);
            }
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 6,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
    image
}
