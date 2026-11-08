# Original texture tiles and application icon

**Goal:** Ship the supplied cut PNG textures unchanged inside a self-contained EXE, and add an application icon based on the supplied logo.

**Input:** `textures_cut.zip`, 17 named RGBA PNGs, each128×128. No combined atlas or original logo file in that archive. The logo is provided inline and is being prepared as an icon asset using the image editing tool.

**Architecture:** Preserve all17 input files byte-for-byte. `BlockAtlas` decodes either the original cut files or a whole-atlas override, assembles only GPU-ready pixel data in RAM, and supplies both PBR materials and cached cube icons. Layer6 is gravel, layer11 is the provided mossy cobblestone. Cut PNGs are embedded as original bytes. Windows icon is embedded as an EXE resource and set on the runtime window/taskbar.

- [x] Inspect ZIP, validate names/dimensions and copy original bytes unchanged.
- [x] Add cut-tile loading, GPU/mipmap and cube-icon reuse; verify every base pixel matches its source.
- [x] Prepare icon PNG/ICO and register Explorer/window/taskbar icons.
- [x] Verify builds, native/Wine gameplay, source hashes, embedded-only startup and all tests.
- [x] Publish source and a verified Windows ZIP with original textures and icon included.
