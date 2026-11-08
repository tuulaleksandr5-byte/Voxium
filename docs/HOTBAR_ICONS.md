# Hotbar cube icons

`game_render::bake_block_icons(atlas_png, &block_textures)` returns sixteen
128×128 Bevy images in stable block ID order. AIR is entirely transparent.
Pass the original 768×384 PNG atlas: six columns, three rows, 128×128 tiles,
read left to right and top to bottom. The original file is never modified.
An absent, malformed, or incorrectly sized atlas is an error; the baker does
not invent or obtain replacement artwork.

The visible top uses the block's `top` layer from `assets/blocks.ron`.
Both vertical faces use its `side` layer; `bottom` is hidden. The left face
multiplies source RGB by 0.85, and the right face multiplies it by 0.70, so
they are respectively 15% and 30% darker than the unshaded top. These factors
apply directly to the encoded RGB channels. A thin dark outline marks the
outer silhouette and the three shared edges. Transparent source pixels stay
transparent, including on outlines. Glass and water retain their source alpha.

The CPU rasterizer maps the original tiles onto three isometric parallelograms.
Bilinear filtering clamps taps within each tile, filters premultiplied alpha,
and uses 4× supersampling for smooth silhouette and outline coverage. The
result is an sRGB RGBA image with a linear sampler and transparent background.

Bake once when loading the atlas and face mapping, add each image to
`Assets<Image>`, and cache the returned handles by block ID. UI image nodes
then clone the cached handle when the slot or selected block changes. Baking
does not belong in a per-frame or selection-change system. If the atlas or
mapping is deliberately reloaded, replace the cached images at that point.

Run `cargo test -p game_render icons::tests` to verify face selection, stable
IDs, exact shading, alpha, transparent margins, tile-safe filtering, invalid
inputs, and closed shared-face seams. The colored PNGs in those tests are
synthetic fixtures used only to make incorrect sampling detectable.
