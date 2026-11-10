//! События связывают ввод с независимыми правилами выживания.
use crate::{
    console::ConsoleState,
    session::{Game, Mode},
};
use bevy::prelude::*;
use game_core::{
    AIR, WATER, WORLD_MAX_Y, WORLD_MIN_Y, is_solid,
    survival::{DamageSource, Environment, GameMode, GameRule, StateEvent, SurvivalConfig},
};
use std::{path::PathBuf, time::SystemTime};

#[derive(Clone, Copy, Debug)]
pub enum PlayerAction {
    SetMode(GameMode),
    SetRule(GameRule, bool),
    Respawn,
    Travel { meters: f32, sprinting: bool },
    Jump { sprinting: bool },
    Damage { amount: f32, source: DamageSource },
    MinedBlock,
}
#[derive(Message)]
pub struct PlayerActionMessage(pub PlayerAction);
#[derive(Message)]
pub struct SurvivalStateChanged(pub StateEvent);

#[derive(Default)]
pub struct ReloadState {
    checked: f64,
    path: Option<PathBuf>,
    stamp: Option<(SystemTime, u64)>,
}

pub fn reload_config(time: Res<Time<Real>>, mut game: ResMut<Game>, mut state: Local<ReloadState>) {
    if state.checked > 0. && time.elapsed_secs_f64() - state.checked < 1. {
        return;
    }
    state.checked = time.elapsed_secs_f64().max(f64::EPSILON);
    let explicit_path = std::env::var_os("VOXEL_SURVIVAL_CONFIG").map(PathBuf::from);
    let path = explicit_path
        .clone()
        .or_else(|| {
            let p = std::env::current_exe()
                .ok()?
                .parent()?
                .join("assets/data/survival.ron");
            p.is_file().then_some(p)
        })
        .unwrap_or_else(|| PathBuf::from("assets/data/survival.ron"));
    let stamp = std::fs::metadata(&path)
        .ok()
        .and_then(|m| m.modified().ok().map(|t| (t, m.len())));
    if state.path.as_ref() == Some(&path) && state.stamp == stamp {
        return;
    }
    state.path = Some(path.clone());
    state.stamp = stamp;
    if stamp.is_none() {
        if explicit_path.is_some() {
            warn!(
                "Не удалось прочитать VOXEL_SURVIVAL_CONFIG={}; сохранена предыдущая конфигурация",
                path.display()
            );
        }
        return;
    } // Без внешних файлов работает встроенная конфигурация.
    let result = std::fs::read_to_string(&path)
        .map_err(|e| e.to_string())
        .and_then(|s| SurvivalConfig::from_ron(&s));
    match result {
        Ok(config) => {
            if game.world_id.is_none() {
                game.vitals.respawn(&config);
            }
            game.survival_config = config;
            game.ui_dirty = true;
            info!("Параметры выживания обновлены: {}", path.display());
        }
        Err(error) => warn!(
            "Некорректный {}: {error}; сохранена предыдущая конфигурация",
            path.display()
        ),
    }
}

fn publish(
    game: &mut Game,
    events: Vec<StateEvent>,
    changed: &mut MessageWriter<SurvivalStateChanged>,
) {
    for event in events {
        if let StateEvent::Died { source } = event {
            game.flying = false;
            game.velocity = Vec3::ZERO;
            game.fall_peak = None;
            game.status = format!(
                "Причина смерти: {}",
                match source {
                    DamageSource::Fall => "падение",
                    DamageSource::Drowning => "утопление",
                    DamageSource::Starvation => "голод",
                    DamageSource::Void => "граница мира",
                    DamageSource::Lava => "лава",
                    DamageSource::Fire => "огонь",
                    DamageSource::Mob => "атака",
                }
            );
            crate::inventory_client::on_death(game);
            // Дроп вычисляется логикой до перехода на экран смерти.
            info!(
                keep_inventory = game.game_rules.keep_inventory,
                "Применено правило инвентаря при смерти"
            );
            game.screen(Mode::Dead);
        }
        debug!(?event, "Изменение состояния игрока");
        changed.write(SurvivalStateChanged(event));
    }
}

