//! Жидкости на клиенте: шаг симуляции, цветная пелена перед глазами и вёдра.
use crate::session::{Game, Mode};
use bevy::prelude::*;
use game_core::fluids::{self, TICK_SECONDS};

/// Сколько клеток жидкости обрабатывается за один шаг.
const BUDGET: usize = 768;

#[derive(Resource, Default)]
struct FluidClock {
    accumulated: f32,
    tick: u64,
    fire: f32,
    fire_tick: u64,
    growth: f32,
    growth_tick: u64,
    nature: f32,
    nature_tick: u64,
}

#[derive(Component)]
struct FluidOverlay;

pub fn plugin(app: &mut App) {
    app.init_resource::<FluidClock>()
        .add_systems(Startup, spawn_overlay)
        .add_systems(
            Update,
            (
                tick_fluids,
                tick_fire_and_growth,
                tick_mechanics,
                update_overlay,
            )
                .chain()
                .after(crate::survival_client::simulate)
                .before(crate::player::sky),
        );
}

fn tick_fluids(time: Res<Time<Real>>, mut game: ResMut<Game>, mut clock: ResMut<FluidClock>) {
    if game.world_id.is_none() || !matches!(game.mode, Mode::Playing | Mode::Inventory) {
        return;
    }
    clock.accumulated = (clock.accumulated + time.delta_secs()).min(TICK_SECONDS * 4.0);
    while clock.accumulated >= TICK_SECONDS {
        clock.accumulated -= TICK_SECONDS;
        clock.tick += 1;
        let tick = clock.tick;
        fluids::tick(&mut game.world, tick, BUDGET);
    }
}

/// Огонь (0,5 с) и рост саженцев (5 с) идут на тех же часах, что и жидкости.
fn tick_fire_and_growth(
    time: Res<Time<Real>>,
    mut game: ResMut<Game>,
    mut clock: ResMut<FluidClock>,
) {
    if game.world_id.is_none() || !matches!(game.mode, Mode::Playing | Mode::Inventory) {
        return;
    }
    let delta = time.delta_secs();
    clock.fire = (clock.fire + delta).min(game_core::fire::TICK_SECONDS * 4.0);
    while clock.fire >= game_core::fire::TICK_SECONDS {
        clock.fire -= game_core::fire::TICK_SECONDS;
        clock.fire_tick += 1;
        let tick = clock.fire_tick;
        game_core::fire::tick(&mut game.world, tick, 256);
    }
    clock.nature = (clock.nature + delta).min(game_core::nature::TICK_SECONDS * 4.0);
    while clock.nature >= game_core::nature::TICK_SECONDS {
        clock.nature -= game_core::nature::TICK_SECONDS;
        clock.nature_tick += 1;
        let tick = clock.nature_tick;
        game_core::nature::tick(&mut game.world, tick, 512);
        let center = game.feet.to_array().map(|v| v.floor() as i32);
        game_core::nature::random_tick(
            &mut game.world,
            center,
            tick,
            game_core::nature::RANDOM_SAMPLES_PER_TICK,
        );
    }
    clock.growth = (clock.growth + delta).min(game_core::growth::TICK_SECONDS * 2.0);
    while clock.growth >= game_core::growth::TICK_SECONDS {
        clock.growth -= game_core::growth::TICK_SECONDS;
        clock.growth_tick += 1;
        let tick = clock.growth_tick;
        game_core::growth::tick(&mut game.world, tick, 64);
    }
}

