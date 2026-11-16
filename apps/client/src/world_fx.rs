//! Живые блоки мира: падающий песок, горящий динамит и сам взрыв.
use crate::{
    block_audio::{EffectKind, EffectSound},
    session::Game,
    streaming::Streaming,
};
use bevy::prelude::*;
use game_core::{AIR, BlockId, TNT};
use std::collections::{HashMap, HashSet};

/// Взрыв в точке: рисует вспышку, дым и обломки.
#[derive(Message, Clone, Copy)]
pub struct ExplosionMessage(pub Vec3);

#[derive(Component)]
struct FallingVisual(u64);
#[derive(Component)]
struct PrimedVisual(u64);
#[derive(Component)]
struct FlashOverlay;
#[derive(Component)]
struct DebrisPiece {
    velocity: Vec3,
    spin: Vec3,
    age: f32,
    life: f32,
}
#[derive(Component)]
struct Flash {
    age: f32,
}
#[derive(Component)]
struct Particle {
    velocity: Vec3,
    age: f32,
    life: f32,
    gravity: f32,
    start_scale: f32,
}

#[derive(Default)]
struct Cache {
    blocks: HashMap<BlockId, Vec<(u16, Handle<Mesh>)>>,
    cube: Option<Handle<Mesh>>,
    sphere: Option<Handle<Mesh>>,
    white: Option<Handle<StandardMaterial>>,
    particles: Vec<Handle<StandardMaterial>>,
    counter: u32,
}

pub fn plugin(app: &mut App) {
    app.add_message::<ExplosionMessage>().add_systems(
        Update,
        (
            sync_blocks,
            spawn_debris,
            spawn_explosions,
            animate_particles,
        )
            .chain(),
    );
}

fn hash(n: u32) -> f32 {
    let mut v = n.wrapping_mul(0x9e37_79b9) ^ 0x85eb_ca6b;
    v ^= v >> 15;
    v = v.wrapping_mul(0x2c1b_3c6d);
    v ^= v >> 12;
    v = v.wrapping_mul(0x297a_2d39);
    v ^= v >> 15;
    (v & 0xffff) as f32 / 65535.0
}

#[allow(clippy::too_many_arguments)]
fn spawn_block_children(
    commands: &mut Commands,
    parent: Entity,
    block: BlockId,
    stream: &Streaming,
    meshes: &mut Assets<Mesh>,
    cache: &mut Cache,
) {
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
}

#[allow(clippy::too_many_arguments)]
fn sync_blocks(
    mut commands: Commands,
    game: Res<Game>,
    stream: Res<Streaming>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cache: Local<Cache>,
    mut falling: Query<(Entity, &FallingVisual, &mut Transform), Without<PrimedVisual>>,
    mut primed: Query<(Entity, &PrimedVisual, &mut Transform, &Children), Without<FallingVisual>>,
    mut overlays: Query<&mut Visibility, With<FlashOverlay>>,
    mut sounds: MessageWriter<EffectSound>,
) {
    let active = game.world_id.is_some();
    let mut seen = HashSet::new();
    for (entity, visual, mut transform) in &mut falling {
        match active
            .then(|| game.world.falling.iter().find(|f| f.id == visual.0))
            .flatten()
        {
            Some(block) => {
                transform.translation = Vec3::from_array(block.position);
                seen.insert(visual.0);
            }
            None => commands.entity(entity).despawn(),
        }
    }
    let mut seen_primed = HashSet::new();
    for (entity, visual, mut transform, children) in &mut primed {
        match active
            .then(|| game.world.primed.iter().find(|t| t.id == visual.0))
            .flatten()
        {
            Some(tnt) => {
                transform.translation = Vec3::from_array(tnt.position);
                // Перед взрывом блок раздувается, как в Minecraft.
                let swell = (1.0 - tnt.fuse / 1.5).clamp(0.0, 1.0);
                transform.scale =
                    Vec3::splat(1.0 + 0.18 * swell * swell + 0.03 * (tnt.fuse * 18.0).sin());
                let period = if tnt.fuse < 1.0 { 0.12 } else { 0.25 };
                let on = ((tnt.fuse / period) as i32) % 2 == 0;
                for child in children.iter() {
                    if let Ok(mut visibility) = overlays.get_mut(child) {
                        *visibility = if on {
                            Visibility::Inherited
                        } else {
                            Visibility::Hidden
                        };
                    }
                }
                seen_primed.insert(visual.0);
            }
            None => commands.entity(entity).despawn(),
        }
    }
    if !active {
        return;
    }
    for block in &game.world.falling {
        if seen.contains(&block.id) {
            continue;
        }
        let parent = commands
            .spawn((
                FallingVisual(block.id),
                Transform::from_translation(Vec3::from_array(block.position)),
                Visibility::default(),
            ))
            .id();
        spawn_block_children(
            &mut commands,
            parent,
            block.block,
            &stream,
            &mut meshes,
            &mut cache,
        );
    }
    for tnt in &game.world.primed {
        if seen_primed.contains(&tnt.id) {
            continue;
        }
        let parent = commands
            .spawn((
                PrimedVisual(tnt.id),
                Transform::from_translation(Vec3::from_array(tnt.position)),
                Visibility::default(),
            ))
            .id();
        sounds.write(EffectSound {
            kind: EffectKind::Fuse,
            position: Vec3::from_array(tnt.position),
        });
        spawn_block_children(&mut commands, parent, TNT, &stream, &mut meshes, &mut cache);
        let cube = cache
            .cube
            .get_or_insert_with(|| meshes.add(Cuboid::new(1.0, 1.0, 1.0)))
            .clone();
        let white = cache
            .white
            .get_or_insert_with(|| {
                materials.add(StandardMaterial {
                    base_color: Color::srgba(1.0, 1.0, 1.0, 0.7),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..default()
                })
            })
            .clone();
        let overlay = commands
            .spawn((
                FlashOverlay,
                Mesh3d(cube),
                MeshMaterial3d(white),
                Transform::from_scale(Vec3::splat(1.004)),
                Visibility::Hidden,
            ))
            .id();
        commands.entity(parent).add_child(overlay);
    }
}

