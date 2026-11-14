//! Вид от первого лица: рука игрока и предмет в ней.
//!
//! Модель висит на камере и уменьшена, чтобы не проходить сквозь стены.
//! Пустая рука — рукав с кистью, блок — маленький куб с настоящими гранями,
//! остальные предметы — плоский значок. Клик даёт замах, ходьба — покачивание.
use crate::{
    graphics_assets::BlockIcons,
    item_icons::ItemIcons,
    player::PlayerCamera,
    session::{Game, Mode},
    streaming::Streaming,
};
use bevy::{
    asset::RenderAssetUsages,
    light::NotShadowCaster,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use game_core::content::ItemId;
use std::{collections::HashMap, f32::consts::PI};

const ANCHOR: Vec3 = Vec3::new(0.16, -0.105, -0.3);
const SWING_SECONDS: f32 = 0.28;

#[derive(Default)]
struct Hand {
    root: Option<Entity>,
    arm: Option<Entity>,
    block: Option<Entity>,
    faces: Vec<Entity>,
    item: Option<Entity>,
    /// Что сейчас показано: предмет и стиль графики.
    shown: Option<(Option<ItemId>, Option<bool>, usize)>,
    swing: f32,
    phase: f32,
    sway: f32,
    icons: HashMap<AssetId<Image>, Handle<StandardMaterial>>,
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update
            .after(crate::player::controls)
            .before(crate::player::sky),
    );
}

/// Грань единичного куба: (ось, положительная сторона) → квад с UV всей текстуры.
fn face_mesh(axis: usize, positive: bool) -> Mesh {
    let mut normal = Vec3::ZERO;
    normal[axis] = if positive { 1.0 } else { -1.0 };
    let up = match (axis, positive) {
        (1, true) => Vec3::NEG_Z,
        (1, false) => Vec3::Z,
        _ => Vec3::Y,
    };
    let across = up.cross(normal);
    let center = normal * 0.5;
    let corner = |a: f32, b: f32| (center + across * a + up * b).to_array();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            corner(-0.5, -0.5),
            corner(0.5, -0.5),
            corner(0.5, 0.5),
            corner(-0.5, 0.5),
        ],
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![normal.to_array(); 4])
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]))
}

const FACES: [(usize, bool); 6] = [
    (0, false),
    (0, true),
    (1, false),
    (1, true),
    (2, false),
    (2, true),
];

fn spawn(
    commands: &mut Commands,
    camera: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    hand: &mut Hand,
) {
    let root = commands
        .spawn((Transform::from_translation(ANCHOR), Visibility::Hidden))
        .id();
    commands.entity(camera).add_child(root);
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let mut solid = |color: Color| {
        materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.95,
            reflectance: 0.05,
            ..default()
        })
    };
    // Рука: бирюзовый рукав и кисть, вытянутые вперёд и чуть к центру экрана.
    let arm = commands
        .spawn((
            Transform::from_xyz(0.02, -0.02, 0.05)
                .with_rotation(Quat::from_rotation_y(0.3) * Quat::from_rotation_x(0.55)),
            Visibility::Inherited,
        ))
        .id();
    for (color, offset, length) in [
        (Color::srgb(0.16, 0.56, 0.52), 0.065, 0.15),
        (Color::srgb(0.86, 0.66, 0.52), -0.05, 0.08),
    ] {
        let part = commands
            .spawn((
                Mesh3d(cube.clone()),
                MeshMaterial3d(solid(color)),
                Transform::from_xyz(0.0, 0.0, offset).with_scale(Vec3::new(0.062, 0.062, length)),
                NotShadowCaster,
            ))
            .id();
        commands.entity(arm).add_child(part);
    }
    let block = commands
        .spawn((
            Transform::from_rotation(Quat::from_rotation_y(0.7) * Quat::from_rotation_x(0.12))
                .with_scale(Vec3::splat(0.07)),
            Visibility::Hidden,
        ))
        .id();
    hand.faces = FACES
        .iter()
        .map(|&(axis, positive)| {
            let face = commands
                .spawn((
                    Mesh3d(meshes.add(face_mesh(axis, positive))),
                    Transform::IDENTITY,
                    NotShadowCaster,
                ))
                .id();
            commands.entity(block).add_child(face);
            face
        })
        .collect();
    let item = commands
        .spawn((
            Mesh3d(meshes.add(Rectangle::new(1.0, 1.0))),
            Transform::from_xyz(-0.01, 0.025, 0.0)
                .with_rotation(Quat::from_rotation_y(-0.45) * Quat::from_rotation_z(0.12))
                .with_scale(Vec3::splat(0.16)),
            Visibility::Hidden,
            NotShadowCaster,
        ))
        .id();
    commands.entity(root).add_children(&[arm, block, item]);
    hand.root = Some(root);
    hand.arm = Some(arm);
    hand.block = Some(block);
    hand.item = Some(item);
}

/// Смещение и поворот руки от замаха (0…1) и шага.
fn pose(swing: f32, phase: f32, sway: f32) -> Transform {
    let hit = (swing * PI).sin();
    let translation = ANCHOR
        + Vec3::new(-0.035, -0.02, -0.05) * hit
        + Vec3::new(phase.sin() * 0.008, -phase.cos().abs() * 0.007, 0.0) * sway;
    Transform::from_translation(translation)
        .with_rotation(Quat::from_rotation_x(-0.75 * hit) * Quat::from_rotation_y(0.35 * hit))
}

