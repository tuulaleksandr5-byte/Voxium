//! Unchanged PNG assets: legacy atlas/cut tiles plus the supplied new block tiles.
use bevy::prelude::*;
use std::path::PathBuf;
include!(concat!(env!("OUT_DIR"), "/atlas_asset.rs"));

#[derive(Resource, Default)]
pub struct BlockIcons {
    pub handles: Vec<Handle<Image>>,
    pub error: Option<String>,
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
    let next_to_exe = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join(relative)));
    if let Some(p) = next_to_exe.filter(|p| p.is_file()) {
        return Some(p);
    }
    let p = PathBuf::from(relative);
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

pub fn load_atlas() -> Result<game_render::BlockAtlas, String> {
    let atlas = load_original_atlas()?;
    let Some(first) = asset_path("assets/textures/blocks/128x128/gold_ore.png") else {
        // The renderer already appended embedded new PNGs for standalone builds
        // and legacy whole-atlas overrides.
        return Ok(atlas);
    };
    let folder = first.parent().expect("tile has a parent directory");
    let files = game_render::ADDITIONAL_TILE_NAMES
        .iter()
        .map(|name| {
            let path = folder.join(format!("{name}.png"));
            std::fs::read(&path)
                .map_err(|e| format!("Не удалось прочитать текстуру {}: {e}", path.display()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let tiles = files.iter().map(Vec::as_slice).collect::<Vec<_>>();
    atlas.with_additional_tiles(&tiles)
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
