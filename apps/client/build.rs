use std::{env, fs, path::PathBuf};
fn main() {
    let path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("../../assets/textures/atlas_128x128.png");
    // Track creation of the optional file without an always-missing dependency
    // that would otherwise force every Cargo invocation to rebuild the client.
    println!(
        "cargo:rerun-if-changed={}",
        path.parent().unwrap().display()
    );
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let source = if path.is_file() {
        println!("cargo:rerun-if-changed={}", path.display());
        fs::copy(&path, out.join("atlas.png")).unwrap();
        "pub const EMBEDDED_ATLAS: Option<&[u8]> = Some(include_bytes!(concat!(env!(\"OUT_DIR\"), \"/atlas.png\")));"
    } else {
        "pub const EMBEDDED_ATLAS: Option<&[u8]> = None;"
    };
    fs::write(out.join("atlas_asset.rs"), source).unwrap();
}
