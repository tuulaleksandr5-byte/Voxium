//! Исходные PNG инструментов: читаем RGBA без поворота, обрезки и изменения файлов.
use crate::graphics_assets::asset_path;
use bevy::log::warn;
use std::{io::Cursor, path::Path};

macro_rules! tool {
    ($name:literal) => {
        (
            $name,
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../assets/textures/tools/128x128/",
                $name,
                ".png"
            )) as &'static [u8],
        )
    };
}

const TOOLS: [(&str, &[u8]); 25] = [
    tool!("diamond_axe"),
    tool!("diamond_hoe"),
    tool!("diamond_pickaxe"),
    tool!("diamond_shovel"),
    tool!("diamond_sword"),
    tool!("golden_axe"),
    tool!("golden_hoe"),
    tool!("golden_pickaxe"),
    tool!("golden_shovel"),
    tool!("golden_sword"),
    tool!("iron_axe"),
    tool!("iron_hoe"),
    tool!("iron_pickaxe"),
    tool!("iron_shovel"),
    tool!("iron_sword"),
    tool!("stone_axe"),
    tool!("stone_hoe"),
    tool!("stone_pickaxe"),
    tool!("stone_shovel"),
    tool!("stone_sword"),
    tool!("wooden_axe"),
    tool!("wooden_hoe"),
    tool!("wooden_pickaxe"),
    tool!("wooden_shovel"),
    tool!("wooden_sword"),
];

/// Имена инструментов совпадают с файлами оригинального набора.
pub(crate) fn pixels(id: &str) -> Option<Vec<u8>> {
    let name = id.strip_prefix("survival:")?;
    let (_, embedded) = TOOLS.iter().find(|(known, _)| *known == name)?;
    let path = asset_path(&format!("assets/textures/tools/128x128/{name}.png"));
    match load_pixels(embedded, path.as_deref()) {
        Ok(pixels) => Some(pixels),
        Err(error) => {
            warn!("Не удалось загрузить рисунок {id}: {error}");
            None
        }
    }
}

pub(crate) fn load_pixels(embedded: &[u8], external: Option<&Path>) -> Result<Vec<u8>, String> {
    if let Some(path) = external {
        let result = std::fs::read(path)
            .map_err(|e| e.to_string())
            .and_then(|bytes| decode_png(&bytes));
        match result {
            Ok(pixels) => return Ok(pixels),
            Err(error) => warn!(
                "Некорректный {}: {error}; используем встроенный PNG",
                path.display()
            ),
        }
    }
    decode_png(embedded)
}

fn decode_png(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let dimensions = image::ImageReader::with_format(Cursor::new(bytes), image::ImageFormat::Png)
        .into_dimensions()
        .map_err(|e| e.to_string())?;
    if dimensions != (128, 128) {
        return Err(format!(
            "Ожидался PNG 128×128, получено {}×{}",
            dimensions.0, dimensions.1
        ));
    }
    image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .map(|image| image.into_rgba8().into_raw())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_uploaded_tool_ids_keep_source_rgba_and_partial_alpha() {
        let mut unique = std::collections::HashSet::new();
        for (name, bytes) in TOOLS {
            assert!(unique.insert(name));
            let expected = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
                .unwrap()
                .into_rgba8()
                .into_raw();
            let actual = pixels(&format!("survival:{name}")).unwrap();
            assert_eq!(actual, expected, "Неверная привязка PNG: {name}");
            assert!(actual.as_chunks::<4>().0.iter().any(|p| p[3] == 0));
            assert!(
                actual
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .any(|p| p[3] > 0 && p[3] < 255)
            );
        }
        assert!(pixels("survival:not_a_tool").is_none());
        assert!(pixels("foundation:wooden_pickaxe").is_none());
    }

    #[test]
    fn incorrect_size_and_invalid_png_are_rejected_without_resizing() {
        assert!(
            decode_png(include_bytes!(
                "../../../assets/textures/tools/64x64/wooden_pickaxe.png"
            ))
            .is_err()
        );
        assert!(decode_png(b"not a PNG").is_err());
    }

    #[test]
    fn missing_or_corrupt_external_png_uses_original_embedded_art() {
        let source = include_bytes!("../../../assets/textures/tools/128x128/wooden_pickaxe.png");
        let expected = decode_png(source).unwrap();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "voxel-tool-icon-{}-{stamp}.png",
            std::process::id()
        ));
        assert_eq!(load_pixels(source, Some(&path)).unwrap(), expected);
        std::fs::write(&path, b"broken PNG").unwrap();
        assert_eq!(load_pixels(source, Some(&path)).unwrap(), expected);
        std::fs::write(&path, source).unwrap();
        assert_eq!(load_pixels(source, Some(&path)).unwrap(), expected);
        std::fs::remove_file(path).unwrap();
    }
}
