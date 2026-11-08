# Uploaded original PNG tiles

All17 PNG files here are copied unchanged from the user's `textures_cut.zip`. See `source-hashes.json` for file hashes. No combined atlas PNG was generated or recompressed; assembly happens only in memory for GPU upload.

Layer order:0 grass_top,1 grass_side,2 dirt,3 stone,4 deepslate,5 sand,6 gravel,7 oak_log,8 oak_log_top,9 oak_leaves,10 oak_planks,11 cobblestone,12 glass,13 water,14 snow,15 sandstone,16 sandstone_top. Layer17 is unused/transparent.

The supplied `cobblestone.png` is the mossy reference pattern. BlockID10 maps to layer11; gravel keeps its own layer6. Grass maps0/1/2 and logs8/7/8.

The EXE embeds these exact PNG bytes. External files are optional; a complete override set can be placed in `assets/textures` beside it. See [asset instructions](../README.md).
