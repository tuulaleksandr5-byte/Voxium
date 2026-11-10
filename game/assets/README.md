# Original artwork and sounds

The supplied `textures_cut.zip` contains17 original128×128 RGBA PNG tiles. They are preserved byte-for-byte in `textures/`; `textures/source-hashes.json` records their SHA-256 values and the uploaded archive's checksum.

The client decodes those original pixels and assembles18 GPU layers in memory, leaving the unused last layer transparent. It never resizes, redraws or re-encodes the input PNG files. The same decoded pixels produce the cached hotbar cube icons. All17 original PNG byte arrays are embedded in the EXE, so no external atlas is required to run.

Optional overrides: a complete `assets/atlas_128x128.png` (768×384) or the older `assets/textures/atlas_128x128.png` path; `VOXEL_ATLAS` may point elsewhere. A complete `assets/textures/` cut set beside the EXE can also replace the embedded set. Invalid explicit overrides report an error rather than silently substituting unrelated artwork.

`blocks.ron` assigns top/side/bottom layers to stable block IDs. `sounds/` contains135 original offline-generated WAV clips, also embedded. `icons/` contains the application icon prepared from the supplied inline logo reference. See [original texture integration](../docs/ORIGINAL_TEXTURES.md), [audio notes](../docs/BLOCK_AUDIO.md) and [icon notes](../docs/HOTBAR_ICONS.md).
