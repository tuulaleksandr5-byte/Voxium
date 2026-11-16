//! Native UI primitives skinned with the supplied Icons and Colourful Containers.
use super::*;
use bevy::{
    asset::RenderAssetUsages,
    image::{CompressedImageFormats, ImageSampler, ImageType},
    sprite::{BorderRect, TextureSlicer},
};

const PANEL_ART: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83ef0");
const SLOT_ART: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83ef1");
const CHEST_SLOT: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83ef2");
const MAIN_ICONS: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83ef3");
const WORLD_ICONS: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83ef4");
const INVENTORY: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83ef5");
const CRAFTING: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83ef6");
const FURNACE: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83ef7");
const SMOKER: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83ef8");
const BLAST: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83ef9");
const CHEST: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83efa");
const BARREL: Handle<Image> = uuid_handle!("0431afcb-9372-44eb-afec-f0d614b83efb");

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, load);
}

fn load(mut images: ResMut<Assets<Image>>) {
    for (handle, name) in [
        (PANEL_ART, "gui/panel.png"),
        (SLOT_ART, "gui/slot.png"),
        (CHEST_SLOT, "gui/chest_slot.png"),
        (MAIN_ICONS, "gui/icons_main_menu.png"),
        (WORLD_ICONS, "gui/icons_singleplayer.png"),
        (INVENTORY, "gui/inventory.png"),
        (CRAFTING, "gui/crafting_table.png"),
        (FURNACE, "gui/furnace.png"),
        (SMOKER, "gui/smoker.png"),
        (BLAST, "gui/blast_furnace.png"),
        (CHEST, "gui/chest_compact.png"),
        (BARREL, "gui/barrel_compact.png"),
    ] {
        let image = Image::from_buffer(
            crate::pack_assets::file(name).expect("embedded UI artwork"),
            ImageType::Extension("png"),
            CompressedImageFormats::NONE,
            true,
            ImageSampler::nearest(),
            RenderAssetUsages::default(),
        )
        .expect("valid embedded UI PNG");
        images
            .insert(handle.id(), image)
            .expect("insert UI artwork");
    }
}

pub(super) fn inventory_image(open: Option<game_core::BlockId>) -> Option<Handle<Image>> {
    use game_core::*;
    Some(match open {
        None => INVENTORY,
        Some(CRAFTING_TABLE) => CRAFTING,
        Some(game_core::FURNACE | FURNACE_LIT) => self::FURNACE,
        Some(game_core::SMOKER | SMOKER_LIT) => self::SMOKER,
        Some(BLAST_FURNACE | BLAST_FURNACE_LIT) => BLAST,
        Some(game_core::CHEST | TRAPPED_CHEST | ENDER_CHEST) => self::CHEST,
        Some(game_core::BARREL) => self::BARREL,
        _ => return None,
    })
}

pub(super) fn frame(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).insert(BackgroundColor(Color::NONE));
    let background = commands
        .spawn((
            ImageNode {
                image: PANEL_ART,
                image_mode: NodeImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(7.0),
                    max_corner_scale: 3.0,
                    ..default()
                }),
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                width: percent(100),
                height: percent(100),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();
    commands.entity(entity).add_child(background);
}

pub(super) fn slot(commands: &mut Commands, entity: Entity, chest: bool) {
    commands.entity(entity).insert((
        ImageNode::new(if chest { CHEST_SLOT } else { SLOT_ART }),
        BackgroundColor(Color::NONE),
    ));
}

pub(super) fn button_icon(commands: &mut Commands, parent: Entity, world: bool, index: u32) {
    let image = commands
        .spawn((
            ImageNode {
                image: if world { WORLD_ICONS } else { MAIN_ICONS },
                rect: Some(Rect::new(
                    index as f32 * 20.0,
                    0.0,
                    (index + 1) as f32 * 20.0,
                    20.0,
                )),
                ..default()
            },
            Node {
                width: px(28),
                height: px(28),
                margin: UiRect::left(px(12)),
                flex_shrink: 0.0,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();
    commands.entity(parent).add_child(image);
}
