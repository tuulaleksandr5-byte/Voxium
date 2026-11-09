//! Визуальные сущности дропа читают готовое состояние независимой логики.
use crate::{
    graphics_assets::BlockIcons, item_icons::ItemIcons, session::Game, streaming::Streaming,
};
use bevy::prelude::*;
use game_core::{AIR, content::ItemId};
use std::collections::{HashMap, HashSet};

#[derive(Component)]
struct DropVisual(u64);

#[derive(Default)]
struct Cache {
    blocks: HashMap<u8, Vec<(u8, Handle<Mesh>)>>,
    sprites: HashMap<ItemId, Handle<StandardMaterial>>,
    quad: Option<Handle<Mesh>>,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, render.after(crate::inventory_client::tick_drops));
}

#[allow(clippy::too_many_arguments)]
fn render(
    mut commands: Commands,
    time: Res<Time<Real>>,
    game: Res<Game>,
    stream: Res<Streaming>,
    item_icons: Res<ItemIcons>,
    block_icons: Res<BlockIcons>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cache: Local<Cache>,
    mut visuals: Query<(Entity, &DropVisual, &mut Transform)>,
) {
    let mut present = HashSet::new();
    for (entity, visual, mut transform) in &mut visuals {
        let drop = game
            .world_id
            .as_ref()
            .and_then(|_| game.drops.entities.get(&visual.0));
        if let Some(drop) = drop {
            *transform = pose(drop.id, drop.position, time.elapsed_secs());
            present.insert(drop.id);
        } else {
            commands.entity(entity).despawn();
        }
    }
    if game.world_id.is_none() {
        return;
    }
    for drop in game.drops.entities.values() {
        if present.contains(&drop.id) {
            continue;
        }
        let parent = commands
            .spawn((
                DropVisual(drop.id),
                pose(drop.id, drop.position, time.elapsed_secs()),
                Visibility::default(),
            ))
            .id();
        if let Some(block) = game.content.block_for_item(drop.stack.item) {
            let faces = cache.blocks.entry(block).or_insert_with(|| {
                game_render::mesh_section_with_textures(
                    [0, 0, 0],
                    |p| if p == [0, 0, 0] { block } else { AIR },
                    &stream.textures,
                )
                .into_iter()
                .map(|(layer, mesh)| (layer, meshes.add(mesh)))
                .collect()
            });
            for (layer, mesh) in faces.iter() {
                if let Some(material) = stream.materials.get(*layer as usize) {
                    let child = commands
                        .spawn((
                            Mesh3d(mesh.clone()),
                            MeshMaterial3d(material.clone()),
                            Transform::from_translation(Vec3::splat(-0.5)),
                        ))
                        .id();
                    commands.entity(parent).add_child(child);
                }
            }
        } else {
            let texture = item_icons.get(drop.stack.item, &game.content, &block_icons);
            let material = cache
                .sprites
                .entry(drop.stack.item)
                .or_insert_with(|| {
                    materials.add(StandardMaterial {
                        base_color_texture: Some(texture),
                        alpha_mode: AlphaMode::Mask(0.05),
                        double_sided: true,
                        cull_mode: None,
                        unlit: true,
                        ..default()
                    })
                })
                .clone();
            let quad = cache
                .quad
                .get_or_insert_with(|| meshes.add(Rectangle::new(1., 1.)))
                .clone();
            let child = commands
                .spawn((Mesh3d(quad), MeshMaterial3d(material), Transform::default()))
                .id();
            commands.entity(parent).add_child(child);
        }
    }
}

fn pose(id: u64, position: [f32; 3], seconds: f32) -> Transform {
    Transform::from_translation(
        Vec3::from_array(position) + Vec3::Y * (0.045 * (seconds * 2.5 + id as f32).sin()),
    )
    .with_rotation(Quat::from_rotation_y(seconds * 0.85 + id as f32))
    .with_scale(Vec3::splat(0.24))
}
