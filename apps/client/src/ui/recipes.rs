//! Обзор рецептов: R над предметом — «как получить», U — «где используется».
//! ЛКМ по предмету в рецепте открывает его рецепты, ПКМ — его применение.
use super::{
    AMBER, LINE, MUTED, PANEL, SURFACE, TEXT,
    inventory::{CatalogItem, InventorySlot, stack_at},
    label,
};
use crate::{
    graphics_assets::BlockIcons,
    item_icons::ItemIcons,
    session::{Game, Mode},
};
use bevy::{picking::hover::Hovered, prelude::*};
use game_core::{
    content::ItemId,
    creative::Tab,
    recipe_view::{self, RecipeCard},
    survival::GameMode,
};

#[derive(Component)]
struct RecipeOverlay;
#[derive(Component)]
struct RecipeCell(ItemId);
#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum RecipeNav {
    Prev,
    Next,
    Close,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (open_keys, nav_clicks, sync_overlay).chain());
}

fn hovered_item(
    game: &Game,
    slots: &Query<(&Hovered, &InventorySlot)>,
    catalog: &Query<(&Hovered, &CatalogItem)>,
) -> Option<ItemId> {
    if let Some((_, slot)) = slots.iter().find(|(h, _)| h.get()) {
        return stack_at(game, slot.0).map(|stack| stack.item);
    }
    catalog
        .iter()
        .find(|(h, _)| h.get())
        .map(|(_, item)| item.0)
}

#[allow(clippy::type_complexity)]
fn open_keys(
    mut game: ResMut<Game>,
    keys: Res<ButtonInput<KeyCode>>,
    slots: Query<(&Hovered, &InventorySlot)>,
    catalog: Query<(&Hovered, &CatalogItem)>,
) {
    if game.mode != Mode::Inventory {
        if game.recipe_view.is_some() {
            game.recipe_view = None;
        }
        return;
    }
    let typing =
        game.game_mode == GameMode::Creative && Tab::from_index(game.catalog_tab) == Tab::Search;
    if typing {
        return;
    }
    let wanted = if keys.just_pressed(KeyCode::KeyR) {
        Some(false)
    } else if keys.just_pressed(KeyCode::KeyU) {
        Some(true)
    } else {
        None
    };
    if let Some(uses) = wanted {
        match hovered_item(&game, &slots, &catalog) {
            Some(item) => game.recipe_view = Some((item, uses, 0)),
            None => game.recipe_view = None,
        }
    }
    if let Some((item, uses, page)) = game.recipe_view {
        let last = page_count(&game, item, uses).saturating_sub(1);
        if keys.just_pressed(KeyCode::ArrowRight) {
            game.recipe_view = Some((item, uses, (page + 1).min(last)));
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            game.recipe_view = Some((item, uses, page.saturating_sub(1)));
        }
        if keys.just_pressed(KeyCode::Backspace) {
            game.recipe_view = None;
        }
    }
}

fn cards(game: &Game, item: ItemId, uses: bool) -> Vec<RecipeCard> {
    recipe_view::find(item, uses, &game.recipes, &game.smelting, &game.content)
}

fn page_count(game: &Game, item: ItemId, uses: bool) -> usize {
    cards(game, item, uses).len().max(1)
}

fn nav_clicks(
    mut game: ResMut<Game>,
    mouse: Res<ButtonInput<MouseButton>>,
    cells: Query<(&Hovered, &RecipeCell)>,
    nav: Query<(&Hovered, &RecipeNav)>,
) {
    let Some((item, uses, page)) = game.recipe_view else {
        return;
    };
    let left = mouse.just_pressed(MouseButton::Left);
    let right = mouse.just_pressed(MouseButton::Right);
    if !left && !right {
        return;
    }
    if let Some((_, cell)) = cells.iter().find(|(h, _)| h.get()) {
        game.recipe_view = Some((cell.0, right, 0));
        return;
    }
    if let Some((_, action)) = nav.iter().find(|(h, _)| h.get()) {
        let last = page_count(&game, item, uses) - 1;
        game.recipe_view = match action {
            RecipeNav::Prev => Some((item, uses, page.saturating_sub(1))),
            RecipeNav::Next => Some((item, uses, (page + 1).min(last))),
            RecipeNav::Close => None,
        };
    }
}

fn item_name(game: &Game, item: ItemId) -> String {
    game.content
        .get(item)
        .map_or_else(|| "?".to_string(), |definition| definition.name.clone())
}

