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
    let can_craft = if game.open_workbench.is_some() {
        game.recipes
            .preview_workbench(&game.inventory.workbench, &game.content)
            .is_some()
    } else {
        game.recipes
            .preview(&game.inventory.crafting, &game.content)
            .is_some()
    };
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

const PIXEL_SCALE: f32 = 3.0;
const SLOT_SIZE: f32 = 18.0 * PIXEL_SCALE;
const INVENTORY_CANVAS_WIDTH: f32 = 176.0 * PIXEL_SCALE;

#[derive(Component)]
struct InventoryBoard;
#[derive(Component)]
struct PackGuiBackground;
#[derive(Component)]
struct AvatarPreview;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InventoryScreen {
    Personal,
    Workbench,
    Furnace,
    Chest,
}

impl InventoryScreen {
    fn from_game(game: &Game) -> Self {
        if game.open_workbench.is_some() {
            Self::Workbench
        } else if game
            .open_container
            .is_some_and(|pos| game.world.block(pos) == game_core::FURNACE)
        {
            Self::Furnace
        } else if game.open_container.is_some() {
            Self::Chest
        } else {
            Self::Personal
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Personal => "Инвентарь",
            Self::Workbench => "Верстак · 3×3",
            Self::Furnace => "Печь",
            Self::Chest => "Сундук · 27 ячеек",
        }
    }

    fn skin_name(self) -> &'static str {
        match self {
            Self::Personal => "inventory",
            Self::Workbench => "workbench",
            Self::Furnace => "furnace",
            Self::Chest => "chest",
        }
    }

    fn height(self) -> f32 {
        if self == Self::Chest {
            222.0 * PIXEL_SCALE
        } else {
            166.0 * PIXEL_SCALE
        }
    }

    fn main_y(self) -> f32 {
        if self == Self::Chest { 140.0 } else { 84.0 }
    }

    fn hotbar_y(self) -> f32 {
        if self == Self::Chest { 198.0 } else { 142.0 }
    }
}

