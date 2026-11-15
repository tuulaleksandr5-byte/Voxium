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
    /// Творческое меню, ЛКМ: взять стопку на курсор или удалить то, что на курсоре.
    CreativeGrant(ItemId),
    /// Творческое меню, ПКМ: взять по одному.
    CreativeGrantOne(ItemId),
    /// Творческое меню, Shift: полная стопка прямо в инвентарь.
    CreativeShift(ItemId),
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
    /// ПКМ по кровати: точка возрождения; ночью — пропустить её.
    Sleep {
        position: [i32; 3],
    },
    /// ПКМ по двери, люку или рычагу: открыть, закрыть, переключить.
    Toggle {
        position: [i32; 3],
    },
    /// ПКМ мотыгой по траве или земле: вспахать грядку.
    Till {
        position: [i32; 3],
    },
    /// ПКМ по кнопке: сигнал на секунду.
    Press {
        position: [i32; 3],
    },
    /// ПКМ растением по пустому цветочному горшку.
    Pot {
        position: [i32; 3],
    },
    /// Особый предмет в руке: огниво, костная мука, яйцо призыва.
    UseItem {
        target: [i32; 3],
        previous: [i32; 3],
    },
    /// ПКМ по повторителю: задержка 1…4 шага.
    Delay {
        position: [i32; 3],
    },
    /// ПКМ ведром по котлу: наполнить или опустошить.
    Cauldron {
        position: [i32; 3],
    },
    /// ПКМ топором по бревну или древесине: снять кору.
    Strip {
        position: [i32; 3],
    },
    /// Ведро в руке: набрать источник или вылить жидкость.
    UseBucket {
        target: [i32; 3],
        previous: [i32; 3],
    },
}

/// Какое ведро выбрано в хотбаре.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bucket {
    Empty,
    Water,
    Lava,
}

impl Bucket {
    fn item(self) -> &'static str {
        match self {
            Self::Empty => "fluid:bucket",
            Self::Water => "fluid:water_bucket",
            Self::Lava => "fluid:lava_bucket",
        }
    }
}

/// Ведро в выбранном слоте, если оно там.
pub(crate) fn bucket_kind(game: &Game) -> Option<Bucket> {
    let id = &game
        .content
        .get(game.item_for_slot(game.selected)?.item)?
        .id;
    [Bucket::Empty, Bucket::Water, Bucket::Lava]
        .into_iter()
        .find(|bucket| bucket.item() == id)
}

/// Блок, который видит прицел: жидкости прозрачны, пока в руке нет ведра.
pub(crate) fn target_block(game: &Game, position: [i32; 3]) -> game_core::BlockId {
    let block = loaded_block(&game.world, position).unwrap_or(BEDROCK);
    if game_core::is_fluid(block) && bucket_kind(game).is_none() {
        AIR
    } else {
        block
    }
}

/// Ночь: Солнце под горизонтом.
pub(crate) fn is_night(clock: f64) -> bool {
    ((clock / 1200.0) * std::f64::consts::TAU).sin() < 0.05
}

/// Раздатчик или выбрасыватель стреляет: берёт предмет из первой занятой ячейки.
/// Раздатчик ставит блок в свободную клетку перед собой, иначе (и выбрасыватель)
/// выбрасывает предмет.
pub(crate) fn dispense(game: &mut Game, position: [i32; 3], dir: [i32; 3], dropper: bool) {
    let key = position;
    let Some(container) = game.containers.get_mut(&key) else {
        return;
    };
    let Some(slot) = container.slots.iter().position(Option::is_some) else {
        return;
    };
    let stack = container.slots[slot].as_mut().expect("занято");
    let item = stack.item;
    stack.count -= 1;
    if stack.count == 0 {
        container.slots[slot] = None;
    }
    let front = [
        position[0] + dir[0],
        position[1] + dir[1],
        position[2] + dir[2],
    ];
    if !dropper
        && let Some(block) = game.content.block_for_item(item)
        && loaded_block(&game.world, front) == Some(AIR)
        && game.world.set(front, block)
    {
        game.revision += 1;
        return;
    }
    let center =
        block_center(position) + Vec3::new(dir[0] as f32, dir[1] as f32, dir[2] as f32) * 0.8;
    let velocity = Vec3::new(dir[0] as f32, dir[1] as f32 + 0.3, dir[2] as f32) * 3.0;
    spawn_drop(game, ItemStack::new(item, 1), center, velocity);
}

/// Предмет в руке, у которого есть особое действие по блоку.
pub(crate) fn special_item(game: &Game) -> Option<String> {
    let id = game
        .content
        .get(game.item_for_slot(game.selected)?.item)?
        .id
        .clone();
    (id == "voxium:flint_and_steel" || id == "voxium:bone_meal" || id.ends_with("_spawn_egg"))
        .then_some(id)
}

/// Заменить одно ведро в выбранном слоте другим; лишние пустые ведра остаются в стопке.
fn swap_bucket(game: &mut Game, to: Bucket) {
    let Some(item) = game.content.find(to.item()) else {
        return;
    };
    let slot = game.selected;
    let remaining = match game.inventory.hotbar[slot].as_mut() {
        Some(stack) if stack.count > 1 => {
            stack.count -= 1;
            true
        }
        _ => false,
    };
    let filled = ItemStack::new(item, 1);
    if remaining {
        if let Some(overflow) = game.inventory.insert(filled, &game.content) {
            spawn_drop(game, overflow, game.feet + Vec3::Y * 0.6, Vec3::Y * 1.5);
        }
    } else {
        game.inventory.hotbar[slot] = Some(filled);
    }
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
            tick_furnaces
                .after(crate::survival_client::simulate)
                .before(crate::player::sky),
        )
        .add_systems(
            Update,
            tick_drops
                .after(crate::survival_client::simulate)
                .before(crate::player::sky),
        );
}

pub fn reset_session(game: &mut Game, creative: bool) {
    game.mining.reset();
    game.world_mouse.disarm();
    game.inventory = Inventory::default();
    game.containers.clear();
    game.furnaces.clear();
    game.mobs.clear();
    game.attack_lock = false;
    game.open_container = None;
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
    game.open_container = None;
    sync_hotbar(game);
    game.ui_dirty = true;
}

