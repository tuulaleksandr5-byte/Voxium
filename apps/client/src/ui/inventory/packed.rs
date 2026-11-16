//! Click targets placed over the actual Colourful Containers artwork.
use super::*;

const SCALE: f32 = 3.0;

fn position(commands: &mut Commands, entity: Entity, x: f32, y: f32) {
    commands.entity(entity).insert(Node {
        position_type: PositionType::Absolute,
        left: px(x * SCALE),
        top: px(y * SCALE),
        width: px(54),
        height: px(54),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    });
}

#[allow(clippy::too_many_arguments)]
fn at(
    commands: &mut Commands,
    parent: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
    reference: SlotRef,
    x: f32,
    y: f32,
) {
    let entity = slot(commands, parent, game, icons, blocks, reference, "");
    commands
        .entity(entity)
        .remove::<ImageNode>()
        .insert(BackgroundColor(Color::NONE));
    position(commands, entity, x, y);
}

fn result(
    commands: &mut Commands,
    parent: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
    x: f32,
    y: f32,
) {
    let output = craft_preview(game);
    let entity = commands
        .spawn((
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
            Node::default(),
        ))
        .id();
    commands.entity(parent).add_child(entity);
    position(commands, entity, x, y);
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
        commands.entity(entity).add_child(image);
        let number = label(commands, entity, stack.count.to_string(), 13., TEXT);
        commands.entity(number).insert(Node {
            position_type: PositionType::Absolute,
            right: px(3),
            bottom: px(1),
            ..default()
        });
    } else {
        label(commands, entity, "—", 20., MUTED);
    }
}

