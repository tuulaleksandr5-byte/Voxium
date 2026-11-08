//! The original PNG is required for worlds; menus remain usable if it is missing.
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

fn asset_path(relative: &str) -> Option<PathBuf> {
    let next_to_exe = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join(relative)));
    if let Some(p) = next_to_exe.filter(|p| p.is_file()) {
        return Some(p);
    }
    let p = PathBuf::from(relative);
    p.is_file().then_some(p)
}

pub fn load_atlas() -> Result<Vec<u8>, String> {
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
        return Ok(bytes);
    }
    EMBEDDED_ATLAS.map(<[u8]>::to_vec).ok_or_else(||
        "Нужен assets/atlas_128x128.png (768×384). Положите PNG рядом с игрой по этому пути и перезапустите её.".into())
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