pub fn on_death(game: &mut Game) {
    game.mining.reset();
    game.world_mouse.disarm();
    if game.game_rules.keep_inventory {
        game.inventory.cursor = game.inventory.close_cursor(&game.content);
        game.inventory.crafting = game.inventory.close_crafting(&game.content);
    } else {
        for stack in game.inventory.drain_all() {
            spawn_drop(game, stack, game.feet + Vec3::Y * 0.6, Vec3::Y * 1.5);
        }
    }
    game.open_container = None;
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
            let changed = if open_block(game) == Some(CRAFTING_TABLE) {
                // Верстак хранит сетку 3×3 в своём контейнере.
                let position = game.open_container.expect("верстак открыт");
                let Some(bench) = game.containers.get_mut(&position) else {
                    return effects;
                };
                game.recipes.take_grid_result(
                    &mut bench.slots,
                    3,
                    &mut game.inventory,
                    *all,
                    &game.content,
                )
            } else {
                game.recipes
                    .take_result(&mut game.inventory, *all, &game.content)
            };
            if changed {
                game.ui_dirty = true;
            }
        }
        InventoryAction::CreativeGrant(item)
            if game.mode == Mode::Inventory && game.game_mode == GameMode::Creative =>
        {
            let count = game.content.max_stack(*item);
            if game.inventory.cursor.is_some() {
                // Положить предмет обратно в каталог — значит удалить его.
                game.inventory.cursor = None;
                game.ui_dirty = true;
            } else if count > 0 {
                game.inventory.cursor = Some(ItemStack::new(*item, count));
                game.ui_dirty = true;
            }
        }
        InventoryAction::CreativeGrantOne(item)
            if game.mode == Mode::Inventory && game.game_mode == GameMode::Creative =>
        {
            let max = game.content.max_stack(*item);
            let next = match game.inventory.cursor.take() {
                Some(mut held) if held.item == *item && held.count < max => {
                    held.count += 1;
                    Some(held)
                }
                Some(held) => Some(held),
                None if max > 0 => Some(ItemStack::new(*item, 1)),
                None => None,
            };
            game.inventory.cursor = next;
            game.ui_dirty = true;
        }
        InventoryAction::CreativeShift(item)
            if game.mode == Mode::Inventory && game.game_mode == GameMode::Creative =>
        {
            let count = game.content.max_stack(*item);
            if count > 0 {
                let rest = game
                    .inventory
                    .insert(ItemStack::new(*item, count), &game.content);
                if let Some(left) = rest {
                    game.status = format!("Нет места: осталось {} шт.", left.count);
                }
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
                // Дверь ломается целиком.
                if let Some(half) = game_core::models::door_other_half(block) {
                    let up = game_core::models::is_door_upper(half);
                    let cell = [
                        position[0],
                        position[1] + if up { 1 } else { -1 },
                        position[2],
                    ];
                    if game.world.block(cell) == half {
                        game.world.set(cell, AIR);
                    }
                }
                let center = block_center(*position);
                game.furnaces.remove(position);
                if let Some(container) = game.containers.remove(position) {
                    for stack in container.slots.into_iter().flatten() {
                        spawn_drop(game, stack, center, Vec3::Y * 1.5);
                    }
                }
                if game.game_mode == GameMode::Survival {
                    if can_harvest && let Some(item) = game.content.drop_for_block(block) {
                        // Глина рассыпается на четыре комка, остальные блоки дают один предмет.
                        let count = if block == game_core::CLAY { 4 } else { 1 };
                        spawn_drop(game, ItemStack::new(item, count), center, Vec3::Y * 1.5);
                    }
                    // Листва иногда роняет саженец своей породы.
                    if let Some(sapling) = game_core::growth::sapling_for_leaves(block)
                        && game_core::growth::chance(game.world.seed, *position, 11, 7)
                        && let Some(item) = game.content.drop_for_block(sapling)
                    {
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
            let mut block = game.selected_block();
            let hit = visible_hit(game);
            // Факел, поставленный на боковую грань, становится настенным.
            let outward = hit.as_ref().and_then(|hit| {
                (hit.previous[1] == hit.block[1] && hit.previous != hit.block)
                    .then(|| {
                        game_core::Facing::from_offset(std::array::from_fn(|axis| {
                            hit.previous[axis] - hit.block[axis]
                        }))
                    })
                    .flatten()
            });
            let wall_torch = match block {
                game_core::TORCH => Some(game_core::WALL_TORCH),
                game_core::REDSTONE_TORCH => Some(game_core::REDSTONE_WALL_TORCH),
                game_core::SOUL_TORCH => Some(game_core::SOUL_WALL_TORCH),
                _ => None,
            };
            if let (Some(wall), Some(facing)) = (wall_torch, outward) {
                block = game_core::facing_variant(wall, facing);
            }
            block = oriented_block(game, block, *position, outward);
            let door_top = [position[0], position[1] + 1, position[2]];
            if game_core::models::is_door(block)
                && !loaded_block(&game.world, door_top)
                    .is_some_and(|above| above == AIR || game_core::is_plant(above))
            {
                return effects;
            }
            if block != AIR
                && hit.as_ref().is_some_and(|hit| {
                    hit.previous == *position
                        || (hit.block == *position
                            && game_core::is_plant(game.world.block(hit.block)))
                })
                && loaded_block(&game.world, *position).is_some_and(|old| {
                    old == AIR || game_core::is_plant(old) || game_core::is_fluid(old)
                })
                && !player_intersects_block(game.feet.to_array(), game.height, *position)
                && game.world.set(*position, block)
            {
                if let Some(upper) = game_core::models::door_other_half(block) {
                    game.world.set(door_top, upper);
                }
                if game.game_mode == GameMode::Survival {
                    game.inventory.take_selected(game.selected, false);
                }
                if let Some(size) = container_size(block) {
                    game.containers
                        .entry(container_key(block, *position))
                        .or_insert_with(|| Container::new(size));
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
        InventoryAction::Sleep { position }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            if !visible_hit(game).is_some_and(|hit| hit.block == *position)
                || !game_core::is_bed(game.world.block(*position))
            {
                return effects;
            }
            game.spawn = block_center(*position) + Vec3::Y * 0.6;
            game.status = if is_night(game.clock) {
                // Сутки длятся 1200 с; новый день начинается на границе суток.
                game.clock = ((game.clock / 1200.0).floor() + 1.0) * 1200.0 + 30.0;
                "Вы проспали ночь. Точка возрождения установлена".into()
            } else {
                "Точка возрождения установлена. Спать можно только ночью".into()
            };
            game.ui_dirty = true;
        }
        InventoryAction::Press { position }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            let block = game.world.block(*position);
            if game_core::models::shape_of(block) != Some(game_core::models::Shape::Button) {
                return effects;
            }
            game.world.rs.press(*position);
            effects.sounds.push(BlockSound {
                block,
                kind: crate::block_audio::SoundKind::Place,
                position: block_center(*position),
            });
            game.revision += 1;
        }
        InventoryAction::UseItem { target, previous }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            if !visible_hit(game)
                .is_some_and(|hit| hit.block == *target && hit.previous == *previous)
            {
                return effects;
            }
            let Some(id) = special_item(game) else {
                return effects;
            };
            let used = match id.as_str() {
                "voxium:flint_and_steel" => {
                    let block = game.world.block(*target);
                    if block == game_core::TNT {
                        game.world.blasts.push(*target);
                        true
                    } else if loaded_block(&game.world, *previous) == Some(AIR) {
                        game.world.set(*previous, game_core::FIRE)
                    } else {
                        false
                    }
                }
                "voxium:bone_meal" => game_core::growth::grow_now(&mut game.world, *target),
                other if other.ends_with("_spawn_egg") => {
                    let name = other
                        .trim_start_matches("voxium:")
                        .trim_end_matches("_spawn_egg");
                    let wanted = if name == "zombie" { "ghoul" } else { name };
                    if let Some(kind) = game_core::mobs::MobKind::ALL
                        .into_iter()
                        .find(|kind| kind.spec().id == wanted)
                    {
                        let at = block_center(*previous);
                        game.mobs.spawn(kind, at.to_array());
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
            if used {
                // Огниво не расходуется; костная мука и яйцо — по одной штуке.
                if game.game_mode == GameMode::Survival && id != "voxium:flint_and_steel" {
                    game.inventory.take_selected(game.selected, false);
                }
                game.revision += 1;
                game.ui_dirty = true;
            }
        }
        InventoryAction::Delay { position }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            if game_core::models::shape_of(game.world.block(*position))
                != Some(game_core::models::Shape::Repeater)
            {
                return effects;
            }
            let delay = game.world.rs.delay.entry(*position).or_insert(1);
            *delay = *delay % 4 + 1;
            game.status = format!("Задержка повторителя: {} шаг.", *delay);
            game.ui_dirty = true;
        }
        InventoryAction::Pot { position }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            let plant = game.selected_block();
            if game.world.block(*position) != game_core::FLOWER_POT {
                return effects;
            }
            let Some(potted) = game_core::mechanics::potted_for(plant) else {
                return effects;
            };
            if game.world.set(*position, potted) {
                if game.game_mode == GameMode::Survival {
                    game.inventory.take_selected(game.selected, false);
                }
                game.revision += 1;
                game.ui_dirty = true;
            }
        }
        InventoryAction::Cauldron { position }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            let block = game.world.block(*position);
            let Some(bucket) = bucket_kind(game) else {
                return effects;
            };
            let next = match (bucket, block) {
                (Bucket::Water, game_core::CAULDRON) => {
                    Some((game_core::WATER_CAULDRON, Bucket::Empty))
                }
                (Bucket::Lava, game_core::CAULDRON) => {
                    Some((game_core::LAVA_CAULDRON, Bucket::Empty))
                }
                (Bucket::Empty, game_core::WATER_CAULDRON) => {
                    Some((game_core::CAULDRON, Bucket::Water))
                }
                (Bucket::Empty, game_core::LAVA_CAULDRON) => {
                    Some((game_core::CAULDRON, Bucket::Lava))
                }
                _ => None,
            };
            if let Some((cauldron, to)) = next
                && game.world.set(*position, cauldron)
            {
                swap_bucket(game, to);
                effects.sounds.push(BlockSound {
                    block: game_core::WATER,
                    kind: crate::block_audio::SoundKind::Place,
                    position: block_center(*position),
                });
                game.revision += 1;
                game.ui_dirty = true;
            }
        }
        InventoryAction::Till { position }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            let block = game.world.block(*position);
            let above = [position[0], position[1] + 1, position[2]];
            if matches!(block, game_core::GRASS | game_core::DIRT)
                && matches!(
                    crate::mining_client::chosen_tool(game),
                    game_core::mining::Tool::Equipped {
                        kind: game_core::mining::ToolKind::Hoe,
                        ..
                    }
                )
                && loaded_block(&game.world, above) == Some(AIR)
                && visible_hit(game).is_some_and(|hit| hit.block == *position)
                && game.world.set(*position, game_core::FARMLAND)
            {
                effects.sounds.push(BlockSound {
                    block,
                    kind: crate::block_audio::SoundKind::Break,
                    position: block_center(*position),
                });
                game.revision += 1;
            }
        }
        InventoryAction::Toggle { position }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            let block = game.world.block(*position);
            let (sin, cos) = (game.yaw.sin(), game.yaw.cos());
            let toward = if sin.abs() > cos.abs() {
                if sin > 0.0 {
                    game_core::Facing::East
                } else {
                    game_core::Facing::West
                }
            } else if cos > 0.0 {
                game_core::Facing::South
            } else {
                game_core::Facing::North
            };
            let Some(next) = game_core::models::toggled(block, toward) else {
                return effects;
            };
            // Дверь меняется обеими половинами сразу (если вторая на месте).
            let other = game_core::models::door_other_half(block).and_then(|half| {
                let up = game_core::models::is_door_upper(half);
                let cell = [
                    position[0],
                    position[1] + if up { 1 } else { -1 },
                    position[2],
                ];
                (game.world.block(cell) == half)
                    .then(|| (cell, game_core::models::toggled(half, toward)))
            });
            game.world.set(*position, next);
            if let Some((cell, Some(next))) = other {
                game.world.set(cell, next);
            }
            effects.sounds.push(BlockSound {
                block,
                kind: crate::block_audio::SoundKind::Place,
                position: block_center(*position),
            });
            game.revision += 1;
        }
        InventoryAction::Strip { position }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            let block = game.world.block(*position);
            let Some(stripped) = game_core::stripped_variant(block) else {
                return effects;
            };
            if matches!(
                crate::mining_client::chosen_tool(game),
                game_core::mining::Tool::Equipped {
                    kind: game_core::mining::ToolKind::Axe,
                    ..
                }
            ) && visible_hit(game).is_some_and(|hit| hit.block == *position)
                && game.world.set(*position, stripped)
            {
                effects.sounds.push(BlockSound {
                    block,
                    kind: crate::block_audio::SoundKind::Break,
                    position: block_center(*position),
                });
                game.revision += 1;
                game.ui_dirty = true;
            }
        }
        InventoryAction::UseBucket { target, previous }
            if game.mode == Mode::Playing && game.world_mouse.allowed() =>
        {
            let Some(bucket) = bucket_kind(game) else {
                return effects;
            };
            if !visible_hit(game)
                .is_some_and(|hit| hit.block == *target && hit.previous == *previous)
            {
                return effects;
            }
            let block = game.world.block(*target);
            match bucket {
                Bucket::Empty => {
                    let filled = match block {
                        game_core::WATER => Some(Bucket::Water),
                        game_core::LAVA => Some(Bucket::Lava),
                        _ => None,
                    };
                    if let Some(filled) = filled
                        && game.world.set(*target, AIR)
                    {
                        swap_bucket(game, filled);
                        effects.sounds.push(BlockSound {
                            block: game_core::WATER,
                            kind: crate::block_audio::SoundKind::Break,
                            position: block_center(*target),
                        });
                    }
                }
                Bucket::Water | Bucket::Lava => {
                    // Жидкость выливается в клетку с течением или растением, а
                    // у твёрдой стены — перед ней.
                    let position = if game_core::is_fluid(block) || game_core::is_plant(block) {
                        *target
                    } else {
                        *previous
                    };
                    let old = loaded_block(&game.world, position);
                    let free = old.is_some_and(|old| {
                        old == AIR
                            || game_core::is_plant(old)
                            || (game_core::is_fluid(old)
                                && old != game_core::WATER
                                && old != game_core::LAVA)
                    });
                    let source = if bucket == Bucket::Water {
                        game_core::WATER
                    } else {
                        game_core::LAVA
                    };
                    if free && game.world.set(position, source) {
                        swap_bucket(game, Bucket::Empty);
                        effects.sounds.push(BlockSound {
                            block: game_core::WATER,
                            kind: crate::block_audio::SoundKind::Place,
                            position: block_center(position),
                        });
                    }
                }
            }
            game.revision += 1;
            game.ui_dirty = true;
        }
        InventoryAction::OpenContainer { position }
            if game.mode == Mode::Playing
                && game.world_mouse.allowed()
                && loaded_block(&game.world, *position)
                    .and_then(container_size)
                    .is_some()
                && visible_hit(game).is_some_and(|hit| hit.block == *position) =>
        {
            let block = game.world.block(*position);
            let size = container_size(block).expect("проверено выше");
            let key = container_key(block, *position);
            let container = game
                .containers
                .entry(key)
                .or_insert_with(|| Container::new(size));
            // Блок на месте старого контейнера мог смениться: приводим размер.
            if container.slots.len() != size {
                *container = Container::new(size);
            }
            game.open_container = Some(key);
            game.screen(Mode::Inventory);
        }
        _ => {}
    }
    sync_hotbar(game);
    effects
}