/// Кнопки, нажимные плиты, взрывы динамита и опасные блоки.
#[allow(clippy::too_many_arguments)]
fn tick_mechanics(
    time: Res<Time<Real>>,
    mut game: ResMut<Game>,
    mut actions: MessageWriter<crate::survival_client::PlayerActionMessage>,
    mut explosions: MessageWriter<crate::world_fx::ExplosionMessage>,
    mut sounds: MessageWriter<crate::block_audio::BlockSound>,
    mut effects: MessageWriter<crate::block_audio::EffectSound>,
    mut contact: Local<f32>,
    mut redstone_clock: Local<f32>,
) {
    use crate::survival_client::{PlayerAction, PlayerActionMessage};
    if game.world_id.is_none() || !matches!(game.mode, Mode::Playing | Mode::Inventory) {
        return;
    }
    let delta = time.delta_secs();
    // Нажимная плита работает, пока игрок стоит на ней.
    let cell = [
        game.feet.x.floor() as i32,
        game.feet.y.floor() as i32,
        game.feet.z.floor() as i32,
    ];
    let on_plate = (game_core::models::shape_of(game.world.block(cell))
        == Some(game_core::models::Shape::Plate))
    .then_some(cell);
    let expected: std::collections::HashSet<[i32; 3]> = on_plate.into_iter().collect();
    if game.world.rs.pressed != expected {
        game.world.rs.pressed = expected;
    }
    // Редстоун считается шагами по 0,1 с.
    *redstone_clock += delta;
    let mut steps = 0;
    while *redstone_clock >= game_core::redstone::TICK_SECONDS && steps < 4 {
        *redstone_clock -= game_core::redstone::TICK_SECONDS;
        steps += 1;
        game.world.rs.day = !crate::inventory_client::is_night(game.clock);
        game_core::redstone::tick(&mut game.world);
        game.revision += 1;
    }
    *redstone_clock = redstone_clock.min(game_core::redstone::TICK_SECONDS);
    let events = std::mem::take(&mut game.world.rs.events);
    for event in events {
        match event {
            game_core::redstone::Event::Dispense {
                position,
                dir,
                dropper,
            } => crate::inventory_client::dispense(&mut game, position, dir, dropper),
            game_core::redstone::Event::Note { .. } => {
                game.status = "♪ Нотный блок".into();
                game.ui_dirty = true;
            }
        }
    }
    // Взрывы: динамит сначала загорается, потом горит и только потом взрывается.
    game_core::mechanics::run_blasts(&mut game.world, 64);
    let centers = game_core::mechanics::step_primed(&mut game.world, delta);
    if !centers.is_empty() {
        game.revision += 1;
    }
    for center in centers {
        explosions.write(crate::world_fx::ExplosionMessage(Vec3::from_array(center)));
        let eye = game.feet + Vec3::Y * 0.9;
        let distance = eye.distance(Vec3::from_array(center));
        let amount = game_core::mechanics::blast_damage(distance);
        if amount > 0.0 {
            actions.write(PlayerActionMessage(PlayerAction::Damage {
                amount,
                source: game_core::survival::DamageSource::Fire,
            }));
        }
    }
    // Падающие блоки: наковальня бьёт того, кто оказался под ней.
    strike_with_anvils(&mut game, &mut actions);
    if game_core::nature::step_falling(&mut game.world, delta.min(0.1)) > 0 {
        game.revision += 1;
    }
    for event in std::mem::take(&mut game.world.nature_events) {
        match event {
            game_core::nature::NatureEvent::Broke { position, block } => {
                crate::inventory_client::natural_drops(&mut game, position, block);
            }
            game_core::nature::NatureEvent::Landed {
                position,
                block,
                fallen,
            } => {
                let at = Vec3::new(
                    position[0] as f32 + 0.5,
                    position[1] as f32 + 0.5,
                    position[2] as f32 + 0.5,
                );
                if game_core::nature::is_anvil(block) {
                    effects.write(crate::block_audio::EffectSound {
                        kind: crate::block_audio::EffectKind::Anvil,
                        position: at,
                    });
                } else if fallen > 0.5 {
                    let material = match block {
                        game_core::GRAVEL => game_core::GRAVEL,
                        game_core::DRAGON_EGG => game_core::STONE,
                        _ => game_core::SAND,
                    };
                    sounds.write(crate::block_audio::BlockSound {
                        block: material,
                        kind: crate::block_audio::SoundKind::Place,
                        position: at,
                    });
                }
            }
        }
        game.revision += 1;
    }
    // Кактус, магма и колючий куст ранят каждые полсекунды.
    *contact += delta;
    if *contact >= 0.5 {
        *contact = 0.0;
        let worst = std::cell::Cell::new(0.0_f32);
        let _ = game_core::interaction::body_touches(
            game.feet.to_array(),
            game.height,
            |p| game.world.block(p),
            |id| {
                if let Some(damage) = game_core::mechanics::contact_damage(id) {
                    worst.set(worst.get().max(damage));
                }
                false
            },
        );
        if worst.get() > 0.0 {
            actions.write(PlayerActionMessage(PlayerAction::Damage {
                amount: worst.get(),
                source: game_core::survival::DamageSource::Contact,
            }));
        }
    }
}

fn spawn_overlay(mut commands: Commands) {
    commands.spawn((
        FluidOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        BackgroundColor(Color::NONE),
        GlobalZIndex(-10),
        Pickable::IGNORE,
    ));
}

/// Цвет пелены для блока на уровне глаз.
fn overlay_color(block: game_core::BlockId) -> Color {
    if game_core::is_lava(block) {
        Color::srgba(1.0, 0.35, 0.05, 0.78)
    } else if game_core::is_water(block) {
        Color::srgba(0.1, 0.3, 0.75, 0.4)
    } else {
        Color::NONE
    }
}

