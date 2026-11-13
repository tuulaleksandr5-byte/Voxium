//! Import Minecraft Java texture packs without extracting files to disk.
//! OptiFine support is deliberately limited to unconditional custom container GUI textures.
use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use image::{RgbaImage, imageops::FilterType};
use std::{
    collections::HashMap,
    io::{Cursor, Read},
    path::PathBuf,
    sync::OnceLock,
};

const MAX_ENTRIES: usize = 30_000;
const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_TOTAL_BYTES: u64 = 256 * 1024 * 1024;

#[derive(Debug, Default)]
pub(crate) struct ResourcePack {
    files: HashMap<String, Vec<u8>>,
}

impl ResourcePack {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
            .map_err(|e| format!("некорректный ZIP ресурспака: {e}"))?;
        if archive.len() > MAX_ENTRIES {
            return Err("Ресурспак содержит слишком много файлов".into());
        }
        let mut result = Self::default();
        let mut total = 0u64;
        for index in 0..archive.len() {
            let mut entry = archive.by_index(index).map_err(|e| e.to_string())?;
            if entry.is_dir() || !safe_entry(entry.name()) || !interesting(entry.name()) {
                continue;
            }
            if entry.size() > MAX_FILE_BYTES {
                continue;
            }
            total = total.saturating_add(entry.size());
            if total > MAX_TOTAL_BYTES {
                return Err("Ресурспак превышает разрешённый объём изображений".into());
            }
            let key = entry.name().to_owned();
            let mut data = Vec::new();
            entry
                .take(MAX_FILE_BYTES + 1)
                .read_to_end(&mut data)
                .map_err(|e| format!("не удалось распаковать {key}: {e}"))?;
            if data.len() as u64 > MAX_FILE_BYTES {
                continue;
            }
            result.files.insert(key, data);
        }
        Ok(result)
    }

    fn read_path(&self, path: &str) -> Option<&[u8]> {
        self.files.get(path).map(Vec::as_slice)
    }

    /// Translate stable Voxium atlas names into Minecraft Java block texture names.
    pub(crate) fn block_tile(&self, name: &str) -> Option<RgbaImage> {
        let minecraft = match name {
            "grass_top" => "grass_block_top",
            "grass_side" => "grass_block_side",
            "oak_planks" => "oak_planks",
            other => other,
        };
        for directory in ["block", "blocks"] {
            let path = format!("assets/minecraft/textures/{directory}/{minecraft}.png");
            if let Some(raw) = self.read_path(&path) {
                return decode_tile(raw);
            }
        }
        None
    }

    pub(crate) fn item_pixels(&self, id: &str) -> Option<Vec<u8>> {
        let name = id.strip_prefix("survival:")?;
        for directory in ["item", "items"] {
            let path = format!("assets/minecraft/textures/{directory}/{name}.png");
            if let Some(raw) = self.read_path(&path) {
                return decode_tile(raw).map(RgbaImage::into_raw);
            }
        }
        None
    }

    /// Only apply unconditional OptiFine GUI rules; gameplay does not track
    /// the NBT, biome, profession, or custom-name predicates required by other rules.
    fn optifine_gui_image(&self, container: &str) -> Option<RgbaImage> {
        let mut rules: Vec<_> = self
            .files
            .keys()
            .filter(|name| {
                name.starts_with("assets/minecraft/optifine/gui/container/")
                    && name.ends_with(".properties")
            })
            .collect();
        rules.sort();
        for path in rules {
            let source = std::str::from_utf8(self.read_path(path)?).ok()?;
            let properties: HashMap<&str, &str> = source
                .lines()
                .filter_map(|line| {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        return None;
                    }
                    line.split_once('=').map(|(k, v)| (k.trim(), v.trim()))
                })
                .collect();
            if properties.get("container").copied() != Some(container) {
                continue;
            }
            // Only these boolean restrictions can be evaluated without gameplay context.
            if properties.iter().any(|(key, value)| match *key {
                "container" | "texture" => false,
                "ender" | "christmas" | "trapped" | "large" => *value != "false",
                _ => true,
            }) {
                continue;
            }
            let Some(file) = properties.get("texture") else {
                continue;
            };
            if file.contains("..") || file.contains('\\') || file.starts_with('/') {
                continue;
            }
            let parent = path.rsplit_once('/')?.0;
            let image = if let Some(name) = file.strip_prefix("minecraft:") {
                format!("assets/minecraft/{name}")
            } else if let Some(name) = file.strip_prefix("./") {
                format!("{parent}/{name}")
            } else {
                format!("{parent}/{file}")
            };
            let image = if image.ends_with(".png") {
                image
            } else {
                format!("{image}.png")
            };
            if let Some(data) = self.read_path(&image) {
                if let Some(pixels) = decode_gui(data) {
                    return Some(pixels);
                }
            }
        }
        None
    }

    pub(crate) fn gui_image(&self, name: &str) -> Option<RgbaImage> {
        let (container, default_name) = match name {
            "inventory" => ("inventory", "inventory"),
            "workbench" => ("crafting", "crafting_table"),
            "furnace" => ("furnace", "furnace"),
            "chest" => ("chest", "generic_54"),
            _ => return None,
        };
        self.optifine_gui_image(container).or_else(|| {
            let path = format!("assets/minecraft/textures/gui/container/{default_name}.png");
            self.read_path(&path).and_then(decode_gui)
        })
    }

    pub(crate) fn apply_block_tiles(&self, atlas: &mut game_render::BlockAtlas) -> usize {
        const OLD_TILES: [&str; 17] = [
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
        let names = OLD_TILES.into_iter().map(str::to_owned).chain(
            game_render::ADDITIONAL_TILE_NAMES
                .iter()
                .map(|s| (*s).to_owned()),
        );
        let mut replaced = 0;
        for (layer, name) in names.enumerate() {
            let layer = if layer < 17 { layer } else { layer + 1 };
            if let Some(image) = self.block_tile(&name) {
                if atlas.replace_layer_rgba(layer, &image).is_ok() {
                    replaced += 1;
                }
            }
        }
        replaced
    }
}