/// Блок с учётом поворота: механизмы смотрят «лицом» на игрока, рельсы ложатся вдоль
/// соседних рельсов или взгляда.
pub(crate) fn oriented_block(
    game: &Game,
    block: game_core::BlockId,
    position: [i32; 3],
    outward: Option<game_core::Facing>,
) -> game_core::BlockId {
    use game_core::{Facing, FamilyKind, models::Shape};
    let base = game_core::base_block(block);
    let toward = {
        let (sin, cos) = (game.yaw.sin(), game.yaw.cos());
        if sin.abs() > cos.abs() {
            if sin > 0.0 {
                Facing::East
            } else {
                Facing::West
            }
        } else if cos > 0.0 {
            Facing::South
        } else {
            Facing::North
        }
    };
    if base >= game_core::PART2_FIRST {
        let shape = game_core::models::shape_of(base);
        // Табличка на боковой грани становится настенной.
        if let Some(face) = outward.filter(|_| shape == Some(Shape::Sign)) {
            let wall = if base >= game_core::CRIMSON_SIGN {
                game_core::CRIMSON_WALL_SIGN + (base - game_core::CRIMSON_SIGN)
            } else {
                game_core::OAK_WALL_SIGN + (base - game_core::OAK_SIGN)
            };
            return game_core::facing_variant(wall, face);
        }
        if let Some(face) = outward.filter(|_| shape == Some(Shape::Banner)) {
            let color = (base - game_core::WHITE_BANNER) / 4;
            return game_core::facing_variant(game_core::WHITE_WALL_BANNER + color * 4, face);
        }
        if let Some(face) = outward.filter(|_| shape == Some(Shape::Head)) {
            return game_core::facing_variant(base + 1, face);
        }
        if outward.is_some() && shape == Some(Shape::Hanging) {
            let wood = (base - game_core::OAK_HANGING_SIGN) / 2;
            let along_x = matches!(outward, Some(Facing::East | Facing::West));
            return game_core::OAK_WALL_HANGING_SIGN
                + wood * 2
                + game_core::BlockId::from(!along_x);
        }
        match game_core::family_of(base) {
            Some((first, _, FamilyKind::H4)) => {
                let wall_attached = matches!(
                    shape,
                    Some(
                        Shape::Ladder
                            | Shape::WallSign
                            | Shape::Button
                            | Shape::Hook
                            | Shape::WallHead
                            | Shape::WallBanner
                            | Shape::WallFan
                    )
                );
                let face = if wall_attached {
                    outward.unwrap_or(toward)
                } else {
                    toward
                };
                return game_core::facing_variant(first, face);
            }
            Some((first, _, FamilyKind::Lever)) => {
                return game_core::facing_variant_index(first, outward.unwrap_or(toward));
            }
            Some((first, _, FamilyKind::AX2)) => {
                let looking_along_x = matches!(toward, Facing::East | Facing::West);
                return first + game_core::BlockId::from(looking_along_x);
            }
            Some((first, _, FamilyKind::Door | FamilyKind::Gate)) => {
                return game_core::facing_variant_index(first, toward);
            }
            _ => {}
        }
    }
    if game_core::is_rail(base) {
        let rail_at = |dx: i32, dz: i32| {
            loaded_block(
                &game.world,
                [position[0] + dx, position[1], position[2] + dz],
            )
            .filter(|id| game_core::is_rail(*id))
        };
        let along_x = if let Some(id) = rail_at(1, 0).or_else(|| rail_at(-1, 0)) {
            let _ = id;
            true
        } else if rail_at(0, 1).or_else(|| rail_at(0, -1)).is_some() {
            false
        } else {
            game.yaw.sin().abs() > game.yaw.cos().abs()
        };
        return game_core::rail_variant(base, along_x);
    }
    if [
        game_core::DISPENSER,
        game_core::DROPPER,
        game_core::OBSERVER,
        game_core::PISTON,
        game_core::STICKY_PISTON,
    ]
    .contains(&base)
    {
        // Вектор взгляда: лицом блок должен смотреть на игрока, то есть против взгляда.
        let look = [
            -game.yaw.sin() * game.pitch.cos(),
            game.pitch.sin(),
            -game.yaw.cos() * game.pitch.cos(),
        ];
        let axis = (0..3)
            .max_by(|a, b| look[*a].abs().total_cmp(&look[*b].abs()))
            .unwrap_or(2);
        return game_core::facing_variant(base, Facing::from_axis(axis, look[axis] < 0.0));
    }
    block
}

