//! Unchanged PNG assets: legacy atlas/cut tiles plus the supplied new block tiles.
use bevy::prelude::*;
use std::path::{Path, PathBuf};
include!(concat!(env!("OUT_DIR"), "/atlas_asset.rs"));

#[derive(Resource, Default)]
pub struct BlockIcons {
    pub handles: Vec<Handle<Image>>,
    pub error: Option<String>,
    /// Стиль, из которого собраны текущие материалы и иконки.
    pub pixel: Option<bool>,
}
impl BlockIcons {
    pub fn get(&self, block: u8) -> Handle<Image> {
        self.handles
            .get(block as usize)
            .cloned()
            .unwrap_or_default()
    }
}

pub(crate) fn asset_path(relative: &str) -> Option<PathBuf> {
    let executable_directory = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf));
    asset_path_in(relative, executable_directory.as_deref(), Path::new(""))
}

fn asset_path_in(
    relative: &str,
    executable_directory: Option<&Path>,
    working_directory: &Path,
) -> Option<PathBuf> {
    let next_to_exe = executable_directory.map(|p| p.join(relative));
    if let Some(p) = next_to_exe.filter(|p| p.is_file()) {
        return Some(p);
    }
    let p = working_directory.join(relative);
    p.is_file().then_some(p)
}

const TILE_NAMES: [&str; 17] = [
    "grass_top",
    "grass_side",
    "dirt",
    "stone",
    "deepslate",
    "sand",
    "gravel",
    "oak_log",
    "oak_log_top",
    "oak_leaves",
    "oak_planks",
    "cobblestone",
    "glass",
    "water",
    "snow",
    "sandstone",
    "sandstone_top",
];
macro_rules! original_tile {
    ($name:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/textures/",
            $name,
            ".png"
        )) as &'static [u8]
    };
}
const ORIGINAL_TILES: [&[u8]; 17] = [
    original_tile!("grass_top"),
    original_tile!("grass_side"),
    original_tile!("dirt"),
    original_tile!("stone"),
    original_tile!("deepslate"),
    original_tile!("sand"),
    original_tile!("gravel"),
    original_tile!("oak_log"),
    original_tile!("oak_log_top"),
    original_tile!("oak_leaves"),
    original_tile!("oak_planks"),
    original_tile!("cobblestone"),
    original_tile!("glass"),
    original_tile!("water"),
    original_tile!("snow"),
    original_tile!("sandstone"),
    original_tile!("sandstone_top"),
];

/// Пиксельный набор полностью встроен; рисованный читает исходные PNG и замены.
pub fn load_atlas_for(pixel: bool) -> Result<game_render::BlockAtlas, String> {
    if pixel {
        game_render::BlockAtlas::pixel()
    } else {
        load_atlas()
    }
}

/// Переключение стиля в настройках: новые материалы и иконки без перезапуска.
pub fn apply_style(
    mut game: ResMut<crate::session::Game>,
    mut stream: ResMut<crate::streaming::Streaming>,
    mut icons: ResMut<BlockIcons>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<game_render::VoxelMaterial>>,
) {
    let wanted = game.settings.pixel_textures;
    if icons.pixel.is_none() || icons.pixel == Some(wanted) {
        return;
    }
    icons.pixel = Some(wanted);
    let loaded = load_atlas_for(wanted).and_then(|atlas| {
        let baked = game_render::bake_atlas_icons(&atlas, &stream.textures)?;
        let handles =
            game_render::create_materials_from_atlas(&mut images, &mut materials, &atlas)?;
        Ok((baked, handles))
    });
    match loaded {
        Ok((baked, handles)) => {
            icons.handles = baked.into_iter().map(|image| images.add(image)).collect();
            stream.materials = handles;
            // Секции перестраиваются с новыми материалами по обычной очереди.
            let sections: Vec<[i32; 3]> = game
                .world
                .chunks
                .values()
                .flat_map(|chunk| {
                    chunk
                        .sections
                        .keys()
                        .map(|y| [chunk.position[0], *y, chunk.position[1]])
                        .collect::<Vec<_>>()
                })
                .collect();
            game.world.dirty.extend(sections);
        }
        Err(error) => {
            warn!("Не удалось сменить графику: {error}");
            game.status = error;
        }
    }
    game.ui_dirty = true;
}