/// Блоки, разрушенные взрывом, разлетаются кусками и падают.
#[allow(clippy::too_many_arguments)]
fn spawn_debris(
    mut commands: Commands,
    mut game: ResMut<Game>,
    stream: Res<Streaming>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cache: Local<Cache>,
    existing: Query<(), With<DebrisPiece>>,
) {
    if game.world_id.is_none() {
        game.world.debris.clear();
        return;
    }
    let pending = std::mem::take(&mut game.world.debris);
    let mut room = 160_usize.saturating_sub(existing.iter().count());
    for piece in pending {
        if room == 0 {
            break;
        }
        room -= 1;
        cache.counter = cache.counter.wrapping_add(1);
        let n = cache.counter.wrapping_mul(13);
        let at = Vec3::new(
            piece.position[0] as f32 + 0.5,
            piece.position[1] as f32 + 0.5,
            piece.position[2] as f32 + 0.5,
        );
        let away = (at - Vec3::from_array(piece.center)).normalize_or(Vec3::Y);
        let velocity = away * (4.0 + 5.0 * hash(n))
            + Vec3::new(hash(n + 1) - 0.5, 0.0, hash(n + 2) - 0.5) * 3.0
            + Vec3::Y * (3.0 + 4.0 * hash(n + 3));
        let spin = Vec3::new(hash(n + 4) - 0.5, hash(n + 5) - 0.5, hash(n + 6) - 0.5) * 12.0;
        let parent = commands
            .spawn((
                DebrisPiece {
                    velocity,
                    spin,
                    age: 0.0,
                    life: 1.3 + 0.7 * hash(n + 7),
                },
                Transform::from_translation(at).with_scale(Vec3::splat(0.55 + 0.4 * hash(n + 8))),
                Visibility::default(),
            ))
            .id();
        spawn_block_children(
            &mut commands,
            parent,
            piece.block,
            &stream,
            &mut meshes,
            &mut cache,
        );
    }
}

