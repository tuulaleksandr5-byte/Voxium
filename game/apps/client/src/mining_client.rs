//! Удержание мыши готовит завершение; потребитель заново проверяет цель и время.
use crate::{
    inventory_client::visible_hit,
    session::{Game, Mode},
};
use bevy::prelude::*;
use game_core::{
    mining::{MiningTarget, Tool},
    survival::GameMode,
};

pub fn chosen_tool(game: &Game) -> Tool {
    Tool::from_item(
        game.item_for_slot(game.selected).map(|s| s.item),
        &game.content,
    )
}
pub fn target(game: &Game, position: [i32; 3]) -> MiningTarget {
    MiningTarget {
        position,
        block: game.world.block(position),
        item: game.item_for_slot(game.selected).map(|s| s.item),
        slot: game.selected,
    }
}
pub fn advance(game: &mut Game, held: bool, seconds: f32) -> Option<[i32; 3]> {
    if !held
        || !game.world_mouse.allowed()
        || game.mode != Mode::Playing
        || game.game_mode != GameMode::Survival
        || game.world_id.is_none()
        || game.vitals.is_dead()
        || game.pending_respawn
    {
        game.mining.reset();
        return None;
    }
    let Some(hit) = visible_hit(game) else {
        game.mining.reset();
        return None;
    };
    let target = target(game, hit.block);
    let Some(duration) = game.mining_config.duration(target.block, chosen_tool(game)) else {
        game.mining.reset();
        return None;
    };
    game.mining
        .advance(target, seconds, duration)
        .then_some(hit.block)
}
pub fn progress(game: &Game) -> Option<(MiningTarget, f32)> {
    if game.world_id.is_none()
        || game.mode != Mode::Playing
        || game.game_mode != GameMode::Survival
        || game.vitals.is_dead()
        || game.pending_respawn
        || !game.world_mouse.allowed()
    {
        return None;
    }
    let current = game.mining.target()?;
    if target(game, current.position) != current
        || !visible_hit(game).is_some_and(|h| h.block == current.position)
    {
        return None;
    }
    let duration = game
        .mining_config
        .duration(current.block, chosen_tool(game))?;
    Some((current, game.mining.fraction(duration)))
}

pub fn draw_cracks(game: &Game, gizmos: &mut Gizmos) {
    let Some((target, fraction)) = progress(game) else {
        return;
    };
    let origin = Vec3::from_array(target.position.map(|v| v as f32));
    let count = (fraction * 10.).ceil() as usize;
    // Ломаные на шести гранях не затрагивают исходные текстуры блоков.
    let lines = [
        ([0.18, 0.05], [0.43, 0.35]),
        ([0.43, 0.35], [0.31, 0.59]),
        ([0.31, 0.59], [0.47, 0.83]),
        ([0.47, 0.83], [0.41, 0.98]),
        ([0.43, 0.35], [0.70, 0.48]),
        ([0.70, 0.48], [0.95, 0.31]),
        ([0.70, 0.48], [0.77, 0.75]),
        ([0.77, 0.75], [0.97, 0.93]),
        ([0.31, 0.59], [0.07, 0.73]),
        ([0.43, 0.35], [0.66, 0.07]),
    ];
    for axis in 0..3 {
        for face in [-0.003, 1.003] {
            for (a, b) in lines.iter().take(count) {
                let point = |p: &[f32; 2]| match axis {
                    0 => Vec3::new(face, p[0], p[1]),
                    1 => Vec3::new(p[0], face, p[1]),
                    _ => Vec3::new(p[0], p[1], face),
                };
                gizmos.line(
                    origin + point(a),
                    origin + point(b),
                    Color::srgb(0.08, 0.055, 0.035),
                );
            }
        }
    }
}

#[derive(Component)]
struct MiningHud;
#[derive(Component)]
struct MiningFill;
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_hud)
        .add_systems(Update, update_hud.after(crate::survival_client::simulate));
}
fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: percent(44),
                bottom: percent(25),
                width: percent(12),
                height: px(7),
                ..default()
            },
            BackgroundColor(Color::srgba(0., 0., 0., 0.65)),
            Visibility::Hidden,
            Pickable::IGNORE,
            MiningHud,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: percent(0),
                    height: percent(100),
                    ..default()
                },
                BackgroundColor(Color::srgb(1., 0.78, 0.25)),
                Pickable::IGNORE,
                MiningFill,
            ));
        });
}
fn update_hud(
    game: Res<Game>,
    mut hud: Query<&mut Visibility, With<MiningHud>>,
    mut fill: Query<&mut Node, With<MiningFill>>,
) {
    let fraction = progress(&game).map(|(_, fraction)| fraction);
    for mut visibility in &mut hud {
        *visibility = if fraction.is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut node in &mut fill {
        node.width = percent(fraction.unwrap_or_default() * 100.);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_bar_only_appears_during_a_live_survival_hold() {
        let mut game = Game {
            mode: Mode::Playing,
            world_id: Some("mining-hud".into()),
            feet: Vec3::new(8.5, 100., 8.5),
            pitch: 0.,
            ..Default::default()
        };
        game.world.insert(game_core::Chunk {
            position: [0, 0],
            sections: Default::default(),
        });
        game.world.set([8, 101, 5], game_core::STONE);
        game.world_mouse.update(true, false, false);
        let mut app = App::new();
        app.insert_resource(game)
            .add_systems(Startup, setup_hud)
            .add_systems(Update, update_hud);
        app.update();
        let world = app.world_mut();
        let hud = world
            .query_filtered::<Entity, With<MiningHud>>()
            .single(world)
            .unwrap();
        let fill = world
            .query_filtered::<Entity, With<MiningFill>>()
            .single(world)
            .unwrap();
        assert_eq!(*world.get::<Visibility>(hud).unwrap(), Visibility::Hidden);
        assert!(advance(&mut world.resource_mut::<Game>(), true, 3.75).is_none());
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(hud).unwrap(),
            Visibility::Visible
        );
        assert_eq!(app.world().get::<Node>(fill).unwrap().width, percent(50));
        advance(&mut app.world_mut().resource_mut::<Game>(), false, 1.0);
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(hud).unwrap(),
            Visibility::Hidden
        );
        assert_eq!(app.world().get::<Node>(fill).unwrap().width, percent(0));
    }
}