pub(super) fn menu(
    commands: &mut Commands,
    panel: Entity,
    game: &Game,
    icons: &ItemIcons,
    blocks: &BlockIcons,
) -> bool {
    let open = crate::inventory_client::open_block(game);
    let Some(image) = super::super::pack_art::inventory_image(open) else {
        return false;
    };
    let frame = commands
        .spawn((
            ImageNode {
                image,
                rect: Some(Rect::new(0., 0., 176., 166.)),
                ..default()
            },
            Node {
                width: px(176. * SCALE),
                height: px(166. * SCALE),
                position_type: PositionType::Relative,
                flex_shrink: 0.,
                align_self: AlignSelf::Center,
                ..default()
            },
        ))
        .id();
    commands.entity(panel).add_child(frame);
    for row in 0..3 {
        for col in 0..9 {
            at(
                commands,
                frame,
                game,
                icons,
                blocks,
                SlotRef::Main(row * 9 + col),
                7. + col as f32 * 18.,
                83. + row as f32 * 18.,
            );
        }
    }
    for col in 0..9 {
        at(
            commands,
            frame,
            game,
            icons,
            blocks,
            SlotRef::Hotbar(col),
            7. + col as f32 * 18.,
            141.,
        );
    }
    match open {
        None => {
            for (row, armor) in [
                ArmorSlot::Head,
                ArmorSlot::Chest,
                ArmorSlot::Legs,
                ArmorSlot::Feet,
            ]
            .into_iter()
            .enumerate()
            {
                at(
                    commands,
                    frame,
                    game,
                    icons,
                    blocks,
                    SlotRef::Armor(armor),
                    7.,
                    7. + row as f32 * 18.,
                );
            }
            at(
                commands,
                frame,
                game,
                icons,
                blocks,
                SlotRef::Offhand,
                76.,
                61.,
            );
            for row in 0..2 {
                for col in 0..2 {
                    at(
                        commands,
                        frame,
                        game,
                        icons,
                        blocks,
                        SlotRef::Crafting(row * 2 + col),
                        97. + col as f32 * 18.,
                        17. + row as f32 * 18.,
                    );
                }
            }
            result(commands, frame, game, icons, blocks, 153., 27.);
        }
        Some(game_core::CRAFTING_TABLE) => {
            for row in 0..3 {
                for col in 0..3 {
                    at(
                        commands,
                        frame,
                        game,
                        icons,
                        blocks,
                        SlotRef::Container(row * 3 + col),
                        29. + col as f32 * 18.,
                        16. + row as f32 * 18.,
                    );
                }
            }
            result(commands, frame, game, icons, blocks, 123., 34.);
        }
        Some(block) if game_core::smelting::FurnaceKind::of_block(block).is_some() => {
            use game_core::smelting::{FURNACE_FUEL, FURNACE_INPUT, FURNACE_OUTPUT};
            for (reference, x, y) in [
                (FURNACE_INPUT, 55., 16.),
                (FURNACE_FUEL, 55., 52.),
                (FURNACE_OUTPUT, 115., 34.),
            ] {
                at(
                    commands,
                    frame,
                    game,
                    icons,
                    blocks,
                    SlotRef::Container(reference),
                    x,
                    y,
                );
            }
            let state = game
                .open_container
                .and_then(|p| game.furnaces.get(&p))
                .copied()
                .unwrap_or_default();
            meter(
                commands,
                panel,
                "Плавка",
                state.progress_fraction(&game.smelting),
                AMBER,
            );
            meter(commands, panel, "Топливо", state.fuel_fraction(), TEAL);
        }
        Some(_) => {
            for row in 0..3 {
                for col in 0..9 {
                    at(
                        commands,
                        frame,
                        game,
                        icons,
                        blocks,
                        SlotRef::Container(row * 9 + col),
                        7. + col as f32 * 18.,
                        17. + row as f32 * 18.,
                    );
                }
            }
        }
    }
    // Keep the existing equipment and personal crafting controls accessible while
    // a container is open; the pack's painted panel has no cells for them.
    if open.is_some() {
        let extras = row(commands, panel, 16.);
        commands.entity(extras).insert(Node {
            column_gap: px(16),
            row_gap: px(12),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        });
        let equipment = column(commands, extras, 6.);
        commands.entity(equipment).insert(Node {
            width: px(294),
            flex_direction: FlexDirection::Column,
            row_gap: px(6),
            ..default()
        });
        label(commands, equipment, "ЭКИПИРОВКА", 12., AMBER);
        let line = row(commands, equipment, 6.);
        for armor in [
            ArmorSlot::Head,
            ArmorSlot::Chest,
            ArmorSlot::Legs,
            ArmorSlot::Feet,
        ] {
            slot(
                commands,
                line,
                game,
                icons,
                blocks,
                SlotRef::Armor(armor),
                "",
            );
        }
        slot(commands, line, game, icons, blocks, SlotRef::Offhand, "");
        if open != Some(game_core::CRAFTING_TABLE) {
            let personal = column(commands, extras, 6.);
            commands.entity(personal).insert(Node {
                width: px(220),
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                ..default()
            });
            crafting(commands, personal, game, icons, blocks);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn survival_controls_match_the_supplied_painted_slots_without_covering_the_art() {
        let game = Game::default();
        let mut world = World::new();
        let panel = world.spawn_empty().id();
        let mut queue = bevy::ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &world);
        assert!(menu(
            &mut commands,
            panel,
            &game,
            &ItemIcons::default(),
            &BlockIcons::default()
        ));
        queue.apply(&mut world);
        let slots: Vec<_> = world
            .query::<(&InventorySlot, &Node, Option<&ImageNode>)>()
            .iter(&world)
            .collect();
        assert_eq!(slots.len(), 45);
        let (_, node, image) = slots
            .iter()
            .find(|(slot, _, _)| slot.0 == SlotRef::Crafting(0))
            .unwrap();
        assert_eq!(node.left, px(97. * SCALE));
        assert_eq!(node.top, px(17. * SCALE));
        assert!(image.is_none());
        let result = world
            .query_filtered::<&Node, With<CraftResult>>()
            .single(&world)
            .unwrap();
        assert_eq!(result.left, px(153. * SCALE));
        assert_eq!(result.top, px(27. * SCALE));
    }
}
