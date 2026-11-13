//! Общий кэш: исходные кубики, PNG инструментов, материалов и брони.
use crate::{graphics_assets::BlockIcons, session::Game};
use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use game_core::content::{ArmorSlot, ContentRegistry, ItemId, ItemKind};
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct ItemIcons {
    handles: HashMap<ItemId, Handle<Image>>,
}
impl ItemIcons {
    pub fn get(
        &self,
        item: ItemId,
        registry: &ContentRegistry,
        blocks: &BlockIcons,
    ) -> Handle<Image> {
        if let Some(block) = registry.block_for_item(item) {
            return blocks.get(block);
        }
        self.handles
            .get(&item)
            .cloned()
            .unwrap_or_else(|| blocks.get(0))
    }
}
pub fn plugin(app: &mut App) {
    app.init_resource::<ItemIcons>()
        .add_systems(PreUpdate, cache_icons);
}
fn cache_icons(
    mut game: ResMut<Game>,
    mut icons: ResMut<ItemIcons>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut added = false;
    for (id, definition) in game.content.iter() {
        if game.content.block_for_item(id).is_some() || icons.handles.contains_key(&id) {
            continue;
        }
        // Блоки уже пропущены выше: они сохраняют изометрические кубы из граней.
        let source_pixels = crate::resource_packs::active()
            .and_then(|pack| pack.item_pixels(&definition.id))
            .or_else(|| crate::tool_textures::pixels(&definition.id))
            .or_else(|| crate::item_textures::pixels(&definition.id))
            .unwrap_or_else(|| placeholder_pixels(&definition.id, &definition.kind));
        let pixel_size = game.settings.pixel_size;
        let pixels = if pixel_size == 128 {
            source_pixels
        } else {
            game_render::pixelate_icon_rgba(&source_pixels, pixel_size).unwrap_or(source_pixels)
        };
        let mut image = Image::new(
            Extent3d {
                width: 128,
                height: 128,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            pixels,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );
        image.sampler = if pixel_size == 128 {
            ImageSampler::linear()
        } else {
            ImageSampler::nearest()
        };
        icons.handles.insert(id, images.add(image));
        added = true;
    }
    if added {
        game.ui_dirty = true;
    }
}
fn placeholder_pixels(id: &str, kind: &ItemKind) -> Vec<u8> {
    let mut pixels = vec![0; 32 * 32 * 4];
    for y in 3_i32..29 {
        for x in 3_i32..29 {
            let (inside, color): (bool, [u8; 3]) = match kind {
                ItemKind::Tool => (
                    ((x - y).abs() <= 2 && (10..26).contains(&x))
                        || ((6..25).contains(&x) && (6..10).contains(&y))
                        || ((5..9).contains(&x) && (8..16).contains(&y)),
                    if y < 10 || x < 9 {
                        [176, 132, 75]
                    } else {
                        [112, 72, 40]
                    },
                ),
                ItemKind::Armor { slot } => {
                    let inside = match slot {
                        ArmorSlot::Head => {
                            (7..25).contains(&x)
                                && (6..22).contains(&y)
                                && !(y > 12 && (11..21).contains(&x))
                        }
                        ArmorSlot::Chest => {
                            ((9..23).contains(&x) && (8..26).contains(&y))
                                || ((4..28).contains(&x) && (9..16).contains(&y))
                        }
                        ArmorSlot::Legs => {
                            (8..24).contains(&x)
                                && (6..27).contains(&y)
                                && !(y > 12 && (14..18).contains(&x))
                        }
                        ArmorSlot::Feet => {
                            ((7..13).contains(&x) || (19..25).contains(&x)) && (11..26).contains(&y)
                                || ((5..13).contains(&x) || (17..25).contains(&x))
                                    && (22..27).contains(&y)
                        }
                    };
                    (inside, [165, 101, 65])
                }
                ItemKind::Food => (
                    ((x - 16).pow(2) * 2 + (y - 17).pow(2)) < 130,
                    [235, 218, 164],
                ),
                _ if id.ends_with(":stick") => (
                    (x + y - 31).abs() <= 2 && (7..25).contains(&x),
                    [166, 116, 65],
                ),
                _ if id.ends_with(":egg") => (
                    ((x - 16).pow(2) * 2 + (y - 17).pow(2)) < 130,
                    [235, 228, 196],
                ),
                _ => ((x - 16).abs() + (y - 16).abs() < 12, [66, 65, 78]),
            };
            if inside {
                let offset = ((y * 32 + x) * 4) as usize;
                for channel in 0..3 {
                    pixels[offset + channel] =
                        color[channel].saturating_add(if x < 15 { 18 } else { 0 });
                }
                pixels[offset + 3] = 255;
            }
        }
    }
    let mut output = vec![0; 128 * 128 * 4];
    for y in 0..128 {
        for x in 0..128 {
            let source = ((y / 4) * 32 + x / 4) * 4;
            let target = (y * 128 + x) * 4;
            output[target..target + 4].copy_from_slice(&pixels[source..source + 4]);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn material_and_armor_cache_preserves_uploaded_rgba_and_partial_alpha() {
        let mut app = App::new();
        app.init_resource::<Game>()
            .init_resource::<Assets<Image>>()
            .init_resource::<ItemIcons>()
            .add_systems(Update, cache_icons);
        app.update();
        let mut art = vec![];
        for name in [
            "coal",
            "raw_iron",
            "raw_gold",
            "iron_ingot",
            "gold_ingot",
            "diamond",
            "leather",
        ] {
            art.push((name.to_string(), "icons"));
        }
        for material in ["leather", "iron", "golden", "diamond"] {
            for part in ["helmet", "chestplate", "leggings", "boots"] {
                art.push((format!("{material}_{part}"), "armor"));
            }
        }
        for (name, directory) in art {
            let source = std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
                format!("../../assets/textures/{directory}/128x128/{name}.png"),
            ))
            .expect("uploaded original PNG");
            let expected = image::load_from_memory_with_format(&source, image::ImageFormat::Png)
                .unwrap()
                .into_rgba8();
            let item = app
                .world()
                .resource::<Game>()
                .content
                .find(&format!("survival:{name}"))
                .unwrap();
            let handle = &app.world().resource::<ItemIcons>().handles[&item];
            let actual = app.world().resource::<Assets<Image>>().get(handle).unwrap();
            assert_eq!(
                actual.data.as_deref(),
                Some(expected.as_raw().as_slice()),
                "{name}"
            );
            assert!(expected.pixels().any(|pixel| pixel[3] == 0), "{name}");
            assert!(
                expected
                    .pixels()
                    .any(|pixel| pixel[3] > 0 && pixel[3] < 255),
                "{name}"
            );
        }
    }

    #[test]
    fn wooden_pickaxe_uses_uploaded_png_pixels_instead_of_placeholder() {
        let mut app = App::new();
        app.init_resource::<Game>()
            .init_resource::<Assets<Image>>()
            .init_resource::<ItemIcons>()
            .add_systems(Update, cache_icons);
        app.update();
        let item = app
            .world()
            .resource::<Game>()
            .content
            .find("survival:wooden_pickaxe")
            .unwrap();
        let handle = &app.world().resource::<ItemIcons>().handles[&item];
        let actual = app.world().resource::<Assets<Image>>().get(handle).unwrap();
        let expected = image::load_from_memory_with_format(
            include_bytes!("../../../assets/textures/tools/128x128/wooden_pickaxe.png"),
            image::ImageFormat::Png,
        )
        .unwrap()
        .into_rgba8();
        assert!(
            actual.data.as_deref() == Some(expected.as_raw().as_slice()),
            "Значок деревянной кирки должен точно совпадать с PNG пользователя"
        );
    }
    #[test]
    fn new_item_art_has_transparent_margin_and_visible_silhouette() {
        let pixels = placeholder_pixels("survival:stick", &ItemKind::Material);
        assert_eq!(&pixels[..4], &[0, 0, 0, 0]);
        assert!(
            pixels
                .as_chunks::<4>()
                .0
                .iter()
                .any(|pixel| pixel[3] == 255)
        );
    }
    #[test]
    fn nonblock_icons_are_cached_across_frames() {
        let mut app = App::new();
        app.init_resource::<Game>()
            .init_resource::<Assets<Image>>()
            .init_resource::<ItemIcons>()
            .add_systems(Update, cache_icons);
        app.update();
        let expected = app
            .world()
            .resource::<Game>()
            .content
            .iter()
            .filter(|(id, _)| {
                app.world()
                    .resource::<Game>()
                    .content
                    .block_for_item(*id)
                    .is_none()
            })
            .count();
        let first = app.world().resource::<ItemIcons>().handles.clone();
        assert!(expected > 0);
        assert_eq!(first.len(), expected);
        app.update();
        let images = app.world().resource::<Assets<Image>>();
        assert_eq!(images.len(), expected);
        assert!(
            images
                .iter()
                .all(|(_, image)| image.texture_descriptor.size.width == 128
                    && image.texture_descriptor.size.height == 128)
        );
        assert_eq!(app.world().resource::<ItemIcons>().handles, first);
    }
    #[test]
    fn newly_reloaded_item_requests_exactly_one_ui_refresh() {
        let mut app = App::new();
        app.init_resource::<Game>()
            .init_resource::<Assets<Image>>()
            .init_resource::<ItemIcons>()
            .add_systems(Update, cache_icons);
        app.update();
        let initial_images = app.world().resource::<Assets<Image>>().len();
        let mut source = include_str!("../../../assets/data/items.ron").to_string();
        source.insert_str(source.rfind(']').unwrap(),
            "(id: \"survival:ui_cache_fixture\", name: \"Тест кэша\", max_stack: 64, kind: Material),\n");
        let mut game = app.world_mut().resource_mut::<Game>();
        game.content = game.content.reload(&source).unwrap();
        game.ui_dirty = false;
        let item = game.content.find("survival:ui_cache_fixture").unwrap();
        app.update();
        assert!(app.world().resource::<Game>().ui_dirty);
        assert!(
            app.world()
                .resource::<ItemIcons>()
                .handles
                .contains_key(&item)
        );
        assert_eq!(
            app.world().resource::<Assets<Image>>().len(),
            initial_images + 1
        );
        app.world_mut().resource_mut::<Game>().ui_dirty = false;
        app.update();
        assert!(!app.world().resource::<Game>().ui_dirty);
        assert_eq!(
            app.world().resource::<Assets<Image>>().len(),
            initial_images + 1
        );
    }
    #[test]
    fn block_items_reuse_original_cube_handles() {
        let registry = ContentRegistry::default();
        let mut blocks = BlockIcons::default();
        let mut images = Assets::<Image>::default();
        blocks.handles = (0..game_core::blocks::BLOCKS.len())
            .map(|_| images.add(Image::default()))
            .collect();
        let icons = ItemIcons::default();
        for (id, _) in registry.iter() {
            if let Some(block) = registry.block_for_item(id) {
                assert_eq!(icons.get(id, &registry, &blocks), blocks.get(block));
            }
        }
    }
}
