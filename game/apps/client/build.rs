use std::{env, fs, path::PathBuf};
fn main() {
    let assets = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("../../assets");
    let primary = assets.join("atlas_128x128.png");
    let legacy = assets.join("textures/atlas_128x128.png");
    let path = if primary.is_file() { primary } else { legacy };
    println!("cargo:rerun-if-changed={}", assets.display());
    // Track creation of the optional file without an always-missing dependency
    // that would otherwise force every Cargo invocation to rebuild the client.
    println!(
        "cargo:rerun-if-changed={}",
        path.parent().unwrap().display()
    );
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    embed_windows_icon(&assets, &out);
    let source = if path.is_file() {
        println!("cargo:rerun-if-changed={}", path.display());
        fs::copy(&path, out.join("atlas.png")).unwrap();
        "pub const EMBEDDED_ATLAS: Option<&[u8]> = Some(include_bytes!(concat!(env!(\"OUT_DIR\"), \"/atlas.png\")));"
    } else {
        "pub const EMBEDDED_ATLAS: Option<&[u8]> = None;"
    };
    fs::write(out.join("atlas_asset.rs"), source).unwrap();
}

fn embed_windows_icon(assets: &std::path::Path, out: &std::path::Path) {
    use std::process::Command;
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let icon = assets.join("icons/app.ico");
    println!("cargo:rerun-if-changed={}", icon.display());
    let rc = out.join("app-icon.rc");
    let normalized = icon.to_string_lossy().replace('\\', "/");
    fs::write(&rc, format!("1 ICON DISCARDABLE \"{normalized}\"\n"))
        .expect("write icon resource script");
    let (status, resource) = if env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        let resource = out.join("app-icon.res");
        // Cargo can discover link.exe while rc.exe is absent from PATH in a
        // normal PowerShell session. Search the installed VS/Windows SDK too.
        let target = env::var("TARGET").expect("Cargo target");
        let mut compiler = if let Some(tool) = env::var_os("RC") {
            Command::new(tool)
        } else {
            find_msvc_tools::find(&target, "rc.exe").unwrap_or_else(|| Command::new("rc.exe"))
        };
        let status = compiler
            .arg("/nologo")
            .arg("/fo")
            .arg(&resource)
            .arg(&rc)
            .status()
            .expect("Windows SDK rc.exe is required to embed the application icon");
        (status, resource)
    } else {
        let resource = out.join("app-icon.o");
        let tool = env::var_os("WINDRES").unwrap_or_else(|| "x86_64-w64-mingw32-windres".into());
        let mut compiler = Command::new(tool);
        // Respect the cross compiler selected by Cargo/cc. Minimal MinGW
        // installations can provide gcc-posix without a generic gcc symlink.
        if let Some(preprocessor) = env::var_os("CC_x86_64_pc_windows_gnu") {
            compiler.arg("--preprocessor").arg(preprocessor);
            compiler.args([
                "--preprocessor-arg=-E",
                "--preprocessor-arg=-xc",
                "--preprocessor-arg=-DRC_INVOKED",
            ]);
        }
        let status = compiler
            .arg("--input")
            .arg(&rc)
            .arg("--output")
            .arg(&resource)
            .arg("--output-format=coff")
            .status()
            .expect("MinGW windres is required to embed the application icon");
        (status, resource)
    };
    assert!(
        status.success(),
        "application icon resource compiler failed"
    );
    println!("cargo:rustc-link-arg-bins={}", resource.display());
}
