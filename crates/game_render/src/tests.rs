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
fn leaves_are_plain_cubes_that_hide_faces_between_neighbors() {
    // Сплошной объём листвы без границ секции не создаёт ни одной грани.
    assert!(mesh_section([0; 3], |_| 8).is_empty());
    // Куб 3×3×3: видна только оболочка, 6 сторон по 9 клеток в слитых гранях.
    let inner = mesh_section([0; 3], |[x, y, z]| {
        if [x, y, z].iter().all(|n| (4..=6).contains(n)) {
            8
        } else {
            0
        }
    });
    assert_eq!(inner.len(), 1);
    assert_quads(&inner[0].1, 6);
    // Берёзовая листва (30) использует свой слой и тоже строится кубом.
    let birch = mesh_section([0; 3], |p| if p == [3, 4, 5] { 30 } else { 0 });
    assert_eq!(birch.len(), 1);
    assert_eq!(birch[0].0, 41);
    assert_quads(&birch[0].1, 6);
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
    assert_eq!(handles.len(), MATERIAL_COUNT);
    let first = materials.get(&handles[0]).unwrap().extension.image.clone();
    for (layer, handle) in handles.iter().enumerate() {
        let m = materials.get(handle).unwrap();
        assert_eq!(m.extension.image, first);
        assert_eq!(m.extension.layer, layer as u32);
    }
    let image = images.get(&first).unwrap();
    assert_eq!((image.width(), image.height()), (128, 128));
    assert_eq!(
        image.texture_descriptor.size.depth_or_array_layers,
        MATERIAL_COUNT as u32
    );
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

#[test]
fn uploaded_cobblestone_and_gravel_have_distinct_face_layers() {
    let textures = BlockTextures::default();
    for axis in 0..3 {
        for positive in [false, true] {
            assert_eq!(textures.layer(6, axis, positive), 6);
            assert_eq!(textures.layer(10, axis, positive), 11);
        }
    }
    let meshes = mesh_section([0; 3], |p| match p {
        [0, 0, 0] => 6,
        [1, 0, 0] => 10,
        _ => 0,
    });
    assert_eq!(meshes.len(), 2);
    for (layer, mesh) in meshes {
        assert!([6, 11].contains(&layer));
        assert_quads(&mesh, 5);
    }
}

#[test]
fn workstation_front_faces_are_meshed_on_negative_z() {
    for (block, front, side) in [
        (game_core::CRAFTING_TABLE, 26, 27),
        (game_core::FURNACE, 29, 31),
    ] {
        let meshes = mesh_section([0; 3], |p| if p == [0; 3] { block } else { 0 });
        let front_mesh = &meshes
            .iter()
            .find(|(layer, _)| *layer == front)
            .expect("front material")
            .1;
        assert_quads(front_mesh, 1);
        assert!(normals(front_mesh).iter().all(|n| *n == [0., 0., -1.]));
        assert_quads(
            &meshes.iter().find(|(layer, _)| *layer == side).unwrap().1,
            3,
        );
    }
}

#[test]
fn slab_is_a_half_height_box_and_does_not_hide_neighbor_faces() {
    let slab = game_core::OAK_SLAB;
    let meshes = mesh_section([0; 3], move |p| if p == [4, 4, 4] { slab } else { 0 });
    assert_eq!(meshes.len(), 1);
    assert_quads(&meshes[0].1, 6);
    let VertexAttributeValues::Float32x3(points) =
        meshes[0].1.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
    else {
        unreachable!()
    };
    assert!(points.iter().all(|p| (4.0..=4.5).contains(&p[1])));
    assert!(points.iter().any(|p| p[1] == 4.5));
    let normal_sum: [f32; 3] = normals(&meshes[0].1).iter().fold([0.0; 3], |sum, n| {
        [sum[0] + n[0], sum[1] + n[1], sum[2] + n[2]]
    });
    assert_eq!(normal_sum, [0.0; 3]);
    // Камень под плитой сохраняет верхнюю грань, а плита теряет нижнюю.
    let stacked = mesh_section([0; 3], move |p| match p {
        [4, 4, 4] => slab,
        [4, 3, 4] => 3,
        _ => 0,
    });
    let quads: usize = stacked
        .iter()
        .map(|(_, mesh)| mesh.count_vertices() / 4)
        .sum();
    assert_eq!(quads, 6 + 5);
}

/// Каждый треугольник смотрит туда же, куда его нормаль: иначе грань пропала бы из-за отсечения.
fn assert_outward_winding(mesh: &Mesh) {
    let (points, normals) = (positions(mesh).to_vec(), normals(mesh).to_vec());
    let indices: Vec<usize> = mesh.indices().unwrap().iter().collect();
    for triangle in indices.chunks(3) {
        let [a, b, c] = [0, 1, 2].map(|i| Vec3::from_array(points[triangle[i]]));
        let geometric = (b - a).cross(c - a);
        let declared = Vec3::from_array(normals[triangle[0]]);
        assert!(geometric.dot(declared) > 0.0, "{triangle:?}");
    }
}

fn single(block: game_core::BlockId) -> Vec<(u16, Mesh)> {
    mesh_section([0; 3], move |p| if p == [4, 4, 4] { block } else { 0 })
}

#[test]
fn lava_is_one_full_box_in_the_lava_layer_with_outward_faces() {
    let meshes = single(game_core::LAVA);
    assert_eq!(meshes.len(), 1);
    assert_eq!(usize::from(meshes[0].0), crate::LAVA_LAYER);
    assert_quads(&meshes[0].1, 6);
    assert_outward_winding(&meshes[0].1);
    let points = positions(&meshes[0].1).to_vec();
    assert!(points.iter().all(|p| (4.0..=5.0).contains(&p[1])));
    assert!(points.iter().any(|p| p[1] == 5.0));
}

#[test]
fn flowing_water_height_follows_its_level_and_shares_the_water_material() {
    let level_four = game_core::fluids::FluidKind::Water.block(4);
    let meshes = single(level_four);
    assert_eq!(meshes.len(), 1);
    assert_eq!(usize::from(meshes[0].0), crate::WATER_LAYER);
    assert_quads(&meshes[0].1, 6);
    assert_outward_winding(&meshes[0].1);
    let top = positions(&meshes[0].1)
        .iter()
        .map(|p| p[1])
        .fold(f32::MIN, f32::max);
    assert!((top - 4.5).abs() < 1e-5, "{top}");
    // Столб воды: внутренние грани между клетками одной жидкости скрыты.
    let column = mesh_section([0; 3], move |p| {
        if p[0] == 4 && p[2] == 4 && (4..=5).contains(&p[1]) {
            level_four
        } else {
            0
        }
    });
    assert_quads(&column[0].1, 4 * 2 + 2);
    assert_outward_winding(&column[0].1);
    // Два одинаковых течения рядом скрывают общие боковые грани.
    let pair = mesh_section([0; 3], move |p| {
        if p[1] == 4 && p[2] == 4 && (4..=5).contains(&p[0]) {
            level_four
        } else {
            0
        }
    });
    assert_quads(&pair[0].1, 10);
}

#[test]
fn level_steps_show_a_strip_on_the_taller_side_and_source_water_merges_with_flow() {
    let high = game_core::fluids::FluidKind::Water.block(7);
    let low = game_core::fluids::FluidKind::Water.block(3);
    let meshes = mesh_section([0; 3], move |p| match p {
        [4, 4, 4] => high,
        [5, 4, 4] => low,
        _ => 0,
    });
    // Высокая клетка: верх, низ, три полноценных бока и полоска к низкой; низкая: верх, низ и три бока.
    assert_quads(&meshes[0].1, 6 + 5);
    assert_outward_winding(&meshes[0].1);
    let strip = positions(&meshes[0].1)
        .iter()
        .copied()
        .filter(|p| p[0] == 5.0 && p[2] >= 4.0 && p[2] <= 5.0 && p[1] > 4.3 && p[1] < 4.9)
        .count();
    assert!(strip >= 4, "полоска между уровнями 0.375 и 0.875");
    // Источник и течение рядом: общая боковая грань не рисуется ни у кого.
    let mixed = mesh_section([0; 3], move |p| match p {
        [4, 4, 4] => game_core::WATER,
        [5, 4, 4] => low,
        _ => 0,
    });
    let quads: usize = mixed.iter().map(|(_, m)| m.count_vertices() / 4).sum();
    assert_eq!(quads, 5 + 5);
    for (_, mesh) in &mixed {
        assert_outward_winding(mesh);
    }
}

#[test]
fn slab_faces_wind_outward_too() {
    let meshes = single(game_core::OAK_SLAB);
    assert_outward_winding(&meshes[0].1);
}

#[test]
fn rails_are_one_flat_quad_and_moving_pistons_are_invisible() {
    let rail = single(game_core::POWERED_RAIL);
    assert_eq!(rail.len(), 1);
    assert!(crate::PLANT2_LAYERS.contains(&usize::from(rail[0].0)));
    assert_quads(&rail[0].1, 1);
    assert!(
        positions(&rail[0].1)
            .iter()
            .all(|p| (p[1] - 4.03).abs() < 1e-5)
    );
    assert!(single(game_core::MOVING_PISTON).is_empty());
}

#[test]
fn beds_are_half_blocks_and_torches_stand_in_the_cell_centre() {
    // У кровати разные текстуры верха, боков и низа: по мешу на материал.
    let bed = single(game_core::RED_BED);
    assert_eq!(bed.len(), 3);
    let quads: usize = bed.iter().map(|(_, mesh)| mesh.count_vertices() / 4).sum();
    assert_eq!(quads, 6);
    let mut top = f32::MIN;
    for (_, mesh) in &bed {
        assert_outward_winding(mesh);
        top = positions(mesh).iter().map(|p| p[1]).fold(top, f32::max);
    }
    assert!((top - 4.5).abs() < 1e-5);
    let torch = single(game_core::TORCH);
    assert_quads(&torch[0].1, 2);
    let xs: Vec<f32> = positions(&torch[0].1).iter().map(|p| p[0]).collect();
    let centre = (xs.iter().copied().fold(f32::MIN, f32::max)
        + xs.iter().copied().fold(f32::MAX, f32::min))
        / 2.0;
    assert!((centre - 4.5).abs() < 0.01, "{centre}");
}

#[test]
fn new_leaf_and_plant_layers_are_alpha_masked_and_plants_double_sided() {
    let textures = BlockTextures::default();
    for block in [
        game_core::AZALEA_LEAVES,
        game_core::MANGROVE_ROOTS,
        game_core::SPRUCE_LEAVES,
    ] {
        let layer = usize::from(textures.layer(block, 1, true));
        assert!(crate::is_cutout_layer(layer), "{block}");
        assert!(!crate::is_double_sided_layer(layer), "{block}");
    }
    for block in [
        game_core::FIRE,
        game_core::OAK_SAPLING,
        game_core::COBWEB,
        game_core::DETECTOR_RAIL,
    ] {
        let layer = usize::from(textures.layer(block, 1, true));
        assert!(crate::is_cutout_layer(layer), "{block}");
        assert!(crate::is_double_sided_layer(layer), "{block}");
    }
}

fn all_quads(meshes: &[(u16, Mesh)]) -> usize {
    meshes
        .iter()
        .map(|(_, mesh)| positions(mesh).len() / 4)
        .sum()
}

#[test]
fn every_model_block_meshes_with_outward_faces_inside_its_cell() {
    use game_core::*;
    let samples = [
        OAK_STAIRS,
        OAK_STAIRS + 1,
        OAK_STAIRS + 2,
        OAK_STAIRS + 3,
        COBBLESTONE_STAIRS,
        OAK_SIGN + 2,
        OAK_WALL_SIGN + 3,
        OAK_HANGING_SIGN + 1,
        OAK_WALL_HANGING_SIGN,
        OAK_DOOR,
        OAK_DOOR + 6,
        OAK_DOOR + 12,
        IRON_DOOR + 9,
        OAK_TRAPDOOR,
        OAK_TRAPDOOR + 3,
        LADDER + 1,
        LEVER,
        LEVER + 6,
        STONE_BUTTON + 2,
        STONE_PRESSURE_PLATE,
        FARMLAND,
        SNOW_LAYER,
        CACTUS,
        CAKE,
        REPEATER + 3,
        CHAIN,
        NETHER_PORTAL + 1,
        OAK_FENCE,
        IRON_BARS,
        GLASS_PANE,
        OAK_FENCE_GATE,
        OAK_FENCE_GATE + 5,
        COBBLESTONE_WALL,
        FLOWER_POT,
        ENCHANTING_TABLE,
        BREWING_STAND,
        CAULDRON,
        CAULDRON + 1,
        END_PORTAL,
        END_PORTAL_FRAME,
        DRAGON_EGG,
        ENDER_CHEST,
        TRIPWIRE_HOOK + 2,
        SKELETON_SKULL,
        SKELETON_WALL_SKULL + 1,
        ANVIL + 1,
        HOPPER,
        DAYLIGHT_DETECTOR,
        COMPARATOR + 2,
        OAK_BUTTON + 5,
        BRICK_STAIRS + 3,
        QUARTZ_STAIRS + 1,
        WHITE_STAINED_GLASS_PANE,
        IRON_TRAPDOOR + 2,
        WHITE_CARPET,
        WHITE_BANNER + 1,
        WHITE_WALL_BANNER + 2,
        PRISMARINE_STAIRS,
        PRISMARINE_SLAB,
    ];
    for block in samples {
        let meshes = single(block);
        let quads = all_quads(&meshes);
        assert!(quads >= 6, "блок {block}: граней {quads}");
        for (_, mesh) in &meshes {
            assert_outward_winding(mesh);
            for p in positions(mesh) {
                assert!(
                    (4.0..=5.0).contains(&p[0])
                        && (4.0..=5.0).contains(&p[1])
                        && (4.0..=5.0).contains(&p[2]),
                    "блок {block}: вершина {p:?} вне клетки"
                );
            }
        }
    }
}

#[test]
fn fences_grow_arms_toward_neighbours_and_solid_blocks() {
    let alone = all_quads(&single(game_core::OAK_FENCE));
    let pair = mesh_section([0; 3], |p| match p {
        [4, 4, 4] | [5, 4, 4] => game_core::OAK_FENCE,
        _ => 0,
    });
    // Два забора соединены: у каждого появились руки к соседу.
    assert!(all_quads(&pair) > alone * 2 - 4);
    let wall = mesh_section([0; 3], |p| match p {
        [4, 4, 4] => game_core::OAK_FENCE,
        [3, 4, 4] => game_core::STONE,
        _ => 0,
    });
    let lone = all_quads(&single(game_core::OAK_FENCE));
    assert!(all_quads(&wall) > lone, "рука к камню добавляет грани");
}

#[test]
fn model_faces_hide_behind_opaque_neighbours_only_on_the_cell_boundary() {
    // Плита-коврик у пола: нижняя грань на границе клетки пропадает над камнем.
    let free = all_quads(&single(game_core::STONE_PRESSURE_PLATE));
    let covered = all_quads(&mesh_section([0; 3], |p| match p {
        [4, 4, 4] => game_core::SNOW_LAYER,
        [4, 3, 4] => game_core::STONE,
        _ => 0,
    }));
    assert!(covered > 0 && free > 0);
    let snow_alone = all_quads(&single(game_core::SNOW_LAYER));
    assert!(
        covered < snow_alone + 6,
        "под снегом камень: нижняя грань скрыта, камень добавляет не больше пяти граней"
    );
}

#[test]
fn potted_plants_draw_a_pot_and_a_centred_plant_in_two_layers() {
    let meshes = single(game_core::POTTED_POPPY);
    assert!(meshes.len() >= 2, "горшок и цветок лежат в разных слоях");
    let walls = mesh_section([0; 3], |p| match p {
        [4, 4, 4] => game_core::COBBLESTONE_WALL,
        [5, 4, 4] => game_core::COBBLESTONE_WALL,
        _ => 0,
    });
    assert!(all_quads(&walls) > all_quads(&single(game_core::COBBLESTONE_WALL)) * 2 - 4);
}
