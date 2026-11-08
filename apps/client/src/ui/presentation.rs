//! Embedded menu art and native pointer-driven settings controls.
use super::*;
use bevy::ui::RelativeCursorPosition;
use bevy::{
    asset::RenderAssetUsages,
    image::{CompressedImageFormats, ImageSampler, ImageType},
};
const LANDSCAPE: Handle<Image> = uuid_handle!("059786e5-7722-43e8-980b-3dc228a01eae");
#[derive(Component)]
struct Backdrop;
#[derive(Component, Clone, Copy, PartialEq)]
pub(super) enum Range {
    Sensitivity,
    Volume,
}
#[derive(Component)]
struct RangeFill(Range);
#[derive(Component)]
struct RangeKnob(Range);
#[derive(Component)]
struct RangeText(Range);
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, load_art)
        .add_systems(Update, (fit_backdrop, range_input, range_view).chain());
}
fn load_art(mut images: ResMut<Assets<Image>>) {
    let image = Image::from_buffer(
        include_bytes!("../../assets/menu/landscape.png"),
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        true,
        ImageSampler::linear(),
        RenderAssetUsages::default(),
    )
    .expect("embedded menu art");
    images
        .insert(LANDSCAPE.id(), image)
        .expect("insert menu art");
}
pub(super) fn background(commands: &mut Commands, parent: Entity) {
    let image = commands
        .spawn((
            ImageNode {
                image: LANDSCAPE.clone(),
                image_mode: NodeImageMode::Stretch,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                left: px(0),
                top: px(0),
                ..default()
            },
            ZIndex(-2),
            Pickable::IGNORE,
            Backdrop,
        ))
        .id();
    commands.entity(parent).add_child(image);
    let shade = node(
        commands,
        parent,
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            left: px(0),
            top: px(0),
            ..default()
        },
        Color::srgba(0.0, 0.025, 0.025, 0.15),
    );
    commands
        .entity(shade)
        .insert((ZIndex(-1), Pickable::IGNORE));
}
fn fit_backdrop(window: Single<&Window>, mut backdrops: Query<&mut ImageNode, With<Backdrop>>) {
    let aspect = window.width() / window.height().max(1.);
    let width = 1672.;
    let height = 941.;
    let crop = if aspect > width / height {
        let h = width / aspect;
        Rect::new(0., (height - h) * 0.5, width, (height + h) * 0.5)
    } else {
        let w = height * aspect;
        Rect::new((width - w) * 0.5, 0., (width + w) * 0.5, height)
    };
    for mut image in &mut backdrops {
        if image.rect != Some(crop) {
            image.rect = Some(crop);
        }
    }
}
pub(super) fn picture(commands: &mut Commands, parent: Entity, width: Val, height: f32) {
    let e = commands
        .spawn((
            ImageNode {
                image: LANDSCAPE.clone(),
                rect: Some(if height < 100. {
                    Rect::new(75., 0., 1597., 941.)
                } else {
                    Rect::new(390., 0., 1331., 941.)
                }),
                image_mode: NodeImageMode::Stretch,
                ..default()
            },
            Node {
                width,
                height: px(height),
                border_radius: BorderRadius::all(px(10)),
                border: UiRect::all(px(1)),
                overflow: Overflow::clip(),
                flex_shrink: 0.,
                ..default()
            },
            BorderColor::all(Color::srgba(0.35, 0.70, 0.63, 0.5)),
            Pickable::IGNORE,
        ))
        .id();
    commands.entity(parent).add_child(e);
}
pub(super) fn section(commands: &mut Commands, parent: Entity, title: &str) {
    let row = row(commands, parent, 10.);
    let line = node(
        commands,
        row,
        Node {
            width: px(14),
            height: px(14),
            border: UiRect::all(px(2)),
            ..default()
        },
        Color::NONE,
    );
    commands.entity(line).insert(BorderColor::all(TEAL));
    label(commands, row, title, 13., TEAL);
}
pub(super) fn fit_button(commands: &mut Commands, button: Entity, width: Val) {
    commands.entity(button).insert(Node {
        width,
        min_height: px(48),
        padding: UiRect::axes(px(14), px(11)),
        border: UiRect::all(px(1)),
        border_radius: BorderRadius::all(px(6)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_shrink: 0.,
        ..default()
    });
}
pub(super) fn range(commands: &mut Commands, parent: Entity, title: &str, kind: Range) {
    let row = row(commands, parent, 12.);
    let title = label(commands, row, title, 15., TEXT);
    commands.entity(title).insert(Node {
        flex_grow: 1.,
        ..default()
    });
    let track = node(
        commands,
        row,
        Node {
            width: px(172),
            height: px(40),
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            flex_shrink: 0.,
            ..default()
        },
        Color::NONE,
    );
    commands
        .entity(track)
        .insert((kind, RelativeCursorPosition::default()));
    let rail = node(
        commands,
        track,
        Node {
            width: percent(100),
            height: px(4),
            border_radius: BorderRadius::MAX,
            ..default()
        },
        Color::srgb(0.19, 0.29, 0.27),
    );
    commands.entity(rail).insert(Pickable::IGNORE);
    let fill = node(
        commands,
        track,
        Node {
            position_type: PositionType::Absolute,
            left: px(0),
            height: px(4),
            width: percent(50),
            border_radius: BorderRadius::MAX,
            ..default()
        },
        TEAL,
    );
    commands
        .entity(fill)
        .insert((RangeFill(kind), Pickable::IGNORE));
    let knob = node(
        commands,
        track,
        Node {
            position_type: PositionType::Absolute,
            left: percent(50),
            width: px(13),
            height: px(13),
            border_radius: BorderRadius::MAX,
            ..default()
        },
        Color::srgb(0.98, 0.92, 0.76),
    );
    commands
        .entity(knob)
        .insert((RangeKnob(kind), Pickable::IGNORE));
    let value_box = node(
        commands,
        row,
        Node {
            width: px(65),
            height: px(32),
            border: UiRect::all(px(1)),
            border_radius: BorderRadius::all(px(4)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_shrink: 0.,
            ..default()
        },
        Color::srgba(0.1, 0.19, 0.18, 0.7),
    );
    commands.entity(value_box).insert(BorderColor::all(LINE));
    let value = label(commands, value_box, "", 13., TEXT);
    commands.entity(value).insert(RangeText(kind));
}
fn range_input(
    mouse: Res<ButtonInput<MouseButton>>,
    mut game: ResMut<Game>,
    ranges: Query<(&Range, &RelativeCursorPosition)>,
    mut active: Local<Option<Range>>,
    mut intents: ResMut<UiIntent>,
) {
    if game.mode != Mode::Settings {
        if active.take().is_some() {
            intents.0.push(Action::CommitSettings);
        }
        return;
    }
    for (kind, cursor) in &ranges {
        if mouse.just_pressed(MouseButton::Left) && cursor.cursor_over() {
            *active = Some(*kind);
        }
        if *active == Some(*kind)
            && mouse.pressed(MouseButton::Left)
            && let Some(pos) = cursor.normalized
        {
            let value = pointer_fraction(pos.x);
            match kind {
                Range::Sensitivity => game.settings.sensitivity = 0.0005 + value * 0.0045,
                Range::Volume => game.settings.volume = value,
            }
        }
    }
    if !mouse.pressed(MouseButton::Left) && active.take().is_some() {
        intents.0.push(Action::CommitSettings);
    }
}
fn fraction(game: &Game, kind: Range) -> f32 {
    match kind {
        Range::Sensitivity => (game.settings.sensitivity - 0.0005) / 0.0045,
        Range::Volume => game.settings.volume,
    }
    .clamp(0., 1.)
}
fn range_view(
    game: Res<Game>,
    mut fills: Query<(&RangeFill, &mut Node), Without<RangeKnob>>,
    mut knobs: Query<(&RangeKnob, &mut Node), Without<RangeFill>>,
    mut labels: Query<(&RangeText, &mut Text)>,
) {
    for (kind, mut node) in &mut fills {
        node.width = percent(fraction(&game, kind.0) * 100.);
    }
    for (kind, mut node) in &mut knobs {
        node.left = percent(fraction(&game, kind.0) * 100.);
        node.margin.left = px(-6.5);
    }
    for (kind, mut text) in &mut labels {
        let value = match kind.0 {
            Range::Sensitivity => game.settings.sensitivity / 0.002,
            Range::Volume => game.settings.volume,
        };
        let next = format!("{:.0}%", value * 100.);
        if text.0 != next {
            text.0 = next;
        }
    }
}

fn pointer_fraction(centered: f32) -> f32 {
    (centered + 0.5).clamp(0., 1.)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn slider_maps_bevys_centered_cursor_to_full_range() {
        assert_eq!(pointer_fraction(-0.5), 0.);
        assert_eq!(pointer_fraction(0.), 0.5);
        assert_eq!(pointer_fraction(0.5), 1.);
    }
}