#[allow(clippy::too_many_arguments)]
fn update(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mouse: Res<ButtonInput<MouseButton>>,
    game: Res<Game>,
    stream: Res<Streaming>,
    icons: Res<ItemIcons>,
    block_icons: Res<BlockIcons>,
    camera: Single<Entity, With<PlayerCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transforms: Query<&mut Transform, Without<PlayerCamera>>,
    mut hand: Local<Hand>,
) {
    let Some(root) = hand.root else {
        spawn(
            &mut commands,
            *camera,
            &mut meshes,
            &mut materials,
            &mut hand,
        );
        return;
    };
    let visible = game.world_id.is_some()
        && !game.vitals.is_dead()
        && matches!(
            game.mode,
            Mode::Playing | Mode::Paused | Mode::Inventory | Mode::Console
        );
    let stack = game.item_for_slot(game.selected).map(|stack| stack.item);
    let key = (
        stack.filter(|_| visible),
        block_icons.pixel,
        usize::from(visible),
    );
    if hand.shown != Some(key) && !stream.materials.is_empty() {
        hand.shown = Some(key);
        let show = |flag: bool| {
            if flag {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            }
        };
        let block = stack.and_then(|item| game.content.block_for_item(item));
        commands.entity(root).insert(show(visible));
        if let (Some(arm), Some(cube), Some(item)) = (hand.arm, hand.block, hand.item) {
            commands.entity(arm).insert(show(stack.is_none()));
            commands.entity(cube).insert(show(block.is_some()));
            commands
                .entity(item)
                .insert(show(stack.is_some() && block.is_none()));
            if let Some(block) = block {
                for (face, &(axis, positive)) in hand.faces.iter().zip(&FACES) {
                    let layer = stream.textures.layer(block, axis, positive);
                    commands
                        .entity(*face)
                        .insert(MeshMaterial3d(stream.materials[usize::from(layer)].clone()));
                }
            } else if let Some(id) = stack {
                let image = icons.get(id, &game.content, &block_icons);
                let material = hand
                    .icons
                    .entry(image.id())
                    .or_insert_with(|| {
                        materials.add(StandardMaterial {
                            base_color_texture: Some(image.clone()),
                            alpha_mode: AlphaMode::Mask(0.5),
                            cull_mode: None,
                            double_sided: true,
                            perceptual_roughness: 1.0,
                            reflectance: 0.0,
                            ..default()
                        })
                    })
                    .clone();
                commands.entity(item).insert(MeshMaterial3d(material));
            }
        }
    }
    let dt = time.delta_secs().min(0.1);
    let playing = game.mode == Mode::Playing;
    if playing
        && (mouse.just_pressed(MouseButton::Left)
            || mouse.just_pressed(MouseButton::Right)
            || (mouse.pressed(MouseButton::Left) && hand.swing <= 0.0))
    {
        hand.swing = 1.0;
    }
    hand.swing = (hand.swing - dt / SWING_SECONDS).max(0.0);
    let speed = Vec2::new(game.velocity.x, game.velocity.z).length();
    let walking = if playing && game.grounded {
        (speed / 4.3).min(1.2)
    } else {
        0.0
    };
    hand.sway += (walking - hand.sway) * (dt * 8.0).min(1.0);
    hand.phase += speed * dt * 1.6;
    if let Ok(mut transform) = transforms.get_mut(root) {
        *transform = pose(1.0 - hand.swing, hand.phase, hand.sway);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_faces_point_outwards_with_counter_clockwise_winding() {
        for (axis, positive) in FACES {
            let mesh = face_mesh(axis, positive);
            let bevy::mesh::VertexAttributeValues::Float32x3(points) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
            else {
                unreachable!()
            };
            let [a, b, c] = [0, 1, 2].map(|index| Vec3::from_array(points[index]));
            let normal = (b - a).cross(c - a).normalize();
            let mut expected = Vec3::ZERO;
            expected[axis] = if positive { 1.0 } else { -1.0 };
            assert!(normal.distance(expected) < 1e-5, "{axis} {positive}");
            assert!(points.iter().all(|p| p[axis].abs() == 0.5));
        }
    }

    #[test]
    fn resting_hand_stays_in_the_lower_right_and_swing_returns_to_rest() {
        let rest = pose(0.0, 0.0, 0.0);
        assert!(rest.translation.x > 0.0 && rest.translation.y < 0.0 && rest.translation.z < 0.0);
        assert!(rest.rotation.angle_between(Quat::IDENTITY) < 1e-5);
        assert!(pose(0.5, 0.0, 0.0).rotation.angle_between(Quat::IDENTITY) > 0.5);
        assert!(pose(1.0, 0.0, 0.0).translation.distance(rest.translation) < 1e-5);
        // Покачивание от шага не уводит руку из кадра.
        assert!(pose(0.0, 1.3, 1.2).translation.distance(rest.translation) < 0.02);
    }
}