pub fn load_atlas() -> Result<game_render::BlockAtlas, String> {
    load_atlas_with_overrides(load_original_atlas, asset_path)
}

fn load_atlas_with_overrides(
    load_original_atlas: impl FnOnce() -> Result<game_render::BlockAtlas, String>,
    asset_path: impl Fn(&str) -> Option<PathBuf>,
) -> Result<game_render::BlockAtlas, String> {
    let mut atlas = load_original_atlas()?;
    for (offset, name) in game_render::ADDITIONAL_TILE_NAMES.iter().enumerate() {
        let Some(path) = asset_path(&format!("assets/textures/blocks/128x128/{name}.png")) else {
            continue;
        };
        // Необязательная замена влияет только на свой слой после полной проверки PNG.
        let result = std::fs::read(&path)
            .map_err(|e| e.to_string())
            .and_then(|bytes| atlas.replace_additional_tile(offset, &bytes));
        if let Err(error) = result {
            warn!(
                "Не удалось загрузить текстуру {}: {error}; используется встроенный оригинал",
                path.display()
            );
        }
    }
    Ok(atlas)
}

fn load_original_atlas() -> Result<game_render::BlockAtlas, String> {
    let path = std::env::var_os("VOXEL_ATLAS")
        .map(PathBuf::from)
        .or_else(|| asset_path("assets/atlas_128x128.png"))
        .or_else(|| asset_path("assets/textures/atlas_128x128.png"));
    if let Some(path) = path {
        let bytes = std::fs::read(&path)
            .map_err(|e| format!("Не удалось прочитать атлас {}: {e}", path.display()))?;
        info!(
            "Loading original atlas without modifying PNG: {}",
            path.display()
        );
        return game_render::BlockAtlas::from_png(&bytes);
    }
    if let Some(first) = asset_path("assets/textures/grass_top.png") {
        let folder = first.parent().expect("tile has a parent directory");
        let mut files = Vec::with_capacity(TILE_NAMES.len());
        for name in TILE_NAMES {
            let path = folder.join(format!("{name}.png"));
            files.push(
                std::fs::read(&path).map_err(|e| {
                    format!("Не удалось прочитать текстуру {}: {e}", path.display())
                })?,
            );
        }
        let tiles = files.iter().map(Vec::as_slice).collect::<Vec<_>>();
        info!(
            "Loading 17 original cut PNG textures unchanged from {}",
            folder.display()
        );
        return game_render::BlockAtlas::from_tiles(&tiles);
    }
    if let Some(bytes) = EMBEDDED_ATLAS {
        return game_render::BlockAtlas::from_png(bytes);
    }
    info!("Loading 17 original user PNG textures embedded byte-for-byte in EXE");
    game_render::BlockAtlas::from_tiles(&ORIGINAL_TILES)
}

