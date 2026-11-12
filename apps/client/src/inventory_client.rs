//! Инвентарь, контейнеры и предметы мира изменяются только проверенными событиями.
use crate::{
    block_audio::BlockSound,
    session::{Game, Mode},
};
use bevy::prelude::*;
use game_core::{
    AIR, BEDROCK, CHEST, CRAFTING_TABLE, FURNACE, World,
    content::ItemId,
    crafting::RecipeBook,
    drops::DropConfig,
    interaction::{player_intersects_block, raycast},
    inventory::{ClickButton, Container, Inventory, SlotRef},
    items::ItemStack,
    survival::GameMode,
};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum InventoryAction {
    Click {
        slot: SlotRef,
        button: ClickButton,
        shift: bool,
    },
    CreativeGrant(ItemId),
    TakeCraftResult {
        all: bool,
    },
    DropSelected {
        all: bool,
    },
    Close,
    Break {
        position: [i32; 3],
    },
    Place {
        position: [i32; 3],
    },
    OpenContainer {
        position: [i32; 3],
    },
}

#[derive(Message, Clone)]
pub struct InventoryActionMessage(pub InventoryAction);

pub fn plugin(app: &mut App) {
    app.add_message::<InventoryActionMessage>()
        .add_systems(Update, reload_config.before(crate::actions))
        .add_systems(
            Update,
            apply_actions
                .after(crate::player::controls)
                .before(crate::survival_client::apply_actions),
        )
        .add_systems(
            Update,
            tick_drops
                .after(crate::survival_client::simulate)
                .before(crate::player::sky),
        )
        .add_systems(Update, tick_furnaces.after(apply_actions));
}

pub fn reset_session(game: &mut Game, creative: bool) {
    game.mining.reset();
    game.world_mouse.disarm();
    game.inventory = Inventory::default();
    game.containers.clear();
    game.open_container = None;
    game.open_workbench = None;
    game.furnaces.clear();
    game.drops.clear();
    game.selected = 0;
    if creative {
        for (slot, block) in [1, 2, 3, 5, 7, 9, 10, 11, 14].into_iter().enumerate() {
            if let Some(item) = game.content.item_for_block(block) {
                game.inventory.hotbar[slot] =
                    Some(ItemStack::new(item, game.content.max_stack(item)));
            }
        }
    }
    sync_hotbar(game);
    game.ui_dirty = true;
}

pub fn close_inventory(game: &mut Game) {
    if let Some(stack) = game.inventory.close_cursor(&game.content) {
        if game.game_rules.keep_inventory && (game.vitals.is_dead() || game.mode == Mode::Dead) {
            game.inventory.cursor = Some(stack);
        } else {
            spawn_drop(game, stack, game.feet + Vec3::Y * 0.6, Vec3::Y * 1.5);
        }
    }
    let overflow = game.inventory.close_crafting(&game.content);
    if game.game_rules.keep_inventory && (game.vitals.is_dead() || game.mode == Mode::Dead) {
        game.inventory.crafting = overflow;
    } else {
        for stack in overflow.into_iter().flatten() {
            spawn_drop(game, stack, game.feet + Vec3::Y * 0.6, Vec3::Y * 1.5);
        }
    }
    let leftovers = game.inventory.close_workbench(&game.content);
    if game.game_rules.keep_inventory && (game.vitals.is_dead() || game.mode == Mode::Dead) {
        game.inventory.workbench = leftovers;
    } else {
        for stack in leftovers.into_iter().flatten() {
            spawn_drop(game, stack, game.feet + Vec3::Y * 0.6, Vec3::Y * 1.5);
        }
    }
    game.open_container = None;
    game.open_workbench = None;
    sync_hotbar(game);
    game.ui_dirty = true;
}

pub fn on_death(game: &mut Game) {
    game.mining.reset();
    game.world_mouse.disarm();
    if game.game_rules.keep_inventory {
        game.inventory.cursor = game.inventory.close_cursor(&game.content);
        game.inventory.crafting = game.inventory.close_crafting(&game.content);
        game.inventory.workbench = game.inventory.close_workbench(&game.content);
    } else {
        for stack in game.inventory.drain_all() {
            spawn_drop(game, stack, game.feet + Vec3::Y * 0.6, Vec3::Y * 1.5);
        }
    }
    game.open_container = None;
    game.open_workbench = None;
    sync_hotbar(game);
    game.ui_dirty = true;
}

pub fn sync_hotbar(game: &mut Game) {
    game.hotbar = std::array::from_fn(|index| {
        game.inventory.hotbar[index]
            .as_ref()
            .and_then(|stack| game.content.block_for_item(stack.item))
            .unwrap_or(AIR)
    });
}

