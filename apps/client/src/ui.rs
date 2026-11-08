//! Native Bevy menus and the in-game overlay. Screen changes are handled by the client.
use crate::session::{Game, Mode};
use bevy::{
    asset::uuid_handle,
    input::{
        ButtonState,
        keyboard::KeyboardInput,
        mouse::{MouseScrollUnit, MouseWheel},
    },
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::Button,
};

#[derive(Clone, Debug)]
pub enum Action {
    New,
    Create,
    LoadMenu,
    Load(String),
    Resume,
    Save,
    MainMenu,
    Exit,
    Settings,
    Back,
    SelectBlock(u8),
    SelectSlot(usize),
    CycleDistance,
    CycleSensitivity,
    ToggleVsync,
    ToggleFullscreen,
    CycleResolution,
    CycleVolume,
    FocusName,
    FocusSeed,
}

#[derive(Resource, Default)]
pub struct UiIntent(pub Vec<Action>);

const UI_FONT: Handle<Font> = uuid_handle!("d809b322-114c-4312-b95a-0e57969c514e");
fn setup_font(mut fonts: ResMut<Assets<Font>>) {
    fonts
        .insert(
            UI_FONT.id(),
            Font::from_bytes(include_bytes!("../assets/fonts/DejaVuSansMono.ttf").to_vec()),
        )
        .expect("insert UI font");
}
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_font);
    app.init_resource::<UiIntent>().add_systems(
        Update,
        (
            button_input,
            edit_world_form,
            scroll_panels,
            rebuild,
            update_hud,
        )
            .chain(),
    );
}

const INK: Color = Color::srgb(0.055, 0.075, 0.086);
const PANEL: Color = Color::srgb(0.095, 0.122, 0.136);
const SURFACE: Color = Color::srgb(0.145, 0.18, 0.195);
const LINE: Color = Color::srgb(0.30, 0.37, 0.39);
const TEXT: Color = Color::srgb(0.94, 0.94, 0.88);
const MUTED: Color = Color::srgb(0.66, 0.75, 0.75);
const AMBER: Color = Color::srgb(0.96, 0.68, 0.31);
const TEAL: Color = Color::srgb(0.35, 0.80, 0.71);

#[derive(Component)]
struct UiRoot;
#[derive(Component)]
struct ScrollArea;
#[derive(Clone, Copy)]
enum Tone {
    Primary,
    Normal,
    Quiet,
    Selected,
}
#[derive(Component)]
struct UiButton {
    action: Action,
    tone: Tone,
}
#[derive(Component)]
struct HotbarTile(usize);
#[derive(Component)]
struct SlotLabel(usize);
#[derive(Component)]
struct SlotSwatch(usize);
#[derive(Component)]
struct HudText(HudKind);
#[derive(Component)]
struct PanelStatus;
enum HudKind {
    Position,
    Status,
    Block,
}

fn node(commands: &mut Commands, parent: Entity, layout: Node, color: Color) -> Entity {
    let child = commands.spawn((layout, BackgroundColor(color))).id();
    commands.entity(parent).add_child(child);
    child
}

