use crate::{
    session::{Game, Mode},
    ui::{Action, UiIntent},
};
use bevy::{
    input::{
        ButtonState,
        keyboard::KeyboardInput,
        mouse::{AccumulatedMouseMotion, MouseScrollUnit, MouseWheel},
    },
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use game_core::{
    BEDROCK, World,
    interaction::{can_stand, move_player, raycast},
    survival::{DamageSource, GameMode, fall_damage},
    valid_world_height,
};
#[derive(Component)]
pub struct PlayerCamera;
#[derive(Component)]
pub struct Sun;

// Bevy injects each input/resource independently into this system.
#[allow(clippy::too_many_arguments)]
pub fn controls(
    time: Res<Time<Real>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    mut game: ResMut<Game>,
    mut camera: Single<&mut Transform, With<PlayerCamera>>,
    mut window: Single<(&Window, &mut CursorOptions)>,
    mut intents: ResMut<UiIntent>,
    mut gizmos: Gizmos,
    mut wheel: MessageReader<MouseWheel>,
    mut keyboard: MessageReader<KeyboardInput>,
    mut input: Local<crate::control_input::ControlInput>,
    mut steps: Local<crate::footsteps::StepCadence>,
    mut sounds: MessageWriter<crate::block_audio::BlockSound>,
    mut player_actions: MessageWriter<crate::survival_client::PlayerActionMessage>,
    mut inventory_actions: MessageWriter<crate::inventory_client::InventoryActionMessage>,
) {
    let presses: Vec<_> = keyboard
        .read()
        .filter(|e| e.state == ButtonState::Pressed && !e.repeat)
        .map(|e| e.key_code)
        .collect();
    input.frames += 1;
    input.frame_time += time.delta_secs();
    if input.frame_time >= 0.5 {
        game.fps = input.frames as f32 / input.frame_time;
        input.frames = 0;
        input.frame_time = 0.;
    }
    let (debug, grid) = input.debug_keys(
        keys.just_pressed(KeyCode::F3),
        keys.pressed(KeyCode::F3),
        keys.just_released(KeyCode::F3),
        keys.just_pressed(KeyCode::KeyG),
    );
    if game.world_id.is_some() {
        if debug {
            game.debug_open = !game.debug_open;
        }
        if grid {
            game.chunk_grid = !game.chunk_grid;
        }
    }
    let scroll: f32 = wheel
        .read()
        .map(|w| {
            if w.unit == MouseScrollUnit::Pixel {
                w.y / 40.
            } else {
                w.y
            }
        })
        .sum();
    let (window, cursor) = &mut *window;
    if matches!(game.mode, Mode::Console | Mode::Dead) || game.pending_respawn {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
        steps.reset();
        input.clear_movement();
        game.mining.reset();
        game.world_mouse.disarm();
        return;
    }
    if game.mode == Mode::Playing && !window.focused {
        game.screen(Mode::Paused);
    }
    if keys.just_pressed(KeyCode::Escape) {
        match game.mode {
            Mode::Playing => game.screen(Mode::Paused),
            Mode::Paused => game.screen(Mode::Playing),
            Mode::Inventory => {
                inventory_actions.write(crate::inventory_client::InventoryActionMessage(
                    crate::inventory_client::InventoryAction::Close,
                ));
            }
            Mode::Menu => {}
            _ => intents.0.push(Action::Back),
        }
    }
    if keys.just_pressed(KeyCode::KeyE) {
        match game.mode {
            Mode::Playing => game.screen(Mode::Inventory),
            Mode::Inventory => {
                inventory_actions.write(crate::inventory_client::InventoryActionMessage(
                    crate::inventory_client::InventoryAction::Close,
                ));
            }
            _ => {}
        }
    }
    let captured = game.mode == Mode::Playing && window.focused && game.world_id.is_some();
    let world_mouse = game.world_mouse.update(
        captured,
        mouse.pressed(MouseButton::Left),
        mouse.pressed(MouseButton::Right),
    );
    cursor.visible = !captured;
    cursor.grab_mode = if captured {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
    if !captured {
        game.mining.reset();
        steps.reset();
        input.clear_movement();
    }
    if game.world_id.is_none() {
        steps.reset();
        return;
    }
    if keys.just_pressed(KeyCode::F5) {
        intents.0.push(Action::Save);
    }
    if game.mode == Mode::Inventory {
        for (index, key) in [
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
        ]
        .into_iter()
        .enumerate()
        {
            if keys.just_pressed(key) {
                game.selected = index;
                game.ui_dirty = true;
            }
        }
    }
    if captured && keys.just_pressed(KeyCode::KeyQ) {
        inventory_actions.write(crate::inventory_client::InventoryActionMessage(
            crate::inventory_client::InventoryAction::DropSelected {
                all: keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight),
            },
        ));
    }
    if captured {
        game.selected = input.scroll(game.selected, scroll);
        for key in &presses {
            if *key == KeyCode::KeyW {
                input.forward_tap(time.elapsed_secs_f64());
            }
            if *key == KeyCode::Space
                && game.game_mode == GameMode::Creative
                && input.jump_tap(time.elapsed_secs_f64())
            {
                game.flying = !game.flying;
                game.velocity.y = 0.;
                game.grounded = false;
            }
        }
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
        let previous_feet = game.feet;
        let was_grounded = game.grounded;
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
        let crouch =
            !game.flying && (keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight));
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
        let auto_sprint = input.forward_sprint(keys.pressed(KeyCode::KeyW));
        let sprint = keys.pressed(KeyCode::ControlLeft)
            || keys.pressed(KeyCode::ControlRight)
            || auto_sprint;
        let speed = if game.flying {
            if sprint { 20. } else { 10. }
        } else if game.height < 1.8 {
            2.2
        } else if sprint {
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
            if !game.flying && keys.just_pressed(KeyCode::Space) && game.grounded {
                game.velocity.y = 8.0;
                player_actions.write(crate::survival_client::PlayerActionMessage(
                    crate::survival_client::PlayerAction::Jump { sprinting: sprint },
                ));
            }
            if game.flying {
                game.velocity.y = (i32::from(keys.pressed(KeyCode::Space))
                    - i32::from(
                        keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight),
                    )) as f32
                    * speed;
            } else {
                game.velocity.y = (game.velocity.y - 24.0 * dt).max(-45.0);
            }
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
        let traveled = (game.feet - previous_feet).xz().length();
        player_actions.write(crate::survival_client::PlayerActionMessage(
            crate::survival_client::PlayerAction::Travel {
                meters: traveled,
                sprinting: sprint && game.height >= 1.8,
            },
        ));
        if game.game_mode == GameMode::Survival {
            let wet = game.world.block(game.feet.floor().as_ivec3().to_array()) == game_core::WATER;
            if wet {
                game.fall_peak = None;
            } else if game.grounded {
                if !was_grounded && let Some(peak) = game.fall_peak {
                    let amount = fall_damage(peak - game.feet.y, &game.survival_config);
                    if amount > 0. {
                        player_actions.write(crate::survival_client::PlayerActionMessage(
                            crate::survival_client::PlayerAction::Damage {
                                amount,
                                source: DamageSource::Fall,
                            },
                        ));
                    }
                }
                game.fall_peak = None;
            } else {
                game.fall_peak = Some(
                    game.fall_peak
                        .unwrap_or(previous_feet.y)
                        .max(previous_feet.y)
                        .max(game.feet.y),
                );
            }
        } else {
            game.fall_peak = None;
        }
        if steps.advance(
            time.delta_secs(),
            traveled,
            game.grounded,
            game.flying,
            sprint && game.height >= 1.8,
        ) {
            let p = [
                game.feet.x.floor() as i32,
                (game.feet.y - 0.05).floor() as i32,
                game.feet.z.floor() as i32,
            ];
            sounds.write(crate::block_audio::BlockSound {
                block: game.world.block(p),
                kind: crate::block_audio::SoundKind::Step,
                position: Vec3::from_array(p.map(|v| v as f32)) + Vec3::splat(0.5),
            });
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
            use crate::inventory_client::{InventoryAction, InventoryActionMessage};
            if world_mouse
                && game.game_mode == GameMode::Creative
                && mouse.just_pressed(MouseButton::Left)
            {
                inventory_actions.write(InventoryActionMessage(InventoryAction::Break {
                    position: hit.block,
                }));
            }
            if world_mouse && mouse.just_pressed(MouseButton::Right) {
                let action = if game.world.block(hit.block) == game_core::CHEST
                    && !(keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight))
                {
                    Some(InventoryAction::OpenContainer {
                        position: hit.block,
                    })
                } else if game.selected_block() != game_core::AIR {
                    Some(InventoryAction::Place {
                        position: hit.previous,
                    })
                } else {
                    None
                };
                if let Some(action) = action {
                    inventory_actions.write(InventoryActionMessage(action));
                }
            }
        }
    }
    if captured {
        // Добыча использует всё реальное время, а не ограниченный dt движения.
        if let Some(position) = crate::mining_client::advance(
            &mut game,
            mouse.pressed(MouseButton::Left),
            time.delta_secs(),
        ) {
            inventory_actions.write(crate::inventory_client::InventoryActionMessage(
                crate::inventory_client::InventoryAction::Break { position },
            ));
        }
        crate::mining_client::draw_cracks(&game, &mut gizmos);
    }
    if game.chunk_grid {
        draw_chunk_grid(game.feet, &mut gizmos);
    }
    camera.translation = game.feet + Vec3::Y * (game.height - 0.15);
    camera.rotation = Quat::from_rotation_y(game.yaw) * Quat::from_rotation_x(game.pitch);
}

