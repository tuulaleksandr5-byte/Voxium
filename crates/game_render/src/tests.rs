use super::*;
use bevy::{
    image::{ImageAddressMode, ImageFilterMode, ImageSampler},
    mesh::VertexAttributeValues,
    prelude::*,
};

fn positions(mesh: &Mesh) -> &[[f32; 3]] {
    match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
        VertexAttributeValues::Float32x3(values) => values,
        _ => panic!("positions must be Float32x3"),
    }
}

fn normals(mesh: &Mesh) -> &[[f32; 3]] {
    match mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap() {
        VertexAttributeValues::Float32x3(values) => values,
        _ => panic!("normals must be Float32x3"),
    }
}

fn uvs(mesh: &Mesh) -> &[[f32; 2]] {
    match mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap() {
        VertexAttributeValues::Float32x2(values) => values,
        _ => panic!("UVs must be Float32x2"),
    }
}

fn assert_quads(mesh: &Mesh, count: usize) {
    assert_eq!(mesh.count_vertices(), count * 4);
    assert_eq!(mesh.indices().unwrap().len(), count * 6);
}

#[test]
fn air_produces_no_meshes() {
    assert!(mesh_section([0; 3], |_| 0).is_empty());
}

#[test]
fn exposed_concave_corner_has_soft_ambient_occlusion() {
    let meshes = mesh_section([0; 3], |p| match p {
        [1, 1, 1] | [0, 2, 1] | [1, 2, 0] => 3,
        _ => 0,
    });
    let mesh = &meshes[0].1;
    let Some(VertexAttributeValues::Float32x4(colors)) = mesh.attribute(Mesh::ATTRIBUTE_COLOR)
    else {
        panic!("voxel corners need ambient occlusion colors");
    };
    assert!(colors.iter().any(|c| c[0] < 0.9));
    assert!(colors.iter().all(|c| c[0] >= 0.72 && c[3] == 1.));
    assert!(colors.iter().any(|c| c[0] == 1.));
}

#[test]
fn ambient_occlusion_agrees_at_shared_section_vertices() {
    let sample = |p: [i32; 3]| match p {
        [15, 0, 0] | [16, 0, 0] | [15, 1, 1] => 3,
        _ => 0,
    };
    let brightness = |section, point: [f32; 3]| {
        let meshes = mesh_section(section, sample);
        let mesh = &meshes[0].1;
        let Some(VertexAttributeValues::Float32x4(colors)) = mesh.attribute(Mesh::ATTRIBUTE_COLOR)
        else {
            panic!("AO required");
        };
        let i = positions(mesh)
            .iter()
            .zip(normals(mesh))
            .position(|(p, n)| *p == point && *n == [0., 1., 0.])
            .unwrap();
        colors[i][0]
    };
    let left = brightness([0, 0, 0], [16., 1., 1.]);
    let right = brightness([1, 0, 0], [0., 1., 1.]);
    assert_eq!(left, right);
    assert!(left < 1.);
}

#[test]
fn one_block_has_six_outward_faces() {
    let meshes = mesh_section([0; 3], |p| if p == [0; 3] { 3 } else { 0 });
    assert_eq!(meshes.len(), 1);
    assert_eq!(meshes[0].0, 3);
    let mesh = &meshes[0].1;
    assert_quads(mesh, 6);
    let vertices = positions(mesh);
    let normals = normals(mesh);
    let indices: Vec<_> = mesh.indices().unwrap().iter().collect();
    for [a, b, c] in indices.as_chunks::<3>().0 {
        let a = *a;
        let b = *b;
        let c = *c;
        let cross = (Vec3::from(vertices[b]) - Vec3::from(vertices[a]))
            .cross(Vec3::from(vertices[c]) - Vec3::from(vertices[a]));
        let normal = Vec3::from(normals[a]);
        assert!(cross.dot(normal) > 0.0, "triangle must be outward CCW");
        assert!(
            (Vec3::from(vertices[a]) - Vec3::splat(0.5)).dot(normal) > 0.0,
            "normal must point away from the cube center"
        );
    }
}

#[test]
fn adjacent_equal_blocks_merge_with_repeating_uvs() {
    let meshes = mesh_section([0; 3], |[x, y, z]| {
        if (0..2).contains(&x) && y == 0 && z == 0 {
            3
        } else {
            0
        }
    });
    assert_eq!(meshes.len(), 1);
    assert_quads(&meshes[0].1, 6);
    assert!(uvs(&meshes[0].1).iter().flatten().any(|uv| *uv == 2.0));
}

#[test]
fn adjacent_different_blocks_keep_material_boundaries() {
    let meshes = mesh_section([0; 3], |p| match p {
        [0, 0, 0] => 2,
        [1, 0, 0] => 3,
        _ => 0,
    });
    assert_eq!(meshes.len(), 2);
    assert_eq!(meshes[0].0, 2);
    assert_eq!(meshes[1].0, 3);
    for (_, mesh) in meshes {
        assert_quads(&mesh, 5);
    }
}

#[test]
fn solid_section_collapses_to_six_quads() {
    let meshes = mesh_section([0; 3], |p| {
        if p.into_iter().all(|v| (0..16).contains(&v)) {
            3
        } else {
            0
        }
    });
    assert_eq!(meshes.len(), 1);
    assert_quads(&meshes[0].1, 6);
    assert!(uvs(&meshes[0].1).iter().flatten().any(|uv| *uv == 16.0));
}