fn spawn_explosions(
    mut commands: Commands,
    mut messages: MessageReader<ExplosionMessage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cache: Local<Cache>,
    mut sounds: MessageWriter<EffectSound>,
) {
    for ExplosionMessage(center) in messages.read() {
        sounds.write(EffectSound {
            kind: EffectKind::Explosion,
            position: *center,
        });
        let sphere = cache
            .sphere
            .get_or_insert_with(|| meshes.add(Sphere::new(1.0).mesh().ico(2).unwrap()))
            .clone();
        let cube = cache
            .cube
            .get_or_insert_with(|| meshes.add(Cuboid::new(1.0, 1.0, 1.0)))
            .clone();
        if cache.particles.is_empty() {
            for color in [
                Color::srgb(0.18, 0.18, 0.18),
                Color::srgb(0.35, 0.33, 0.30),
                Color::srgb(0.55, 0.52, 0.48),
                Color::srgb(0.95, 0.55, 0.12),
                Color::srgb(1.0, 0.82, 0.3),
            ] {
                cache.particles.push(materials.add(StandardMaterial {
                    base_color: color,
                    unlit: true,
                    ..default()
                }));
            }
        }
        let flash_material = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.85, 0.5, 0.85),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });
        commands.spawn((
            Flash { age: 0.0 },
            Mesh3d(sphere),
            MeshMaterial3d(flash_material),
            Transform::from_translation(*center).with_scale(Vec3::splat(0.5)),
        ));
        for i in 0..48 {
            cache.counter = cache.counter.wrapping_add(1);
            let n = cache.counter.wrapping_mul(7) + i;
            let direction = Vec3::new(
                hash(n) - 0.5,
                hash(n + 101) * 0.8 + 0.1,
                hash(n + 202) - 0.5,
            )
            .normalize_or(Vec3::Y);
            let kind = (hash(n + 303) * cache.particles.len() as f32) as usize;
            let smoke = kind < 3;
            let speed = if smoke {
                1.5 + 3.0 * hash(n + 404)
            } else {
                5.0 + 6.0 * hash(n + 404)
            };
            commands.spawn((
                Particle {
                    velocity: direction * speed,
                    age: 0.0,
                    life: if smoke {
                        1.3 + hash(n + 505)
                    } else {
                        0.7 + 0.5 * hash(n + 505)
                    },
                    gravity: if smoke { -0.8 } else { 14.0 },
                    start_scale: if smoke {
                        0.7 + 0.8 * hash(n + 606)
                    } else {
                        0.12 + 0.12 * hash(n + 606)
                    },
                },
                Mesh3d(cube.clone()),
                MeshMaterial3d(cache.particles[kind.min(cache.particles.len() - 1)].clone()),
                Transform::from_translation(*center + direction * 0.4).with_scale(Vec3::splat(0.2)),
            ));
        }
    }
}

fn animate_particles(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut particles: Query<
        (Entity, &mut Particle, &mut Transform),
        (Without<Flash>, Without<DebrisPiece>),
    >,
    mut debris: Query<
        (Entity, &mut DebrisPiece, &mut Transform),
        (Without<Flash>, Without<Particle>),
    >,
    mut flashes: Query<(
        Entity,
        &mut Flash,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
    )>,
) {
    let dt = time.delta_secs().min(0.1);
    for (entity, mut particle, mut transform) in &mut particles {
        particle.age += dt;
        if particle.age >= particle.life {
            commands.entity(entity).despawn();
            continue;
        }
        let gravity = particle.gravity;
        particle.velocity.y -= gravity * dt;
        particle.velocity *= 1.0 - 1.5 * dt;
        transform.translation += particle.velocity * dt;
        let fraction = particle.age / particle.life;
        let grow = if gravity < 0.0 {
            1.0 + 1.5 * fraction
        } else {
            1.0 - fraction
        };
        transform.scale = Vec3::splat(particle.start_scale * grow.max(0.05));
        transform.rotate_y(dt * 3.0);
    }
    for (entity, mut piece, mut transform) in &mut debris {
        piece.age += dt;
        if piece.age >= piece.life {
            commands.entity(entity).despawn();
            continue;
        }
        piece.velocity.y -= 16.0 * dt;
        let step = piece.velocity * dt;
        transform.translation += step;
        let spin = piece.spin * dt;
        transform.rotate(Quat::from_euler(EulerRot::XYZ, spin.x, spin.y, spin.z));
        let left = piece.life - piece.age;
        if left < 0.35 {
            transform.scale *= (1.0 - dt * 4.0).max(0.1);
        }
    }
    for (entity, mut flash, mut transform, material) in &mut flashes {
        flash.age += dt;
        let fraction = flash.age / 0.4;
        if fraction >= 1.0 {
            materials.remove(&material.0);
            commands.entity(entity).despawn();
            continue;
        }
        transform.scale = Vec3::splat(0.5 + 3.5 * fraction.sqrt());
        if let Some(mut m) = materials.get_mut(&material.0) {
            m.base_color = Color::srgba(
                1.0,
                0.85 - 0.4 * fraction,
                0.5 - 0.3 * fraction,
                0.85 * (1.0 - fraction),
            );
        }
    }
}