pub fn sky(
    game: Res<Game>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut sun: Single<(&mut DirectionalLight, &mut Transform), With<Sun>>,
    mut camera: Single<
        (
            &mut DistanceFog,
            &mut bevy::light::Skybox,
            &bevy::camera::Exposure,
        ),
        With<PlayerCamera>,
    >,
) {
    let angle = (game.clock as f32 / 1200.0) * std::f32::consts::TAU;
    let daylight = angle.sin().clamp(0.0, 1.0);
    let color = Color::srgb(0.72, 0.84, 0.90);
    let brightness = 0.025 + daylight * 0.975;
    clear.0 = if game.world_id.is_some() {
        color
    } else {
        Color::srgb(0.035, 0.05, 0.07)
    };
    sun.0.illuminance = 100.0 + 10000.0 * daylight;
    sun.0.color = Color::srgb(1., 0.94 + daylight * 0.04, 0.85 + daylight * 0.13);
    sun.1.rotation = Quat::from_euler(EulerRot::XYZ, -angle, 0.45, 0.0);
    ambient.brightness = 100.0 + daylight * 260.0;
    // Fog and sky share linear radiance and camera exposure, including at night.
    let horizon = color.to_linear();
    let intensity = 900. * brightness * camera.2.exposure();
    camera.0.color = Color::linear_rgb(
        horizon.red * intensity,
        horizon.green * intensity,
        horizon.blue * intensity,
    );
    let end = (game.settings.distance * 16) as f32;
    camera.0.falloff = FogFalloff::Linear {
        start: end * 0.55,
        end,
    };
    camera.1.brightness = 900. * brightness;
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

fn draw_chunk_grid(feet: Vec3, gizmos: &mut Gizmos) {
    let cx = (feet.x.floor() as i32).div_euclid(16);
    let cz = (feet.z.floor() as i32).div_euclid(16);
    let x0 = ((cx - 1) * 16) as f32;
    let x1 = ((cx + 2) * 16) as f32;
    let z0 = ((cz - 1) * 16) as f32;
    let z1 = ((cz + 2) * 16) as f32;
    let color = Color::srgb(1., 0.75, 0.12);
    for x in cx - 1..=cx + 2 {
        for z in cz - 1..=cz + 2 {
            gizmos.line(
                Vec3::new((x * 16) as f32, -64., (z * 16) as f32),
                Vec3::new((x * 16) as f32, 321., (z * 16) as f32),
                color,
            );
        }
    }
    for y in (-64..=320).step_by(16).chain(std::iter::once(321)) {
        for x in cx - 1..=cx + 2 {
            gizmos.line(
                Vec3::new((x * 16) as f32, y as f32, z0),
                Vec3::new((x * 16) as f32, y as f32, z1),
                color,
            );
        }
        for z in cz - 1..=cz + 2 {
            gizmos.line(
                Vec3::new(x0, y as f32, (z * 16) as f32),
                Vec3::new(x1, y as f32, (z * 16) as f32),
                color,
            );
        }
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