#[derive(Default)]
struct Effects {
    sounds: Vec<BlockSound>,
    mined: bool,
}
fn process_action(game: &mut Game, action: &InventoryAction) -> Effects {
    let mut effects = Effects::default();
    if game.world_id.is_none() || game.vitals.is_dead() || game.pending_respawn {
        game.mining.reset();
        return effects;
    }
    match action {
        InventoryAction::Click {
            slot,
            button,
            shift,
        } if game.mode == Mode::Inventory => {
            if matches!(slot, SlotRef::Workbench(_)) && game.open_workbench.is_none() {
                return effects;
            }
            if matches!(slot, SlotRef::Container(2)) && game.open_container.is_some_and(|p| game.world.block(p) == FURNACE) && game.inventory.cursor.is_some() {
                return effects;
            }
            let container = game
                .open_container
                .and_then(|p| game.containers.get_mut(&p));
            if game
                .inventory
                .click(container, *slot, *button, *shift, &game.content)
            {
                game.ui_dirty = true;
            }
        }
        InventoryAction::TakeCraftResult { all } if game.mode == Mode::Inventory => {
            let crafted = if game.open_workbench.is_some() {
                game.recipes.take_workbench_result(&mut game.inventory, *all, &game.content)
            } else {
                game.recipes.take_result(&mut game.inventory, *all, &game.content)
            };
            if crafted {
                game.ui_dirty = true;
            }
        }
        InventoryAction::CreativeGrant(item)
            if game.mode == Mode::Inventory && game.game_mode == GameMode::Creative =>
        {
            let count = game.content.max_stack(*item);
            if count > 0
                && let Some(slot) = game.inventory.hotbar.get_mut(game.selected)
            {
                *slot = Some(ItemStack::new(*item, count));
                game.ui_dirty = true;
            }
        }
        InventoryAction::Close if game.mode == Mode::Inventory => {
            close_inventory(game);
            game.screen(Mode::Playing);
        }
        InventoryAction::DropSelected { all } if game.mode == Mode::Playing => {
            let stack = if game.game_mode == GameMode::Creative {
                game.item_for_slot(game.selected).cloned().map(|mut stack| {
                    if !all {
                        stack.count = 1;
                    }
                    stack
                })
            } else {
                game.inventory.take_selected(game.selected, *all)
            };
            if let Some(stack) = stack {
                let look = look_direction(game);
                spawn_drop(
                    game,
                    stack,
                    game.feet + Vec3::Y * (game.height - 0.3),
                    look * 3. + Vec3::Y,
                );
                game.ui_dirty = true;
            }
        }
        InventoryAction::Break { position }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            if !visible_hit(game).is_some_and(|hit| hit.block == *position) {
                game.mining.reset();
                return effects;
            }
            let block = game.world.block(*position);
            let tool = crate::mining_client::chosen_tool(game);
            let target = crate::mining_client::target(game, *position);
            let can_harvest = game.mining_config.can_harvest(block, tool);
            if block == BEDROCK || block == AIR {
                game.mining.reset();
                return effects;
            }
            if game.game_mode == GameMode::Survival {
                let Some(duration) = game.mining_config.duration(block, tool) else {
                    game.mining.reset();
                    return effects;
                };
                if !game.mining.consume(target, duration) {
                    game.mining.reset();
                    return effects;
                }
            }
            if game.world.set(*position, AIR) {
                let center = block_center(*position);
                game.furnaces.remove(position);
                if let Some(container) = game.containers.remove(position) {
                    for stack in container.slots.into_iter().flatten() {
                        spawn_drop(game, stack, center, Vec3::Y * 1.5);
                    }
                }
                if game.game_mode == GameMode::Survival {
                    if can_harvest && let Some(item) = game.content.drop_for_block(block) {
                        spawn_drop(game, ItemStack::new(item, 1), center, Vec3::Y * 1.5);
                    }
                    effects.mined = true;
                }
                effects.sounds.push(BlockSound {
                    block,
                    kind: crate::block_audio::SoundKind::Break,
                    position: center,
                });
                game.revision += 1;
                game.ui_dirty = true;
            }
        }
        InventoryAction::Place { position }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            let block = game.selected_block();
            if block != AIR
                && visible_hit(game).is_some_and(|hit| hit.previous == *position)
                && loaded_block(&game.world, *position) == Some(AIR)
                && !player_intersects_block(game.feet.to_array(), game.height, *position)
                && game.world.set(*position, block)
            {
                if game.game_mode == GameMode::Survival {
                    game.inventory.take_selected(game.selected, false);
                }
                if block == CHEST || block == FURNACE {
                    game.containers
                        .entry(*position)
                        .or_insert_with(|| Container::new(if block == CHEST { 27 } else { 3 }));
                }
                if block == FURNACE {
                    game.furnaces.entry(*position).or_default();
                }
                effects.sounds.push(BlockSound {
                    block,
                    kind: crate::block_audio::SoundKind::Place,
                    position: block_center(*position),
                });
                game.revision += 1;
                game.ui_dirty = true;
            }
        }
        InventoryAction::OpenContainer { position }
            if game.mode == Mode::Playing
                && game.world_mouse.allowed()
                && matches!(loaded_block(&game.world, *position), Some(CHEST | FURNACE | CRAFTING_TABLE))
                && visible_hit(game).is_some_and(|hit| hit.block == *position) =>
        {
            let block = game.world.block(*position);
            game.open_container = None;
            game.open_workbench = None;
            if block == CRAFTING_TABLE {
                game.open_workbench = Some(*position);
            } else {
                game.containers
                    .entry(*position)
                    .or_insert_with(|| Container::new(if block == CHEST { 27 } else { 3 }));
                if block == FURNACE {
                    game.furnaces.entry(*position).or_default();
                }
                game.open_container = Some(*position);
            }
            game.screen(Mode::Inventory);
        }
        _ => {}
    }
    sync_hotbar(game);
    effects
}

fn look_direction(game: &Game) -> Vec3 {
    Quat::from_rotation_y(game.yaw) * Quat::from_rotation_x(game.pitch) * Vec3::NEG_Z
}

fn block_center(position: [i32; 3]) -> Vec3 {
    Vec3::from_array(position.map(|value| value as f32)) + Vec3::splat(0.5)
}

fn loaded_block(world: &World, position: [i32; 3]) -> Option<u8> {
    (game_core::valid_world_height(position[1])
        && world
            .chunks
            .contains_key(&[position[0].div_euclid(16), position[2].div_euclid(16)]))
    .then(|| world.block(position))
}

pub(crate) fn visible_hit(game: &Game) -> Option<game_core::interaction::RayHit> {
    let eye = game.feet + Vec3::Y * (game.height - 0.15);
    let hit = raycast(
        eye.to_array(),
        look_direction(game).to_array(),
        6.,
        |position| loaded_block(&game.world, position).unwrap_or(BEDROCK),
    )?;
    loaded_block(&game.world, hit.block).map(|_| hit)
}

