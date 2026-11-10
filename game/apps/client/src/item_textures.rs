//! Оригинальные PNG материалов и брони; плоские значки блоков не используются.
use crate::{graphics_assets::asset_path, tool_textures::load_pixels};
use bevy::log::warn;

macro_rules! original {
    ($directory:literal, $name:literal) => {
        (
            $name,
            concat!("assets/textures/", $directory, "/128x128/", $name, ".png"),
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../assets/textures/",
                $directory,
                "/128x128/",
                $name,
                ".png"
            )) as &'static [u8],
        )
    };
}

const ORIGINALS: [(&str, &str, &[u8]); 23] = [
    original!("icons", "coal"),
    original!("icons", "raw_iron"),
    original!("icons", "raw_gold"),
    original!("icons", "iron_ingot"),
    original!("icons", "gold_ingot"),
    original!("icons", "diamond"),
    original!("icons", "leather"),
    original!("armor", "leather_helmet"),
    original!("armor", "leather_chestplate"),
    original!("armor", "leather_leggings"),
    original!("armor", "leather_boots"),
    original!("armor", "iron_helmet"),
    original!("armor", "iron_chestplate"),
    original!("armor", "iron_leggings"),
    original!("armor", "iron_boots"),
    original!("armor", "golden_helmet"),
    original!("armor", "golden_chestplate"),
    original!("armor", "golden_leggings"),
    original!("armor", "golden_boots"),
    original!("armor", "diamond_helmet"),
    original!("armor", "diamond_chestplate"),
    original!("armor", "diamond_leggings"),
    original!("armor", "diamond_boots"),
];

pub(crate) fn pixels(id: &str) -> Option<Vec<u8>> {
    let name = id.strip_prefix("survival:")?;
    let (_, relative, embedded) = ORIGINALS.iter().find(|(known, _, _)| *known == name)?;
    let path = asset_path(relative);
    match load_pixels(embedded, path.as_deref()) {
        Ok(pixels) => Some(pixels),
        Err(error) => {
            warn!("Не удалось загрузить рисунок {id}: {error}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_named_original_preserves_rgba_and_translucent_edges() {
        let mut names = std::collections::HashSet::new();
        for (name, _, bytes) in ORIGINALS {
            assert!(names.insert(name));
            let expected = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
                .unwrap()
                .into_rgba8();
            assert_eq!(
                pixels(&format!("survival:{name}")).unwrap(),
                *expected.as_raw(),
                "{name}"
            );
            assert!(expected.pixels().any(|p| p[3] == 0), "{name}");
            assert!(expected.pixels().any(|p| p[3] > 0 && p[3] < 255), "{name}");
        }
        for id in [
            "foundation:coal",
            "survival:coal_ore",
            "survival:iron_block",
            "survival:unknown",
        ] {
            assert!(
                pixels(id).is_none(),
                "block items must use cube icons: {id}"
            );
        }
    }

    #[test]
    fn invalid_external_art_falls_back_and_valid_override_has_priority() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "voxel-item-icon-{}-{stamp}.png",
            std::process::id()
        ));
        for (_, _, embedded) in ORIGINALS {
            let expected = load_pixels(embedded, None).unwrap();
            assert_eq!(load_pixels(embedded, Some(&path)).unwrap(), expected);
            std::fs::write(&path, b"invalid PNG").unwrap();
            assert_eq!(load_pixels(embedded, Some(&path)).unwrap(), expected);
            std::fs::write(
                &path,
                include_bytes!("../../../assets/textures/icons/64x64/coal.png"),
            )
            .unwrap();
            assert_eq!(load_pixels(embedded, Some(&path)).unwrap(), expected);
            let replacement = ORIGINALS[0].2;
            std::fs::write(&path, replacement).unwrap();
            assert_eq!(
                load_pixels(embedded, Some(&path)).unwrap(),
                load_pixels(replacement, None).unwrap()
            );
            std::fs::remove_file(&path).unwrap();
        }
    }
}