/// No filesystem extraction; all active images are read once at startup.
pub(crate) fn active() -> Option<&'static ResourcePack> {
    static ACTIVE: OnceLock<Option<ResourcePack>> = OnceLock::new();
    ACTIVE
        .get_or_init(|| {
            let path = std::env::var_os("VOXIUM_RESOURCE_PACK")
                .map(PathBuf::from)
                .or_else(|| {
                    crate::graphics_assets::asset_path("assets/resourcepacks/active.zip")
                })?;
            match std::fs::read(&path)
                .map_err(|e| e.to_string())
                .and_then(|bytes| ResourcePack::from_bytes(&bytes))
            {
                Ok(pack) => {
                    info!(
                        "Loaded Minecraft resource pack: {} ({} supported files)",
                        path.display(),
                        pack.files.len()
                    );
                    Some(pack)
                }
                Err(error) => {
                    warn!("Could not load resource pack {}: {error}", path.display());
                    None
                }
            }
        })
        .as_ref()
}

fn safe_entry(path: &str) -> bool {
    path.len() <= 512
        && !path.contains('\\')
        && path
            .split('/')
            .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
}
fn interesting(path: &str) -> bool {
    let base = "assets/minecraft/";
    let Some(path) = path.strip_prefix(base) else {
        return false;
    };
    let png = path.ends_with(".png");
    let properties = path.ends_with(".properties");
    (png && (path.starts_with("textures/block/")
        || path.starts_with("textures/blocks/")
        || path.starts_with("textures/item/")
        || path.starts_with("textures/items/")
        || path.starts_with("textures/gui/container/")
        || path.starts_with("optifine/gui/container/")))
        || (properties && path.starts_with("optifine/gui/container/"))
}
fn decode_tile(raw: &[u8]) -> Option<RgbaImage> {
    let (width, height) =
        image::ImageReader::with_format(Cursor::new(raw), image::ImageFormat::Png)
            .into_dimensions()
            .ok()?;
    if width != height || ![16, 32, 64, 128, 256, 512].contains(&width) {
        return None;
    }
    let image = image::load_from_memory_with_format(raw, image::ImageFormat::Png)
        .ok()?
        .into_rgba8();
    if width == 128 {
        Some(image)
    } else {
        Some(image::imageops::resize(
            &image,
            128,
            128,
            FilterType::Nearest,
        ))
    }
}
fn decode_gui(raw: &[u8]) -> Option<RgbaImage> {
    let (width, height) =
        image::ImageReader::with_format(Cursor::new(raw), image::ImageFormat::Png)
            .into_dimensions()
            .ok()?;
    if width == 0 || height == 0 || width > 1024 || height > 1024 {
        return None;
    }
    image::load_from_memory_with_format(raw, image::ImageFormat::Png)
        .ok()
        .map(image::DynamicImage::into_rgba8)
}

#[derive(Resource, Default)]
pub(crate) struct GuiSkins {
    inventory: Option<Handle<Image>>,
    workbench: Option<Handle<Image>>,
    furnace: Option<Handle<Image>>,
    chest: Option<Handle<Image>>,
}
impl GuiSkins {
    pub(crate) fn get(&self, name: &str) -> Option<Handle<Image>> {
        match name {
            "inventory" => self.inventory.clone(),
            "workbench" => self.workbench.clone(),
            "furnace" => self.furnace.clone(),
            "chest" => self.chest.clone(),
            _ => None,
        }
    }
}
pub(crate) fn load_gui_skins(images: &mut Assets<Image>) -> GuiSkins {
    let Some(pack) = active() else {
        return GuiSkins::default();
    };
    let mut skins = GuiSkins::default();
    for (name, output) in [
        ("inventory", &mut skins.inventory),
        ("workbench", &mut skins.workbench),
        ("furnace", &mut skins.furnace),
        ("chest", &mut skins.chest),
    ] {
        let Some(img) = pack.gui_image(name) else {
            continue;
        };
        // Minecraft GUI panels occupy the top-left 176×166 (large chest: 176×222)
        // of the vanilla 256×256 texture. Keep its pixel artwork intact.
        let width = (img.width() * 176 / 256).max(1).min(img.width());
        let height = (img.height() * if name == "chest" { 222 } else { 166 } / 256)
            .max(1)
            .min(img.height());
        let cropped = image::imageops::crop_imm(&img, 0, 0, width, height).to_image();
        let mut texture = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            cropped.into_raw(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );
        texture.sampler = ImageSampler::nearest();
        *output = Some(images.add(texture));
    }
    skins
}

