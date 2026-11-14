//! Пиксельные значки предметов из `tools/generate_pixel_art.py`, встроенные в EXE.

macro_rules! icon {
    ($id:literal, $name:literal) => {
        (
            $id,
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../assets/textures/pixel/icons/",
                $name,
                ".png"
            )) as &'static [u8],
        )
    };
}

const ICONS: [(&str, &[u8]); 15] = [
    icon!("survival:stick", "stick"),
    icon!("survival:egg", "egg"),
    icon!("builder:clay_ball", "clay_ball"),
    icon!("builder:brick", "brick"),
    icon!("builder:string", "string"),
    icon!("builder:feather", "feather"),
    icon!("builder:raw_beef", "raw_beef"),
    icon!("builder:cooked_beef", "cooked_beef"),
    icon!("builder:raw_pork", "raw_pork"),
    icon!("builder:cooked_pork", "cooked_pork"),
    icon!("builder:raw_mutton", "raw_mutton"),
    icon!("builder:cooked_mutton", "cooked_mutton"),
    icon!("builder:raw_chicken", "raw_chicken"),
    icon!("builder:cooked_chicken", "cooked_chicken"),
    icon!("builder:rotten_meat", "rotten_meat"),
];

/// RGBA 128×128 для предмета с пиксельным значком.
pub(crate) fn pixels(id: &str) -> Option<Vec<u8>> {
    let (_, bytes) = ICONS.iter().find(|(known, _)| *known == id)?;
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .ok()?
        .into_rgba8();
    (image.dimensions() == (128, 128)).then(|| image.into_raw())
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_core::content::{ContentRegistry, ItemKind};

    #[test]
    fn every_new_flat_item_has_a_decodable_transparent_icon() {
        let catalog = ContentRegistry::default();
        for (id, _) in ICONS {
            assert!(catalog.find(id).is_some(), "{id}");
            let rgba = pixels(id).unwrap();
            assert_eq!(rgba.len(), 128 * 128 * 4);
            assert!(rgba.as_chunks::<4>().0.iter().any(|p| p[3] == 0), "{id}");
            assert!(rgba.as_chunks::<4>().0.iter().any(|p| p[3] == 255), "{id}");
        }
        for (_, item) in catalog.iter() {
            if item.id.starts_with("builder:") && !matches!(item.kind, ItemKind::Block { .. }) {
                assert!(pixels(&item.id).is_some(), "{} has no icon", item.id);
            }
        }
    }
}
