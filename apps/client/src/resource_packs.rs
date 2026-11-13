//! Ресурс-паки в стиле OptiFine/Minecraft: папки в `resourcepacks/`.
//!
//! ```text
//! resourcepacks/MyPack/
//!   pack.json                                  { "name": "...", "pixel_art": true, "priority": 0 }
//!   assets/voxium/textures/block/<имя>.png     (или assets/minecraft/textures/block/<имя MC>.png)
//!   assets/voxium/models/block/<блок>.json     3D-модель блока (см. game_render::BlockModel)
//! ```
//! Текстуры любого квадратного размера (16×16 … 4096×4096) приводятся к 128×128 без
//! размытия при увеличении. Паки применяются по возрастанию `priority`, затем по имени
//! папки; поздний пак перекрывает ранний. Ошибки отдельных файлов не мешают запуску игры.
use bevy::prelude::{info, warn};
use game_render::{BlockAtlas, BlockModel, BlockModels};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct PackMeta {
    name: Option<String>,
    description: Option<String>,
    /// Резкие пиксели вблизи (nearest). Включается, если хоть один пак просит.
    pixel_art: bool,
    priority: i32,
    enabled: Option<bool>,
}

#[derive(Debug, Default)]
pub struct PackReport {
    pub packs: Vec<String>,
    pub textures: usize,
    pub models: BlockModels,
    pub pixel_art: bool,
    pub warnings: Vec<String>,
}

const NAMESPACES: [&str; 2] = ["voxium", "minecraft"];

/// Имена текстур Minecraft, отличающиеся от внутренних.
fn candidates(name: &str) -> Vec<&str> {
    let mut list = vec![name];
    match name {
        "grass_top" => list.push("grass_block_top"),
        "grass_side" => list.push("grass_block_side"),
        _ => {}
    }
    list
}

pub fn packs_directory() -> Option<PathBuf> {
    let exe = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("resourcepacks")));
    exe.filter(|p| p.is_dir())
        .or_else(|| Some(PathBuf::from("resourcepacks")).filter(|p| p.is_dir()))
}

/// Применяет все паки из `root`. Атлас меняется только для успешно прочитанных файлов.
pub fn apply(atlas: &mut BlockAtlas, root: &Path) -> PackReport {
    let mut report = PackReport::default();
    let Ok(entries) = std::fs::read_dir(root) else {
        return report;
    };
    let mut packs = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let meta = match std::fs::read(path.join("pack.json")) {
            Ok(bytes) => serde_json::from_slice::<PackMeta>(&bytes).unwrap_or_else(|e| {
                report
                    .warnings
                    .push(format!("{}: pack.json: {e}", path.display()));
                PackMeta::default()
            }),
            Err(_) => PackMeta::default(),
        };
        if meta.enabled == Some(false) {
            continue;
        }
        packs.push((
            meta.priority,
            entry.file_name().to_string_lossy().into_owned(),
            path,
            meta,
        ));
    }
    packs.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    for (_, folder, path, meta) in packs {
        let label = meta.name.clone().unwrap_or(folder);
        let before = report.textures;
        apply_textures(atlas, &path, &mut report);
        apply_models(&path, &mut report);
        report.pixel_art |= meta.pixel_art;
        info!(
            "Ресурс-пак «{label}»{}: текстур {}",
            meta.description
                .as_deref()
                .map(|d| format!(" — {d}"))
                .unwrap_or_default(),
            report.textures - before
        );
        report.packs.push(label);
    }
    atlas.pixel_art = report.pixel_art;
    for warning in &report.warnings {
        warn!("{warning}");
    }
    report
}

fn apply_textures(atlas: &mut BlockAtlas, pack: &Path, report: &mut PackReport) {
    let names = game_render::BASE_TILE_NAMES
        .iter()
        .chain(game_render::ADDITIONAL_TILE_NAMES.iter());
    for name in names {
        let layer = game_render::layer_of(name).expect("имя из списка слоёв");
        for namespace in NAMESPACES {
            for candidate in candidates(name) {
                let file = pack
                    .join("assets")
                    .join(namespace)
                    .join("textures/block")
                    .join(format!("{candidate}.png"));
                if !file.is_file() {
                    continue;
                }
                let result = image::open(&file)
                    .map_err(|e| e.to_string())
                    .and_then(|img| atlas.replace_layer(layer, &img.into_rgba8()));
                match result {
                    Ok(()) => report.textures += 1,
                    Err(e) => report.warnings.push(format!("{}: {e}", file.display())),
                }
            }
        }
    }
}

/// Блок по имени файла модели: `crafting_table` → «Crafting Table».
fn block_by_stem(stem: &str) -> Option<u8> {
    game_core::BLOCKS
        .iter()
        .find(|b| b.name.to_lowercase().replace(' ', "_") == stem)
        .map(|b| b.id)
}