fn cell(
    commands: &mut Commands,
    parent: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
    item: ItemId,
    count: u16,
) {
    let slot = commands
        .spawn((
            RecipeCell(item),
            Hovered::default(),
            Node {
                width: px(48),
                height: px(48),
                flex_shrink: 0.0,
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(4)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(SURFACE),
            BorderColor::all(LINE),
        ))
        .id();
    let image = commands
        .spawn((
            ImageNode::new(icons.get(item, &game.content, blocks)),
            Node {
                width: px(36),
                height: px(36),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();
    commands.entity(slot).add_child(image);
    if count > 1 {
        let text = label(commands, slot, count.to_string(), 14.0, TEXT);
        commands.entity(text).insert(Node {
            position_type: PositionType::Absolute,
            right: px(2),
            bottom: px(0),
            ..default()
        });
    }
    commands.entity(parent).add_child(slot);
}

fn empty_cell(commands: &mut Commands, parent: Entity) {
    let slot = commands
        .spawn((
            Node {
                width: px(48),
                height: px(48),
                flex_shrink: 0.0,
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(4)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.12, 0.22, 0.20, 0.35)),
            BorderColor::all(LINE),
            Pickable::IGNORE,
        ))
        .id();
    commands.entity(parent).add_child(slot);
}

fn row(commands: &mut Commands, parent: Entity, gap: f32) -> Entity {
    let entity = commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(gap),
            ..default()
        })
        .id();
    commands.entity(parent).add_child(entity);
    entity
}

fn card_ui(
    commands: &mut Commands,
    parent: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
    card: &RecipeCard,
) {
    let line = row(commands, parent, 14.0);
    match card {
        RecipeCard::Crafting(view) => {
            label(
                commands,
                parent,
                if view.width <= 2 && view.height <= 2 {
                    "Сетка 2×2 или верстак"
                } else {
                    "Верстак"
                },
                13.0,
                MUTED,
            );
            let grid = commands
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: px(4),
                    ..default()
                })
                .id();
            commands.entity(line).add_child(grid);
            for y in 0..view.height {
                let r = row(commands, grid, 4.0);
                for x in 0..view.width {
                    match view.inputs[y * view.width + x] {
                        Some((item, count)) => cell(commands, r, game, icons, blocks, item, count),
                        None => empty_cell(commands, r),
                    }
                }
            }
            let arrow = label(commands, line, "→", 30.0, AMBER);
            let _ = arrow;
            cell(
                commands,
                line,
                game,
                icons,
                blocks,
                view.output.0,
                view.output.1,
            );
        }
        RecipeCard::Smelting { input, output } => {
            label(commands, parent, "Печь", 13.0, MUTED);
            cell(commands, line, game, icons, blocks, *input, 1);
            label(commands, line, "→", 30.0, AMBER);
            cell(commands, line, game, icons, blocks, *output, 1);
        }
        RecipeCard::Stonecutting { input, output } => {
            label(commands, parent, "Камнерез", 13.0, MUTED);
            cell(commands, line, game, icons, blocks, *input, 1);
            label(commands, line, "→", 30.0, AMBER);
            cell(commands, line, game, icons, blocks, *output, 1);
        }
        RecipeCard::Fuel { item, seconds } => {
            label(commands, parent, "Топливо для печи", 13.0, MUTED);
            cell(commands, line, game, icons, blocks, *item, 1);
            label(commands, line, format!("горит {seconds} с"), 18.0, TEXT);
        }
    }
}

fn nav_button(commands: &mut Commands, parent: Entity, text: &str, nav: RecipeNav) {
    let button = commands
        .spawn((
            nav,
            Hovered::default(),
            Node {
                min_width: px(40),
                min_height: px(34),
                padding: UiRect::axes(px(10), px(4)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(4)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(SURFACE),
            BorderColor::all(LINE),
        ))
        .id();
    let text = label(commands, button, text, 16.0, TEXT);
    commands.entity(text).insert(Pickable::IGNORE);
    commands.entity(parent).add_child(button);
}

fn sync_overlay(
    mut commands: Commands,
    game: Res<Game>,
    icons: Res<ItemIcons>,
    blocks: Res<BlockIcons>,
    existing: Query<Entity, With<RecipeOverlay>>,
    mut shown: Local<Option<(ItemId, bool, usize)>>,
) {
    let wanted = if game.mode == Mode::Inventory {
        game.recipe_view
    } else {
        None
    };
    if *shown == wanted && (wanted.is_none() == existing.is_empty()) {
        return;
    }
    for entity in &existing {
        commands.entity(entity).despawn();
    }
    *shown = wanted;
    let Some((item, uses, page)) = wanted else {
        return;
    };
    let all = cards(&game, item, uses);
    let root = commands
        .spawn((
            RecipeOverlay,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            GlobalZIndex(60),
            Pickable::IGNORE,
        ))
        .id();
    let panel = commands
        .spawn((
            Node {
                min_width: px(420),
                flex_direction: FlexDirection::Column,
                row_gap: px(10),
                padding: UiRect::all(px(16)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(6)),
                ..default()
            },
            BackgroundColor(PANEL),
            BorderColor::all(AMBER),
        ))
        .id();
    commands.entity(root).add_child(panel);
    let head = row(&mut commands, panel, 10.0);
    let title = format!(
        "{}: {}",
        if uses {
            "Где используется"
        } else {
            "Как получить"
        },
        item_name(&game, item)
    );
    let text = label(&mut commands, head, title, 18.0, AMBER);
    commands.entity(text).insert(Node {
        flex_grow: 1.0,
        ..default()
    });
    nav_button(&mut commands, head, "×", RecipeNav::Close);
    match all.get(page) {
        Some(card) => card_ui(&mut commands, panel, &game, &icons, &blocks, card),
        None => {
            label(&mut commands, panel, "Рецептов нет", 16.0, MUTED);
        }
    }
    let foot = row(&mut commands, panel, 10.0);
    nav_button(&mut commands, foot, "◀", RecipeNav::Prev);
    label(
        &mut commands,
        foot,
        format!(
            "{}/{}",
            page.min(all.len().saturating_sub(1)) + 1,
            all.len().max(1)
        ),
        16.0,
        TEXT,
    );
    nav_button(&mut commands, foot, "▶", RecipeNav::Next);
    label(
        &mut commands,
        panel,
        "R — как получить, U — где используется (над предметом). ЛКМ по предмету в рецепте — его рецепты, ПКМ — применение. ←/→ — страницы, Backspace — закрыть.",
        12.0,
        MUTED,
    );
}