fn label(
    commands: &mut Commands,
    parent: Entity,
    value: impl Into<String>,
    size: f32,
    color: Color,
) -> Entity {
    let child = commands
        .spawn((
            Text::new(value),
            TextFont {
                font: UI_FONT.clone().into(),
                font_size: FontSize::Px(size),
                ..default()
            },
            TextColor(color),
            Node {
                flex_shrink: 0.0,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();
    commands.entity(parent).add_child(child);
    child
}

fn column(commands: &mut Commands, parent: Entity, gap: f32) -> Entity {
    node(
        commands,
        parent,
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(gap),
            flex_shrink: 0.0,
            ..default()
        },
        Color::NONE,
    )
}

fn row(commands: &mut Commands, parent: Entity, gap: f32) -> Entity {
    node(
        commands,
        parent,
        Node {
            width: percent(100),
            align_items: AlignItems::Center,
            column_gap: px(gap),
            flex_shrink: 0.0,
            ..default()
        },
        Color::NONE,
    )
}

fn colors(tone: Tone, hover: bool) -> (Color, Color, Color) {
    match tone {
        Tone::Primary => (
            if hover {
                Color::srgb(1.0, 0.77, 0.43)
            } else {
                AMBER
            },
            AMBER,
            INK,
        ),
        Tone::Selected => (
            if hover {
                Color::srgb(0.14, 0.36, 0.33)
            } else {
                Color::srgb(0.10, 0.27, 0.25)
            },
            TEAL,
            TEXT,
        ),
        Tone::Normal => (
            if hover {
                Color::srgb(0.21, 0.28, 0.29)
            } else {
                SURFACE
            },
            if hover { TEAL } else { LINE },
            TEXT,
        ),
        Tone::Quiet => (
            if hover { SURFACE } else { PANEL },
            if hover { MUTED } else { LINE },
            MUTED,
        ),
    }
}

fn button(
    commands: &mut Commands,
    parent: Entity,
    value: impl Into<String>,
    action: Action,
    tone: Tone,
) -> Entity {
    let (background, border, foreground) = colors(tone, false);
    let child = commands
        .spawn((
            Button,
            Hovered::default(),
            UiButton { action, tone },
            Node {
                width: percent(100),
                min_height: px(46),
                flex_shrink: 0.0,
                padding: UiRect::axes(px(16), px(10)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(4)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(background),
            BorderColor::all(border),
        ))
        .id();
    commands.entity(parent).add_child(child);
    label(commands, child, value, 17.0, foreground);
    child
}

fn button_input(
    mouse: Res<ButtonInput<MouseButton>>,
    mut intents: ResMut<UiIntent>,
    mut buttons: Query<(&Hovered, &UiButton, &mut BackgroundColor, &mut BorderColor), With<Button>>,
) {
    for (hovered, button, mut background, mut border) in &mut buttons {
        let (base, edge, _) = colors(button.tone, hovered.get());
        let pressed = hovered.get() && mouse.pressed(MouseButton::Left);
        let color = if pressed {
            Color::srgb(0.25, 0.48, 0.43)
        } else {
            base
        };
        if background.0 != color {
            background.0 = color;
        }
        border.set_all(if pressed { TEAL } else { edge });
        if hovered.get() && mouse.just_pressed(MouseButton::Left) {
            intents.0.push(button.action.clone());
        }
    }
}

fn edit_world_form(
    mut keyboard: MessageReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut intents: ResMut<UiIntent>,
) {
    for event in keyboard.read() {
        if game.mode != Mode::Create || event.state != ButtonState::Pressed {
            continue;
        }
        match event.key_code {
            KeyCode::Tab if !event.repeat => {
                game.input_field = (game.input_field + 1) % 2;
                game.ui_dirty = true;
            }
            KeyCode::Enter | KeyCode::NumpadEnter if !event.repeat => {
                intents.0.push(Action::Create)
            }
            KeyCode::Backspace => {
                let value = if game.input_field == 0 {
                    &mut game.name_input
                } else {
                    &mut game.seed_input
                };
                if value.pop().is_some() {
                    game.ui_dirty = true;
                }
            }
            _ => {
                if keys.any_pressed([
                    KeyCode::ControlLeft,
                    KeyCode::ControlRight,
                    KeyCode::AltLeft,
                    KeyCode::AltRight,
                ]) {
                    continue;
                }
                let Some(value) = &event.text else {
                    continue;
                };
                let is_name = game.input_field == 0;
                let target = if is_name {
                    &mut game.name_input
                } else {
                    &mut game.seed_input
                };
                let mut count = target.chars().count();
                let limit = if is_name { 40 } else { 20 };
                let mut changed = false;
                for ch in value.chars() {
                    if count >= limit {
                        break;
                    }
                    if ch.is_control() || (!is_name && !ch.is_ascii_digit()) {
                        continue;
                    }
                    target.push(ch);
                    count += 1;
                    changed = true;
                }
                if changed {
                    game.ui_dirty = true;
                }
            }
        }
    }
}

fn scroll_panels(
    mut wheel: MessageReader<MouseWheel>,
    mut panels: Query<(&Hovered, &mut ScrollPosition, &ComputedNode), With<ScrollArea>>,
) {
    let delta: f32 = wheel
        .read()
        .map(|event| {
            -event.y
                * if event.unit == MouseScrollUnit::Line {
                    38.0
                } else {
                    1.0
                }
        })
        .sum();
    if delta == 0.0 {
        return;
    }
    for (hovered, mut scroll, computed) in &mut panels {
        if hovered.get() {
            let max = ((computed.content_size() - computed.size())
                * computed.inverse_scale_factor())
            .y
            .max(0.0);
            scroll.y = (scroll.y + delta).clamp(0.0, max);
        }
    }
}

fn rebuild(mut commands: Commands, mut game: ResMut<Game>, old: Query<Entity, With<UiRoot>>) {
    if !game.ui_dirty {
        return;
    }
    game.ui_dirty = false;
    for entity in &old {
        commands.entity(entity).despawn();
    }
    let playing = game.mode == Mode::Playing;
    let root = commands
        .spawn((
            UiRoot,
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(if playing {
                Color::NONE
            } else {
                Color::srgba(0.035, 0.055, 0.066, 0.86)
            }),
        ))
        .id();
    if playing {
        hud(&mut commands, root, &game);
        return;
    }
    if game.mode == Mode::Menu {
        main_menu(&mut commands, root, &game);
        return;
    }
    let width = if game.mode == Mode::Inventory {
        790.0
    } else {
        540.0
    };
    let panel = card(&mut commands, root, width);
    match game.mode {
        Mode::Create => create_menu(&mut commands, panel, &game),
        Mode::Load => load_menu(&mut commands, panel, &game),
        Mode::Settings => settings_menu(&mut commands, panel, &game),
        Mode::Paused => pause_menu(&mut commands, panel, &game),
        Mode::Inventory => inventory_menu(&mut commands, panel, &game),
        Mode::Menu | Mode::Playing => {}
    }
}

fn card(commands: &mut Commands, parent: Entity, width: f32) -> Entity {
    let card = node(
        commands,
        parent,
        Node {
            width: percent(92),
            max_width: px(width),
            max_height: Val::Vh(92.0),
            padding: UiRect::all(px(if width > 700.0 { 24.0 } else { 28.0 })),
            border: UiRect::all(px(1)),
            border_radius: BorderRadius::all(px(8)),
            flex_direction: FlexDirection::Column,
            row_gap: px(if width > 700.0 { 12.0 } else { 16.0 }),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        PANEL,
    );
    commands.entity(card).insert((
        BorderColor::all(LINE),
        Hovered::default(),
        ScrollPosition::default(),
        ScrollArea,
        BoxShadow::new(
            Color::srgba(0.0, 0.0, 0.0, 0.4),
            px(0),
            px(16),
            px(0),
            px(40),
        ),
    ));
    card
}

fn heading(commands: &mut Commands, parent: Entity, eyebrow: &str, title: &str, subtitle: &str) {
    let header = column(commands, parent, 7.0);
    label(commands, header, eyebrow, 12.0, TEAL);
    label(commands, header, title, 32.0, TEXT);
    if !subtitle.is_empty() {
        label(commands, header, subtitle, 15.0, MUTED);
    }
    node(
        commands,
        parent,
        Node {
            width: percent(100),
            height: px(1),
            flex_shrink: 0.0,
            margin: UiRect::vertical(px(3)),
            ..default()
        },
        LINE,
    );
}

fn status(commands: &mut Commands, parent: Entity, game: &Game) {
    let text = label(commands, parent, &game.status, 14.0, AMBER);
    commands.entity(text).insert(PanelStatus);
}

fn main_menu(commands: &mut Commands, root: Entity, game: &Game) {
    let frame = node(
        commands,
        root,
        Node {
            width: percent(92),
            max_width: px(1030),
            align_items: AlignItems::Center,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::Center,
            column_gap: px(64),
            row_gap: px(24),
            ..default()
        },
        Color::NONE,
    );
    let hero = node(
        commands,
        frame,
        Node {
            width: px(480),
            max_width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(18),
            ..default()
        },
        Color::NONE,
    );
    let mark = row(commands, hero, 5.0);
    for (height, color) in [
        (22.0, TEAL),
        (36.0, AMBER),
        (28.0, Color::srgb(0.52, 0.64, 0.50)),
    ] {
        node(
            commands,
            mark,
            Node {
                width: px(22),
                height: px(height),
                align_self: AlignSelf::End,
                ..default()
            },
            color,
        );
    }
    label(commands, hero, "VOXEL SANDBOX", 14.0, TEAL);
    label(commands, hero, "Строй свой\nбольшой мир.", 53.0, TEXT);
    label(commands, hero, "Исследуй мир. Создавай своё.", 18.0, MUTED);
    let tags = row(commands, hero, 12.0);
    for value in ["МИРЫ ПО SEED", "ТВОРЧЕСКИЙ РЕЖИМ"] {
        let tag = node(
            commands,
            tags,
            Node {
                padding: UiRect::axes(px(10), px(7)),
                border: UiRect::all(px(1)),
                ..default()
            },
            PANEL,
        );
        commands.entity(tag).insert(BorderColor::all(LINE));
        label(commands, tag, value, 11.0, MUTED);
    }
    let menu = card(commands, frame, 370.0);
    commands.entity(menu).insert(Node {
        width: px(370),
        max_width: percent(100),
        max_height: Val::Vh(88.0),
        padding: UiRect::all(px(26)),
        flex_direction: FlexDirection::Column,
        row_gap: px(12),
        border: UiRect::all(px(1)),
        border_radius: BorderRadius::all(px(8)),
        overflow: Overflow::scroll_y(),
        ..default()
    });
    label(commands, menu, "С ВОЗВРАЩЕНИЕМ", 12.0, MUTED);
    label(commands, menu, "Твоё приключение", 24.0, TEXT);
    button(
        commands,
        menu,
        if game.world_id.is_some() {
            "Продолжить"
        } else {
            "Играть"
        },
        if game.world_id.is_some() {
            Action::Resume
        } else {
            Action::New
        },
        Tone::Primary,
    );
    button(commands, menu, "Создать мир", Action::New, Tone::Normal);
    button(
        commands,
        menu,
        "Загрузить мир",
        Action::LoadMenu,
        Tone::Normal,
    );
    button(commands, menu, "Настройки", Action::Settings, Tone::Normal);
    button(commands, menu, "Выход", Action::Exit, Tone::Quiet);
    status(commands, menu, game);
    let footer = node(
        commands,
        root,
        Node {
            position_type: PositionType::Absolute,
            left: px(28),
            bottom: px(20),
            ..default()
        },
        Color::NONE,
    );
    label(
        commands,
        footer,
        "Исследуй. Строй. Сохраняй свой мир.",
        12.0,
        MUTED,
    );
}

fn create_menu(commands: &mut Commands, panel: Entity, game: &Game) {
    heading(
        commands,
        panel,
        "НОВОЕ НАЧАЛО",
        "Создание мира",
        "Придумай название своему миру.",
    );
    let form = column(commands, panel, 9.0);
    label(commands, form, "Название мира", 14.0, MUTED);
    let name = if game.input_field == 0 {
        format!("{} |", game.name_input)
    } else {
        game.name_input.clone()
    };
    button(
        commands,
        form,
        name,
        Action::FocusName,
        if game.input_field == 0 {
            Tone::Selected
        } else {
            Tone::Normal
        },
    );
    label(commands, form, "Зерно мира (необязательно)", 14.0, MUTED);
    let seed = if game.seed_input.is_empty() {
        if game.input_field == 1 {
            "|".into()
        } else {
            "Случайный мир".into()
        }
    } else if game.input_field == 1 {
        format!("{} |", game.seed_input)
    } else {
        game.seed_input.clone()
    };
    button(
        commands,
        form,
        seed,
        Action::FocusSeed,
        if game.input_field == 1 {
            Tone::Selected
        } else {
            Tone::Normal
        },
    );
    label(
        commands,
        form,
        "Одинаковое зерно создаёт одинаковый мир.\nTab: сменить поле. Enter: создать мир.",
        13.0,
        MUTED,
    );
    status(commands, panel, game);
    button(
        commands,
        panel,
        "Создать и исследовать",
        Action::Create,
        Tone::Primary,
    );
    button(commands, panel, "Назад", Action::Back, Tone::Quiet);
}

fn load_menu(commands: &mut Commands, panel: Entity, game: &Game) {
    heading(
        commands,
        panel,
        "ПРОДОЛЖИ СВОЮ ИСТОРИЮ",
        "Твои миры",
        "Твои постройки ждут тебя.",
    );
    if game.worlds.is_empty() {
        label(
            commands,
            panel,
            "Сохранённых миров пока нет.\nСоздай свой первый мир.",
            18.0,
            TEXT,
        );
        button(
            commands,
            panel,
            "Создать первый мир",
            Action::New,
            Tone::Primary,
        );
    } else {
        label(
            commands,
            panel,
            format!("Миров: {}{}", game.worlds.len(), ""),
            13.0,
            MUTED,
        );
        let list = column(commands, panel, 8.0);
        for (id, name) in &game.worlds {
            button(
                commands,
                list,
                format!("{}   /   Открыть", shorten(name, 34)),
                Action::Load(id.clone()),
                Tone::Normal,
            );
        }
        label(commands, panel, "Колёсико: прокрутить список.", 12.0, MUTED);
    }
    status(commands, panel, game);
    button(commands, panel, "Назад", Action::Back, Tone::Quiet);
}

fn pause_menu(commands: &mut Commands, panel: Entity, game: &Game) {
    heading(
        commands,
        panel,
        "ВРЕМЯ ПЕРЕДОХНУТЬ",
        "Игра на паузе",
        &shorten(&game.name_input, 40),
    );
    button(
        commands,
        panel,
        "Продолжить игру",
        Action::Resume,
        Tone::Primary,
    );
    button(commands, panel, "Сохранить мир", Action::Save, Tone::Normal);
    button(commands, panel, "Настройки", Action::Settings, Tone::Normal);
    button(
        commands,
        panel,
        "Главное меню",
        Action::MainMenu,
        Tone::Quiet,
    );
    status(commands, panel, game);
    label(commands, panel, "Esc: продолжить", 12.0, MUTED);
}

fn settings_row(commands: &mut Commands, panel: Entity, name: &str, value: String, action: Action) {
    let row = row(commands, panel, 12.0);
    let caption = label(commands, row, name, 16.0, TEXT);
    commands.entity(caption).insert(Node {
        flex_grow: 1.0,
        ..default()
    });
    let control = button(commands, row, value, action, Tone::Normal);
    commands.entity(control).insert(Node {
        width: px(184),
        min_height: px(44),
        padding: UiRect::axes(px(10), px(9)),
        border: UiRect::all(px(1)),
        border_radius: BorderRadius::all(px(4)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_shrink: 0.0,
        ..default()
    });
}

fn settings_menu(commands: &mut Commands, panel: Entity, game: &Game) {
    heading(
        commands,
        panel,
        "ИГРАЙ ПО-СВОЕМУ",
        "Настройки",
        "Нажми на значение, чтобы изменить его.",
    );
    let settings = &game.settings;
    settings_row(
        commands,
        panel,
        "Дальность прорисовки",
        format!("{} чанков", settings.distance),
        Action::CycleDistance,
    );
    settings_row(
        commands,
        panel,
        "Чувствительность мыши",
        format!("{:.0}%", settings.sensitivity / 0.002 * 100.0),
        Action::CycleSensitivity,
    );
    settings_row(
        commands,
        panel,
        "Вертикальная синхронизация",
        if settings.vsync {
            "Вкл."
        } else {
            "Выкл."
        }
        .into(),
        Action::ToggleVsync,
    );
    settings_row(
        commands,
        panel,
        "Режим экрана",
        if settings.fullscreen {
            "Полный экран"
        } else {
            "Оконный"
        }
        .into(),
        Action::ToggleFullscreen,
    );
    settings_row(
        commands,
        panel,
        "Оконное разрешение",
        ["1280 x 720", "1600 x 900", "1920 x 1080"][settings.resolution % 3].into(),
        Action::CycleResolution,
    );
    settings_row(
        commands,
        panel,
        "Громкость",
        format!("{:.0}%", settings.volume * 100.0),
        Action::CycleVolume,
    );
    label(
        commands,
        panel,
        "Настройки сохраняются автоматически.",
        12.0,
        MUTED,
    );
    status(commands, panel, game);
    button(commands, panel, "Готово", Action::Back, Tone::Primary);
}

fn inventory_menu(commands: &mut Commands, panel: Entity, game: &Game) {
    heading(
        commands,
        panel,
        "МАТЕРИАЛЫ ДЛЯ СТРОИТЕЛЬСТВА",
        "Творческий инвентарь",
        "Выбери слот, затем блок для него.",
    );
    let slots = row(commands, panel, 6.0);
    for index in 0..9 {
        let child = button(
            commands,
            slots,
            (index + 1).to_string(),
            Action::SelectSlot(index),
            if index == game.selected {
                Tone::Selected
            } else {
                Tone::Normal
            },
        );
        commands.entity(child).insert(Node {
            flex_grow: 1.0,
            min_width: px(34),
            min_height: px(42),
            border: UiRect::all(px(1)),
            border_radius: BorderRadius::all(px(4)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        });
    }
    label(
        commands,
        panel,
        format!(
            "СЛОТ {}  /  {}",
            game.selected + 1,
            block_name(game.hotbar[game.selected.min(8)])
        ),
        13.0,
        TEAL,
    );
    let grid = node(
        commands,
        panel,
        Node {
            width: percent(100),
            flex_wrap: FlexWrap::Wrap,
            column_gap: px(8),
            row_gap: px(8),
            flex_shrink: 0.0,
            ..default()
        },
        Color::NONE,
    );
    for id in 1..=15 {
        if id == 13 {
            continue; // The world boundary is not an obtainable building block.
        }
        let selected = game.hotbar[game.selected.min(8)] == id;
        let child = button(
            commands,
            grid,
            block_name(id),
            Action::SelectBlock(id),
            if selected {
                Tone::Selected
            } else {
                Tone::Normal
            },
        );
        commands.entity(child).insert(Node {
            width: percent(18.8),
            min_height: px(70),
            padding: UiRect::all(px(6)),
            flex_direction: FlexDirection::Column,
            row_gap: px(7),
            border: UiRect::all(px(1)),
            border_radius: BorderRadius::all(px(4)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_shrink: 0.0,
            ..default()
        });
        let swatch = node(
            commands,
            child,
            Node {
                width: px(27),
                height: px(18),
                ..default()
            },
            block_color(id),
        );
        commands.entity(swatch).insert(Pickable::IGNORE);
    }
    button(
        commands,
        panel,
        "Вернуться в мир",
        Action::Resume,
        Tone::Primary,
    );
    label(
        commands,
        panel,
        "E / Esc: закрыть. В мире: 1-9 или колёсико.",
        12.0,
        MUTED,
    );
}

fn hud(commands: &mut Commands, root: Entity, game: &Game) {
    commands.entity(root).insert(Pickable::IGNORE);
    let info = node(
        commands,
        root,
        Node {
            position_type: PositionType::Absolute,
            left: px(22),
            top: px(20),
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
            padding: UiRect::axes(px(14), px(10)),
            border_radius: BorderRadius::all(px(4)),
            ..default()
        },
        Color::srgba(0.045, 0.07, 0.08, 0.74),
    );
    label(
        commands,
        info,
        "VOXEL SANDBOX   /   СТРОЙ И ИССЛЕДУЙ",
        11.0,
        TEAL,
    );
    let position = label(commands, info, "", 14.0, TEXT);
    commands.entity(position).insert(HudText(HudKind::Position));
    let status = label(commands, info, "", 12.0, MUTED);
    commands.entity(status).insert(HudText(HudKind::Status));
    let controls = node(
        commands,
        root,
        Node {
            position_type: PositionType::Absolute,
            right: px(22),
            top: px(20),
            flex_direction: FlexDirection::Column,
            row_gap: px(4),
            padding: UiRect::axes(px(14), px(10)),
            border_radius: BorderRadius::all(px(4)),
            ..default()
        },
        Color::srgba(0.045, 0.07, 0.08, 0.68),
    );
    label(
        commands,
        controls,
        "WASD: движение / Shift: бег / Space: прыжок",
        12.0,
        TEXT,
    );
    label(
        commands,
        controls,
        "ЛКМ: сломать / ПКМ: поставить / E: блоки",
        12.0,
        MUTED,
    );
    label(
        commands,
        controls,
        "Space x2: полёт / Ctrl: вниз / Space: вверх",
        12.0,
        MUTED,
    );
    label(
        commands,
        controls,
        "F3: данные / F3+G: чанки / Esc: пауза",
        12.0,
        MUTED,
    );
    let crosshair = node(
        commands,
        root,
        Node {
            position_type: PositionType::Absolute,
            left: percent(50),
            top: percent(50),
            width: px(20),
            height: px(20),
            margin: UiRect {
                left: px(-10),
                top: px(-10),
                ..default()
            },
            ..default()
        },
        Color::NONE,
    );
    commands.entity(crosshair).insert(Pickable::IGNORE);
    for (width, height, x, y, color) in [
        (20.0, 3.0, 0.0, 8.0, Color::srgba(0.0, 0.0, 0.0, 0.7)),
        (3.0, 20.0, 8.0, 0.0, Color::srgba(0.0, 0.0, 0.0, 0.7)),
        (18.0, 1.0, 1.0, 9.0, TEXT),
        (1.0, 18.0, 9.0, 1.0, TEXT),
    ] {
        let line = node(
            commands,
            crosshair,
            Node {
                position_type: PositionType::Absolute,
                width: px(width),
                height: px(height),
                left: px(x),
                top: px(y),
                ..default()
            },
            color,
        );
        commands.entity(line).insert(Pickable::IGNORE);
    }
    let bar_frame = node(
        commands,
        root,
        Node {
            position_type: PositionType::Absolute,
            bottom: px(22),
            width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(9),
            align_items: AlignItems::Center,
            ..default()
        },
        Color::NONE,
    );
    let selected = label(commands, bar_frame, "", 15.0, TEXT);
    commands
        .entity(selected)
        .insert((HudText(HudKind::Block), TextShadow::default()));
    let bar = node(
        commands,
        bar_frame,
        Node {
            padding: UiRect::all(px(6)),
            column_gap: px(5),
            border_radius: BorderRadius::all(px(6)),
            border: UiRect::all(px(1)),
            ..default()
        },
        Color::srgba(0.045, 0.065, 0.075, 0.9),
    );
    commands.entity(bar).insert(BorderColor::all(LINE));
    for index in 0..9 {
        let tile = node(
            commands,
            bar,
            Node {
                width: px(58),
                height: px(61),
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::all(px(3)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(4),
                ..default()
            },
            SURFACE,
        );
        commands
            .entity(tile)
            .insert((HotbarTile(index), BorderColor::all(LINE)));
        let badge = label(commands, tile, (index + 1).to_string(), 11.0, MUTED);
        commands.entity(badge).insert(Node {
            position_type: PositionType::Absolute,
            left: px(4),
            top: px(2),
            ..default()
        });
        let swatch = node(
            commands,
            tile,
            Node {
                width: px(24),
                height: px(19),
                margin: UiRect::top(px(5)),
                ..default()
            },
            block_color(game.hotbar[index]),
        );
        commands.entity(swatch).insert(SlotSwatch(index));
        let name = label(
            commands,
            tile,
            block_short_name(game.hotbar[index]),
            10.0,
            TEXT,
        );
        commands.entity(name).insert(SlotLabel(index));
    }
}

type HudTextData = (
    &'static mut Text,
    Option<&'static HudText>,
    Option<&'static SlotLabel>,
);

fn update_hud(
    game: Res<Game>,
    mut text: Query<HudTextData, Without<PanelStatus>>,
    mut panel_status: Query<&mut Text, With<PanelStatus>>,
    mut tiles: Query<(&HotbarTile, &mut BorderColor, &mut BackgroundColor), Without<SlotSwatch>>,
    mut swatches: Query<(&SlotSwatch, &mut BackgroundColor), Without<HotbarTile>>,
) {
    for mut text in &mut panel_status {
        if text.0 != game.status {
            text.0.clone_from(&game.status);
        }
    }
    if game.mode != Mode::Playing {
        return;
    }
    for (mut value, marker, slot) in &mut text {
        let next = if let Some(marker) = marker {
            match marker.0 {
                HudKind::Position => {
                    if game.debug_open {
                        format!(
                            "X {:.2}  Y {:.2}  Z {:.2}\nFPS: {:.0}  |  Чанк: {} / {}\nПолёт: {}  |  Сетка: {}",
                            game.feet.x,
                            game.feet.y,
                            game.feet.z,
                            game.fps,
                            (game.feet.x.floor() as i32).div_euclid(16),
                            (game.feet.z.floor() as i32).div_euclid(16),
                            if game.flying { "вкл." } else { "выкл." },
                            if game.chunk_grid {
                                "вкл."
                            } else {
                                "выкл."
                            }
                        )
                    } else {
                        String::new()
                    }
                }
                HudKind::Status => {
                    if game.status.is_empty() {
                        format!(
                            "Зерно {} / Видимость: {} чанков",
                            game.world.seed, game.settings.distance
                        )
                    } else {
                        game.status.clone()
                    }
                }
                HudKind::Block => format!(
                    "{}   /   {}",
                    game.selected + 1,
                    block_name(game.hotbar[game.selected.min(8)])
                ),
            }
        } else if let Some(slot) = slot {
            block_short_name(game.hotbar[slot.0]).into()
        } else {
            continue;
        };
        if value.0 != next {
            value.0 = next;
        }
    }
    for (tile, mut border, mut background) in &mut tiles {
        let selected = tile.0 == game.selected;
        border.set_all(if selected { AMBER } else { LINE });
        let next = if selected {
            Color::srgb(0.28, 0.25, 0.18)
        } else {
            SURFACE
        };
        if background.0 != next {
            background.0 = next;
        }
    }
    for (slot, mut background) in &mut swatches {
        let next = block_color(game.hotbar[slot.0]);
        if background.0 != next {
            background.0 = next;
        }
    }
}

fn shorten(value: &str, limit: usize) -> String {
    if value.chars().count() <= limit {
        value.into()
    } else {
        format!(
            "{}...",
            value
                .chars()
                .take(limit.saturating_sub(3))
                .collect::<String>()
        )
    }
}

fn block_name(id: u8) -> &'static str {
    [
        "Воздух",
        "Трава",
        "Земля",
        "Камень",
        "Глуб. камень",
        "Песок",
        "Гравий",
        "Дерево",
        "Листва",
        "Доски",
        "Булыжник",
        "Стекло",
        "Вода",
        "Коренная порода",
        "Снег",
        "Песчаник",
    ]
    .get(id as usize)
    .copied()
    .unwrap_or("Неизвестно")
}
fn block_short_name(id: u8) -> &'static str {
    match id {
        4 => "Глуб.",
        10 => "Булыж.",
        15 => "Песчан.",
        _ => block_name(id),
    }
}
fn block_color(id: u8) -> Color {
    let rgb = match id {
        1 => [0.40, 0.63, 0.28],
        2 => [0.51, 0.34, 0.23],
        3 => [0.55, 0.58, 0.59],
        4 => [0.26, 0.29, 0.34],
        5 => [0.87, 0.79, 0.51],
        6 => [0.54, 0.50, 0.44],
        7 => [0.48, 0.33, 0.20],
        8 => [0.24, 0.53, 0.27],
        9 => [0.70, 0.51, 0.30],
        10 => [0.42, 0.46, 0.45],
        11 => [0.65, 0.85, 0.87],
        12 => [0.25, 0.52, 0.78],
        13 => [0.24, 0.25, 0.26],
        14 => [0.91, 0.95, 0.95],
        15 => [0.78, 0.66, 0.40],
        _ => [0.2, 0.2, 0.2],
    };
    Color::srgb(rgb[0], rgb[1], rgb[2])
}
