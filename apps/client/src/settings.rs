use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub distance: i32,
    pub sensitivity: f32,
    pub vsync: bool,
    pub fullscreen: bool,
    pub resolution: usize,
    pub volume: f32,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            distance: 4,
            sensitivity: 0.002,
            vsync: true,
            fullscreen: false,
            resolution: 0,
            volume: 0.8,
        }
    }
}
pub fn save_root() -> PathBuf {
    std::env::var_os("VOXEL_SAVE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("saves"))
}
impl Settings {
    pub fn load() -> Self {
        fs::read(save_root().join("settings.cfg"))
            .ok()
            .and_then(|v| serde_json::from_slice::<Self>(&v).ok())
            .map(|mut s| {
                s.distance = s.distance.clamp(4, 16);
                if !s.sensitivity.is_finite() {
                    s.sensitivity = 0.002;
                }
                s.sensitivity = s.sensitivity.clamp(0.0005, 0.01);
                s.resolution %= 3;
                if !s.volume.is_finite() {
                    s.volume = 0.8;
                }
                s.volume = s.volume.clamp(0., 1.);
                s
            })
            .unwrap_or_default()
    }
    pub fn save(&self) -> Result<(), String> {
        fs::create_dir_all(save_root()).map_err(|e| e.to_string())?;
        let path = save_root().join("settings.cfg");
        let temp = path.with_extension("cfg.tmp");
        fs::write(
            &temp,
            serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        fs::rename(&temp, path).map_err(|e| e.to_string())
    }
}
