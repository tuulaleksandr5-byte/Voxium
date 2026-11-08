//! Optional user artwork overrides embedded defaults; no external assets required to launch.
use bevy::prelude::*;
use std::path::PathBuf;
include!(concat!(env!("OUT_DIR"), "/atlas_asset.rs"));

fn asset_path(relative: &str) -> Option<PathBuf> {
    let next_to_exe = std::env::current_exe().ok()?.parent()?.join(relative);
    if next_to_exe.is_file() {
        Some(next_to_exe)
    } else {
        let p = PathBuf::from(relative);
        p.is_file().then_some(p)
    }
}

pub fn load_atlas() -> Option<Vec<u8>> {
    let path = std::env::var_os("VOXEL_ATLAS")
        .map(PathBuf::from)
        .or_else(|| asset_path("assets/textures/atlas_128x128.png"));
    if let Some(path) = path {
        match std::fs::read(&path) {
            Ok(bytes) => {
                info!("Loading atlas {}", path.display());
                return Some(bytes);
            }
            Err(e) => error!("Cannot read atlas {}: {e}", path.display()),
        }
    }
    if let Some(bytes) = EMBEDDED_ATLAS {
        return Some(bytes.to_vec());
    }
    warn!("Original atlas_128x128.png was not supplied; using smooth temporary artwork");
    None
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