fn apply_models(pack: &Path, report: &mut PackReport) {
    for namespace in NAMESPACES {
        let dir = pack.join("assets").join(namespace).join("models/block");
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|e| e != "json") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let Some(block) = block_by_stem(stem).filter(|id| *id != game_core::AIR) else {
                report
                    .warnings
                    .push(format!("{}: нет блока «{stem}»", path.display()));
                continue;
            };
            let result = std::fs::read_to_string(&path)
                .map_err(|e| e.to_string())
                .and_then(|s| BlockModel::from_json(&s, game_render::layer_of));
            match result {
                Ok(model) => {
                    report.models.0.insert(block, model);
                }
                Err(e) => report.warnings.push(format!("{}: {e}", path.display())),
            }
        }
    }
    if !report.models.is_empty() {
        info!("3D-моделей блоков: {}", report.models.0.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    struct Dir(PathBuf);
    impl Dir {
        fn new(tag: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "voxium-packs-{tag}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }
        fn write(&self, relative: &str, bytes: &[u8]) {
            let path = self.0.join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, bytes).unwrap();
        }
        fn png(&self, relative: &str, size: u32, color: [u8; 4]) {
            let mut bytes = Vec::new();
            RgbaImage::from_pixel(size, size, Rgba(color))
                .write_to(
                    &mut std::io::Cursor::new(&mut bytes),
                    image::ImageFormat::Png,
                )
                .unwrap();
            self.write(relative, &bytes);
        }
    }
    impl Drop for Dir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    fn pixel(atlas: &BlockAtlas, layer: usize) -> [u8; 4] {
        atlas.pixel(layer, 0, 0)
    }
    fn blank() -> BlockAtlas {
        // 17 пустых квадратов 128×128, встроенных как PNG.
        let mut bytes = Vec::new();
        RgbaImage::from_pixel(128, 128, Rgba([1, 2, 3, 255]))
            .write_to(
                &mut std::io::Cursor::new(&mut bytes),
                image::ImageFormat::Png,
            )
            .unwrap();
        BlockAtlas::from_tiles(&[bytes.as_slice(); 17]).unwrap()
    }

    #[test]
    fn pack_textures_of_any_size_override_in_priority_order() {
        let dir = Dir::new("tex");
        dir.write("a_low/pack.json", br#"{"priority": 0}"#);
        dir.png(
            "a_low/assets/voxium/textures/block/dirt.png",
            16,
            [10, 20, 30, 255],
        );
        dir.png(
            "a_low/assets/minecraft/textures/block/grass_block_top.png",
            32,
            [9, 9, 9, 255],
        );
        dir.write("b_high/pack.json", br#"{"priority": 5, "pixel_art": true}"#);
        dir.png(
            "b_high/assets/voxium/textures/block/dirt.png",
            256,
            [200, 100, 50, 255],
        );
        dir.write("c_off/pack.json", br#"{"enabled": false}"#);
        dir.png(
            "c_off/assets/voxium/textures/block/stone.png",
            16,
            [7, 7, 7, 255],
        );
        let mut atlas = blank();
        let report = apply(&mut atlas, &dir.0);
        assert_eq!(report.packs, ["a_low", "b_high"]);
        assert!(report.pixel_art && atlas.pixel_art);
        assert_eq!(
            pixel(&atlas, 2),
            [200, 100, 50, 255],
            "высокий приоритет побеждает"
        );
        assert_eq!(
            pixel(&atlas, 0),
            [9, 9, 9, 255],
            "псевдоним grass_block_top"
        );
        assert_eq!(
            pixel(&atlas, 3),
            [1, 2, 3, 255],
            "выключенный пак не применён"
        );
        // Увеличение — ближайшим соседом: внутри слоя нет промежуточных цветов.
        assert_eq!(atlas.pixel(0, 127, 127), [9, 9, 9, 255]);
    }

    #[test]
    fn bad_files_become_warnings_and_models_bind_to_blocks() {
        let dir = Dir::new("model");
        dir.write("p/assets/voxium/textures/block/dirt.png", b"not a png");
        dir.write(
            "p/assets/voxium/models/block/crafting_table.json",
            br##"{"textures":{"t":"crafting_table_top"},"elements":[{"from":[0,0,0],"to":[16,12,16],
                "faces":{"up":{"texture":"#t"}}}]}"##,
        );
        dir.write("p/assets/voxium/models/block/nonsense.json", b"{}");
        dir.write("p/assets/voxium/models/block/furnace.json", b"{ broken");
        let mut atlas = blank();
        let report = apply(&mut atlas, &dir.0);
        assert!(report.models.get(game_core::CRAFTING_TABLE).is_some());
        assert!(report.models.get(game_core::FURNACE).is_none());
        assert_eq!(report.textures, 0);
        assert_eq!(report.warnings.len(), 3, "{:?}", report.warnings);
        assert_eq!(pixel(&atlas, 2), [1, 2, 3, 255]);
    }

    #[test]
    fn bundled_example_pack_loads_without_warnings() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resourcepacks");
        let mut atlas = blank();
        let report = apply(&mut atlas, &root);
        assert_eq!(report.packs, ["Voxium 3D Example"]);
        assert!(report.warnings.is_empty(), "{:?}", report.warnings);
        assert!(report.models.get(game_core::CRAFTING_TABLE).is_some());
        assert!(report.models.get(game_core::CHEST).is_some());
        assert!(report.pixel_art);
    }

    #[test]
    fn missing_directory_is_a_no_op() {
        let mut atlas = blank();
        let report = apply(&mut atlas, Path::new("/definitely/not/here"));
        assert!(report.packs.is_empty() && !report.pixel_art);
    }
}
