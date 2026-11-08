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
            7
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
    assert_eq!(meshes[1].0, 11);
    assert_quads(&meshes[1].1, 5);
}

#[test]
fn materials_have_distinct_pixels_and_repeat_nearest_sampling() {
    let mut images = Assets::<Image>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    let handles = create_materials(&mut images, &mut materials);
    assert_eq!(handles.len(), 16);
    let mut textures = Vec::new();
    for handle in &handles {
        let material = materials.get(handle).unwrap();
        let image = images
            .get(material.base_color_texture.as_ref().unwrap())
            .unwrap();
        assert_eq!(image.width(), 16);
        assert_eq!(image.height(), 16);
        let ImageSampler::Descriptor(sampler) = &image.sampler else {
            panic!("each block texture must explicitly request pixel sampling")
        };
        assert_eq!(sampler.address_mode_u, ImageAddressMode::Repeat);
        assert_eq!(sampler.address_mode_v, ImageAddressMode::Repeat);
        assert_eq!(sampler.mag_filter, ImageFilterMode::Nearest);
        assert_eq!(sampler.min_filter, ImageFilterMode::Nearest);
        let data = image.data.as_ref().unwrap();
        assert_eq!(data.len(), 16 * 16 * 4);
        textures.push(data.clone());
    }
    for a in 1..16 {
        for b in a + 1..16 {
            assert_ne!(textures[a], textures[b], "block textures must differ");
        }
    }
    for id in [11, 12] {
        assert_eq!(
            materials.get(&handles[id]).unwrap().alpha_mode,
            AlphaMode::Blend
        );
        assert!(
            textures[id]
                .as_chunks::<4>()
                .0
                .iter()
                .any(|pixel| pixel[3] > 0 && pixel[3] < 255)
        );
    }
    assert_eq!(
        materials.get(&handles[8]).unwrap().alpha_mode,
        AlphaMode::Mask(0.5)
    );
    assert!(
        textures[8]
            .as_chunks::<4>()
            .0
            .iter()
            .any(|pixel| pixel[3] == 0)
    );
}
