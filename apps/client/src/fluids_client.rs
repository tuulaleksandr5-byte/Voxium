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
    clock.growth = (clock.growth + delta).min(game_core::growth::TICK_SECONDS * 2.0);
    while clock.growth >= game_core::growth::TICK_SECONDS {
        clock.growth -= game_core::growth::TICK_SECONDS;
        clock.growth_tick += 1;
        let tick = clock.growth_tick;
        game_core::growth::tick(&mut game.world, tick, 64);
    }
}

/// Кнопки, нажимные плиты, взрывы динамита и опасные блоки.
fn tick_mechanics(
    time: Res<Time<Real>>,
    mut game: ResMut<Game>,
    mut actions: MessageWriter<crate::survival_client::PlayerActionMessage>,
    mut contact: Local<f32>,
) {
    use crate::survival_client::{PlayerAction, PlayerActionMessage};
    if game.world_id.is_none() || !matches!(game.mode, Mode::Playing | Mode::Inventory) {
        return;
    }
    let delta = time.delta_secs();
    // Кнопки отпускаются через секунду.
    let mut released = Vec::new();
    game.power_timers.retain_mut(|(position, left)| {
        *left -= delta;
        if *left <= 0.0 {
            released.push(*position);
            false
        } else {
            true
        }
    });
    for position in released {
        game_core::mechanics::power_area(&mut game.world, position, false);
        game.revision += 1;
    }
    // Нажимная плита работает, пока игрок стоит на ней.
    let cell = [
        game.feet.x.floor() as i32,
        game.feet.y.floor() as i32,
        game.feet.z.floor() as i32,
    ];
    let on_plate = (game_core::models::shape_of(game.world.block(cell))
        == Some(game_core::models::Shape::Plate))
    .then_some(cell);
    if on_plate != game.pressed_plate {
        if let Some(old) = game.pressed_plate {
            game_core::mechanics::power_area(&mut game.world, old, false);
        }
        if let Some(new) = on_plate {
            game_core::mechanics::power_area(&mut game.world, new, true);
        }
        game.pressed_plate = on_plate;
        game.revision += 1;
    }
    // Взрывы.
    let centers = game_core::mechanics::run_blasts(&mut game.world, 8);
    if !centers.is_empty() {
        game.revision += 1;
    }
    for center in centers {
        let eye = game.feet + Vec3::Y * 0.9;
        let distance = eye.distance(Vec3::new(
            center[0] as f32 + 0.5,
            center[1] as f32 + 0.5,
            center[2] as f32 + 0.5,
        ));
        let amount = game_core::mechanics::blast_damage(distance);
        if amount > 0.0 {
            actions.write(PlayerActionMessage(PlayerAction::Damage {
                amount,
                source: game_core::survival::DamageSource::Fire,
            }));
        }
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