/// Число ячеек блока-контейнера: сундук 27, верстак 3×3, печь 3.
pub(crate) fn container_size(block: game_core::BlockId) -> Option<usize> {
    match block {
        CHEST | game_core::BARREL | game_core::VAULT => Some(27),
        CRAFTING_TABLE => Some(9),
        game_core::ENDER_CHEST | game_core::TRAPPED_CHEST => Some(27),
        block
            if matches!(
                game_core::base_block(block),
                game_core::DISPENSER | game_core::DROPPER
            ) =>
        {
            Some(9)
        }
        block if game_core::is_shulker(block) => Some(27),
        game_core::STONECUTTER => Some(1),
        _ => game_core::smelting::FurnaceKind::of_block(block)
            .map(|_| game_core::smelting::FURNACE_SLOTS),
    }
}

/// Ключ общего хранилища: все общие сундуки открывают один контейнер.
pub(crate) const SHARED_VAULT: [i32; 3] = [0, i32::MIN, 0];

/// Где лежит контейнер блока: у общего сундука — в общем хранилище.
fn container_key(block: game_core::BlockId, position: [i32; 3]) -> [i32; 3] {
    if block == game_core::ENDER_CHEST {
        SHARED_VAULT
    } else {
        position
    }
}

/// Камнерез: превратить один предмет из его ячейки в выбранный вариант.
pub(crate) fn stonecut(game: &mut Game, output: ItemId) -> bool {
    if game.mode != Mode::Inventory || open_block(game) != Some(game_core::STONECUTTER) {
        return false;
    }
    let Some(position) = game.open_container else {
        return false;
    };
    let Some(input) = game
        .containers
        .get(&position)
        .and_then(|cutter| cutter.slots.first().cloned().flatten())
    else {
        return false;
    };
    if !game_core::stonecutting::options(input.item, &game.content).contains(&output) {
        return false;
    }
    // Результат идёт на курсор; несовместимый или полный курсор отменяет резку.
    match &mut game.inventory.cursor {
        None => game.inventory.cursor = Some(ItemStack::new(output, 1)),
        Some(cursor) if cursor.item == output && cursor.count < game.content.max_stack(output) => {
            cursor.count += 1;
        }
        Some(_) => return false,
    }
    let slot = &mut game
        .containers
        .get_mut(&position)
        .expect("проверено выше")
        .slots[0];
    *slot = (input.count > 1).then(|| ItemStack::new(input.item, input.count - 1));
    game.ui_dirty = true;
    true
}

