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
}

#[derive(Component)]
struct FluidOverlay;

pub fn plugin(app: &mut App) {
    app.init_resource::<FluidClock>()
        .add_systems(Startup, spawn_overlay)
        .add_systems(
            Update,
            (tick_fluids, update_overlay)
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
fn overlay_color(block: u8) -> Color {
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
