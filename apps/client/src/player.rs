use crate::{
    session::{Game, Mode},
    ui::{Action, UiIntent},
};
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use game_core::{
    BEDROCK, World,
    interaction::{can_stand, move_player, player_intersects_block, raycast},
    valid_world_height,
};
#[derive(Component)]
pub struct PlayerCamera;
#[derive(Component)]
pub struct Sun;

// Bevy injects each input/resource independently into this system.
#[allow(clippy::too_many_arguments)]
pub fn controls(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    mut game: ResMut<Game>,
    mut camera: Single<&mut Transform, With<PlayerCamera>>,
    mut window: Single<(&Window, &mut CursorOptions)>,
    mut intents: ResMut<UiIntent>,
    mut gizmos: Gizmos,
) {
    let (window, cursor) = &mut *window;
    if game.mode == Mode::Playing && !window.focused {
        game.screen(Mode::Paused);
    }
    if keys.just_pressed(KeyCode::Escape) {
        match game.mode {
            Mode::Playing => game.screen(Mode::Paused),
            Mode::Paused | Mode::Inventory => game.screen(Mode::Playing),
            Mode::Menu => {}
            _ => intents.0.push(Action::Back),
        }
    }
    if keys.just_pressed(KeyCode::KeyE) {
        match game.mode {
            Mode::Playing => game.screen(Mode::Inventory),
            Mode::Inventory => game.screen(Mode::Playing),
            _ => {}
        }
    }
    let captured = game.mode == Mode::Playing && window.focused;
    cursor.visible = !captured;
    cursor.grab_mode = if captured {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
    if game.world_id.is_none() {
        return;
    }
    if keys.just_pressed(KeyCode::F5) {
        intents.0.push(Action::Save);
    }
    if captured {
        game.yaw -= motion.delta.x * game.settings.sensitivity;
        game.pitch = (game.pitch - motion.delta.y * game.settings.sensitivity).clamp(-1.54, 1.54);
        let slot_keys = [
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
        ];
        for (i, key) in slot_keys.into_iter().enumerate() {
            if keys.just_pressed(key) {
                game.selected = i;
                game.ui_dirty = true;
            }
        }
        let dt = time.delta_secs().min(0.1);
        let rotation = Quat::from_rotation_y(game.yaw);
        let mut movement = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            movement.z -= 1.;
        }
        if keys.pressed(KeyCode::KeyS) {
            movement.z += 1.;
        }
        if keys.pressed(KeyCode::KeyA) {
            movement.x -= 1.;
        }
        if keys.pressed(KeyCode::KeyD) {
            movement.x += 1.;
        }
        let crouch = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
        if crouch {
            game.height = 1.3;
        } else if game.height < 1.8 {
            let loaded = game.world.chunks.contains_key(&[
                (game.feet.x.floor() as i32).div_euclid(16),
                (game.feet.z.floor() as i32).div_euclid(16),
            ]);
            if loaded && can_stand(game.feet.to_array(), |p| collision_block(&game.world, p)) {
                game.height = 1.8;
            }
        }
        let speed = if game.height < 1.8 {
            2.2
        } else if keys.pressed(KeyCode::ShiftLeft) {
            8.0
        } else {
            4.8
        };
        let horizontal = rotation * movement.normalize_or_zero() * speed;
        game.velocity.x = horizontal.x;
        game.velocity.z = horizontal.z;
        let chunk = [
            (game.feet.x.floor() as i32).div_euclid(16),
            (game.feet.z.floor() as i32).div_euclid(16),
        ];
        if game.world.chunks.contains_key(&chunk) {
            if keys.just_pressed(KeyCode::Space) && game.grounded {
                game.velocity.y = 8.0;
            }
            game.velocity.y = (game.velocity.y - 24.0 * dt).max(-45.0);
            let result = move_player(
                game.feet.to_array(),
                game.velocity.to_array(),
                dt,
                game.height,
                |p| collision_block(&game.world, p),
            );
            game.feet = Vec3::from_array(result.position);
            game.velocity = Vec3::from_array(result.velocity);
            game.grounded = result.grounded;
        } else {
            game.velocity = Vec3::ZERO;
        }
        game.clock += time.delta_secs_f64();
        let eye = game.feet + Vec3::Y * (game.height - 0.15);
        let look = rotation * Quat::from_rotation_x(game.pitch);
        if let Some(hit) = raycast(eye.to_array(), (look * Vec3::NEG_Z).to_array(), 6.0, |p| {
            game.world.block(p)
        }) {
            gizmos.cube(
                Transform::from_translation(
                    Vec3::from_array(hit.block.map(|v| v as f32)) + Vec3::splat(0.5),
                )
                .with_scale(Vec3::splat(1.006)),
                Color::srgb(1.0, 0.85, 0.25),
            );
            let selected = game.hotbar[game.selected];
            if mouse.just_pressed(MouseButton::Left) && game.world.set(hit.block, 0) {
                game.revision += 1;
            }
            if mouse.just_pressed(MouseButton::Right)
                && !player_intersects_block(game.feet.to_array(), game.height, hit.previous)
                && game.world.set(hit.previous, selected)
            {
                game.revision += 1;
            }
        }
    }
    camera.translation = game.feet + Vec3::Y * (game.height - 0.15);
    camera.rotation = Quat::from_rotation_y(game.yaw) * Quat::from_rotation_x(game.pitch);
}

pub fn sky(
    game: Res<Game>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut sun: Single<(&mut DirectionalLight, &mut Transform), With<Sun>>,
    mut camera: Single<&mut DistanceFog, With<PlayerCamera>>,
) {
    let angle = (game.clock as f32 / 1200.0) * std::f32::consts::TAU;
    let daylight = angle.sin().clamp(0.0, 1.0);
    let color = Color::srgb(
        0.025 + 0.46 * daylight,
        0.035 + 0.65 * daylight,
        0.075 + 0.76 * daylight,
    );
    clear.0 = if game.world_id.is_some() {
        color
    } else {
        Color::srgb(0.035, 0.05, 0.07)
    };
    sun.0.illuminance = 100.0 + 12000.0 * daylight;
    sun.1.rotation = Quat::from_euler(EulerRot::XYZ, -angle, 0.45, 0.0);
    ambient.brightness = 80.0 + daylight * 220.0;
    camera.color = color;
    let end = (game.settings.distance * 16) as f32;
    camera.falloff = FogFalloff::Linear {
        start: end * 0.55,
        end,
    };
}

// Unknown terrain is solid for movement and standing, including adjacent chunks.
fn collision_block(world: &World, p: [i32; 3]) -> u8 {
    if valid_world_height(p[1])
        && !world
            .chunks
            .contains_key(&[p[0].div_euclid(16), p[2].div_euclid(16)])
    {
        BEDROCK
    } else {
        world.block(p)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn standing_waits_for_neighbor_chunk_collision_data() {
        let mut world = World::new(0);
        world.insert(game_core::Chunk::generate(0, [0, 0]));
        let feet = [15.8, 240., 8.5];
        assert!(!can_stand(feet, |p| collision_block(&world, p)));
        world.insert(game_core::Chunk::generate(0, [1, 0]));
        assert!(can_stand(feet, |p| collision_block(&world, p)));
    }
}