/// Блок открытого сейчас контейнера.
pub(crate) fn open_block(game: &Game) -> Option<game_core::BlockId> {
    let position = game.open_container?;
    if position == SHARED_VAULT {
        return Some(game_core::ENDER_CHEST);
    }
    loaded_block(&game.world, position)
        .filter(|block| container_size(*block).is_some())
        .map(game_core::base_block)
        .or_else(|| {
            // Чанк мог выгрузиться: вид контейнера определяется числом ячеек.
            match game.containers.get(&position)?.slots.len() {
                27 => Some(CHEST),
                9 => Some(CRAFTING_TABLE),
                1 => Some(game_core::STONECUTTER),
                game_core::smelting::FURNACE_SLOTS => Some(FURNACE),
                _ => None,
            }
        })
}

pub(crate) fn look_direction(game: &Game) -> Vec3 {
    Quat::from_rotation_y(game.yaw) * Quat::from_rotation_x(game.pitch) * Vec3::NEG_Z
}

fn block_center(position: [i32; 3]) -> Vec3 {
    Vec3::from_array(position.map(|value| value as f32)) + Vec3::splat(0.5)
}

pub(crate) fn loaded_block(world: &World, position: [i32; 3]) -> Option<game_core::BlockId> {
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
        |position| target_block(game, position),
    )?;
    loaded_block(&game.world, hit.block).map(|_| hit)
}