fn spawn_drop(game: &mut Game, stack: ItemStack, position: Vec3, velocity: Vec3) {
    game.drops.spawn(
        stack,
        position.to_array(),
        velocity.to_array(),
        &game.drop_config,
    );
}

fn apply_actions(
    mut messages: MessageReader<InventoryActionMessage>,
    mut game: ResMut<Game>,
    mut sounds: MessageWriter<BlockSound>,
    mut player_actions: MessageWriter<crate::survival_client::PlayerActionMessage>,
) {
    for message in messages.read() {
        let effects = process_action(&mut game, &message.0);
        for sound in effects.sounds {
            sounds.write(sound);
        }
        if effects.mined {
            player_actions.write(crate::survival_client::PlayerActionMessage(
                crate::survival_client::PlayerAction::MinedBlock,
            ));
        }
    }
}

pub fn tick_furnaces(time: Res<Time<Real>>, mut game: ResMut<Game>) {
    if !matches!(game.mode, Mode::Playing | Mode::Inventory) || game.world_id.is_none() {
        return;
    }
    let elapsed = time.delta_secs();
    let mut changed = false;
    let game: &mut Game = &mut game;
    for (position, state) in &mut game.furnaces {
        if game.world.block(*position) != FURNACE {
            continue;
        }
        if let Some(container) = game.containers.get_mut(position) {
            let progress_before = state.progress.floor();
            changed |= state.tick(container, elapsed, &game.content);
            if game.open_container == Some(*position) && progress_before != state.progress.floor() {
                changed = true;
            }
        }
    }
    if changed {
        game.ui_dirty = true;
    }
}

pub fn tick_drops(time: Res<Time<Real>>, mut game: ResMut<Game>) {
    if game.world_id.is_none()
        || game.mode != Mode::Playing
        || game.pending_respawn
        || game.vitals.is_dead()
    {
        return;
    }
    let player = (game.feet + Vec3::Y * 0.6).to_array();
    let game = &mut *game;
    if game.drops.tick(
        time.delta_secs(),
        Some(player),
        &mut game.inventory,
        &game.content,
        &game.drop_config,
        |position| loaded_block(&game.world, position),
    ) {
        sync_hotbar(game);
        game.ui_dirty = true;
    }
}

fn reload_items(game: &mut Game, source: &str) -> Result<(), String> {
    let candidate = game.content.reload(source)?;
    let stacks = game
        .inventory
        .hotbar
        .iter()
        .chain(&game.inventory.main)
        .chain(&game.inventory.armor)
        .chain(&game.inventory.crafting)
        .chain(std::iter::once(&game.inventory.offhand))
        .chain(std::iter::once(&game.inventory.cursor))
        .filter_map(Option::as_ref)
        .chain(
            game.containers
                .values()
                .flat_map(|container| container.slots.iter().filter_map(Option::as_ref)),
        )
        .chain(game.drops.entities.values().map(|drop| &drop.stack));
    for stack in stacks {
        if stack.count == 0 || stack.count > candidate.max_stack(stack.item) {
            return Err("Новый размер стопки несовместим с существующими предметами".into());
        }
    }
    game.recipes.validate_catalog(&candidate)?;
    game.content = candidate;
    sync_hotbar(game);
    game.ui_dirty = true;
    Ok(())
}

#[derive(Default)]
struct FileState {
    previous: Option<(Option<PathBuf>, Result<String, String>)>,
    accepted: bool,
}

#[derive(Default)]
struct ReloadState {
    checked: Option<f64>,
    items: FileState,
    drops: FileState,
    recipes: FileState,
    mining: FileState,
}

pub(crate) fn config_source(
    variable: &str,
    filename: &str,
    embedded: &str,
) -> (Option<PathBuf>, Result<String, String>) {
    let explicit = std::env::var_os(variable).map(PathBuf::from);
    let path = explicit
        .or_else(|| {
            std::env::current_exe()
                .ok()
                .and_then(|exe| {
                    exe.parent()
                        .map(|dir| dir.join("assets/data").join(filename))
                })
                .filter(|path| path.is_file())
        })
        .or_else(|| {
            let path = PathBuf::from("assets/data").join(filename);
            path.is_file().then_some(path)
        });
    let source = path.as_ref().map_or_else(
        || Ok(embedded.to_owned()),
        |path| {
            std::fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))
        },
    );
    (path, source)
}

fn reload_config(time: Res<Time<Real>>, mut game: ResMut<Game>, mut state: Local<ReloadState>) {
    let now = time.elapsed_secs_f64();
    if state.checked.is_some_and(|checked| now - checked < 1.) {
        return;
    }
    state.checked = Some(now);
    let items = config_source(
        "VOXEL_ITEMS_CONFIG",
        "items.ron",
        include_str!("../../../assets/data/items.ron"),
    );
    reload_file(&mut state.items, items, |source| {
        reload_items(&mut game, source)
    });
    let recipes = config_source(
        "VOXEL_RECIPES_CONFIG",
        "recipes.ron",
        include_str!("../../../assets/data/recipes.ron"),
    );
    reload_file(&mut state.recipes, recipes, |source| {
        let candidate = RecipeBook::from_ron(source, &game.content)?;
        game.recipes = candidate;
        game.ui_dirty = true;
        Ok(())
    });
    let mining = config_source(
        "VOXEL_MINING_CONFIG",
        "mining.ron",
        include_str!("../../../assets/data/mining.ron"),
    );
    reload_file(&mut state.mining, mining, |source| {
        let candidate = game_core::mining::MiningConfig::from_ron(source)?;
        if game.mining_config != candidate {
            game.mining_config = candidate;
            game.mining.reset();
        }
        Ok(())
    });
    let drops = config_source(
        "VOXEL_DROPS_CONFIG",
        "drops.ron",
        include_str!("../../../assets/data/drops.ron"),
    );
    reload_file(&mut state.drops, drops, |source| {
        game.drop_config = DropConfig::from_ron(source)?;
        Ok(())
    });
}