#[test]
fn neighboring_solid_section_culls_shared_boundary() {
    let meshes = mesh_section([0; 3], |[x, y, z]| {
        if (0..32).contains(&x) && (0..16).contains(&y) && (0..16).contains(&z) {
            3
        } else {
            0
        }
    });
    assert_eq!(meshes.len(), 1);
    assert_quads(&meshes[0].1, 5);
    assert!(!normals(&meshes[0].1).contains(&[1.0, 0.0, 0.0]));
}

#[test]
fn negative_section_uses_world_sampling_and_local_vertices() {
    let meshes = mesh_section([-1, 0, -1], |p| match p {
        [-1, 0, -1] | [0, 0, -1] => 3,
        _ => 0,
    });
    assert_eq!(meshes.len(), 1);
    let mesh = &meshes[0].1;
    assert_quads(mesh, 5);
    for &[x, y, z] in positions(mesh) {
        assert!((15.0..=16.0).contains(&x));
        assert!((0.0..=1.0).contains(&y));
        assert!((15.0..=16.0).contains(&z));
    }
}

#[test]
fn identical_transparent_blocks_hide_their_shared_face() {
    for id in [8, 11, 12] {
        let meshes = mesh_section([0; 3], |[x, y, z]| {
            if (0..2).contains(&x) && y == 0 && z == 0 {
                id
            } else {
                0
            }
        });
        assert_eq!(meshes.len(), 1);
        assert_quads(&meshes[0].1, 6);
    }
}

#[test]
fn transparent_opaque_boundary_keeps_visible_solid_face() {
    let meshes = mesh_section([0; 3], |p| match p {
        [0, 0, 0] => 3,
        [1, 0, 0] => 11,
        _ => 0,
    });
    assert_eq!(meshes.len(), 2);
    assert_eq!(meshes[0].0, 3);
    assert_quads(&meshes[0].1, 6);
    assert_eq!(meshes[1].0, 12);
    assert_quads(&meshes[1].1, 5);
}

#[test]
fn array_materials_share_image_and_use_linear_anisotropic_filtering() {
    let mut images = Assets::<Image>::default();
    let mut materials = Assets::<VoxelMaterial>::default();
    let fixture = image::RgbaImage::from_pixel(768, 384, image::Rgba([120, 150, 180, 255]));
    let mut png = std::io::Cursor::new(Vec::new());
    fixture.write_to(&mut png, image::ImageFormat::Png).unwrap();
    let handles = create_materials(&mut images, &mut materials, png.get_ref()).unwrap();
    assert_eq!(handles.len(), 18);
    let first = materials.get(&handles[0]).unwrap().extension.image.clone();
    for (layer, handle) in handles.iter().enumerate() {
        let m = materials.get(handle).unwrap();
        assert_eq!(m.extension.image, first);
        assert_eq!(m.extension.layer, layer as u32);
    }
    let image = images.get(&first).unwrap();
    assert_eq!((image.width(), image.height()), (128, 128));
    assert_eq!(image.texture_descriptor.size.depth_or_array_layers, 18);
    assert_eq!(image.texture_descriptor.mip_level_count, 8);
    let ImageSampler::Descriptor(s) = &image.sampler else {
        panic!("explicit sampler required");
    };
    assert_eq!(
        (s.mag_filter, s.min_filter, s.mipmap_filter),
        (
            ImageFilterMode::Linear,
            ImageFilterMode::Linear,
            ImageFilterMode::Linear
        )
    );
    assert_eq!(s.anisotropy_clamp, 8);
    assert_eq!(
        (s.address_mode_u, s.address_mode_v),
        (ImageAddressMode::Repeat, ImageAddressMode::Repeat)
    );
    for layer in [12, 13] {
        assert_eq!(
            materials.get(&handles[layer]).unwrap().base.alpha_mode,
            AlphaMode::Blend
        );
    }
    assert_eq!(
        materials.get(&handles[9]).unwrap().base.alpha_mode,
        AlphaMode::Mask(0.45)
    );
}

#[test]
fn grass_has_separate_turf_and_soil_with_full_2d_repeating_uvs() {
    let meshes = mesh_section([0; 3], |p| if p == [0; 3] { 1 } else { 0 });
    let side = &meshes.iter().find(|(id, _)| *id == 1).unwrap().1;
    assert_quads(side, 4);
    for i in (0..positions(side).len()).step_by(4) {
        assert!(positions(side)[i + 2][1] > positions(side)[i][1]);
        assert!(uvs(side)[i + 2][1] < uvs(side)[i][1]);
    }
    for (id, normal) in [(0, [0., 1., 0.]), (2, [0., -1., 0.])] {
        let mesh = &meshes
            .iter()
            .find(|(slot, _)| *slot == id)
            .expect("dedicated grass face material")
            .1;
        assert_quads(mesh, 1);
        assert_eq!(normals(mesh)[0], normal);
        for axis in 0..2 {
            let values: Vec<_> = uvs(mesh).iter().map(|uv| uv[axis]).collect();
            assert!(
                values.contains(&0.) && values.contains(&1.),
                "never flatten a texture coordinate"
            );
        }
    }
}
#[test]
fn log_end_is_separate_from_bark() {
    let meshes = mesh_section([0; 3], |p| if p == [0; 3] { 7 } else { 0 });
    assert_quads(&meshes.iter().find(|(id, _)| *id == 7).unwrap().1, 4);
    let ends = &meshes
        .iter()
        .find(|(id, _)| *id == 8)
        .expect("growth rings")
        .1;
    assert_quads(ends, 2);
}