pub(crate) fn spawn_drop(game: &mut Game, stack: ItemStack, position: Vec3, velocity: Vec3) {
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

/// Печи работают, пока мир загружен: и во время игры, и при открытом инвентаре.
pub fn tick_furnaces(time: Res<Time<Real>>, mut game: ResMut<Game>) {
    if game.world_id.is_none() || !matches!(game.mode, Mode::Playing | Mode::Inventory) {
        return;
    }
    let game = &mut *game;
    let positions: Vec<[i32; 3]> = game
        .containers
        .iter()
        .filter(|(_, container)| container.slots.len() == game_core::smelting::FURNACE_SLOTS)
        .map(|(position, _)| *position)
        .collect();
    for position in positions {
        let Some(block) = loaded_block(&game.world, position) else {
            continue;
        };
        let Some((kind, _)) = game_core::smelting::FurnaceKind::of_block(block) else {
            game.furnaces.remove(&position);
            continue;
        };
        let state = game.furnaces.entry(position).or_default();
        let before = (
            state.lit(),
            (state.progress_fraction(&game.smelting) * 20.) as u8,
            (state.fuel_fraction() * 20.) as u8,
        );
        let container = game
            .containers
            .get_mut(&position)
            .expect("позиция из списка");
        let changed = state.tick_kind(
            kind,
            container,
            time.delta_secs(),
            &game.smelting,
            &game.content,
        );
        let after = (
            state.lit(),
            (state.progress_fraction(&game.smelting) * 20.) as u8,
            (state.fuel_fraction() * 20.) as u8,
        );
        if after.0 != before.0 && game.world.set(position, kind.block(after.0)) {
            game.revision += 1;
        }
        if game.open_container == Some(position) && (changed || before != after) {
            game.ui_dirty = true;
        }
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
    fn stack(game: &Game, block: game_core::BlockId, count: u16) -> ItemStack {
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
    fn bedrock_is_never_breakable_and_water_needs_a_bucket_and_creative_mode() {
        for mode in [GameMode::Survival, GameMode::Creative] {
            for (block, bucket) in [
                (game_core::BEDROCK, false),
                (game_core::WATER, false),
                (game_core::WATER, true),
            ] {
                let mut game = world();
                game.game_mode = mode;
                if bucket {
                    equip(&mut game, "fluid:bucket");
                }
                game.world.set([8, 101, 5], block);
                assert_eq!(crate::mining_client::advance(&mut game, true, 1000.), None);
                let effects = process_action(
                    &mut game,
                    &InventoryAction::Break {
                        position: [8, 101, 5],
                    },
                );
                // Вода без ведра невидима для прицела; с ведром её стирает только Creative.
                let allowed = mode == GameMode::Creative && block == game_core::WATER && bucket;
                assert_eq!(
                    !effects.sounds.is_empty(),
                    allowed,
                    "{mode:?} {block} {bucket}"
                );
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
    fn fluids_are_see_through_for_the_crosshair_unless_a_bucket_is_held() {
        let mut game = world();
        game.world.set([8, 101, 7], game_core::WATER);
        game.world.set([8, 101, 6], game_core::FLOW_LAVA_2);
        assert_eq!(target_block(&game, [8, 101, 7]), AIR);
        assert_eq!(target_block(&game, [8, 101, 6]), AIR);
        assert_eq!(target_block(&game, [8, 101, 5]), game_core::STONE);
        let hit = visible_hit(&game).expect("камень виден сквозь воду");
        assert_eq!(hit.block, [8, 101, 5]);
        equip(&mut game, "fluid:bucket");
        assert_eq!(target_block(&game, [8, 101, 7]), game_core::WATER);
        let hit = visible_hit(&game).expect("с ведром виден сам водоём");
        assert_eq!(hit.block, [8, 101, 7]);
        equip(&mut game, "foundation:dirt");
        assert_eq!(bucket_kind(&game), None);
    }

    #[test]
    fn bucket_picks_up_a_source_and_pours_it_back_but_ignores_flowing_water() {
        let mut game = world();
        game.world.set([8, 101, 7], game_core::WATER);
        game.world.set([8, 101, 5], game_core::AIR);
        game.world.set([8, 100, 5], game_core::STONE);
        equip(&mut game, "fluid:bucket");
        game.inventory.hotbar[0].as_mut().unwrap().count = 3;
        // Набрать: источник пропал, в слоте осталось два пустых ведра, полное ушло в рюкзак.
        let take = InventoryAction::UseBucket {
            target: [8, 101, 7],
            previous: [8, 101, 8],
        };
        let hit = visible_hit(&game).unwrap();
        assert_eq!((hit.block, hit.previous), ([8, 101, 7], [8, 101, 8]));
        process_action(&mut game, &take);
        assert_eq!(game.world.block([8, 101, 7]), AIR);
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 2);
        let water_bucket = game.content.find("fluid:water_bucket").unwrap();
        assert_eq!(
            game.inventory.hotbar[1].as_ref().unwrap().item,
            water_bucket
        );
        // Вылить: ведро воды выбрано, у стены появляется источник, ведро снова пустое.
        game.selected = 1;
        game.world.set([8, 101, 4], game_core::STONE);
        let pour = InventoryAction::UseBucket {
            target: [8, 101, 4],
            previous: [8, 101, 5],
        };
        let hit = visible_hit(&game).unwrap();
        assert_eq!((hit.block, hit.previous), ([8, 101, 4], [8, 101, 5]));
        process_action(&mut game, &pour);
        assert_eq!(game.world.block([8, 101, 5]), game_core::WATER);
        let empty = game.content.find("fluid:bucket").unwrap();
        assert_eq!(game.inventory.hotbar[1].as_ref().unwrap().item, empty);
        assert!(game.world.fluid_active.contains(&[8, 101, 5]));
        // Течение набрать нельзя: ведро остаётся пустым, блок на месте.
        game.world.set([8, 101, 6], game_core::FLOW_WATER_3);
        let sip = InventoryAction::UseBucket {
            target: [8, 101, 6],
            previous: [8, 101, 7],
        };
        process_action(&mut game, &sip);
        assert_eq!(game.world.block([8, 101, 6]), game_core::FLOW_WATER_3);
        assert_eq!(game.inventory.hotbar[1].as_ref().unwrap().item, empty);
    }

    #[test]
    fn placing_a_block_into_water_replaces_it_and_buckets_never_empty_into_stone() {
        let mut game = world();
        game.world.set([8, 101, 8], game_core::FLOW_WATER_4);
        game.world.set([8, 100, 8], game_core::STONE);
        equip(&mut game, "foundation:dirt");
        let look = visible_hit(&game).unwrap();
        assert_eq!(look.block, [8, 101, 5]);
        // Лава из ведра не может занять клетку со сплошным блоком.
        equip(&mut game, "fluid:lava_bucket");
        let before = game.world.block([8, 101, 5]);
        process_action(
            &mut game,
            &InventoryAction::UseBucket {
                target: [8, 101, 5],
                previous: [8, 101, 5],
            },
        );
        assert_eq!(game.world.block([8, 101, 5]), before);
        let lava_bucket = game.content.find("fluid:lava_bucket").unwrap();
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().item, lava_bucket);
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
        let click = InventoryAction::Click {
            slot: SlotRef::Hotbar(0),
            button: ClickButton::Left,
            shift: false,
        };
        process_action(&mut game, &InventoryAction::CreativeGrant(egg));
        assert_eq!(game.inventory.cursor, Some(ItemStack::new(egg, 16)));
        process_action(&mut game, &click);
        assert_eq!(game.inventory.hotbar[0], Some(ItemStack::new(egg, 16)));
        let dirt = game.content.item_for_block(game_core::DIRT).unwrap();
        process_action(&mut game, &InventoryAction::CreativeGrant(dirt));
        process_action(&mut game, &click);
        // Яйца ушли на курсор; положить их в каталог значит удалить.
        assert_eq!(game.inventory.cursor, Some(ItemStack::new(egg, 16)));
        process_action(&mut game, &InventoryAction::CreativeGrant(egg));
        assert!(game.inventory.cursor.is_none());
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
    fn creative_one_and_shift_picks_follow_the_vanilla_rules() {
        let mut game = world();
        game.game_mode = GameMode::Creative;
        game.mode = Mode::Inventory;
        let dirt = game.content.item_for_block(game_core::DIRT).unwrap();
        let stone = game.content.item_for_block(game_core::STONE).unwrap();
        // ПКМ берёт по одному и добавляет к тому же предмету.
        process_action(&mut game, &InventoryAction::CreativeGrantOne(dirt));
        process_action(&mut game, &InventoryAction::CreativeGrantOne(dirt));
        assert_eq!(game.inventory.cursor, Some(ItemStack::new(dirt, 2)));
        // Другой предмет на курсоре не заменяется.
        process_action(&mut game, &InventoryAction::CreativeGrantOne(stone));
        assert_eq!(game.inventory.cursor, Some(ItemStack::new(dirt, 2)));
        // Shift кладёт полную стопку в инвентарь, курсор не трогает.
        process_action(&mut game, &InventoryAction::CreativeShift(stone));
        assert_eq!(game.inventory.hotbar[0], Some(ItemStack::new(stone, 64)));
        assert_eq!(game.inventory.cursor, Some(ItemStack::new(dirt, 2)));
        // В выживании каталог недоступен.
        let mut survival = world();
        survival.mode = Mode::Inventory;
        process_action(&mut survival, &InventoryAction::CreativeGrant(dirt));
        process_action(&mut survival, &InventoryAction::CreativeShift(dirt));
        assert!(survival.inventory.cursor.is_none());
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

    #[test]
    fn stonecutter_cuts_one_block_to_cursor_and_rejects_foreign_outputs() {
        let mut game = Game {
            mode: Mode::Inventory,
            open_container: Some([1, 2, 3]),
            ..Default::default()
        };
        let find = |game: &Game, id: &str| game.content.find(id).unwrap();
        let stone = find(&game, "foundation:stone");
        let bricks = find(&game, "builder:stone_bricks");
        let dirt = find(&game, "foundation:dirt");
        let mut cutter = Container::new(1);
        cutter.slots[0] = Some(ItemStack::new(stone, 2));
        game.containers.insert([1, 2, 3], cutter);
        assert_eq!(open_block(&game), Some(game_core::STONECUTTER));
        assert!(!stonecut(&mut game, dirt));
        assert!(stonecut(&mut game, bricks));
        assert!(stonecut(&mut game, bricks));
        assert_eq!(game.inventory.cursor, Some(ItemStack::new(bricks, 2)));
        assert!(game.containers[&[1, 2, 3]].slots[0].is_none());
        assert!(!stonecut(&mut game, bricks), "сырьё закончилось");
    }

    #[test]
    fn every_container_block_has_a_size_and_vaults_share_one_storage() {
        for (block, size) in [
            (CHEST, 27),
            (game_core::BARREL, 27),
            (game_core::VAULT, 27),
            (CRAFTING_TABLE, 9),
            (game_core::STONECUTTER, 1),
            (game_core::DISPENSER, 9),
            (FURNACE, 3),
            (game_core::SMOKER_LIT, 3),
            (game_core::BLAST_FURNACE, 3),
        ] {
            assert_eq!(container_size(block), Some(size), "{block}");
        }
        assert_eq!(container_size(game_core::STONE), None);
        assert_eq!(
            container_key(game_core::ENDER_CHEST, [5, 6, 7]),
            SHARED_VAULT
        );
        assert_eq!(
            container_key(game_core::ENDER_CHEST, [-9, 1, 2]),
            SHARED_VAULT
        );
        assert_eq!(container_key(game_core::BARREL, [5, 6, 7]), [5, 6, 7]);
        let game = Game {
            open_container: Some(SHARED_VAULT),
            ..Default::default()
        };
        assert_eq!(open_block(&game), Some(game_core::ENDER_CHEST));
    }

    #[test]
    fn bed_sets_the_spawn_and_only_skips_the_night_when_it_is_night() {
        let mut game = world();
        game.world.set([8, 101, 7], game_core::RED_BED);
        let sleep = InventoryAction::Sleep {
            position: [8, 101, 7],
        };
        game.clock = 300.0;
        process_action(&mut game, &sleep);
        assert_eq!(game.clock, 300.0, "днём время не меняется");
        assert!((game.spawn - (block_center([8, 101, 7]) + Vec3::Y * 0.6)).length() < 1e-4);
        game.clock = 900.0;
        process_action(&mut game, &sleep);
        assert_eq!(game.clock, 1230.0, "ночь заменена утром");
        assert!(!is_night(game.clock) && is_night(900.0) && !is_night(300.0));
        // Не кровать — ничего не делает.
        game.spawn = Vec3::ZERO;
        process_action(
            &mut game,
            &InventoryAction::Sleep {
                position: [8, 101, 5],
            },
        );
        assert_eq!(game.spawn, Vec3::ZERO);
    }

    #[test]
    fn only_an_axe_strips_bark_and_every_variant_exists_as_a_block() {
        let mut game = world();
        game.world.set([8, 101, 7], game_core::WOOD);
        let strip = InventoryAction::Strip {
            position: [8, 101, 7],
        };
        equip(&mut game, "survival:iron_pickaxe");
        process_action(&mut game, &strip);
        assert_eq!(game.world.block([8, 101, 7]), game_core::WOOD);
        equip(&mut game, "survival:iron_axe");
        process_action(&mut game, &strip);
        assert_eq!(game.world.block([8, 101, 7]), game_core::STRIPPED_OAK_LOG);
        for block in 0..game_core::BLOCKS.len() as game_core::BlockId {
            if let Some(stripped) = game_core::stripped_variant(block) {
                assert!(usize::from(stripped) < game_core::BLOCKS.len());
                assert_ne!(stripped, block);
                assert_eq!(game_core::stripped_variant(stripped), None);
            }
        }
    }

    #[test]
    fn mechanisms_face_the_player_and_rails_follow_neighbours_or_the_view() {
        use game_core::Facing;
        let mut game = world();
        // Взгляд на -Z: «лицо» смотрит на игрока, то есть на юг.
        assert_eq!(
            game_core::facing_of(oriented_block(&game, game_core::PISTON, [8, 101, 6], None)),
            Some(Facing::South)
        );
        game.yaw = std::f32::consts::FRAC_PI_2; // взгляд на -X → лицо на восток
        assert_eq!(
            game_core::facing_of(oriented_block(
                &game,
                game_core::DISPENSER,
                [8, 101, 6],
                None
            )),
            Some(Facing::East)
        );
        game.pitch = -1.4; // взгляд вниз → лицо вверх
        assert_eq!(
            game_core::facing_of(oriented_block(
                &game,
                game_core::STICKY_PISTON,
                [8, 101, 6],
                None
            )),
            Some(Facing::Up)
        );
        // Рельсы: без соседей — по взгляду; рядом рельс вдоль Z — тоже вдоль Z.
        game.pitch = 0.0;
        assert!(game_core::is_rail_x(oriented_block(
            &game,
            game_core::DETECTOR_RAIL,
            [8, 101, 6],
            None
        )));
        game.yaw = 0.0;
        assert!(!game_core::is_rail_x(oriented_block(
            &game,
            game_core::DETECTOR_RAIL,
            [8, 101, 6],
            None
        )));
        game.world.set([9, 101, 6], game_core::POWERED_RAIL);
        assert!(game_core::is_rail_x(oriented_block(
            &game,
            game_core::POWERED_RAIL,
            [8, 101, 6],
            None
        )));
        // Раздатчик любого поворота открывает свой экран на 9 ячеек.
        for facing in Facing::ALL {
            let block = game_core::facing_variant(game_core::DISPENSER, facing);
            assert_eq!(container_size(block), Some(9));
            game.world.set([8, 101, 7], block);
            game.containers.insert([8, 101, 7], Container::new(9));
            game.open_container = Some([8, 101, 7]);
            assert_eq!(open_block(&game), Some(game_core::DISPENSER));
        }
    }

    #[test]
    fn doors_place_two_halves_toggle_together_and_stairs_face_the_player() {
        let mut game = world();
        equip(&mut game, "voxium:oak_door");
        process_action(
            &mut game,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        let lower = game.world.block([8, 101, 6]);
        assert_eq!(
            lower,
            game_core::OAK_DOOR + 1,
            "дверь повёрнута к игроку (юг)"
        );
        assert_eq!(game.world.block([8, 102, 6]), game_core::OAK_DOOR + 9);
        process_action(
            &mut game,
            &InventoryAction::Toggle {
                position: [8, 102, 6],
            },
        );
        assert_eq!(game.world.block([8, 101, 6]), game_core::OAK_DOOR + 5);
        assert_eq!(game.world.block([8, 102, 6]), game_core::OAK_DOOR + 13);
        // Дверь не ставится, если над ней нет места.
        let mut cramped = world();
        cramped.world.set([8, 102, 6], game_core::STONE);
        equip(&mut cramped, "voxium:oak_door");
        process_action(
            &mut cramped,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(cramped.world.block([8, 101, 6]), AIR);
        // Ступеньки смотрят на игрока.
        let mut stairs = world();
        equip(&mut stairs, "voxium:oak_stairs");
        process_action(
            &mut stairs,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(stairs.world.block([8, 101, 6]), game_core::OAK_STAIRS + 1);
    }

    #[test]
    fn gates_toggle_and_heads_on_walls_become_wall_heads() {
        let mut game = world();
        equip(&mut game, "voxium:oak_fence_gate");
        process_action(
            &mut game,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        let gate = game.world.block([8, 101, 6]);
        assert_eq!(gate, game_core::OAK_FENCE_GATE + 1);
        process_action(
            &mut game,
            &InventoryAction::Toggle {
                position: [8, 101, 6],
            },
        );
        assert_eq!(game.world.block([8, 101, 6]), game_core::OAK_FENCE_GATE + 5);
        let mut heads = world();
        equip(&mut heads, "voxium:creeper_head");
        process_action(
            &mut heads,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(
            heads.world.block([8, 101, 6]),
            game_core::CREEPER_WALL_HEAD + 1
        );
    }

    #[test]
    fn droppers_face_the_player_open_a_nine_slot_screen_and_banners_hang_on_walls() {
        let mut game = world();
        equip(&mut game, "voxium:dropper");
        process_action(
            &mut game,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        let block = game.world.block([8, 101, 6]);
        assert_eq!(block, game_core::DROPPER + 1);
        assert_eq!(container_size(block), Some(9));
        assert!(
            game.containers
                .contains_key(&container_key(block, [8, 101, 6]))
        );
        let mut banner = world();
        equip(&mut banner, "voxium:red_banner");
        process_action(
            &mut banner,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(
            banner.world.block([8, 101, 6]),
            game_core::RED_WALL_BANNER + 1
        );
    }

    #[test]
    fn shulker_boxes_hold_27_items_and_new_wooden_doors_and_observers_orient() {
        let mut game = world();
        equip(&mut game, "voxium:red_shulker_box");
        process_action(
            &mut game,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        let block = game.world.block([8, 101, 6]);
        assert_eq!(block, game_core::RED_SHULKER_BOX);
        assert_eq!(container_size(block), Some(27));
        assert!(
            game.containers
                .contains_key(&container_key(block, [8, 101, 6]))
        );
        let mut door = world();
        equip(&mut door, "voxium:bamboo_door");
        process_action(
            &mut door,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(door.world.block([8, 101, 6]), game_core::BAMBOO_DOOR + 1);
        assert_eq!(door.world.block([8, 102, 6]), game_core::BAMBOO_DOOR + 9);
        let mut observer = world();
        equip(&mut observer, "voxium:observer");
        process_action(
            &mut observer,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(observer.world.block([8, 101, 6]), game_core::OBSERVER + 1);
    }

    #[test]
    fn wall_fans_stick_to_walls_and_stairs_of_new_stones_face_the_player() {
        let mut game = world();
        equip(&mut game, "voxium:brain_coral_wall_fan");
        process_action(
            &mut game,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(
            game.world.block([8, 101, 6]),
            game_core::BRAIN_CORAL_WALL_FAN + 1
        );
        let mut stairs = world();
        equip(&mut stairs, "voxium:granite_stairs");
        process_action(
            &mut stairs,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(
            stairs.world.block([8, 101, 6]),
            game_core::GRANITE_STAIRS + 1
        );
    }

    #[test]
    fn nether_signs_become_wall_signs_and_nether_doors_place_two_halves() {
        let mut game = world();
        equip(&mut game, "voxium:warped_sign");
        process_action(
            &mut game,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(
            game.world.block([8, 101, 6]),
            game_core::WARPED_WALL_SIGN + 1
        );
        let mut door = world();
        equip(&mut door, "voxium:crimson_door");
        process_action(
            &mut door,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(door.world.block([8, 101, 6]), game_core::CRIMSON_DOOR + 1);
        assert_eq!(door.world.block([8, 102, 6]), game_core::CRIMSON_DOOR + 9);
    }

    #[test]
    fn lever_button_pots_cauldrons_and_ender_chests_work() {
        // Рычаг рядом с дверью: включил — открылась, выключил — закрылась.
        let mut game = world();
        game.world.set([8, 101, 6], game_core::OAK_DOOR + 1);
        game.world.set([8, 102, 6], game_core::OAK_DOOR + 9);
        game.world.set([8, 101, 7], game_core::LEVER + 1);
        game_core::redstone::tick(&mut game.world);
        let toggle = InventoryAction::Toggle {
            position: [8, 101, 7],
        };
        process_action(&mut game, &toggle);
        assert_eq!(game.world.block([8, 101, 7]), game_core::LEVER + 5);
        game_core::redstone::tick(&mut game.world);
        game_core::redstone::tick(&mut game.world);
        assert_eq!(game.world.block([8, 101, 6]), game_core::OAK_DOOR + 5);
        process_action(&mut game, &toggle);
        game_core::redstone::tick(&mut game.world);
        game_core::redstone::tick(&mut game.world);
        assert_eq!(game.world.block([8, 101, 6]), game_core::OAK_DOOR + 1);
        // Кнопка включает сигнал на 10 шагов.
        game.world.set([8, 101, 8], game_core::STONE_BUTTON);
        process_action(
            &mut game,
            &InventoryAction::Press {
                position: [8, 101, 8],
            },
        );
        assert!(game.world.rs.timers.contains_key(&[8, 101, 8]));
        // Повторитель: ПКМ меняет задержку по кругу 1→2→3→4→1.
        game.world.set([8, 101, 9], game_core::REPEATER);
        for expected in [2, 3, 4, 1] {
            process_action(
                &mut game,
                &InventoryAction::Delay {
                    position: [8, 101, 9],
                },
            );
            assert_eq!(game.world.rs.delay[&[8, 101, 9]], expected);
        }
        // Горшок принимает цветок.
        let mut pot = world();
        pot.world.set([8, 101, 6], game_core::FLOWER_POT);
        equip(&mut pot, "flora:poppy");
        process_action(
            &mut pot,
            &InventoryAction::Pot {
                position: [8, 101, 6],
            },
        );
        assert_eq!(pot.world.block([8, 101, 6]), game_core::POTTED_POPPY);
        // Эндер-сундуки делят одно хранилище, трёхрядный контейнер.
        assert_eq!(container_size(game_core::ENDER_CHEST), Some(27));
        assert_eq!(container_size(game_core::TRAPPED_CHEST), Some(27));
        assert_eq!(
            container_key(game_core::ENDER_CHEST, [1, 2, 3]),
            SHARED_VAULT
        );
        assert_eq!(
            container_key(game_core::TRAPPED_CHEST, [1, 2, 3]),
            [1, 2, 3]
        );
    }

    #[test]
    fn dispensers_place_blocks_and_droppers_throw_items() {
        let mut game = world();
        game.world.set([8, 101, 8], game_core::DISPENSER + 1);
        let dirt = game.content.find("foundation:dirt").unwrap();
        let mut container = Container::new(9);
        container.slots[2] = Some(ItemStack::new(dirt, 2));
        game.containers.insert([8, 101, 8], container);
        dispense(&mut game, [8, 101, 8], [0, 0, 1], false);
        assert_eq!(game.world.block([8, 101, 9]), game_core::DIRT);
        assert_eq!(
            game.containers[&[8, 101, 8]].slots[2]
                .as_ref()
                .unwrap()
                .count,
            1
        );
        // Занятая клетка впереди: предмет улетает на землю.
        let before = game.drops.entities.len();
        dispense(&mut game, [8, 101, 8], [0, 0, 1], false);
        assert_eq!(game.drops.entities.len(), before + 1);
        assert!(game.containers[&[8, 101, 8]].slots[2].is_none());
    }

    #[test]
    fn flint_and_steel_lights_tnt_and_fire_bone_meal_grows_trees_and_eggs_spawn_mobs() {
        let use_item = InventoryAction::UseItem {
            target: [8, 101, 5],
            previous: [8, 101, 6],
        };
        let mut game = world();
        game.world.set([8, 101, 5], game_core::TNT);
        equip(&mut game, "voxium:flint_and_steel");
        process_action(&mut game, &use_item);
        assert_eq!(game.world.blasts, vec![[8, 101, 5]]);
        // Огниво на камне: огонь в клетке перед блоком.
        let mut game = world();
        equip(&mut game, "voxium:flint_and_steel");
        process_action(&mut game, &use_item);
        assert_eq!(game.world.block([8, 101, 6]), game_core::FIRE);
        // Яйцо коровы создаёт моба.
        let mut game = world();
        equip(&mut game, "voxium:cow_spawn_egg");
        let before = format!("{:?}", game.mobs);
        process_action(&mut game, &use_item);
        assert_ne!(format!("{:?}", game.mobs), before);
        // Яйцо блока без моба ничего не делает.
        let mut game = world();
        equip(&mut game, "voxium:vex_spawn_egg");
        let before = format!("{:?}", game.mobs);
        process_action(&mut game, &use_item);
        assert_eq!(format!("{:?}", game.mobs), before);
    }

    #[test]
    fn signs_become_wall_signs_on_walls_and_the_hoe_ploughs_grass() {
        let mut game = world();
        equip(&mut game, "voxium:oak_sign");
        process_action(
            &mut game,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(
            game.world.block([8, 101, 6]),
            game_core::OAK_WALL_SIGN + 1,
            "на стене табличка настенная и смотрит от стены"
        );
        let mut field = world();
        field.world.set([8, 100, 8], game_core::GRASS);
        field.world.set([8, 101, 7], game_core::GRASS);
        assert_eq!(field.world.block([8, 101, 7]), game_core::GRASS);
        // Без мотыги вспашки нет.
        process_action(
            &mut field,
            &InventoryAction::Till {
                position: [8, 101, 7],
            },
        );
        assert_eq!(field.world.block([8, 101, 7]), game_core::GRASS);
    }

    #[test]
    fn torch_on_a_wall_becomes_a_wall_torch_and_the_item_is_consumed() {
        let mut game = world();
        game.game_mode = GameMode::Survival;
        equip(&mut game, "voxium:torch");
        game.inventory.hotbar[0].as_mut().unwrap().count = 2;
        sync_hotbar(&mut game);
        let effects = process_action(
            &mut game,
            &InventoryAction::Place {
                position: [8, 101, 6],
            },
        );
        assert_eq!(
            game.world.block([8, 101, 6]),
            game_core::facing_variant(game_core::WALL_TORCH, game_core::Facing::South)
        );
        assert!(!effects.sounds.is_empty());
        assert_eq!(game.inventory.hotbar[0].as_ref().unwrap().count, 1);
        // Сломанный настенный факел возвращает обычный факел.
        let torch = game.content.find("voxium:torch").unwrap();
        assert_eq!(
            game.content.drop_for_block(game_core::WALL_TORCH),
            Some(torch)
        );
    }
}
