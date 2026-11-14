//! Мобы на клиенте: бой, еда, модели из коробок и процедурные анимации.
//!
//! Поведение считает `game_core::mobs`; этот модуль превращает состояние моба
//! в иерархию сущностей (корень → части тела) и каждый кадр поворачивает части:
//! шаг ног, поворот головы за игроком, щипание травы, взмахи крыльев, замах
//! гуля, перебор лап паука, вспышка урона и падение при гибели.
use crate::{
    inventory_client::{loaded_block, look_direction, spawn_drop, sync_hotbar, visible_hit},
    session::{Game, Mode},
    survival_client::{PlayerAction, PlayerActionMessage},
};
use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use game_core::{
    content::ItemKind,
    food::{attack_damage, food_value},
    items::ItemStack,
    mobs::{Behavior, DEATH_SECONDS, Mob, MobKind},
    survival::{DamageSource, GameMode},
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, TAU},
};

const REACH: f32 = 3.6;

#[derive(Deserialize)]
struct PartDef {
    name: String,
    parent: Option<String>,
    pivot: [f32; 3],
    from: [f32; 3],
    size: [f32; 3],
    uv: (u32, u32),
}
#[derive(Deserialize)]
struct ModelDef {
    kind: String,
    texture: (u32, u32),
    parts: Vec<PartDef>,
}

/// Роль части тела в анимации; определяется по имени в `mob_models.ron`.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Role {
    Body,
    Head,
    /// Нога: знак фазы шага.
    Leg(f32),
    /// Рука гуля: сторона (+1 левая).
    Arm(f32),
    Tail,
    /// Крыло: сторона.
    Wing(f32),
    /// Лапа паука: сторона и номер от головы.
    SpiderLeg(f32, usize),
    Fixed,
}

fn role(name: &str) -> Role {
    match name {
        "body" => Role::Body,
        "head" => Role::Head,
        "tail" => Role::Tail,
        "leg_fl" | "leg_br" | "leg_l" => Role::Leg(1.0),
        "leg_fr" | "leg_bl" | "leg_r" => Role::Leg(-1.0),
        "arm_l" => Role::Arm(1.0),
        "arm_r" => Role::Arm(-1.0),
        "wing_l" => Role::Wing(1.0),
        "wing_r" => Role::Wing(-1.0),
        _ => {
            let spider = |prefix: &str, side: f32| {
                name.strip_prefix(prefix)
                    .and_then(|index| index.parse().ok())
                    .map(|index| Role::SpiderLeg(side, index))
            };
            spider("leg_l", 1.0)
                .or_else(|| spider("leg_r", -1.0))
                .unwrap_or(Role::Fixed)
        }
    }
}

struct PartModel {
    role: Role,
    parent: Option<usize>,
    /// Точка вращения в блоках относительно родителя (или ступней модели).
    pivot: Vec3,
    mesh: Handle<Mesh>,
}
struct MobModel {
    parts: Vec<PartModel>,
    material: Handle<StandardMaterial>,
    hurt: Handle<StandardMaterial>,
    scale: f32,
}

#[derive(Resource, Default)]
struct MobAssets(HashMap<MobKind, MobModel>);

struct MobVisual {
    root: Entity,
    parts: Vec<Entity>,
    /// Сглаженная амплитуда шага 0…1.
    gait: f32,
    hurt: bool,
}
#[derive(Resource, Default)]
struct MobVisuals(HashMap<u64, MobVisual>);

pub fn plugin(app: &mut App) {
    app.init_resource::<MobAssets>()
        .init_resource::<MobVisuals>()
        .add_systems(Startup, load_assets)
        .add_systems(
            Update,
            combat.after(crate::actions).before(crate::player::controls),
        )
        .add_systems(
            Update,
            (simulate, sync)
                .chain()
                .after(crate::survival_client::simulate)
                .before(crate::player::sky),
        );
}

macro_rules! skin {
    ($name:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/textures/mobs/",
            $name,
            ".png"
        )) as &'static [u8]
    };
}

fn skin_bytes(kind: MobKind) -> &'static [u8] {
    match kind {
        MobKind::Cow => skin!("cow"),
        MobKind::Pig => skin!("pig"),
        MobKind::Sheep => skin!("sheep"),
        MobKind::Chicken => skin!("chicken"),
        MobKind::Ghoul => skin!("ghoul"),
        MobKind::Spider => skin!("spider"),
    }
}