fn update_overlay(game: Res<Game>, mut overlay: Single<&mut BackgroundColor, With<FluidOverlay>>) {
    let color = if game.world_id.is_some() {
        let eye = game.feet + Vec3::Y * (game.height - 0.15);
        overlay_color(game.world.block(eye.floor().as_ivec3().to_array()))
    } else {
        Color::NONE
    };
    if overlay.0 != color {
        overlay.0 = color;
    }
}

/// Наковальня на лету бьёт игрока и мобов под собой: 2 урона за блок падения, до 40.
fn strike_with_anvils(
    game: &mut Game,
    actions: &mut MessageWriter<crate::survival_client::PlayerActionMessage>,
) {
    use crate::survival_client::{PlayerAction, PlayerActionMessage};
    let feet = game.feet;
    let (half, height) = (0.3_f32, game.height);
    let mut struck_mobs = Vec::new();
    let mut player_damage = 0.0_f32;
    for anvil in &mut game.world.falling {
        if !game_core::nature::is_anvil(anvil.block) || anvil.struck || anvil.fallen < 1.0 {
            continue;
        }
        let damage = (2.0 * (anvil.fallen - 1.0)).min(40.0);
        let c = Vec3::from_array(anvil.position);
        let overlaps = |min: Vec3, max: Vec3| {
            c.x + 0.5 > min.x
                && c.x - 0.5 < max.x
                && c.y + 0.5 > min.y
                && c.y - 0.5 < max.y
                && c.z + 0.5 > min.z
                && c.z - 0.5 < max.z
        };
        let mut hit = false;
        if overlaps(
            feet - Vec3::new(half, 0.0, half),
            feet + Vec3::new(half, height, half),
        ) {
            player_damage = player_damage.max(damage);
            hit = true;
        }
        for mob in game.mobs.mobs.values() {
            let spec = mob.kind.spec();
            let at = Vec3::from_array(mob.position);
            if mob.dying.is_none()
                && overlaps(
                    at - Vec3::new(spec.half_width, 0.0, spec.half_width),
                    at + Vec3::new(spec.half_width, spec.height, spec.half_width),
                )
            {
                struck_mobs.push((mob.id, damage, c));
                hit = true;
            }
        }
        anvil.struck = hit;
    }
    if player_damage > 0.0 && game.game_mode == game_core::survival::GameMode::Survival {
        actions.write(PlayerActionMessage(PlayerAction::Damage {
            amount: player_damage,
            source: game_core::survival::DamageSource::Contact,
        }));
    }
    for (id, damage, from) in struck_mobs {
        game.mobs.hit(id, damage, from.to_array());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_tints_only_when_the_eye_is_inside_a_fluid() {
        assert_eq!(overlay_color(game_core::STONE), Color::NONE);
        assert_eq!(overlay_color(game_core::AIR), Color::NONE);
        assert_ne!(overlay_color(game_core::WATER), Color::NONE);
        assert_ne!(overlay_color(game_core::FLOW_WATER_3), Color::NONE);
        let lava = overlay_color(game_core::LAVA).to_srgba();
        let water = overlay_color(game_core::WATER).to_srgba();
        assert!(lava.red > lava.blue && water.blue > water.red);
        assert!(lava.alpha > water.alpha, "лава непрозрачнее воды");
    }

    #[test]
    fn the_clock_runs_a_bounded_number_of_steps_per_frame() {
        let mut game = Game {
            mode: Mode::Playing,
            world_id: Some("fluids".into()),
            ..Default::default()
        };
        for cx in -1..=1 {
            for cz in -1..=1 {
                game.world.chunks.insert(
                    [cx, cz],
                    std::sync::Arc::new(game_core::Chunk {
                        position: [cx, cz],
                        sections: Default::default(),
                    }),
                );
            }
        }
        game.world.set([0, 60, 0], game_core::STONE);
        game.world.set([0, 61, 0], game_core::WATER);
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(game)
            .init_resource::<FluidClock>()
            .insert_resource(Time::<Real>::default())
            .add_systems(Update, tick_fluids);
        // Огромный кадр не вызывает лавину шагов: накопление ограничено четырьмя тиками.
        app.world_mut()
            .resource_mut::<Time<Real>>()
            .advance_by(std::time::Duration::from_secs(30));
        app.update();
        let clock = app.world().resource::<FluidClock>();
        assert!(clock.tick <= 4, "{}", clock.tick);
        assert!(clock.tick >= 1);
        let game = app.world().resource::<Game>();
        assert_eq!(game.world.block([1, 61, 0]), game_core::FLOW_WATER_7);
    }
}