pub fn load_face_textures() -> game_render::BlockTextures {
    if let Some(path) = asset_path("assets/blocks.ron") {
        let parsed = std::fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|s| game_render::BlockTextures::from_ron(&s));
        match parsed {
            Ok(textures) => return textures,
            Err(e) => error!(
                "Invalid face textures {}: {e}; using embedded mapping",
                path.display()
            ),
        }
    }
    game_render::BlockTextures::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_core::{COAL_ORE, GOLD_ORE};
    use std::{
        io::Cursor,
        sync::atomic::{AtomicUsize, Ordering},
    };

    const GOLD: [u8; 4] = [200, 120, 80, 113];
    const COAL: [u8; 4] = [80, 200, 120, 173];

    struct Overrides(PathBuf);

    impl Overrides {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "voxel-block-overrides-{}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                NEXT.fetch_add(1, Ordering::Relaxed),
            ));
            std::fs::create_dir(&path).unwrap();
            Self(path)
        }

        fn path(&self, root: &str, name: &str) -> PathBuf {
            self.0
                .join(root)
                .join(format!("assets/textures/blocks/128x128/{name}.png"))
        }

        fn write(&self, root: &str, name: &str, bytes: &[u8]) {
            let path = self.path(root, name);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, bytes).unwrap();
        }

        fn resolve(&self, relative: &str) -> Option<PathBuf> {
            asset_path_in(relative, Some(&self.0.join("exe")), &self.0.join("cwd"))
        }

        fn complete(&self) {
            for name in game_render::ADDITIONAL_TILE_NAMES {
                let source = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../assets/textures/blocks/128x128")
                    .join(format!("{name}.png"));
                self.write("exe", name, &std::fs::read(source).unwrap());
            }
        }

        fn load(&self) -> game_render::BlockAtlas {
            load_atlas_with_overrides(embedded, |relative| self.resolve(relative)).unwrap()
        }
    }

    impl Drop for Overrides {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }

    fn embedded() -> Result<game_render::BlockAtlas, String> {
        game_render::BlockAtlas::from_tiles(&ORIGINAL_TILES)
    }

    fn png(width: u32, height: u32, rgba: [u8; 4]) -> Vec<u8> {
        let mut bytes = Cursor::new(Vec::new());
        image::RgbaImage::from_pixel(width, height, image::Rgba(rgba))
            .write_to(&mut bytes, image::ImageFormat::Png)
            .unwrap();
        bytes.into_inner()
    }

    fn outputs(atlas: &game_render::BlockAtlas) -> (Image, Vec<Image>) {
        let mut images = Assets::<Image>::default();
        let mut materials = Assets::<game_render::VoxelMaterial>::default();
        let handles =
            game_render::create_materials_from_atlas(&mut images, &mut materials, atlas).unwrap();
        let array = materials.get(&handles[0]).unwrap().extension.image.clone();
        let gpu = images.get(&array).unwrap().clone();
        assert_eq!(handles.len(), 72);
        assert_eq!(gpu.texture_descriptor.size.depth_or_array_layers, 72);
        (
            gpu,
            game_render::bake_atlas_icons(atlas, &game_render::BlockTextures::default()).unwrap(),
        )
    }

    fn layer(gpu: &Image, index: usize) -> &[u8] {
        let bytes = gpu.data.as_ref().unwrap();
        &bytes[index * (bytes.len() / 72)..][..128 * 128 * 4]
    }

    fn assert_composition(atlas: &game_render::BlockAtlas, changed: &[(usize, u8, [u8; 4])]) {
        let (gpu, icons) = outputs(atlas);
        let (defaults, default_icons) = outputs(&embedded().unwrap());
        for index in 0..72 {
            if let Some((_, _, rgba)) = changed.iter().find(|&&(i, _, _)| i == index) {
                assert!(
                    layer(&gpu, index)
                        .as_chunks::<4>()
                        .0
                        .iter()
                        .all(|p| p == rgba),
                    "GPU layer {index}"
                );
            } else {
                assert_eq!(
                    layer(&gpu, index),
                    layer(&defaults, index),
                    "unchanged GPU layer {index}"
                );
            }
        }
        for (block, icon) in icons.iter().enumerate() {
            if let Some((_, _, rgba)) = changed.iter().find(|&&(_, b, _)| b as usize == block) {
                let [top, front, side] = match *rgba {
                    GOLD => [GOLD, [170, 102, 68, 113], [140, 84, 56, 113]],
                    COAL => [COAL, [68, 170, 102, 173], [56, 140, 84, 173]],
                    _ => panic!("unexpected fixture color"),
                };
                for (x, y, expected) in [(64, 32, top), (40, 75, front), (88, 75, side)] {
                    assert_eq!(
                        &icon.data.as_ref().unwrap()[(y * 128 + x) * 4..][..4],
                        expected,
                        "icon {block} at {x},{y}"
                    );
                }
            } else {
                assert_eq!(
                    icon.data, default_icons[block].data,
                    "unchanged icon {block}"
                );
            }
        }
    }

    #[test]
    fn missing_new_override_directory_keeps_embedded_materials_and_icons() {
        assert_composition(&Overrides::new().load(), &[]);
    }

    #[test]
    fn partial_gold_override_reaches_materials_and_cube_icons() {
        let files = Overrides::new();
        files.write("exe", "gold_ore", &png(128, 128, GOLD));
        assert_composition(&files.load(), &[(18, GOLD_ORE, GOLD)]);
    }

    #[test]
    fn lone_non_gold_override_reaches_materials_and_cube_icons() {
        let files = Overrides::new();
        files.write("cwd", "coal_ore", &png(128, 128, COAL));
        assert_composition(&files.load(), &[(19, COAL_ORE, COAL)]);
    }

    #[test]
    fn corrupt_override_keeps_own_embedded_tile_and_other_valid_override() {
        let files = Overrides::new();
        files.complete();
        files.write("exe", "gold_ore", &png(128, 128, GOLD));
        files.write("exe", "coal_ore", b"invalid PNG");
        assert_composition(&files.load(), &[(18, GOLD_ORE, GOLD)]);
    }

    #[test]
    fn wrong_size_override_keeps_own_embedded_tile_and_other_valid_override() {
        let files = Overrides::new();
        files.complete();
        files.write("exe", "gold_ore", &png(127, 128, GOLD));
        files.write("exe", "coal_ore", &png(128, 128, COAL));
        assert_composition(&files.load(), &[(19, COAL_ORE, COAL)]);
    }

    #[test]
    fn truncated_override_keeps_embedded_tile_and_later_valid_override() {
        let files = Overrides::new();
        files.complete();
        let mut truncated = png(128, 128, GOLD);
        truncated.truncate(33);
        files.write("exe", "gold_ore", &truncated);
        files.write("exe", "coal_ore", &png(128, 128, COAL));
        assert_composition(&files.load(), &[(19, COAL_ORE, COAL)]);
    }

    #[test]
    fn original_atlas_failure_still_propagates() {
        let result = load_atlas_with_overrides(
            || Err("invalid legacy atlas".into()),
            |_| panic!("overrides require a valid original atlas"),
        );
        assert_eq!(result.unwrap_err(), "invalid legacy atlas");
    }

    #[test]
    fn unreadable_override_keeps_own_embedded_tile_and_other_valid_override() {
        let files = Overrides::new();
        files.complete();
        files.write("exe", "gold_ore", &png(128, 128, GOLD));
        let unreadable = files.path("exe", "coal_ore");
        std::fs::remove_file(&unreadable).unwrap();
        std::fs::create_dir(&unreadable).unwrap();
        let atlas = load_atlas_with_overrides(embedded, |relative| {
            if relative.ends_with("coal_ore.png") {
                Some(unreadable.clone())
            } else {
                files.resolve(relative)
            }
        })
        .unwrap();
        assert_composition(&atlas, &[(18, GOLD_ORE, GOLD)]);
    }

    #[test]
    fn each_override_uses_executable_then_working_directory_precedence() {
        let files = Overrides::new();
        files.write("exe", "gold_ore", &png(128, 128, GOLD));
        files.write("cwd", "gold_ore", b"invalid PNG");
        files.write("cwd", "coal_ore", &png(128, 128, COAL));
        assert_composition(&files.load(), &[(18, GOLD_ORE, GOLD), (19, COAL_ORE, COAL)]);
    }
}
