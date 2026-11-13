// Resource-pack import tests are written before the ZIP loader implementation.
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};
    use zip::{write::SimpleFileOptions, ZipWriter};

    fn png(size: u32) -> Vec<u8> {
        let mut dst = Cursor::new(Vec::new());
        image::RgbaImage::from_fn(size, size, |x, y| {
            if x == 0 && y == 0 { image::Rgba([31, 63, 95, 127]) }
            else { image::Rgba([12, 24, 36, 255]) }
        })
        .write_to(&mut dst, image::ImageFormat::Png).unwrap();
        dst.into_inner()
    }
    fn pack(entries: &[(&str, &[u8])]) -> ResourcePack {
        let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
        for &(path, bytes) in entries {
            writer.start_file(path, SimpleFileOptions::default()).unwrap();
            writer.write_all(bytes).unwrap();
        }
        let zip = writer.finish().unwrap().into_inner();
        ResourcePack::from_bytes(&zip).unwrap()
    }

    #[test]
    fn accepts_minecraft_block_png_and_scales_16_nearest_with_alpha() {
        let image = png(16);
        let pack = pack(&[("assets/minecraft/textures/block/grass_block_top.png", &image)]);
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
            ("assets/minecraft/textures/gui/container/generic_54.png", &base),
            ("assets/minecraft/optifine/gui/container/chests/chest.properties", rule),
            ("assets/minecraft/optifine/gui/container/chests/chest.png", &custom),
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
            ("assets/minecraft/textures/gui/container/generic_54.png", &base),
            ("assets/minecraft/optifine/gui/container/test.properties", rules),
            ("assets/minecraft/optifine/gui/container/custom.png", &custom),
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
