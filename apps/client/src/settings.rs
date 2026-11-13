use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
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
        .unwrap_or_else(|| PathBuf::from("saves-v2"))
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
        self.save_to(&save_root())
    }
    fn save_to(&self, root: &Path) -> Result<(), String> {
        fs::create_dir_all(root).map_err(|e| e.to_string())?;
        let path = root.join("settings.cfg");
        let temp = path.with_extension("cfg.tmp");
        fs::write(
            &temp,
            serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        fs::rename(&temp, &path).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_size_migrates_and_cycles_128_to_16_to_32() {
        let old = br#"{"distance":4,"resolution":0,"volume":0.5}"#;
        let mut settings: Settings = serde_json::from_slice(old).unwrap();
        assert_eq!(settings.pixel_size, 128);
        settings.cycle_pixel_size();
        assert_eq!(settings.pixel_size, 16);
        settings.cycle_pixel_size();
        assert_eq!(settings.pixel_size, 32);
        settings.cycle_pixel_size();
        assert_eq!(settings.pixel_size, 128);
        assert_eq!(Settings::normalized_pixel_size(17), 128);
        let json = serde_json::to_vec(&settings).unwrap();
        assert_eq!(serde_json::from_slice::<Settings>(&json).unwrap().pixel_size, 128);
    }

    #[test]
    fn settings_can_be_saved_repeatedly_with_fullscreen_preserved() {
        let root = std::env::temp_dir().join(format!(
            "voxel-settings-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut settings = Settings::default();
        settings.save_to(&root).unwrap();
        settings.fullscreen = true;
        settings.volume = 0.4;
        settings.save_to(&root).unwrap();
        settings.distance = 8;
        settings.save_to(&root).unwrap();
        let stored: Settings =
            serde_json::from_slice(&fs::read(root.join("settings.cfg")).unwrap()).unwrap();
        assert!(stored.fullscreen);
        assert_eq!(stored.volume, 0.4);
        assert_eq!(stored.distance, 8);
        fs::remove_dir_all(root).unwrap();
    }
}