/// Create an interactable Minecraft-size 18×18 slot in a fixed 176-unit canvas.
#[allow(clippy::too_many_arguments)]
fn slot_at(
    commands: &mut Commands,
    parent: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
    reference: SlotRef,
    logical_x: f32,
    logical_y: f32,
    skinned: bool,
    empty_hint: &str,
) {
    let stack = stack_at(game, reference);
    let selected = matches!(reference, SlotRef::Hotbar(index) if index == game.selected);
    let border = if selected {
        2.0
    } else if skinned {
        0.0
    } else {
        2.0
    };
    let cell = node(
        commands,
        parent,
        Node {
            position_type: PositionType::Absolute,
            left: px(logical_x * PIXEL_SCALE),
            top: px(logical_y * PIXEL_SCALE),
            width: px(SLOT_SIZE),
            height: px(SLOT_SIZE),
            flex_shrink: 0.0,
            border: UiRect::all(px(border)),
            border_radius: BorderRadius::all(px(0.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        if skinned {
            Color::NONE
        } else {
            Color::srgba(0.19, 0.19, 0.20, 0.93)
        },
    );
    commands.entity(cell).insert((
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
                if empty_hint.is_empty() {
                    "Пустая ячейка"
                } else {
                    empty_hint
                },
            )
        }),
        BorderColor::all(if selected {
            Color::srgb(0.99, 0.88, 0.30)
        } else if skinned {
            Color::NONE
        } else {
            Color::srgb(0.35, 0.35, 0.36)
        }),
    ));
    if let Some(stack) = stack {
        let icon = commands
            .spawn((
                ImageNode::new(icons.get(stack.item, &game.content, blocks)),
                Node {
                    width: px(46.0),
                    height: px(46.0),
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .id();
        commands.entity(cell).add_child(icon);
        let count = label(commands, cell, stack.count.to_string(), 13.0, TEXT);
        commands.entity(count).insert(Node {
            position_type: PositionType::Absolute,
            right: px(2.0),
            bottom: px(1.0),
            ..default()
        });
    }
}

/// Visible crafting result is a preview, not a writable inventory slot.
fn craft_result_at(
    commands: &mut Commands,
    board: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
    screen: InventoryScreen,
    x: f32,
    y: f32,
    skinned: bool,
) {
    let output = if screen == InventoryScreen::Workbench {
        game.recipes
            .preview_workbench(&game.inventory.workbench, &game.content)
    } else {
        game.recipes
            .preview(&game.inventory.crafting, &game.content)
    };
    let result = node(
        commands,
        board,
        Node {
            position_type: PositionType::Absolute,
            left: px(x * PIXEL_SCALE),
            top: px(y * PIXEL_SCALE),
            width: px(SLOT_SIZE),
            height: px(SLOT_SIZE),
            flex_shrink: 0.0,
            border: UiRect::all(px(if skinned { 0.0 } else { 2.0 })),
            border_radius: BorderRadius::all(px(0.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        if skinned {
            Color::NONE
        } else {
            Color::srgba(0.19, 0.19, 0.20, 0.93)
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
            "Нет рецепта · разместите ингредиенты в сетке".into()
        }),
        BorderColor::all(if output.is_some() {
            Color::srgb(0.99, 0.88, 0.30)
        } else if skinned {
            Color::NONE
        } else {
            Color::srgb(0.35, 0.35, 0.36)
        }),
    ));
    if let Some(stack) = output {
        let image = commands
            .spawn((
                ImageNode::new(icons.get(stack.item, &game.content, blocks)),
                Node {
                    width: px(46.0),
                    height: px(46.0),
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .id();
        commands.entity(result).add_child(image);
        let count = label(commands, result, stack.count.to_string(), 13.0, TEXT);
        commands.entity(count).insert(Node {
            position_type: PositionType::Absolute,
            right: px(2.0),
            bottom: px(1.0),
            ..default()
        });
    }
}

fn board_caption(commands: &mut Commands, board: Entity, text: &str, x: f32, y: f32) {
    let entity = label(commands, board, text, 14.0, Color::srgb(0.17, 0.17, 0.18));
    commands.entity(entity).insert(Node {
        position_type: PositionType::Absolute,
        left: px(x * PIXEL_SCALE),
        top: px(y * PIXEL_SCALE),
        ..default()
    });
}

fn avatar_piece(
    commands: &mut Commands,
    avatar: Entity,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
) {
    let piece = node(
        commands,
        avatar,
        Node {
            position_type: PositionType::Absolute,
            left: px(x * PIXEL_SCALE),
            top: px(y * PIXEL_SCALE),
            width: px(width * PIXEL_SCALE),
            height: px(height * PIXEL_SCALE),
            ..default()
        },
        color,
    );
    commands.entity(piece).insert(Pickable::IGNORE);
}

/// Until the Voxium character mesh is available in UI, show a static original silhouette.
fn avatar_preview(commands: &mut Commands, board: Entity) {
    let avatar = node(
        commands,
        board,
        Node {
            position_type: PositionType::Absolute,
            left: px(26.0 * PIXEL_SCALE),
            top: px(8.0 * PIXEL_SCALE),
            width: px(51.0 * PIXEL_SCALE),
            height: px(72.0 * PIXEL_SCALE),
            flex_shrink: 0.0,
            border: UiRect::all(px(2.0)),
            ..default()
        },
        Color::srgba(0.10, 0.17, 0.24, 0.90),
    );
    commands.entity(avatar).insert((
        AvatarPreview,
        Pickable::IGNORE,
        BorderColor::all(Color::srgb(0.30, 0.32, 0.34)),
    ));
    avatar_piece(
        commands,
        avatar,
        19.0,
        8.0,
        13.0,
        13.0,
        Color::srgb(0.72, 0.53, 0.37),
    );
    avatar_piece(
        commands,
        avatar,
        19.0,
        8.0,
        13.0,
        4.0,
        Color::srgb(0.20, 0.16, 0.14),
    );
    avatar_piece(
        commands,
        avatar,
        14.0,
        21.0,
        23.0,
        24.0,
        Color::srgb(0.16, 0.46, 0.75),
    );
    avatar_piece(
        commands,
        avatar,
        7.0,
        22.0,
        7.0,
        23.0,
        Color::srgb(0.67, 0.49, 0.34),
    );
    avatar_piece(
        commands,
        avatar,
        37.0,
        22.0,
        7.0,
        23.0,
        Color::srgb(0.67, 0.49, 0.34),
    );
    avatar_piece(
        commands,
        avatar,
        15.0,
        45.0,
        10.0,
        22.0,
        Color::srgb(0.20, 0.23, 0.40),
    );
    avatar_piece(
        commands,
        avatar,
        26.0,
        45.0,
        10.0,
        22.0,
        Color::srgb(0.20, 0.23, 0.40),
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_main_and_hotbar(
    commands: &mut Commands,
    board: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
    screen: InventoryScreen,
    skinned: bool,
) {
    board_caption(commands, board, "Инвентарь", 8.0, screen.main_y() - 11.0);
    for line in 0..3 {
        for index in 0..9 {
            slot_at(
                commands,
                board,
                game,
                icons,
                blocks,
                SlotRef::Main(line * 9 + index),
                8.0 + index as f32 * 18.0,
                screen.main_y() + line as f32 * 18.0,
                skinned,
                "",
            );
        }
    }
    for index in 0..9 {
        slot_at(
            commands,
            board,
            game,
            icons,
            blocks,
            SlotRef::Hotbar(index),
            8.0 + index as f32 * 18.0,
            screen.hotbar_y(),
            skinned,
            "",
        );
    }
}

/// Fully interactive Minecraft-like inventory with a pixel-aligned GUI canvas.
#[cfg(test)]
pub(super) fn menu(
    commands: &mut Commands,
    panel: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
) {
    menu_with_skins(commands, panel, game, icons, blocks, None);
}

pub(super) fn menu_with_skins(
    commands: &mut Commands,
    panel: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
    skins: Option<&crate::resource_packs::GuiSkins>,
) {
    let screen = InventoryScreen::from_game(game);
    let skin = skins.and_then(|skins| skins.get(screen.skin_name()));
    let skinned = skin.is_some();
    commands.entity(panel).insert((
        BackgroundColor(Color::srgba(0.10, 0.10, 0.12, 0.98)),
        BorderColor::all(Color::srgb(0.40, 0.40, 0.42)),
    ));
    label(commands, panel, screen.title(), 22.0, TEXT);
    let board = node(
        commands,
        panel,
        Node {
            width: px(INVENTORY_CANVAS_WIDTH),
            height: px(screen.height()),
            min_width: px(INVENTORY_CANVAS_WIDTH),
            min_height: px(screen.height()),
            flex_shrink: 0.0,
            align_self: AlignSelf::Center,
            border: UiRect::all(px(3.0)),
            border_radius: BorderRadius::all(px(0.0)),
            ..default()
        },
        if skinned {
            Color::NONE
        } else {
            Color::srgb(0.73, 0.73, 0.73)
        },
    );
    commands.entity(board).insert((
        InventoryBoard,
        BorderColor::all(Color::srgb(0.23, 0.23, 0.24)),
    ));
    if let Some(skin) = skin {
        let backdrop = commands
            .spawn((
                ImageNode::new(skin),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0.0),
                    top: px(0.0),
                    width: px(INVENTORY_CANVAS_WIDTH),
                    height: px(screen.height()),
                    ..default()
                },
                PackGuiBackground,
                Pickable::IGNORE,
            ))
            .id();
        commands.entity(board).add_child(backdrop);
    }

    match screen {
        InventoryScreen::Personal => {
            avatar_preview(commands, board);
            for (index, armor) in [
                ArmorSlot::Head,
                ArmorSlot::Chest,
                ArmorSlot::Legs,
                ArmorSlot::Feet,
            ]
            .into_iter()
            .enumerate()
            {
                slot_at(
                    commands,
                    board,
                    game,
                    icons,
                    blocks,
                    SlotRef::Armor(armor),
                    8.0,
                    8.0 + index as f32 * 18.0,
                    skinned,
                    "",
                );
            }
            slot_at(
                commands,
                board,
                game,
                icons,
                blocks,
                SlotRef::Offhand,
                77.0,
                62.0,
                skinned,
                "",
            );
            board_caption(commands, board, "Создание", 98.0, 5.0);
            for line in 0..2 {
                for index in 0..2 {
                    slot_at(
                        commands,
                        board,
                        game,
                        icons,
                        blocks,
                        SlotRef::Crafting(line * 2 + index),
                        98.0 + index as f32 * 18.0,
                        18.0 + line as f32 * 18.0,
                        skinned,
                        "",
                    );
                }
            }
            craft_result_at(
                commands, board, game, icons, blocks, screen, 154.0, 28.0, skinned,
            );
        }
        InventoryScreen::Workbench => {
            board_caption(commands, board, "Создание", 28.0, 5.0);
            for line in 0..3 {
                for index in 0..3 {
                    slot_at(
                        commands,
                        board,
                        game,
                        icons,
                        blocks,
                        SlotRef::Workbench(line * 3 + index),
                        30.0 + index as f32 * 18.0,
                        17.0 + line as f32 * 18.0,
                        skinned,
                        "",
                    );
                }
            }
            craft_result_at(
                commands, board, game, icons, blocks, screen, 124.0, 35.0, skinned,
            );
        }
        InventoryScreen::Furnace => {
            board_caption(commands, board, "Плавка", 8.0, 5.0);
            for (index, x, y, hint) in [
                (0, 56.0, 17.0, "Сырьё"),
                (1, 56.0, 53.0, "Топливо"),
                (2, 116.0, 35.0, "Готово"),
            ] {
                slot_at(
                    commands,
                    board,
                    game,
                    icons,
                    blocks,
                    SlotRef::Container(index),
                    x,
                    y,
                    skinned,
                    hint,
                );
            }
            if let Some(position) = game.open_container {
                let progress = game
                    .furnaces
                    .get(&position)
                    .map_or(0.0, |furnace| (furnace.progress / 10.0).clamp(0.0, 1.0));
                let indicator = node(
                    commands,
                    board,
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(79.0 * PIXEL_SCALE),
                        top: px(37.0 * PIXEL_SCALE),
                        width: px(23.0 * PIXEL_SCALE * progress),
                        height: px(3.0 * PIXEL_SCALE),
                        ..default()
                    },
                    Color::srgb(0.97, 0.69, 0.25),
                );
                commands.entity(indicator).insert(Pickable::IGNORE);
            }
        }
        InventoryScreen::Chest => {
            board_caption(commands, board, "Сундук", 8.0, 5.0);
            for line in 0..3 {
                for index in 0..9 {
                    slot_at(
                        commands,
                        board,
                        game,
                        icons,
                        blocks,
                        SlotRef::Container(line * 9 + index),
                        8.0 + index as f32 * 18.0,
                        18.0 + line as f32 * 18.0,
                        skinned,
                        "",
                    );
                }
            }
        }
    }

    draw_main_and_hotbar(commands, board, game, icons, blocks, screen, skinned);
    label(
        commands,
        panel,
        "ЛКМ: взять · ПКМ: разделить · Shift: быстрый перенос",
        12.0,
        MUTED,
    );
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
                Color::srgba(0.13, 0.14, 0.16, 0.98),
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
        12.0,
        MUTED,
    );
    button(
        commands,
        panel,
        "Закрыть инвентарь",
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

    fn render_inventory_for_test(game: &Game) -> World {
        let mut world = World::new();
        let panel = world.spawn_empty().id();
        let mut queue = bevy::ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &world);
        menu(
            &mut commands,
            panel,
            game,
            &ItemIcons::default(),
            &BlockIcons::default(),
        );
        queue.apply(&mut world);
        world
    }

    fn assert_slot_at(world: &mut World, target: SlotRef, x: f32, y: f32) {
        let mut slots = world.query::<(&InventorySlot, &Node)>();
        let (_, layout) = slots
            .iter(world)
            .find(|(slot, _)| slot.0 == target)
            .unwrap_or_else(|| panic!("missing interactive slot {target:?}"));
        assert_eq!(layout.position_type, PositionType::Absolute, "{target:?}");
        assert_eq!(layout.left, px(x * 3.), "{target:?} x");
        assert_eq!(layout.top, px(y * 3.), "{target:?} y");
        assert_eq!(layout.width, px(54.), "{target:?} width");
        assert_eq!(layout.height, px(54.), "{target:?} height");
    }

    #[test]
    fn minecraft_personal_inventory_places_avatar_armor_2x2_and_backpack() {
        let mut world = render_inventory_for_test(&Game::default());
        assert_eq!(world.query::<&InventorySlot>().iter(&world).count(), 45);
        assert_eq!(world.query::<&CraftResult>().iter(&world).count(), 1);
        assert_slot_at(&mut world, SlotRef::Armor(ArmorSlot::Head), 8., 8.);
        assert_slot_at(&mut world, SlotRef::Armor(ArmorSlot::Feet), 8., 62.);
        assert_slot_at(&mut world, SlotRef::Offhand, 77., 62.);
        assert_slot_at(&mut world, SlotRef::Crafting(0), 98., 18.);
        assert_slot_at(&mut world, SlotRef::Crafting(3), 116., 36.);
        assert_slot_at(&mut world, SlotRef::Main(0), 8., 84.);
        assert_slot_at(&mut world, SlotRef::Main(26), 152., 120.);
        assert_slot_at(&mut world, SlotRef::Hotbar(0), 8., 142.);
        assert_slot_at(&mut world, SlotRef::Hotbar(8), 152., 142.);
        let mut results = world.query::<(&CraftResult, &Node)>();
        let (_, layout) = results.single(&world).unwrap();
        assert_eq!(layout.left, px(154. * 3.));
        assert_eq!(layout.top, px(28. * 3.));
    }

    #[test]
    fn minecraft_workbench_has_3x3_above_inventory_without_equipment() {
        let game = Game {
            open_workbench: Some([1, 2, 3]),
            ..default()
        };
        let mut world = render_inventory_for_test(&game);
        assert_eq!(world.query::<&InventorySlot>().iter(&world).count(), 45);
        assert_slot_at(&mut world, SlotRef::Workbench(0), 30., 17.);
        assert_slot_at(&mut world, SlotRef::Workbench(8), 66., 53.);
        assert_slot_at(&mut world, SlotRef::Main(0), 8., 84.);
        assert_slot_at(&mut world, SlotRef::Hotbar(0), 8., 142.);
        let mut result = world.query::<(&CraftResult, &Node)>();
        assert_eq!(result.single(&world).unwrap().1.left, px(124. * 3.));
        let mut slots = world.query::<&InventorySlot>();
        assert!(!slots.iter(&world).any(|s| matches!(
            s.0,
            SlotRef::Armor(_) | SlotRef::Crafting(_) | SlotRef::Offhand
        )));
    }

    #[test]
    fn minecraft_furnace_and_chest_use_station_specific_slotted_windows() {
        let position = [1, 85, 1];
        let mut furnace = Game {
            open_container: Some(position),
            ..default()
        };
        furnace.world.edits.insert(position, game_core::FURNACE);
        furnace
            .containers
            .insert(position, game_core::inventory::Container::new(3));
        let mut furnace_world = render_inventory_for_test(&furnace);
        assert_eq!(
            furnace_world
                .query::<&InventorySlot>()
                .iter(&furnace_world)
                .count(),
            39
        );
        assert_slot_at(&mut furnace_world, SlotRef::Container(0), 56., 17.);
        assert_slot_at(&mut furnace_world, SlotRef::Container(1), 56., 53.);
        assert_slot_at(&mut furnace_world, SlotRef::Container(2), 116., 35.);
        assert_slot_at(&mut furnace_world, SlotRef::Main(0), 8., 84.);
        assert_eq!(
            furnace_world
                .query::<&CraftResult>()
                .iter(&furnace_world)
                .count(),
            0
        );

        let mut chest = Game {
            open_container: Some(position),
            ..default()
        };
        chest
            .containers
            .insert(position, game_core::inventory::Container::new(27));
        let mut chest_world = render_inventory_for_test(&chest);
        assert_eq!(
            chest_world
                .query::<&InventorySlot>()
                .iter(&chest_world)
                .count(),
            63
        );
        assert_slot_at(&mut chest_world, SlotRef::Container(0), 8., 18.);
        assert_slot_at(&mut chest_world, SlotRef::Container(26), 152., 54.);
        assert_slot_at(&mut chest_world, SlotRef::Main(0), 8., 140.);
        assert_slot_at(&mut chest_world, SlotRef::Hotbar(0), 8., 198.);
        assert_eq!(
            chest_world
                .query::<&CraftResult>()
                .iter(&chest_world)
                .count(),
            0
        );
    }


    #[test]
    fn minecraft_board_skin_is_behind_clickable_slots_and_avatar_is_personal_only() {
        let game = Game::default();
        let skins = crate::resource_packs::GuiSkins::inventory_test_skin(Handle::default());
        let mut world = World::new();
        let panel = world.spawn_empty().id();
        let mut queue = bevy::ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &world);
        menu_with_skins(
            &mut commands,
            panel,
            &game,
            &ItemIcons::default(),
            &BlockIcons::default(),
            Some(&skins),
        );
        queue.apply(&mut world);
        let mut boards = world.query_filtered::<Entity, With<InventoryBoard>>();
        let board = boards.single(&world).unwrap();
        let children = world.get::<Children>(board).unwrap();
        assert!(world.get::<PackGuiBackground>(children[0]).is_some());
        assert!(world.get::<Pickable>(children[0]).is_some());
        assert_eq!(world.get::<Node>(children[0]).unwrap().width, px(528.));
        assert_eq!(world.query::<&AvatarPreview>().iter(&world).count(), 1);
        assert_eq!(world.query::<&InventorySlot>().iter(&world).count(), 45);

        let station = Game { open_workbench: Some([1, 2, 3]), ..default() };
        let mut workbench = render_inventory_for_test(&station);
        assert_eq!(workbench.query::<&AvatarPreview>().iter(&workbench).count(), 0);
        let mut boards = workbench.query::<(&InventoryBoard, &Node)>();
        assert_eq!(boards.single(&workbench).unwrap().1.height, px(498.));

        let chest = Game { open_container: Some([1, 2, 3]), ..default() };
        let mut chest_world = render_inventory_for_test(&chest);
        let mut boards = chest_world.query::<(&InventoryBoard, &Node)>();
        assert_eq!(boards.single(&chest_world).unwrap().1.height, px(666.));
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
        assert_eq!(world.query::<&InventorySlot>().iter(&world).count(), 63);
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