pub fn apply_actions(
    mut actions: MessageReader<PlayerActionMessage>,
    mut game: ResMut<Game>,
    mut console: ResMut<ConsoleState>,
    mut changed: MessageWriter<SurvivalStateChanged>,
) {
    if game.world_id.is_none() {
        actions.clear();
        return;
    }
    for message in actions.read() {
        let cfg = game.survival_config.clone();
        let mode = game.game_mode;
        if game.vitals.is_dead() && !matches!(message.0, PlayerAction::Respawn) {
            continue;
        }
        match message.0 {
            PlayerAction::SetMode(mode) => {
                game.mining.reset();
                game.world_mouse.disarm();
                game.game_mode = mode;
                game.flying = false;
                game.fall_peak = None;
                if mode == GameMode::Creative {
                    game.vitals.respawn(&cfg);
                    if game.inventory.hotbar.iter().all(Option::is_none) {
                        let catalog = game.content.clone();
                        for (index, block) in [1, 2, 3, 5, 7, 9, 10, 11, 14].into_iter().enumerate()
                        {
                            if let Some(item) = catalog.item_for_block(block) {
                                game.inventory.hotbar[index] = Some(
                                    game_core::items::ItemStack::new(item, catalog.max_stack(item)),
                                );
                            }
                        }
                        crate::inventory_client::sync_hotbar(&mut game);
                    }
                }
                console.feedback = format!(
                    "Режим: {}",
                    if mode == GameMode::Survival {
                        "выживание"
                    } else {
                        "творческий"
                    }
                );
                game.status.clone_from(&console.feedback);
                game.ui_dirty = true;
                info!(?mode, "Режим игрока изменён");
            }
            PlayerAction::SetRule(rule, value) => {
                match rule {
                    GameRule::KeepInventory => game.game_rules.keep_inventory = value,
                    GameRule::NaturalRegeneration => game.game_rules.natural_regeneration = value,
                }
                console.feedback = format!("{rule:?} = {value}");
                game.status.clone_from(&console.feedback);
                info!(?rule, value, "Правило изменено");
            }
            PlayerAction::Respawn => {
                if game.vitals.is_dead() {
                    game.feet = game.spawn;
                    game.velocity = Vec3::ZERO;
                    game.flying = false;
                    game.grounded = false;
                    game.fall_peak = None;
                    game.pending_respawn = true;
                    game.vitals.respawn(&cfg);
                    game.status.clear();
                    game.screen(Mode::Playing);
                }
            }
            PlayerAction::Travel { meters, sprinting } => {
                if sprinting && meters.is_finite() && meters > 0. {
                    game.vitals
                        .exhaust(meters * cfg.sprint_per_meter, mode, &cfg);
                }
            }
            PlayerAction::Jump { sprinting } => game.vitals.exhaust(
                if sprinting {
                    cfg.sprint_jump_exhaustion
                } else {
                    cfg.jump_exhaustion
                },
                mode,
                &cfg,
            ),
            PlayerAction::MinedBlock => game.vitals.exhaust(cfg.mining_exhaustion, mode, &cfg),
            PlayerAction::Damage { amount, source } => {
                let events = game.vitals.damage(amount, source, mode, &cfg);
                publish(&mut game, events, &mut changed);
            }
        }
    }
}

fn safe_spawn(game: &Game) -> Vec3 {
    let p = game.spawn.floor().as_ivec3();
    let start = p.y.clamp(WORLD_MIN_Y + 1, WORLD_MAX_Y + 1);
    let ys = (start..=WORLD_MAX_Y + 1).chain((WORLD_MIN_Y + 1..start).rev());
    for y in ys {
        if is_solid(game.world.block([p.x, y - 1, p.z]))
            && game.world.block([p.x, y, p.z]) == AIR
            && game.world.block([p.x, y + 1, p.z]) == AIR
        {
            return Vec3::new(game.spawn.x, y as f32, game.spawn.z);
        }
    }
    game.spawn
}

pub fn simulate(
    time: Res<Time<Real>>,
    mut game: ResMut<Game>,
    mut changed: MessageWriter<SurvivalStateChanged>,
) {
    if game.world_id.is_none() || game.mode != Mode::Playing {
        return;
    }
    let key = [
        (game.feet.x.floor() as i32).div_euclid(16),
        (game.feet.z.floor() as i32).div_euclid(16),
    ];
    if !game.world.chunks.contains_key(&key) {
        return;
    }
    if game.pending_respawn {
        game.feet = safe_spawn(&game);
        game.pending_respawn = false;
    }
    let eye = game.feet + Vec3::Y * (game.height - 0.15);
    let environment = Environment {
        underwater: game.world.block(eye.floor().as_ivec3().to_array()) == WATER,
        // Блоки лавы/огня появятся на этапе 4; их урон уже проверяется в ядре.
        in_lava: false,
        in_fire: false,
        out_of_world: game.feet.y < (WORLD_MIN_Y - 16) as f32
            || game.feet.y > (WORLD_MAX_Y + 16) as f32,
    };
    let mode = game.game_mode;
    let rules = game.game_rules;
    let cfg = game.survival_config.clone();
    let events = game
        .vitals
        .tick(time.delta_secs(), environment, mode, &rules, &cfg);
    publish(&mut game, events, &mut changed);
}

