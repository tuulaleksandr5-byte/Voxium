//! Конечный инвентарь: интерфейс отправляет намерения, состояние меняет клиент.
use super::*;
use crate::{
    inventory_client::{InventoryAction, InventoryActionMessage},
    item_icons::ItemIcons,
};
use game_core::{
    content::{ArmorSlot, ItemId},
    inventory::{ClickButton, SlotRef},
    items::ItemStack,
};

#[derive(Component)]
struct InventorySlot(SlotRef);
#[derive(Component)]
struct CatalogItem(ItemId);
#[derive(Component)]
struct CraftResult;
#[derive(Component)]
struct SlotHint(String);
#[derive(Component)]
struct Tooltip;
#[derive(Component)]
struct HeldCursor;
#[derive(Resource, Default)]
struct PointerGesture(DragGesture<SlotRef>);

struct DragGesture<T> {
    source: Option<T>,
}
impl<T> Default for DragGesture<T> {
    fn default() -> Self {
        Self { source: None }
    }
}
impl<T: Copy + PartialEq> DragGesture<T> {
    fn press(&mut self, source: T, can_drag: bool) {
        self.source = can_drag.then_some(source);
    }
    fn release(&mut self, target: Option<T>) -> Option<T> {
        let source = self.source.take()?;
        target.filter(|target| *target != source)
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PointerGesture>()
        .add_systems(Update, (input, pointer_overlay).chain());
}

#[allow(clippy::too_many_arguments)]
fn input(
    game: Res<Game>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut drag: ResMut<PointerGesture>,
    slots: Query<(&Hovered, &InventorySlot)>,
    catalog: Query<(&Hovered, &CatalogItem)>,
    result: Query<&Hovered, With<CraftResult>>,
    mut actions: MessageWriter<InventoryActionMessage>,
) {
    if game.mode != Mode::Inventory {
        drag.0.source = None;
        return;
    }
    let target = slots
        .iter()
        .find(|(hovered, _)| hovered.get())
        .map(|(_, slot)| slot.0);
    let result_hovered = result.iter().any(Hovered::get);
    let can_craft = game
        .recipes
        .preview(&game.inventory.crafting, &game.content)
        .is_some();
    let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    if mouse.just_pressed(MouseButton::Left) {
        drag.0.source = None;
        if let Some(slot) = target {
            let has_stack = stack_at(&game, slot).is_some();
            drag.0
                .press(slot, !shift && game.inventory.cursor.is_none() && has_stack);
            actions.write(InventoryActionMessage(InventoryAction::Click {
                slot,
                button: ClickButton::Left,
                shift,
            }));
        } else if result_hovered && can_craft {
            actions.write(InventoryActionMessage(InventoryAction::TakeCraftResult {
                all: shift,
            }));
        } else if game.game_mode == GameMode::Creative
            && let Some((_, item)) = catalog.iter().find(|(hovered, _)| hovered.get())
        {
            actions.write(InventoryActionMessage(InventoryAction::CreativeGrant(
                item.0,
            )));
        }
    }
    if mouse.just_pressed(MouseButton::Right) {
        drag.0.source = None;
        if let Some(slot) = target {
            actions.write(InventoryActionMessage(InventoryAction::Click {
                slot,
                button: ClickButton::Right,
                shift,
            }));
        } else if result_hovered && can_craft {
            actions.write(InventoryActionMessage(InventoryAction::TakeCraftResult {
                all: shift,
            }));
        }
    }
    if mouse.just_released(MouseButton::Left)
        && let Some(slot) = drag.0.release(target)
    {
        actions.write(InventoryActionMessage(InventoryAction::Click {
            slot,
            button: ClickButton::Left,
            shift: false,
        }));
    }
}

fn stack_at(game: &Game, slot: SlotRef) -> Option<&ItemStack> {
    match slot {
        SlotRef::Container(index) => game
            .open_container
            .and_then(|position| game.containers.get(&position))
            .and_then(|container| container.slots.get(index))
            .and_then(Option::as_ref),
        _ => game.inventory.get(slot),
    }
}

fn item_name(game: &Game, stack: Option<&ItemStack>, fallback: &str) -> String {
    stack
        .and_then(|stack| game.content.get(stack.item))
        .map(|item| item.name.clone())
        .unwrap_or_else(|| fallback.to_string())
}

fn slot(
    commands: &mut Commands,
    parent: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
    reference: SlotRef,
    empty_name: &str,
) {
    let stack = stack_at(game, reference);
    let selected = matches!(reference, SlotRef::Hotbar(index) if index == game.selected);
    let child = node(
        commands,
        parent,
        Node {
            width: px(54),
            height: px(54),
            flex_shrink: 0.,
            border: UiRect::all(px(if selected { 2. } else { 1. })),
            border_radius: BorderRadius::all(px(4)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Color::srgba(0.16, 0.07, 0.23, 0.96),
    );
    commands.entity(child).insert((
        Button,
        Hovered::default(),
        InventorySlot(reference),
        SlotHint(if matches!(reference, SlotRef::Armor(_)) {
            format!(
                "{} · бонусы брони ещё не действуют",
                item_name(game, stack, "Ячейка брони")
            )
        } else {
            item_name(
                game,
                stack,
                if empty_name.is_empty() {
                    "Пустая ячейка"
                } else {
                    empty_name
                },
            )
        }),
        BorderColor::all(if selected { AMBER } else { LINE }),
    ));
    if let Some(stack) = stack {
        let icon = commands
            .spawn((
                ImageNode::new(icons.get(stack.item, &game.content, blocks)),
                Node {
                    width: px(46),
                    height: px(46),
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .id();
        commands.entity(child).add_child(icon);
        let count = label(commands, child, stack.count.to_string(), 13., TEXT);
        commands.entity(count).insert(Node {
            position_type: PositionType::Absolute,
            right: px(3),
            bottom: px(1),
            ..default()
        });
    } else if !empty_name.is_empty() {
        label(commands, child, empty_name, 10., MUTED);
    }
    if let SlotRef::Hotbar(index) = reference {
        let number = label(commands, child, (index + 1).to_string(), 10., MUTED);
        commands.entity(number).insert(Node {
            position_type: PositionType::Absolute,
            left: px(3),
            top: px(1),
            ..default()
        });
    }
}

fn crafting(
    commands: &mut Commands,
    parent: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
) {
    label(commands, parent, "СОЗДАНИЕ · 2×2", 12., AMBER);
    let cells = row(commands, parent, 6.);
    let grid = column(commands, cells, 6.);
    commands.entity(grid).insert(Node {
        width: px(114),
        flex_direction: FlexDirection::Column,
        row_gap: px(6),
        flex_shrink: 0.,
        ..default()
    });
    for y in 0..2 {
        let line = row(commands, grid, 6.);
        for x in 0..2 {
            slot(
                commands,
                line,
                game,
                icons,
                blocks,
                SlotRef::Crafting(y * 2 + x),
                "",
            );
        }
    }
    label(commands, cells, "→", 16., MUTED);
    let output = game
        .recipes
        .preview(&game.inventory.crafting, &game.content);
    let result = node(
        commands,
        cells,
        Node {
            width: px(54),
            height: px(54),
            flex_shrink: 0.,
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(px(4)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        if output.is_some() {
            Color::srgba(0.24, 0.11, 0.31, 0.96)
        } else {
            Color::srgba(0.09, 0.045, 0.13, 0.96)
        },
    );
    commands.entity(result).insert((
        Button,
        Hovered::default(),
        CraftResult,
        SlotHint(if output.is_some() {
            format!(
                "{} · ЛКМ: создать · Shift: создать всё",
                item_name(game, output.as_ref(), "Результат")
            )
        } else {
            "Нет рецепта · разместите ингредиенты в сетке 2×2".into()
        }),
        BorderColor::all(if output.is_some() { AMBER } else { LINE }),
    ));
    if let Some(stack) = output {
        let image = commands
            .spawn((
                ImageNode::new(icons.get(stack.item, &game.content, blocks)),
                Node {
                    width: px(46),
                    height: px(46),
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .id();
        commands.entity(result).add_child(image);
        let count = label(commands, result, stack.count.to_string(), 13., TEXT);
        commands.entity(count).insert(Node {
            position_type: PositionType::Absolute,
            right: px(3),
            bottom: px(1),
            ..default()
        });
    } else {
        label(commands, result, "—", 20., MUTED);
    }
    label(commands, parent, "РЕЗУЛЬТАТ · Shift: всё", 11., MUTED);
}

pub(super) fn menu(
    commands: &mut Commands,
    panel: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
) {
    commands.entity(panel).insert((
        BackgroundColor(Color::srgba(0.10, 0.045, 0.16, 0.98)),
        BorderColor::all(AMBER),
    ));
    let title = if game.open_container.is_some() {
        "Сундук"
    } else {
        "Инвентарь"
    };
    label(commands, panel, title, 26., TEXT);
    label(
        commands,
        panel,
        "ЛКМ: взять / положить · ПКМ: половина / один · Shift: перенос",
        12.,
        MUTED,
    );
    let content = row(commands, panel, 24.);
    commands.entity(content).insert(Node {
        width: percent(100),
        column_gap: px(24),
        align_items: AlignItems::FlexStart,
        flex_wrap: FlexWrap::Wrap,
        row_gap: px(16),
        flex_shrink: 0.,
        ..default()
    });
    let storage = column(commands, content, 7.);
    commands.entity(storage).insert(Node {
        width: px(534),
        max_width: percent(100),
        flex_direction: FlexDirection::Column,
        row_gap: px(7),
        flex_shrink: 0.,
        ..default()
    });
    if game.open_container.is_some() {
        label(commands, storage, "СУНДУК · 27 ЯЧЕЕК", 12., AMBER);
        for line in 0..3 {
            let cells = row(commands, storage, 6.);
            for index in 0..9 {
                slot(
                    commands,
                    cells,
                    game,
                    icons,
                    blocks,
                    SlotRef::Container(line * 9 + index),
                    "",
                );
            }
        }
    }
    label(commands, storage, "РЮКЗАК · 27 ЯЧЕЕК", 12., MUTED);
    for line in 0..3 {
        let cells = row(commands, storage, 6.);
        for index in 0..9 {
            slot(
                commands,
                cells,
                game,
                icons,
                blocks,
                SlotRef::Main(line * 9 + index),
                "",
            );
        }
    }
    label(commands, storage, "ПАНЕЛЬ БЫСТРОГО ДОСТУПА", 12., MUTED);
    let hotbar = row(commands, storage, 6.);
    for index in 0..9 {
        slot(
            commands,
            hotbar,
            game,
            icons,
            blocks,
            SlotRef::Hotbar(index),
            "",
        );
    }
    crafting(commands, storage, game, icons, blocks);
    let equipment = column(commands, content, 7.);
    commands.entity(equipment).insert(Node {
        width: px(170),
        flex_direction: FlexDirection::Column,
        row_gap: px(7),
        flex_shrink: 0.,
        ..default()
    });
    label(commands, equipment, "ЭКИПИРОВКА", 12., AMBER);
    for (armor, caption) in [
        (ArmorSlot::Head, "Голова"),
        (ArmorSlot::Chest, "Тело"),
        (ArmorSlot::Legs, "Ноги"),
        (ArmorSlot::Feet, "Ступни"),
    ] {
        let cells = row(commands, equipment, 8.);
        slot(
            commands,
            cells,
            game,
            icons,
            blocks,
            SlotRef::Armor(armor),
            "",
        );
        label(commands, cells, caption, 12., MUTED);
    }
    let cells = row(commands, equipment, 8.);
    slot(commands, cells, game, icons, blocks, SlotRef::Offhand, "");
    label(commands, cells, "Вторая рука", 12., MUTED);
    label(commands, equipment, "Броня пока\nбез бонусов.", 12., MUTED);
    if game.game_mode == GameMode::Creative {
        label(
            commands,
            panel,
            format!("КАТАЛОГ ∞ · выдача в слот {}", game.selected + 1),
            13.,
            AMBER,
        );
        label(
            commands,
            panel,
            "1–9: выбрать слот выдачи · каждый предмет доступен без ограничений",
            12.,
            MUTED,
        );
        let targets = row(commands, panel, 6.);
        for index in 0..9 {
            let target = button(
                commands,
                targets,
                (index + 1).to_string(),
                Action::SelectSlot(index),
                if game.selected == index {
                    Tone::Gold
                } else {
                    Tone::Normal
                },
            );
            commands.entity(target).insert(Node {
                width: px(44),
                height: px(44),
                min_height: px(44),
                flex_shrink: 0.,
                padding: UiRect::all(px(0)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(4)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            });
        }
        let catalog = node(
            commands,
            panel,
            Node {
                width: percent(100),
                flex_wrap: FlexWrap::Wrap,
                column_gap: px(6),
                row_gap: px(6),
                flex_shrink: 0.,
                ..default()
            },
            Color::NONE,
        );
        for (id, definition) in game.content.iter() {
            let child = node(
                commands,
                catalog,
                Node {
                    width: px(54),
                    height: px(54),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(4)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Color::srgba(0.16, 0.07, 0.23, 0.96),
            );
            commands.entity(child).insert((
                Button,
                Hovered::default(),
                CatalogItem(id),
                SlotHint(format!("{} · ∞", definition.name)),
                BorderColor::all(LINE),
            ));
            let image = commands
                .spawn((
                    ImageNode::new(icons.get(id, &game.content, blocks)),
                    Node {
                        width: px(46),
                        height: px(46),
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .id();
            commands.entity(child).add_child(image);
            let infinity = label(commands, child, "∞", 13., AMBER);
            commands.entity(infinity).insert(Node {
                position_type: PositionType::Absolute,
                right: px(3),
                bottom: px(1),
                ..default()
            });
        }
    }
    label(
        commands,
        panel,
        "E / Esc: закрыть · колесо: прокрутить",
        12.,
        MUTED,
    );
    button(
        commands,
        panel,
        "Вернуться в мир",
        Action::Resume,
        Tone::Gold,
    );
}

pub(super) fn overlay(
    commands: &mut Commands,
    root: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
) {
    let tooltip = label(commands, root, "", 13., TEXT);
    commands.entity(tooltip).insert((
        Tooltip,
        GlobalZIndex(40),
        Node {
            display: Display::None,
            position_type: PositionType::Absolute,
            padding: UiRect::all(px(8)),
            max_width: px(300),
            ..default()
        },
        BackgroundColor(Color::srgba(0.08, 0.025, 0.13, 0.98)),
    ));
    if let Some(stack) = &game.inventory.cursor {
        let held = node(
            commands,
            root,
            Node {
                position_type: PositionType::Absolute,
                width: px(50),
                height: px(50),
                ..default()
            },
            Color::NONE,
        );
        commands
            .entity(held)
            .insert((HeldCursor, GlobalZIndex(50), Pickable::IGNORE));
        let image = commands
            .spawn((
                ImageNode::new(icons.get(stack.item, &game.content, blocks)),
                Node {
                    width: px(46),
                    height: px(46),
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .id();
        commands.entity(held).add_child(image);
        let count = label(commands, held, stack.count.to_string(), 15., TEXT);
        commands.entity(count).insert(Node {
            position_type: PositionType::Absolute,
            right: px(0),
            bottom: px(0),
            ..default()
        });
    }
}

#[allow(clippy::type_complexity)]
fn pointer_overlay(
    game: Res<Game>,
    windows: Query<&Window>,
    hints: Query<(&Hovered, &SlotHint)>,
    mut held: Query<&mut Node, (With<HeldCursor>, Without<Tooltip>)>,
    mut tooltip: Query<(&mut Node, &mut Text), With<Tooltip>>,
    mut borders: Query<
        (
            &Hovered,
            &mut BorderColor,
            Option<&InventorySlot>,
            Has<CraftResult>,
        ),
        Or<(With<InventorySlot>, With<CatalogItem>, With<CraftResult>)>,
    >,
) {
    if game.mode != Mode::Inventory {
        return;
    }
    let Some(window) = windows.iter().next() else {
        return;
    };
    let cursor = window
        .cursor_position()
        .unwrap_or(Vec2::new(window.width() / 2., window.height() / 2.));
    for mut node in &mut held {
        node.left = px((cursor.x + 14.).min(window.width() - 52.).max(0.));
        node.top = px((cursor.y + 14.).min(window.height() - 52.).max(0.));
    }
    let hint = hints
        .iter()
        .find(|(hovered, _)| hovered.get())
        .map(|(_, hint)| hint.0.as_str());
    for (mut node, mut text) in &mut tooltip {
        node.display = if hint.is_some() && game.inventory.cursor.is_none() {
            Display::Flex
        } else {
            Display::None
        };
        node.left = px((cursor.x + 18.).min(window.width() - 310.).max(0.));
        node.top = px((cursor.y + 24.).min(window.height() - 52.).max(0.));
        if let Some(hint) = hint
            && text.0 != hint
        {
            text.0 = hint.to_string();
        }
    }
    for (hovered, mut border, slot, result) in &mut borders {
        let selected = slot
            .is_some_and(|slot| matches!(slot.0, SlotRef::Hotbar(index) if index == game.selected));
        let ready = result
            && game
                .recipes
                .preview(&game.inventory.crafting, &game.content)
                .is_some();
        if (hovered.get() && !result) || ready || selected {
            border.set_all(AMBER);
        } else {
            border.set_all(LINE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn scrolling_app() -> App {
        let mut app = App::new();
        app.insert_resource(Game {
            mode: Mode::Inventory,
            game_mode: GameMode::Creative,
            world_id: Some("scroll-fixture".to_string()),
            ..default()
        })
        .init_resource::<BlockIcons>()
        .init_resource::<ItemIcons>()
        .init_resource::<ConsoleState>()
        .init_resource::<Time<Real>>()
        .add_message::<crate::block_audio::BlockSound>()
        .add_message::<crate::survival_client::PlayerActionMessage>()
        .add_systems(PostUpdate, rebuild);
        crate::inventory_client::plugin(&mut app);
        app.update();
        app
    }
    fn set_panel_scroll(app: &mut App, y: f32) {
        let world = app.world_mut();
        world
            .query_filtered::<&mut ScrollPosition, With<ScrollArea>>()
            .single_mut(world)
            .unwrap()
            .y = y;
    }
    fn panel_scroll(app: &mut App) -> f32 {
        let world = app.world_mut();
        world
            .query_filtered::<&ScrollPosition, With<ScrollArea>>()
            .single(world)
            .unwrap()
            .y
    }
    #[test]
    fn result_pointer_emits_only_result_commands_when_a_recipe_exists() {
        for all in [false, true] {
            let mut app = input_app(GameMode::Survival);
            app.world_mut().spawn((Hovered(true), CraftResult));
            app.world_mut()
                .resource_mut::<ButtonInput<MouseButton>>()
                .press(MouseButton::Left);
            app.update();
            assert_eq!(
                app.world_mut()
                    .resource_mut::<Messages<InventoryActionMessage>>()
                    .drain()
                    .count(),
                0
            );
            let mut game = app.world_mut().resource_mut::<Game>();
            game.inventory.crafting[0] = Some(ItemStack::new(
                game.content.find("foundation:oak_log").unwrap(),
                1,
            ));
            if all {
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .press(KeyCode::ShiftLeft);
            }
            app.update();
            let events: Vec<_> = app
                .world_mut()
                .resource_mut::<Messages<InventoryActionMessage>>()
                .drain()
                .collect();
            assert_eq!(events.len(), 1);
            assert!(
                matches!(events[0].0, InventoryAction::TakeCraftResult { all: value } if value == all)
            );
            assert!(app.world().resource::<PointerGesture>().0.source.is_none());
            assert!(app.world().resource::<Game>().inventory.cursor.is_none());
        }
    }
    #[test]
    fn result_is_a_separate_visible_preview_with_disabled_empty_state() {
        let mut game = Game::default();
        for ready in [false, true] {
            if ready {
                game.inventory.crafting[0] = Some(ItemStack::new(
                    game.content.find("foundation:oak_log").unwrap(),
                    1,
                ));
            }
            let mut world = World::new();
            let panel = world.spawn_empty().id();
            let mut queue = bevy::ecs::world::CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &world);
            menu(
                &mut commands,
                panel,
                &game,
                &ItemIcons::default(),
                &BlockIcons::default(),
            );
            queue.apply(&mut world);
            let result = world
                .query_filtered::<Entity, With<CraftResult>>()
                .single(&world)
                .unwrap();
            assert!(world.get::<InventorySlot>(result).is_none());
            assert_eq!(world.get::<Node>(result).unwrap().width, px(54));
            let hint = &world.get::<SlotHint>(result).unwrap().0;
            assert!(if ready {
                hint.contains("Дубовые доски")
            } else {
                hint.contains("Нет рецепта")
            });
            let images = world
                .get::<Children>(result)
                .unwrap()
                .iter()
                .filter(|child| world.get::<ImageNode>(*child).is_some())
                .count();
            assert_eq!(images, usize::from(ready));
        }
    }
    #[test]
    fn inventory_rebuild_preserves_scroll_after_events_and_catalog_reload() {
        let mut app = scrolling_app();
        set_panel_scroll(&mut app, 187.);
        let item = app
            .world()
            .resource::<Game>()
            .content
            .item_for_block(1)
            .unwrap();
        for action in [
            InventoryAction::CreativeGrant(item),
            InventoryAction::Click {
                slot: SlotRef::Hotbar(0),
                button: ClickButton::Left,
                shift: false,
            },
            InventoryAction::Click {
                slot: SlotRef::Main(0),
                button: ClickButton::Right,
                shift: false,
            },
        ] {
            app.world_mut()
                .write_message(InventoryActionMessage(action));
            app.update();
            assert_eq!(panel_scroll(&mut app), 187.);
        }
        assert_eq!(
            app.world().resource::<Game>().inventory.main[0]
                .as_ref()
                .unwrap()
                .count,
            1
        );
        let updated =
            include_str!("../../../../assets/data/items.ron").replace("Дёрн", "Обновлённый дёрн");
        let mut game = app.world_mut().resource_mut::<Game>();
        game.content = game.content.reload(&updated).unwrap();
        game.ui_dirty = true;
        app.update();
        assert_eq!(panel_scroll(&mut app), 187.);
        let world = app.world_mut();
        let mut slots = world.query::<(&InventorySlot, &SlotHint, &Children)>();
        let (_, hint, children) = slots
            .iter(world)
            .find(|(slot, _, _)| slot.0 == SlotRef::Main(0))
            .unwrap();
        assert_eq!(hint.0, "Обновлённый дёрн");
        assert!(
            children
                .iter()
                .any(|child| world.get::<Text>(child).is_some_and(|text| text.0 == "1"))
        );
    }
    #[test]
    fn inventory_scroll_resets_on_container_world_and_screen_changes() {
        let mut app = scrolling_app();
        set_panel_scroll(&mut app, 187.);
        {
            let mut game = app.world_mut().resource_mut::<Game>();
            game.open_container = Some([1, 2, 3]);
            game.ui_dirty = true;
        }
        app.update();
        assert_eq!(panel_scroll(&mut app), 0.);
        set_panel_scroll(&mut app, 72.);
        app.world_mut().resource_mut::<Game>().ui_dirty = true;
        app.update();
        assert_eq!(panel_scroll(&mut app), 72.);
        set_panel_scroll(&mut app, 99.);
        {
            let mut game = app.world_mut().resource_mut::<Game>();
            game.open_container = Some([4, 2, 3]);
            game.ui_dirty = true;
        }
        app.update();
        assert_eq!(panel_scroll(&mut app), 0.);
        set_panel_scroll(&mut app, 66.);
        {
            let mut game = app.world_mut().resource_mut::<Game>();
            game.world_id = Some("another-world".to_string());
            game.ui_dirty = true;
        }
        app.update();
        assert_eq!(panel_scroll(&mut app), 0.);
        set_panel_scroll(&mut app, 55.);
        {
            let mut game = app.world_mut().resource_mut::<Game>();
            game.world.seed = 123;
            game.ui_dirty = true;
        }
        app.update();
        assert_eq!(panel_scroll(&mut app), 0.);
        set_panel_scroll(&mut app, 88.);
        app.world_mut().resource_mut::<Game>().screen(Mode::Playing);
        app.update();
        app.world_mut()
            .resource_mut::<Game>()
            .screen(Mode::Inventory);
        app.update();
        assert_eq!(panel_scroll(&mut app), 0.);
    }
    #[test]
    fn survival_panel_has_all_finite_slots_and_no_catalog() {
        let game = Game {
            game_mode: GameMode::Survival,
            ..default()
        };
        let mut world = World::new();
        let panel = world.spawn_empty().id();
        let mut queue = bevy::ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &world);
        menu(
            &mut commands,
            panel,
            &game,
            &ItemIcons::default(),
            &BlockIcons::default(),
        );
        queue.apply(&mut world);
        assert_eq!(world.query::<&InventorySlot>().iter(&world).count(), 45);
        assert_eq!(world.query::<&CatalogItem>().iter(&world).count(), 0);
    }
    #[test]
    fn open_chest_adds_twenty_seven_slots_and_creative_catalog() {
        let mut game = Game {
            game_mode: GameMode::Creative,
            open_container: Some([1, 2, 3]),
            ..default()
        };
        game.containers
            .insert([1, 2, 3], game_core::inventory::Container::new(27));
        let expected_catalog = game.content.len();
        let mut world = World::new();
        let panel = world.spawn_empty().id();
        let mut queue = bevy::ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &world);
        menu(
            &mut commands,
            panel,
            &game,
            &ItemIcons::default(),
            &BlockIcons::default(),
        );
        queue.apply(&mut world);
        assert_eq!(world.query::<&InventorySlot>().iter(&world).count(), 72);
        assert_eq!(
            world.query::<&CatalogItem>().iter(&world).count(),
            expected_catalog
        );
    }
    fn input_app(mode: GameMode) -> App {
        let mut app = App::new();
        let game = Game {
            mode: Mode::Inventory,
            game_mode: mode,
            ..default()
        };
        app.insert_resource(game)
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<PointerGesture>()
            .add_message::<InventoryActionMessage>()
            .add_systems(Update, input);
        app
    }
    #[test]
    fn drag_release_survives_panel_entity_rebuild() {
        let mut app = input_app(GameMode::Survival);
        let item = app
            .world()
            .resource::<Game>()
            .content
            .item_for_block(1)
            .unwrap();
        app.world_mut().resource_mut::<Game>().inventory.hotbar[0] = Some(ItemStack::new(item, 4));
        let source = app
            .world_mut()
            .spawn((Hovered(true), InventorySlot(SlotRef::Hotbar(0))))
            .id();
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);
        app.update();
        let pickup: Vec<_> = app
            .world_mut()
            .resource_mut::<Messages<InventoryActionMessage>>()
            .drain()
            .collect();
        assert_eq!(pickup.len(), 1);
        assert!(matches!(
            pickup[0].0,
            InventoryAction::Click {
                slot: SlotRef::Hotbar(0),
                button: ClickButton::Left,
                shift: false
            }
        ));
        app.world_mut().despawn(source);
        app.world_mut()
            .spawn((Hovered(true), InventorySlot(SlotRef::Main(7))));
        let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
        mouse.clear();
        mouse.release(MouseButton::Left);
        app.update();
        let place: Vec<_> = app
            .world_mut()
            .resource_mut::<Messages<InventoryActionMessage>>()
            .drain()
            .collect();
        assert_eq!(place.len(), 1);
        assert!(matches!(
            place[0].0,
            InventoryAction::Click {
                slot: SlotRef::Main(7),
                button: ClickButton::Left,
                shift: false
            }
        ));
    }
    #[test]
    fn right_click_emits_split_with_shift_without_mutating_inventory() {
        let mut app = input_app(GameMode::Survival);
        app.world_mut()
            .spawn((Hovered(true), InventorySlot(SlotRef::Main(3))));
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ShiftLeft);
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Right);
        app.update();
        let events: Vec<_> = app
            .world_mut()
            .resource_mut::<Messages<InventoryActionMessage>>()
            .drain()
            .collect();
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0].0,
            InventoryAction::Click {
                slot: SlotRef::Main(3),
                button: ClickButton::Right,
                shift: true
            }
        ));
        assert!(app.world().resource::<Game>().inventory.cursor.is_none());
    }
    #[test]
    fn survival_rejects_catalog_click_and_creative_emits_grant() {
        for (mode, expected) in [(GameMode::Survival, 0), (GameMode::Creative, 1)] {
            let mut app = input_app(mode);
            let item = app
                .world()
                .resource::<Game>()
                .content
                .item_for_block(1)
                .unwrap();
            app.world_mut().spawn((Hovered(true), CatalogItem(item)));
            app.world_mut()
                .resource_mut::<ButtonInput<MouseButton>>()
                .press(MouseButton::Left);
            app.update();
            let events: Vec<_> = app
                .world_mut()
                .resource_mut::<Messages<InventoryActionMessage>>()
                .drain()
                .collect();
            assert_eq!(events.len(), expected);
            if let Some(event) = events.first() {
                assert!(matches!(event.0, InventoryAction::CreativeGrant(id) if id == item));
            }
        }
    }
    #[test]
    fn drag_transfers_only_to_a_different_slot_once() {
        let mut gesture = DragGesture::default();
        gesture.press(SlotRef::Main(2), true);
        assert_eq!(
            gesture.release(Some(SlotRef::Hotbar(5))),
            Some(SlotRef::Hotbar(5))
        );
        assert_eq!(gesture.release(Some(SlotRef::Main(6))), None);
    }
    #[test]
    fn click_click_and_release_outside_keep_cursor() {
        let mut gesture = DragGesture::default();
        gesture.press(SlotRef::Main(2), true);
        assert_eq!(gesture.release(Some(SlotRef::Main(2))), None);
        gesture.press(SlotRef::Main(2), true);
        assert_eq!(gesture.release(None), None);
    }
    #[test]
    fn shift_or_existing_cursor_cannot_start_drag() {
        let mut gesture = DragGesture::default();
        gesture.press(SlotRef::Main(2), false);
        assert_eq!(gesture.release(Some(SlotRef::Hotbar(5))), None);
    }
}