// Resource-pack import tests are written before the ZIP loader implementation.
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};
    use zip::{ZipWriter, write::SimpleFileOptions};

    fn png(size: u32) -> Vec<u8> {
        let mut dst = Cursor::new(Vec::new());
        image::RgbaImage::from_fn(size, size, |x, y| {
            if x == 0 && y == 0 {
                image::Rgba([31, 63, 95, 127])
            } else {
                image::Rgba([12, 24, 36, 255])
            }
        })
        .write_to(&mut dst, image::ImageFormat::Png)
        .unwrap();
        dst.into_inner()
    }
    fn pack(entries: &[(&str, &[u8])]) -> ResourcePack {
        let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
        for &(path, bytes) in entries {
            writer
                .start_file(path, SimpleFileOptions::default())
                .unwrap();
            writer.write_all(bytes).unwrap();
        }
        let zip = writer.finish().unwrap().into_inner();
        ResourcePack::from_bytes(&zip).unwrap()
    }

    #[test]
    fn accepts_minecraft_block_png_and_scales_16_nearest_with_alpha() {
        let image = png(16);
        let pack = pack(&[(
            "assets/minecraft/textures/block/grass_block_top.png",
            &image,
        )]);
        let tile = pack.block_tile("grass_top").unwrap();
        assert_eq!(tile.dimensions(), (128, 128));
        assert_eq!(&tile.get_pixel(0, 0).0, &[31, 63, 95, 127]);
        assert_eq!(&tile.get_pixel(7, 7).0, &[31, 63, 95, 127]);
        assert_eq!(&tile.get_pixel(8, 8).0, &[12, 24, 36, 255]);
        assert!(pack.block_tile("furnace_top").is_none());
    }

    #[test]
    fn reads_item_texture_from_java_pack() {
        let bytes = png(32);
        let pack = pack(&[("assets/minecraft/textures/item/iron_pickaxe.png", &bytes)]);
        let pixels = pack.item_pixels("survival:iron_pickaxe").unwrap();
        assert_eq!(pixels.len(), 128 * 128 * 4);
        assert_eq!(&pixels[..4], &[31, 63, 95, 127]);
        assert!(pack.item_pixels("survival:missing").is_none());
    }

    #[test]
    fn vanilla_and_unconditional_optifine_gui_chest_override() {
        let base = png(128);
        let custom = png(256);
        let rule = b"container=chest\ntexture=chest\nender=false\nchristmas=false\n";
        let pack = pack(&[
            (
                "assets/minecraft/textures/gui/container/generic_54.png",
                &base,
            ),
            (
                "assets/minecraft/optifine/gui/container/chests/chest.properties",
                rule,
            ),
            (
                "assets/minecraft/optifine/gui/container/chests/chest.png",
                &custom,
            ),
        ]);
        let chest = pack.gui_image("chest").unwrap();
        assert_eq!(chest.dimensions(), (256, 256));
        assert_eq!(&chest.get_pixel(0, 0).0, &[31, 63, 95, 127]);
    }

    #[test]
    fn optifine_conditioned_rules_are_not_applied_globally() {
        let base = png(128);
        let rules = b"container=chest\nname=SpecialChest\ntexture=custom\n";
        let custom = png(256);
        let pack = pack(&[
            (
                "assets/minecraft/textures/gui/container/generic_54.png",
                &base,
            ),
            (
                "assets/minecraft/optifine/gui/container/test.properties",
                rules,
            ),
            (
                "assets/minecraft/optifine/gui/container/custom.png",
                &custom,
            ),
        ]);
        assert_eq!(pack.gui_image("chest").unwrap().dimensions(), (128, 128));
    }

    #[test]
    fn ignores_unsafe_names_and_invalid_png() {
        let invalid = b"not a png";
        let pack = pack(&[
            ("../assets/minecraft/textures/block/stone.png", &png(16)),
            ("assets/minecraft/textures/block/stone.png", invalid),
        ]);
        assert!(pack.block_tile("stone").is_none());
        assert!(ResourcePack::from_bytes(b"not zip").is_err());
    }
}