// Подписчик событий отделён от изменения правил; HUD читает готовое состояние.
pub fn observe_changes(mut changes: MessageReader<SurvivalStateChanged>) {
    for event in changes.read() {
        trace!(event=?event.0,"Событие для HUD/эффектов");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn app() -> App {
        let mut app = App::new();
        app.init_resource::<Game>()
            .init_resource::<ConsoleState>()
            .add_message::<PlayerActionMessage>()
            .add_message::<SurvivalStateChanged>()
            .add_systems(Update, apply_actions);
        app.world_mut().resource_mut::<Game>().world_id = Some("test".into());
        app
    }
    #[test]
    fn commands_change_modes_but_dead_players_cannot_bypass_respawn() {
        let mut app = app();
        app.world_mut()
            .write_message(PlayerActionMessage(PlayerAction::SetMode(
                GameMode::Creative,
            )));
        app.update();
        assert_eq!(app.world().resource::<Game>().game_mode, GameMode::Creative);
        app.world_mut()
            .write_message(PlayerActionMessage(PlayerAction::Damage {
                amount: 100.,
                source: DamageSource::Fall,
            }));
        app.update();
        assert!(!app.world().resource::<Game>().vitals.is_dead());
        app.world_mut()
            .write_message(PlayerActionMessage(PlayerAction::SetMode(
                GameMode::Survival,
            )));
        app.world_mut()
            .write_message(PlayerActionMessage(PlayerAction::Damage {
                amount: 100.,
                source: DamageSource::Fall,
            }));
        app.update();
        assert_eq!(app.world().resource::<Game>().mode, Mode::Dead);
        app.world_mut()
            .write_message(PlayerActionMessage(PlayerAction::Respawn));
        app.update();
        let g = app.world().resource::<Game>();
        assert!(!g.vitals.is_dead());
        assert!(g.pending_respawn);
        assert_eq!(g.mode, Mode::Playing);
    }

    #[test]
    fn changing_game_mode_clears_mining_and_requires_mouse_release() {
        let mut app = app();
        {
            let mut game = app.world_mut().resource_mut::<Game>();
            game.world_mouse.update(true, false, false);
            game.mining.advance(
                game_core::mining::MiningTarget {
                    position: [0, 1, 0],
                    block: game_core::STONE,
                    item: None,
                    slot: 0,
                },
                1.,
                2.,
            );
        }
        app.world_mut()
            .write_message(PlayerActionMessage(PlayerAction::SetMode(
                GameMode::Creative,
            )));
        app.update();
        let mut game = app.world_mut().resource_mut::<Game>();
        assert!(game.mining.target().is_none());
        assert!(!game.world_mouse.update(true, true, false));
        assert!(game.world_mouse.update(true, false, false));
    }

    #[test]
    fn respawn_finds_space_above_a_built_roof_or_below_a_removed_floor() {
        use game_core::{Chunk, STONE};
        let mut game = Game {
            spawn: Vec3::new(0.5, 90., 0.5),
            ..Default::default()
        };
        game.world.insert(Chunk {
            position: [0, 0],
            sections: Default::default(),
        });
        for y in 89..=94 {
            game.world.set([0, y, 0], STONE);
        }
        assert_eq!(safe_spawn(&game), Vec3::new(0.5, 95., 0.5));
        for y in 89..=94 {
            game.world.set([0, y, 0], AIR);
        }
        game.world.set([0, 80, 0], STONE);
        assert_eq!(safe_spawn(&game), Vec3::new(0.5, 81., 0.5));
        // Полный столбец до верхнего слоя мира оставляет свободное место над ним.
        for y in 81..=WORLD_MAX_Y {
            game.world.set([0, y, 0], STONE);
        }
        assert_eq!(safe_spawn(&game).y, (WORLD_MAX_Y + 1) as f32);
    }
}
