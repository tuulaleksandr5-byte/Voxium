# Original block artwork and sounds

The original `assets/atlas_128x128.png` has not been supplied as a filesystem attachment yet. Inline chat images are visible as references, but their original PNG bytes are not present in this checkout.

Required atlas: PNG, **768×384**, six columns and three rows of **128×128** tiles. Put the original file here unchanged. It is decoded for GPU layers and icon baking; the source PNG is never redrawn, resized, or re-encoded. A build with the file embeds those exact bytes. A packaged copy is also kept byte-for-byte unchanged.

Without this atlas, the client displays an asset error in the menu and prevents entering a world. There are no procedural block textures or substitute artwork. `VOXEL_ATLAS` can point to the original PNG elsewhere; the older `assets/textures/atlas_128x128.png` path remains supported.

`blocks.ron` assigns top/side/bottom layers to stable block IDs. `sounds/` contains 135 original, offline-generated WAV clips (15 blocks × 3 actions × 3 variants). See [audio notes](../docs/BLOCK_AUDIO.md) and [icon notes](../docs/HOTBAR_ICONS.md).