fn reload_file(
    state: &mut FileState,
    source: (Option<PathBuf>, Result<String, String>),
    mut apply: impl FnMut(&str) -> Result<(), String>,
) {
    let changed = state.previous.as_ref() != Some(&source);
    if !changed && state.accepted {
        return;
    }
    let result = source
        .1
        .as_ref()
        .map_err(Clone::clone)
        .and_then(|text| apply(text));
    state.accepted = result.is_ok();
    if let Err(error) = result
        && changed
    {
        warn!("Конфигурация предметов не обновлена: {error}; сохранена предыдущая версия");
    }
    state.previous = Some(source);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn world() -> Game {
        let mut game = Game {
            mode: Mode::Playing,
            world_id: Some("inventory-test".into()),
            feet: Vec3::new(8.5, 100., 8.5),
            height: 1.8,
            pitch: 0.,
            ..Default::default()
        };
        game.world.chunks.insert(
            [0, 0],
            Arc::new(game_core::Chunk {
                position: [0, 0],
                sections: Default::default(),
            }),
        );
        game.world_mouse.update(true, false, false);
        assert!(game.world.set([8, 101, 5], game_core::STONE));
        game
    }
    fn equip(game: &mut Game, name: &str) {
        game.inventory.hotbar[game.selected] =
            Some(ItemStack::new(game.content.find(name).unwrap(), 1));
    }
    fn finish_mining(game: &mut Game) {
        game.world_mouse.update(true, false, false);
        let block = visible_hit(game).unwrap().block;
        let duration = game
            .mining_config
            .duration(
                game.world.block(block),
                crate::mining_client::chosen_tool(game),
            )
            .unwrap();
        assert_eq!(
            crate::mining_client::advance(game, true, duration),
            Some(block)
        );
    }
    fn stack(game: &Game, block: u8, count: u16) -> ItemStack {
        ItemStack::new(game.content.item_for_block(block).unwrap(), count)
    }
    fn total_drops(game: &Game) -> u16 {
        game.drops
            .entities
            .values()
            .map(|drop| drop.stack.count)
            .sum()
    }
    fn fill(game: &mut Game) {
        let value = stack(game, game_core::DIRT, 64);
        game.inventory.hotbar.fill(Some(value.clone()));
        game.inventory.main.fill(Some(value));
    }

    #[test]
    fn result_commands_are_mode_gated_and_consume_actual_input_once() {
        let mut game = world();
        let log = game.content.find("foundation:oak_log").unwrap();
        let planks = game.content.find("foundation:oak_planks").unwrap();
        game.inventory.crafting[2] = Some(ItemStack::new(log, 1));
        process_action(&mut game, &InventoryAction::TakeCraftResult { all: false });
        assert!(game.inventory.cursor.is_none());
        game.mode = Mode::Inventory;
        game.ui_dirty = false;
        process_action(&mut game, &InventoryAction::TakeCraftResult { all: false });
        assert_eq!(game.inventory.cursor, Some(ItemStack::new(planks, 4)));
        assert!(game.ui_dirty);
        process_action(&mut game, &InventoryAction::TakeCraftResult { all: false });
        assert_eq!(game.inventory.cursor, Some(ItemStack::new(planks, 4)));
        game.inventory.cursor = None;
        game.inventory.crafting[0] = Some(ItemStack::new(log, 20));
        process_action(&mut game, &InventoryAction::TakeCraftResult { all: true });
        assert_eq!(game.inventory.hotbar[0], Some(ItemStack::new(planks, 64)));
        assert_eq!(game.inventory.hotbar[1], Some(ItemStack::new(planks, 16)));
        assert!(game.inventory.crafting.iter().all(Option::is_none));
    }
    #[test]
    fn crafting_close_death_and_keep_inventory_preserve_single_ownership() {
        let mut game = world();
        fill(&mut game);
        game.inventory.crafting[0] = Some(stack(&game, game_core::STONE, 3));
        game.inventory.crafting[3] = Some(stack(&game, game_core::SAND, 2));
        game.inventory.cursor = Some(stack(&game, game_core::STONE, 1));
        close_inventory(&mut game);
        close_inventory(&mut game);
        assert_eq!(total_drops(&game), 6);
        assert!(game.inventory.crafting.iter().all(Option::is_none));
        reset_session(&mut game, false);
        game.inventory.crafting[1] = Some(stack(&game, game_core::STONE, 3));
        on_death(&mut game);
        on_death(&mut game);
        assert_eq!(total_drops(&game), 3);
        assert!(game.inventory.drain_all().is_empty());
        reset_session(&mut game, false);
        fill(&mut game);
        game.game_rules.keep_inventory = true;
        game.mode = Mode::Dead;
        game.inventory.crafting[1] = Some(stack(&game, game_core::STONE, 3));
        game.inventory.cursor = Some(stack(&game, game_core::STONE, 2));
        on_death(&mut game);
        close_inventory(&mut game);
        close_inventory(&mut game);
        assert_eq!(total_drops(&game), 0);
        assert_eq!(
            game.inventory.crafting[1],
            Some(stack(&game, game_core::STONE, 3))
        );
        assert_eq!(
            game.inventory.cursor,
            Some(stack(&game, game_core::STONE, 2))
        );
        game.inventory.main[0] = None;
        on_death(&mut game);
        assert!(game.inventory.cursor.is_none());
        assert!(game.inventory.crafting.iter().all(Option::is_none));
        assert_eq!(
            game.inventory.main[0],
            Some(stack(&game, game_core::STONE, 5))
        );
    }

    #[test]
    fn reset_survival_is_empty_and_creative_has_nine_finite_stacks() {
        let mut game = world();
        reset_session(&mut game, false);
        assert!(game.inventory.hotbar.iter().all(Option::is_none));
        assert_eq!(game.hotbar, [0; 9]);
        reset_session(&mut game, true);
        assert!(
            game.inventory
                .hotbar
                .iter()
                .all(|s| s.as_ref().is_some_and(|s| s.count == 64))
        );
        assert_eq!(game.hotbar, [1, 2, 3, 5, 7, 9, 10, 11, 14]);
    }

    #[test]
    fn survival_place_consumes_exactly_one_after_success() {
        let mut game = world();
        game.inventory.hotbar[0] = Some(stack(&game, game_core::DIRT, 2));
        let effects = process_action(
            &mut game,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(effects.sounds.len(), 1);
        assert_eq!(game.world.block([8, 101, 6]), game_core::DIRT);
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 1);
        let revision = game.revision;
        assert!(
            process_action(
                &mut game,
                &InventoryAction::Place {
                    position: [8, 101, 6]
                }
            )
            .sounds
            .is_empty()
        );
        assert_eq!(game.revision, revision);
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 1);
    }

    #[test]
    fn empty_nonblock_unreachable_occluded_unloaded_and_intersecting_place_fail() {
        let mut game = world();
        assert!(
            process_action(
                &mut game,
                &InventoryAction::Place {
                    position: [8, 101, 6]
                }
            )
            .sounds
            .is_empty()
        );
        let egg = game.content.find("survival:egg").unwrap();
        game.inventory.hotbar[0] = Some(ItemStack::new(egg, 1));
        assert!(
            process_action(
                &mut game,
                &InventoryAction::Place {
                    position: [8, 101, 6]
                }
            )
            .sounds
            .is_empty()
        );
        game.inventory.hotbar[0] = Some(stack(&game, game_core::DIRT, 3));
        for position in [[8, 101, -1], [8, 101, 4], [16, 101, 6], [8, 101, 8]] {
            assert!(
                process_action(&mut game, &InventoryAction::Place { position })
                    .sounds
                    .is_empty()
            );
        }
        assert!(game.world.set([8, 101, 7], game_core::STONE));
        game.feet.z = 7.95;
        assert!(
            process_action(
                &mut game,
                &InventoryAction::Place {
                    position: [8, 101, 8]
                }
            )
            .sounds
            .is_empty()
        );
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 3);
    }

    #[test]
    fn early_survival_break_does_not_change_world_or_emit_effects() {
        let mut game = world();
        let before = game.inventory.clone();
        let effects = process_action(
            &mut game,
            &InventoryAction::Break {
                position: [8, 101, 5],
            },
        );
        assert_eq!(game.world.block([8, 101, 5]), game_core::STONE);
        assert!(!effects.mined);
        assert!(effects.sounds.is_empty());
        assert_eq!(total_drops(&game), 0);
        assert_eq!(game.inventory.hotbar, before.hotbar);
        let target = crate::mining_client::target(&game, [8, 101, 5]);
        assert!(game.mining.advance(target, 0.1, 0.1));
        let effects = process_action(
            &mut game,
            &InventoryAction::Break {
                position: [8, 101, 5],
            },
        );
        assert_eq!(game.world.block([8, 101, 5]), game_core::STONE);
        assert!(!effects.mined);
        assert!(effects.sounds.is_empty());
        assert_eq!(total_drops(&game), 0);
    }

    #[test]
    fn duplicated_break_only_drops_and_exhausts_once() {
        let mut game = world();
        equip(&mut game, "survival:wooden_pickaxe");
        assert!(
            !process_action(
                &mut game,
                &InventoryAction::Break {
                    position: [8, 101, 4]
                }
            )
            .mined
        );
        finish_mining(&mut game);
        let effects = process_action(
            &mut game,
            &InventoryAction::Break {
                position: [8, 101, 5],
            },
        );
        assert!(effects.mined);
        assert_eq!(effects.sounds.len(), 1);
        assert_eq!(total_drops(&game), 1);
        assert_eq!(
            game.drops.entities.values().next().unwrap().stack.item,
            game.content.item_for_block(10).unwrap()
        );
        assert!(
            !process_action(
                &mut game,
                &InventoryAction::Break {
                    position: [8, 101, 5]
                }
            )
            .mined
        );
        assert_eq!(total_drops(&game), 1);
    }

    #[test]
    fn golden_pickaxe_breaks_iron_at_table_time_without_a_drop() {
        let mut game = world();
        equip(&mut game, "survival:golden_pickaxe");
        game.world.set([8, 101, 5], game_core::IRON_ORE);
        assert_eq!(crate::mining_client::advance(&mut game, true, 1.29), None);
        assert_eq!(game.world.block([8, 101, 5]), game_core::IRON_ORE);
        assert_eq!(
            crate::mining_client::advance(&mut game, true, 0.02),
            Some([8, 101, 5])
        );
        let effects = process_action(
            &mut game,
            &InventoryAction::Break {
                position: [8, 101, 5],
            },
        );
        assert!(effects.mined);
        assert_eq!(effects.sounds.len(), 1);
        assert_eq!(game.world.block([8, 101, 5]), AIR);
        assert_eq!(total_drops(&game), 0);
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 1);
    }

    #[test]
    fn completion_revalidates_ray_block_tool_slot_and_loaded_chunk() {
        for change in 0..5 {
            let mut game = world();
            equip(&mut game, "survival:wooden_pickaxe");
            finish_mining(&mut game);
            match change {
                0 => game.yaw = 1.,
                1 => {
                    game.world.set([8, 101, 5], game_core::COBBLE);
                }
                2 => equip(&mut game, "survival:diamond_pickaxe"),
                3 => {
                    game.inventory.hotbar[1] = game.inventory.hotbar[0].clone();
                    game.selected = 1;
                }
                _ => game.world.chunks.clear(),
            }
            let effects = process_action(
                &mut game,
                &InventoryAction::Break {
                    position: [8, 101, 5],
                },
            );
            assert!(!effects.mined, "изменение {change}");
            assert!(effects.sounds.is_empty());
            assert_eq!(total_drops(&game), 0);
            assert!(game.mining.target().is_none());
        }
    }

    #[test]
    fn hold_uses_real_seconds_and_resets_for_target_tool_release_and_ui() {
        let mut game = world();
        assert_eq!(crate::mining_client::advance(&mut game, true, 0.5), None);
        let (_, fraction) = crate::mining_client::progress(&game).unwrap();
        assert!((fraction - 0.5 / 7.5).abs() < 0.00001);
        game.world.set([8, 101, 6], game_core::STONE);
        crate::mining_client::advance(&mut game, true, 0.25);
        assert_eq!(game.mining.target().unwrap().position, [8, 101, 6]);
        assert!((crate::mining_client::progress(&game).unwrap().1 - 0.25 / 7.5).abs() < 0.00001);
        equip(&mut game, "survival:wooden_pickaxe");
        crate::mining_client::advance(&mut game, true, 0.25);
        assert!((crate::mining_client::progress(&game).unwrap().1 - 0.25 / 1.15).abs() < 0.00001);
        crate::mining_client::advance(&mut game, false, 0.5);
        assert!(game.mining.target().is_none());
        for mode in [
            Mode::Paused,
            Mode::Inventory,
            Mode::Console,
            Mode::Dead,
            Mode::Menu,
        ] {
            game.screen(Mode::Playing);
            game.world_mouse.update(true, false, false);
            crate::mining_client::advance(&mut game, true, 0.5);
            game.screen(mode);
            assert!(game.mining.target().is_none());
            assert!(crate::mining_client::progress(&game).is_none());
            game.screen(Mode::Playing);
            assert!(!game.world_mouse.update(true, true, false));
            assert!(game.world_mouse.update(true, false, false));
        }
    }

    #[test]
    fn bedrock_is_never_breakable_and_water_is_creative_only() {
        for mode in [GameMode::Survival, GameMode::Creative] {
            for block in [game_core::BEDROCK, game_core::WATER] {
                let mut game = world();
                game.game_mode = mode;
                game.world.set([8, 101, 5], block);
                assert_eq!(crate::mining_client::advance(&mut game, true, 1000.), None);
                let effects = process_action(
                    &mut game,
                    &InventoryAction::Break {
                        position: [8, 101, 5],
                    },
                );
                let allowed = mode == GameMode::Creative && block == game_core::WATER;
                assert_eq!(!effects.sounds.is_empty(), allowed);
                assert_eq!(
                    game.world.block([8, 101, 5]),
                    if allowed { AIR } else { block }
                );
                assert_eq!(total_drops(&game), 0);
                assert!(!effects.mined);
            }
        }
    }

    #[test]
    fn returning_from_inventory_requires_release_before_any_world_mouse_action() {
        let mut game = world();
        game.game_mode = GameMode::Creative;
        game.inventory.hotbar[0] = Some(stack(&game, game_core::DIRT, 2));
        game.screen(Mode::Inventory);
        process_action(&mut game, &InventoryAction::Close);
        assert_eq!(game.mode, Mode::Playing);
        assert!(!game.world_mouse.update(true, true, true));
        for action in [
            InventoryAction::Break {
                position: [8, 101, 5],
            },
            InventoryAction::Place {
                position: [8, 101, 6],
            },
        ] {
            assert!(process_action(&mut game, &action).sounds.is_empty());
        }
        assert!(game.world_mouse.update(true, false, false));
        assert_eq!(
            process_action(
                &mut game,
                &InventoryAction::Break {
                    position: [8, 101, 5]
                }
            )
            .sounds
            .len(),
            1
        );
        reset_session(&mut game, true);
        assert!(!game.world_mouse.update(true, true, false));
        assert!(game.world_mouse.update(true, false, false));
    }

    #[test]
    fn creative_grant_place_and_drop_leave_infinite_source() {
        let mut game = world();
        game.game_mode = GameMode::Creative;
        game.mode = Mode::Inventory;
        let egg = game.content.find("survival:egg").unwrap();
        process_action(&mut game, &InventoryAction::CreativeGrant(egg));
        assert_eq!(game.inventory.hotbar[0], Some(ItemStack::new(egg, 16)));
        let dirt = game.content.item_for_block(game_core::DIRT).unwrap();
        process_action(&mut game, &InventoryAction::CreativeGrant(dirt));
        game.mode = Mode::Playing;
        assert_eq!(
            process_action(
                &mut game,
                &InventoryAction::Place {
                    position: [8, 101, 6]
                }
            )
            .sounds
            .len(),
            1
        );
        process_action(&mut game, &InventoryAction::DropSelected { all: false });
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 64);
        assert_eq!(total_drops(&game), 1);
    }

    #[test]
    fn mode_gates_prevent_background_ui_and_world_actions() {
        let mut game = world();
        game.inventory.hotbar[0] = Some(stack(&game, game_core::DIRT, 2));
        let item = game.inventory.hotbar[0].as_ref().unwrap().item;
        process_action(
            &mut game,
            &InventoryAction::Click {
                slot: SlotRef::Hotbar(0),
                button: ClickButton::Left,
                shift: false,
            },
        );
        assert!(game.inventory.cursor.is_none());
        game.mode = Mode::Inventory;
        process_action(&mut game, &InventoryAction::CreativeGrant(item));
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 2);
        assert!(
            process_action(
                &mut game,
                &InventoryAction::Break {
                    position: [8, 101, 5]
                }
            )
            .sounds
            .is_empty()
        );
        process_action(&mut game, &InventoryAction::DropSelected { all: true });
        assert_eq!(total_drops(&game), 0);
        process_action(
            &mut game,
            &InventoryAction::Click {
                slot: SlotRef::Hotbar(0),
                button: ClickButton::Left,
                shift: false,
            },
        );
        assert_eq!(game.inventory.cursor, Some(ItemStack::new(item, 2)));
    }

    #[test]
    fn chest_contents_spill_once_and_open_requires_visible_loaded_chest() {
        let mut game = world();
        assert!(game.world.set([8, 101, 5], game_core::CHEST));
        process_action(
            &mut game,
            &InventoryAction::OpenContainer {
                position: [8, 101, 4],
            },
        );
        assert_eq!(game.mode, Mode::Playing);
        process_action(
            &mut game,
            &InventoryAction::OpenContainer {
                position: [8, 101, 5],
            },
        );
        assert_eq!(game.mode, Mode::Inventory);
        assert_eq!(game.containers[&[8, 101, 5]].slots.len(), 27);
        let value = stack(&game, game_core::DIRT, 7);
        game.containers.get_mut(&[8, 101, 5]).unwrap().slots[0] = Some(value);
        close_inventory(&mut game);
        game.mode = Mode::Playing;
        finish_mining(&mut game);
        process_action(
            &mut game,
            &InventoryAction::Break {
                position: [8, 101, 5],
            },
        );
        process_action(
            &mut game,
            &InventoryAction::Break {
                position: [8, 101, 5],
            },
        );
        assert!(!game.containers.contains_key(&[8, 101, 5]));
        assert_eq!(total_drops(&game), 8);
    }

    #[test]
    fn death_without_keep_drains_cursor_and_all_equipment_once() {
        let mut game = world();
        game.inventory.hotbar[0] = Some(stack(&game, game_core::DIRT, 3));
        game.inventory.cursor = Some(stack(&game, game_core::DIRT, 2));
        game.inventory.offhand = Some(stack(&game, game_core::DIRT, 1));
        let helmet = game.content.find("survival:leather_helmet").unwrap();
        game.inventory.armor[0] = Some(ItemStack::new(helmet, 1));
        on_death(&mut game);
        on_death(&mut game);
        assert_eq!(total_drops(&game), 7);
        assert!(game.inventory.drain_all().is_empty());
    }

    #[test]
    fn keep_inventory_restores_cursor_or_preserves_it_when_full() {
        let mut game = world();
        game.game_rules.keep_inventory = true;
        let cursor = stack(&game, game_core::STONE, 2);
        game.inventory.cursor = Some(cursor.clone());
        on_death(&mut game);
        assert_eq!(game.inventory.hotbar[0], Some(cursor.clone()));
        assert!(game.inventory.cursor.is_none());
        fill(&mut game);
        game.inventory.cursor = Some(cursor.clone());
        on_death(&mut game);
        assert_eq!(game.inventory.cursor, Some(cursor));
        assert_eq!(total_drops(&game), 0);
    }

    #[test]
    fn close_full_inventory_drops_cursor_overflow_and_clears_chest() {
        let mut game = world();
        fill(&mut game);
        game.inventory.cursor = Some(stack(&game, game_core::STONE, 2));
        game.open_container = Some([8, 101, 5]);
        close_inventory(&mut game);
        close_inventory(&mut game);
        assert_eq!(total_drops(&game), 2);
        assert!(game.inventory.cursor.is_none());
        assert!(game.open_container.is_none());
    }

    #[test]
    fn new_session_clears_cursor_equipment_containers_drops_and_selection() {
        let mut game = world();
        fill(&mut game);
        game.inventory.cursor = Some(stack(&game, game_core::DIRT, 2));
        game.open_container = Some([8, 101, 5]);
        game.containers.insert([8, 101, 5], Container::new(27));
        game.drops.spawn(
            stack(&game, game_core::DIRT, 1),
            [8., 100., 8.],
            [0.; 3],
            &game.drop_config,
        );
        game.selected = 4;
        reset_session(&mut game, false);
        assert!(game.inventory.drain_all().is_empty());
        assert!(game.containers.is_empty());
        assert!(game.drops.entities.is_empty());
        assert!(game.open_container.is_none());
        assert_eq!(game.selected, 0);
    }

    #[test]
    fn catalog_reload_rejects_caps_that_would_invalidate_live_recipes() {
        let mut game = world();
        let source = include_str!("../../../assets/data/items.ron");
        let changed = source.replace("max_stack: 64", "max_stack: 4");
        let diamond = game.content.find("survival:diamond").unwrap();
        assert!(reload_items(&mut game, &changed).is_err());
        assert_eq!(game.content.max_stack(diamond), 64);
        game.inventory.crafting[0] = Some(ItemStack::new(
            game.content.find("survival:diamond_block").unwrap(),
            1,
        ));
        assert_eq!(
            game.recipes
                .preview(&game.inventory.crafting, &game.content),
            Some(ItemStack::new(diamond, 9))
        );
    }

    #[test]
    fn reload_rejects_live_oversized_stacks_in_every_storage_atomically() {
        let original = include_str!("../../../assets/data/items.ron");
        let changed = original.replace("max_stack: 64", "max_stack: 16");
        for location in 0..8 {
            let mut game = world();
            let value = stack(&game, game_core::DIRT, 32);
            let item = value.item;
            match location {
                0 => game.inventory.hotbar[0] = Some(value),
                1 => game.inventory.main[0] = Some(value),
                2 => game.inventory.armor[0] = Some(value),
                3 => game.inventory.offhand = Some(value),
                4 => game.inventory.cursor = Some(value),
                6 => game.inventory.crafting[0] = Some(value),
                5 => {
                    let mut chest = Container::new(27);
                    chest.slots[0] = Some(value);
                    game.containers.insert([8, 101, 5], chest);
                }
                _ => {
                    game.drops
                        .spawn(value, [8., 100., 8.], [0.; 3], &game.drop_config);
                }
            }
            assert!(
                reload_items(&mut game, &changed).is_err(),
                "location {location}"
            );
            assert_eq!(game.content.max_stack(item), 64);
        }
        let mut game = world();
        let item = game.content.item_for_block(game_core::DIRT).unwrap();
        assert!(reload_items(&mut game, "[]").is_err());
        reload_items(&mut game, &changed).unwrap();
        assert_eq!(game.content.max_stack(item), 16);
        assert!(game.ui_dirty);
    }

    #[test]
    fn messages_apply_world_effect_sound_and_exhaustion_exactly_once() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(world())
            .init_resource::<crate::console::ConsoleState>()
            .add_message::<BlockSound>()
            .add_message::<crate::survival_client::PlayerActionMessage>()
            .add_message::<crate::survival_client::SurvivalStateChanged>();
        plugin(&mut app);
        app.add_systems(Update, crate::survival_client::apply_actions);
        {
            let mut game = app.world_mut().resource_mut::<Game>();
            equip(&mut game, "survival:wooden_pickaxe");
            finish_mining(&mut game);
        }
        let message = InventoryActionMessage(InventoryAction::Break {
            position: [8, 101, 5],
        });
        app.world_mut().write_message(message.clone());
        app.world_mut().write_message(message);
        app.update();
        let game = app.world().resource::<Game>();
        assert_eq!(game.world.block([8, 101, 5]), AIR);
        assert_eq!(total_drops(game), 1);
        assert_eq!(
            game.vitals.exhaustion,
            game.survival_config.mining_exhaustion
        );
        assert_eq!(app.world().resource::<Messages<BlockSound>>().len(), 1);
    }

    #[test]
    fn close_action_restores_cursor_and_only_closes_inventory() {
        let mut game = world();
        process_action(&mut game, &InventoryAction::Close);
        assert_eq!(game.mode, Mode::Playing);
        game.mode = Mode::Inventory;
        game.inventory.cursor = Some(stack(&game, game_core::DIRT, 2));
        process_action(&mut game, &InventoryAction::Close);
        assert_eq!(game.mode, Mode::Playing);
        assert!(game.inventory.cursor.is_none());
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 2);
        assert_eq!(total_drops(&game), 0);
    }

    #[test]
    fn dead_screen_transition_does_not_drop_kept_full_cursor() {
        let mut game = world();
        game.game_rules.keep_inventory = true;
        game.mode = Mode::Inventory;
        fill(&mut game);
        let cursor = stack(&game, game_core::STONE, 2);
        game.inventory.cursor = Some(cursor.clone());
        game.vitals.damage(
            100.,
            game_core::survival::DamageSource::Fall,
            GameMode::Survival,
            &game.survival_config,
        );
        assert!(game.vitals.is_dead());
        on_death(&mut game);
        game.screen(Mode::Dead);
        assert_eq!(game.inventory.cursor, Some(cursor));
        assert_eq!(total_drops(&game), 0);
    }

    #[test]
    fn reload_preserves_ids_and_retries_rejected_live_caps_after_inventory_changes() {
        let mut game = world();
        let item = game.content.item_for_block(game_core::DIRT).unwrap();
        game.inventory.cursor = Some(ItemStack::new(item, 32));
        let source = include_str!("../../../assets/data/items.ron")
            .replace("max_stack: 64", "max_stack: 16");
        let mut state = FileState::default();
        reload_file(&mut state, (None, Ok(source.clone())), |source| {
            reload_items(&mut game, source)
        });
        assert!(!state.accepted);
        assert_eq!(game.content.max_stack(item), 64);
        game.inventory.cursor = None;
        reload_file(&mut state, (None, Ok(source)), |source| {
            reload_items(&mut game, source)
        });
        assert!(state.accepted);
        assert_eq!(game.content.item_for_block(game_core::DIRT), Some(item));
        assert_eq!(game.content.max_stack(item), 16);
        reload_file(
            &mut state,
            (Some(PathBuf::from("missing")), Err("missing file".into())),
            |_| panic!("нечитаемый файл не применяется"),
        );
        assert!(!state.accepted);
        assert_eq!(game.content.max_stack(item), 16);
    }

    #[test]
    fn drop_simulation_pauses_outside_playing_and_pickup_refreshes_hotbar() {
        let mut app = App::new();
        let mut game = world();
        let value = stack(&game, game_core::DIRT, 4);
        game.drops
            .spawn(value, [8.5, 100.6, 8.5], [0.; 3], &game.drop_config);
        let mut time = Time::<Real>::default();
        time.advance_by(std::time::Duration::from_secs(1));
        app.insert_resource(game)
            .insert_resource(time)
            .add_systems(Update, tick_drops);
        for mode in [Mode::Inventory, Mode::Console, Mode::Dead, Mode::Paused] {
            app.world_mut().resource_mut::<Game>().mode = mode;
            app.update();
            let game = app.world().resource::<Game>();
            assert_eq!(game.drops.entities.values().next().unwrap().age, 0.);
            assert!(game.inventory.hotbar[0].is_none());
        }
        app.world_mut().resource_mut::<Game>().mode = Mode::Playing;
        app.update();
        let game = app.world().resource::<Game>();
        assert!(game.drops.entities.is_empty());
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 4);
        assert_eq!(game.hotbar[0], game_core::DIRT);
    }
}