fn model_defs() -> Result<Vec<ModelDef>, String> {
    ron::from_str(include_str!("../../../assets/data/mob_models.ron")).map_err(|e| e.to_string())
}

fn load_assets(
    mut assets: ResMut<MobAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let definitions = match model_defs() {
        Ok(definitions) => definitions,
        Err(error) => {
            error!("Модели мобов недоступны: {error}");
            return;
        }
    };
    for kind in MobKind::ALL {
        let Some(definition) = definitions.iter().find(|d| d.kind == kind.spec().id) else {
            error!("Нет модели моба {}", kind.spec().id);
            continue;
        };
        let Ok(skin) =
            image::load_from_memory_with_format(skin_bytes(kind), image::ImageFormat::Png)
        else {
            error!("Не удалось прочитать текстуру моба {}", kind.spec().id);
            continue;
        };
        let skin = skin.into_rgba8();
        let mut image = Image::new(
            Extent3d {
                width: skin.width(),
                height: skin.height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            skin.into_raw(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );
        image.sampler = ImageSampler::nearest();
        let texture = images.add(image);
        let material = |tint: Color, materials: &mut Assets<StandardMaterial>| {
            materials.add(StandardMaterial {
                base_color: tint,
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 0.95,
                reflectance: 0.08,
                ..default()
            })
        };
        let parts = definition
            .parts
            .iter()
            .map(|part| {
                let parent = part
                    .parent
                    .as_ref()
                    .and_then(|name| definition.parts.iter().position(|p| &p.name == name));
                let pivot = Vec3::from_array(part.pivot)
                    - parent.map_or(Vec3::ZERO, |index| {
                        Vec3::from_array(definition.parts[index].pivot)
                    });
                PartModel {
                    role: role(&part.name),
                    parent,
                    pivot: pivot / 16.0,
                    mesh: meshes.add(part_mesh(part, definition.texture)),
                }
            })
            .collect();
        assets.0.insert(
            kind,
            MobModel {
                parts,
                material: material(Color::WHITE, &mut materials),
                hurt: material(Color::srgb(1.0, 0.38, 0.34), &mut materials),
                scale: match kind {
                    MobKind::Ghoul => 1.06,
                    MobKind::Spider => 1.15,
                    _ => 1.0,
                },
            },
        );
    }
}

/// Коробка части тела с развёрткой граней, как её рисует генератор текстур.
fn part_mesh(part: &PartDef, texture: (u32, u32)) -> Mesh {
    let min = Vec3::from_array(part.from) / 16.0;
    let max = min + Vec3::from_array(part.size) / 16.0;
    let [w, h, d] = part.size.map(|n| n.ceil());
    let (u, v) = (part.uv.0 as f32, part.uv.1 as f32);
    let (x0, y0, z0, x1, y1, z1) = (min.x, min.y, min.z, max.x, max.y, max.z);
    // (нормаль, прямоугольник текстуры, углы: слева-сверху, справа-сверху,
    // справа-снизу, слева-снизу для наблюдателя снаружи).
    let faces = [
        (
            [0., 1., 0.],
            [u + d, v, w, d],
            [[x0, y1, z0], [x1, y1, z0], [x1, y1, z1], [x0, y1, z1]],
        ),
        (
            [0., -1., 0.],
            [u + d + w, v, w, d],
            [[x0, y0, z1], [x1, y0, z1], [x1, y0, z0], [x0, y0, z0]],
        ),
        (
            [-1., 0., 0.],
            [u, v + d, d, h],
            [[x0, y1, z0], [x0, y1, z1], [x0, y0, z1], [x0, y0, z0]],
        ),
        (
            [0., 0., -1.],
            [u + d, v + d, w, h],
            [[x1, y1, z0], [x0, y1, z0], [x0, y0, z0], [x1, y0, z0]],
        ),
        (
            [1., 0., 0.],
            [u + d + w, v + d, d, h],
            [[x1, y1, z1], [x1, y1, z0], [x1, y0, z0], [x1, y0, z1]],
        ),
        (
            [0., 0., 1.],
            [u + 2. * d + w, v + d, w, h],
            [[x0, y1, z1], [x1, y1, z1], [x1, y0, z1], [x0, y0, z1]],
        ),
    ];
    let (mut positions, mut normals, mut uvs, mut indices) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let (tw, th) = (texture.0 as f32, texture.1 as f32);
    for (normal, [rx, ry, rw, rh], corners) in faces {
        let first = positions.len() as u32;
        positions.extend(corners);
        normals.extend([normal; 4]);
        // Небольшой отступ внутрь не даёт соседним граням просачиваться на швах.
        let inset = 0.02;
        let (left, right) = ((rx + inset) / tw, (rx + rw - inset) / tw);
        let (top, bottom) = ((ry + inset) / th, (ry + rh - inset) / th);
        uvs.extend([[left, top], [right, top], [right, bottom], [left, bottom]]);
        indices.extend([0, 3, 2, 0, 2, 1].map(|index| first + index));
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

/// ЛКМ по мобу — удар, ПКМ с едой в руке — приём пищи.
fn combat(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
) {
    if !mouse.pressed(MouseButton::Left) && game.attack_lock {
        game.attack_lock = false;
    }
    if game.mode != Mode::Playing
        || game.world_id.is_none()
        || game.vitals.is_dead()
        || game.pending_respawn
        || !game.world_mouse.allowed()
    {
        return;
    }
    if mouse.just_pressed(MouseButton::Left) {
        let eye = game.feet + Vec3::Y * (game.height - 0.15);
        let look = look_direction(&game);
        if let Some((id, distance)) = game.mobs.raycast(eye.to_array(), look.to_array(), REACH)
            && visible_hit(&game).is_none_or(|hit| distance <= hit.distance)
        {
            let damage = attack_damage(
                game.item_for_slot(game.selected)
                    .and_then(|stack| game.content.get(stack.item))
                    .map(|item| item.id.as_str()),
            );
            let from = game.feet.to_array();
            game.mobs.hit(id, damage, from);
            game.attack_lock = true;
            game.mining.reset();
            let (cfg, mode) = (game.survival_config.clone(), game.game_mode);
            game.vitals.exhaust(cfg.attack_exhaustion, mode, &cfg);
        }
    }
    if mouse.just_pressed(MouseButton::Right) && game.game_mode == GameMode::Survival {
        let sneaking = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
        let at_container = visible_hit(&game).is_some_and(|hit| {
            crate::inventory_client::container_size(game.world.block(hit.block)).is_some()
        });
        if !at_container || sneaking {
            eat(&mut game);
        }
    }
}

/// Съесть один предмет из выбранного слота, если игрок голоден.
fn eat(game: &mut Game) -> bool {
    let Some(definition) = game
        .item_for_slot(game.selected)
        .and_then(|stack| game.content.get(stack.item))
    else {
        return false;
    };
    if definition.kind != ItemKind::Food {
        return false;
    }
    let Some((hunger, saturation)) = food_value(&definition.id) else {
        return false;
    };
    let cfg = &game.survival_config;
    if game.vitals.hunger >= cfg.max_hunger {
        return false;
    }
    game.vitals.hunger = (game.vitals.hunger + hunger).min(cfg.max_hunger);
    game.vitals.saturation = (game.vitals.saturation + saturation)
        .min(cfg.max_saturation)
        .min(game.vitals.hunger);
    game.inventory.take_selected(game.selected, false);
    sync_hotbar(game);
    game.ui_dirty = true;
    true
}

fn daylight(clock: f64) -> f32 {
    ((clock as f32 / 1200.0) * TAU).sin().clamp(0.0, 1.0)
}

fn simulate(
    time: Res<Time<Real>>,
    mut game: ResMut<Game>,
    mut actions: MessageWriter<PlayerActionMessage>,
) {
    if game.world_id.is_none() || game.mode != Mode::Playing {
        return;
    }
    let game = &mut *game;
    let targetable =
        game.game_mode == GameMode::Survival && !game.vitals.is_dead() && !game.pending_respawn;
    let world = &game.world;
    let events = game.mobs.tick(
        time.delta_secs(),
        game.feet.to_array(),
        targetable,
        daylight(game.clock),
        |position| loaded_block(world, position),
    );
    if events.player_damage > 0.0 && targetable {
        actions.write(PlayerActionMessage(PlayerAction::Damage {
            amount: events.player_damage,
            source: DamageSource::Mob,
        }));
        game.velocity += Vec3::from_array(events.knockback);
        game.grounded = false;
    }
    for (item, count, position) in events.drops {
        if let Some(item) = game.content.find(item) {
            spawn_drop(
                game,
                ItemStack::new(item, count),
                Vec3::from_array(position),
                Vec3::Y * 2.5,
            );
        }
    }
}

fn smoothstep(low: f32, high: f32, value: f32) -> f32 {
    let t = ((value - low) / (high - low)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Радианы фазы шага на один пройденный блок.
fn stride(kind: MobKind) -> f32 {
    match kind {
        MobKind::Cow => 4.4,
        MobKind::Pig => 6.0,
        MobKind::Sheep => 5.2,
        MobKind::Chicken => 13.0,
        MobKind::Ghoul => 4.0,
        MobKind::Spider => 8.5,
    }
}

/// Поворот и смещение части тела относительно позы покоя.
fn pose(mob: &Mob, role: Role, gait: f32) -> (Quat, Vec3) {
    let kind = mob.kind;
    let t = mob.age;
    let seed = mob.id as f32 * 1.7;
    let phase = mob.walk * stride(kind);
    let step = phase.sin() * gait;
    let idle = 1.0 - gait.min(1.0);
    // Иногда стоящее животное опускает голову: щиплет траву, роет, клюёт.
    let graze = if matches!(mob.behavior, Behavior::Idle) && !kind.spec().hostile {
        smoothstep(0.72, 0.9, (t * 0.23 + seed).sin()) * idle
    } else {
        0.0
    };
    match (kind, role) {
        (_, Role::Fixed) => (Quat::IDENTITY, Vec3::ZERO),
        (MobKind::Spider, Role::SpiderLeg(side, index)) => {
            let fan = [0.62, 0.22, -0.2, -0.6][index.min(3)];
            let group = if (index % 2 == 0) == (side > 0.0) {
                1.0
            } else {
                -1.0
            };
            let swing = phase.sin() * group * 0.38 * gait;
            let lift = (phase.cos() * group).max(0.0) * 0.4 * gait;
            let twitch = (t * 1.7 + seed + index as f32).sin() * 0.03;
            (
                Quat::from_rotation_y(side * (fan + swing))
                    * Quat::from_rotation_z(-side * (0.72 - lift + twitch)),
                Vec3::ZERO,
            )
        }
        (MobKind::Spider, Role::Body) => (
            Quat::from_rotation_x((t * 2.0 + seed).sin() * 0.03),
            Vec3::Y * ((phase * 2.0).sin() * 0.015 * gait),
        ),
        (MobKind::Spider, Role::Head) => (
            Quat::from_rotation_y(mob.head_yaw * 0.4)
                * Quat::from_rotation_x(mob.head_pitch * 0.4 - mob.swing * 1.2),
            Vec3::ZERO,
        ),
        (MobKind::Ghoul, Role::Body) => (
            Quat::from_rotation_x(-0.16 - mob.swing * 0.5 + (t * 1.1 + seed).sin() * 0.02)
                * Quat::from_rotation_z(step * 0.06),
            Vec3::ZERO,
        ),
        (MobKind::Ghoul, Role::Head) => (
            Quat::from_rotation_y(mob.head_yaw)
                * Quat::from_rotation_x(mob.head_pitch + 0.16)
                * Quat::from_rotation_z((t * 0.7 + seed).sin() * 0.09),
            Vec3::ZERO,
        ),
        (MobKind::Ghoul, Role::Arm(side)) => {
            let reach = if mob.behavior == Behavior::Chase {
                1.4
            } else {
                0.25
            };
            (
                Quat::from_rotation_x(
                    reach + (t * 2.1 + seed + side).sin() * 0.07 - step * side * 0.2
                        + mob.swing * 2.2,
                ) * Quat::from_rotation_z(side * 0.06),
                Vec3::ZERO,
            )
        }
        (MobKind::Chicken, Role::Wing(side)) => {
            let flap = if mob.grounded {
                0.04 + gait * 0.08 + mob.hurt * 2.0
            } else {
                0.9 + (t * 26.0).sin() * 0.7
            };
            (Quat::from_rotation_z(side * flap), Vec3::ZERO)
        }
        (MobKind::Chicken, Role::Head) => {
            let peck = graze * (0.75 + 0.35 * (t * 10.0).sin());
            (
                Quat::from_rotation_y(mob.head_yaw) * Quat::from_rotation_x(mob.head_pitch - peck),
                // Куры при ходьбе покачивают головой вперёд-назад.
                Vec3::NEG_Z * ((phase * 2.0).sin() * 0.03 * gait),
            )
        }
        (MobKind::Chicken, Role::Tail) => (
            Quat::from_rotation_x(-0.5 + (t * 3.0 + seed).sin() * 0.08),
            Vec3::ZERO,
        ),
        (_, Role::Head) => {
            let depth = if kind == MobKind::Pig { 0.55 } else { 0.95 };
            (
                Quat::from_rotation_y(mob.head_yaw * (1.0 - graze))
                    * Quat::from_rotation_x(
                        mob.head_pitch * (1.0 - graze) - graze * depth
                            + (t * 1.3 + seed).sin() * 0.025
                            + (phase * 2.0).sin() * 0.03 * gait,
                    ),
                Vec3::ZERO,
            )
        }
        (_, Role::Leg(sign)) => (Quat::from_rotation_x(step * sign * 0.75), Vec3::ZERO),
        (_, Role::Tail) => {
            let wag = if kind == MobKind::Pig {
                (9.0, 0.45)
            } else {
                (2.6, 0.28)
            };
            (
                Quat::from_rotation_x(-0.3 * gait - 0.08)
                    * Quat::from_rotation_z((t * wag.0 + seed).sin() * wag.1),
                Vec3::ZERO,
            )
        }
        (_, Role::Body) => (
            // Дыхание в покое и лёгкая раскачка на ходу.
            Quat::from_rotation_z(step * 0.025),
            Vec3::Y * ((t * 1.9 + seed).sin() * 0.006 * idle),
        ),
        (_, Role::Arm(_) | Role::Wing(_) | Role::SpiderLeg(..)) => (Quat::IDENTITY, Vec3::ZERO),
    }
}

fn root_transform(mob: &Mob, scale: f32, gait: f32) -> Transform {
    let variation = 0.94 + (mob.id % 7) as f32 * 0.02;
    let phase = mob.walk * stride(mob.kind);
    let bob = (phase * 2.0).sin().abs() * 0.025 * gait;
    let mut transform = Transform::from_translation(Vec3::from_array(mob.position) + Vec3::Y * bob)
        .with_rotation(Quat::from_rotation_y(mob.yaw))
        .with_scale(Vec3::splat(scale * variation));
    if let Some(time) = mob.dying {
        // Моб заваливается набок, затем уходит в землю и сжимается.
        let f = (time / DEATH_SECONDS).clamp(0.0, 1.0);
        let fall = smoothstep(0.0, 0.45, f);
        transform.rotation *= Quat::from_rotation_z(fall * FRAC_PI_2);
        transform.translation.y += fall * mob.kind.spec().half_width * 0.6;
        let fade = smoothstep(0.65, 1.0, f);
        transform.translation.y -= fade * 0.5;
        transform.scale *= 1.0 - fade * 0.85;
    } else if mob.hurt > 0.0 {
        // Короткое вздрагивание от удара.
        transform.scale *= 1.0 + mob.hurt * 0.12;
    }
    transform
}

fn sync(
    mut commands: Commands,
    time: Res<Time<Real>>,
    game: Res<Game>,
    assets: Res<MobAssets>,
    mut visuals: ResMut<MobVisuals>,
    mut transforms: Query<&mut Transform>,
) {
    visuals.0.retain(|id, visual| {
        let alive = game.world_id.is_some() && game.mobs.mobs.contains_key(id);
        if !alive {
            commands.entity(visual.root).despawn();
        }
        alive
    });
    if game.world_id.is_none() {
        return;
    }
    let dt = time.delta_secs().min(0.1);
    for mob in game.mobs.mobs.values() {
        let Some(model) = assets.0.get(&mob.kind) else {
            continue;
        };
        let visual = visuals.0.entry(mob.id).or_insert_with(|| {
            let root = commands
                .spawn((root_transform(mob, model.scale, 0.0), Visibility::default()))
                .id();
            let parts: Vec<Entity> = model
                .parts
                .iter()
                .map(|part| {
                    commands
                        .spawn((
                            Mesh3d(part.mesh.clone()),
                            MeshMaterial3d(model.material.clone()),
                            Transform::from_translation(part.pivot),
                        ))
                        .id()
                })
                .collect();
            for (index, part) in model.parts.iter().enumerate() {
                let parent = part.parent.map_or(root, |parent| parts[parent]);
                commands.entity(parent).add_child(parts[index]);
            }
            MobVisual {
                root,
                parts,
                gait: 0.0,
                hurt: false,
            }
        });
        let target = if mob.dying.is_some() {
            0.0
        } else {
            (mob.speed / mob.kind.spec().speed).clamp(0.0, 1.2)
        };
        visual.gait += (target - visual.gait) * (dt * 9.0).min(1.0);
        let hurt = mob.hurt > 0.05 || mob.dying.is_some();
        if hurt != visual.hurt {
            visual.hurt = hurt;
            let material = if hurt { &model.hurt } else { &model.material };
            for part in &visual.parts {
                commands
                    .entity(*part)
                    .insert(MeshMaterial3d(material.clone()));
            }
        }
        // Только что созданные сущности появятся в запросе со следующего кадра.
        if let Ok(mut transform) = transforms.get_mut(visual.root) {
            *transform = root_transform(mob, model.scale, visual.gait);
        }
        for (part, entity) in model.parts.iter().zip(&visual.parts) {
            if let Ok(mut transform) = transforms.get_mut(*entity) {
                let (rotation, offset) = pose(mob, part.role, visual.gait);
                transform.rotation = rotation;
                transform.translation = part.pivot + offset;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_core::mobs::MobStore;

    #[test]
    fn every_mob_has_a_model_whose_boxes_fit_its_skin() {
        let definitions = model_defs().unwrap();
        for kind in MobKind::ALL {
            let model = definitions
                .iter()
                .find(|d| d.kind == kind.spec().id)
                .unwrap_or_else(|| panic!("no model for {}", kind.spec().id));
            let skin = image::load_from_memory(skin_bytes(kind)).unwrap();
            assert_eq!((skin.width(), skin.height()), model.texture);
            assert!(model.parts.iter().any(|p| role(&p.name) == Role::Head));
            assert!(model.parts.iter().any(|p| role(&p.name) == Role::Body));
            let mut lowest = f32::MAX;
            for part in &model.parts {
                let [w, h, d] = part.size.map(|n| n.ceil() as u32);
                assert!(part.uv.0 + 2 * (w + d) <= model.texture.0, "{}", part.name);
                assert!(part.uv.1 + d + h <= model.texture.1, "{}", part.name);
                if let Some(parent) = &part.parent {
                    assert!(model.parts.iter().any(|p| &p.name == parent));
                }
                lowest = lowest.min(part.pivot[1] + part.from[1]);
                let mesh = part_mesh(part, model.texture);
                assert_eq!(mesh.count_vertices(), 24);
            }
            // Паук висит на согнутых лапах, остальные стоят ступнями на y = 0.
            let allowed = if kind == MobKind::Spider { 6.0 } else { 0.01 };
            assert!(
                (0.0..=allowed).contains(&lowest.max(0.0)) && lowest > -0.01,
                "{} must stand on y = 0",
                model.kind
            );
        }
    }

    #[test]
    fn walking_swings_opposite_legs_and_death_tips_the_body_over() {
        let mut store = MobStore::new(4);
        let id = store.spawn(MobKind::Cow, [0.5, 64.0, 0.5]);
        let mob = store.mobs.get_mut(&id).unwrap();
        mob.walk = 0.3;
        let (left, _) = pose(mob, Role::Leg(1.0), 1.0);
        let (right, _) = pose(mob, Role::Leg(-1.0), 1.0);
        assert!(left.angle_between(right) > 0.5);
        assert!(left.angle_between(right.inverse()) < 1e-4);
        let (still, _) = pose(mob, Role::Leg(1.0), 0.0);
        assert!(still.angle_between(Quat::IDENTITY) < 1e-4);
        let upright = root_transform(mob, 1.0, 0.0);
        mob.dying = Some(DEATH_SECONDS * 0.5);
        let fallen = root_transform(mob, 1.0, 0.0);
        assert!(upright.rotation.angle_between(fallen.rotation) > 1.5);
        mob.dying = Some(DEATH_SECONDS);
        assert!(root_transform(mob, 1.0, 0.0).scale.x < 0.2);
    }

    #[test]
    fn eating_restores_hunger_once_and_only_when_hungry() {
        let mut game = Game::default();
        let steak = game.content.find("builder:cooked_beef").unwrap();
        game.inventory.hotbar[0] = Some(ItemStack::new(steak, 2));
        game.selected = 0;
        assert!(!eat(&mut game), "сытый игрок не ест");
        game.vitals.hunger = 5.0;
        game.vitals.saturation = 0.0;
        assert!(eat(&mut game));
        assert_eq!(game.vitals.hunger, 13.0);
        assert!(game.vitals.saturation > 0.0 && game.vitals.saturation <= 13.0);
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 1);
        let dirt = game.content.find("foundation:dirt").unwrap();
        game.inventory.hotbar[0] = Some(ItemStack::new(dirt, 1));
        assert!(!eat(&mut game));
    }
}
