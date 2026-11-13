# Pixel-art texture modes 16×16 and 32×32 (13 November 2026)

## Intent
Voxium currently accepts Minecraft Java ZIP 16/32 px PNGs, scales them to fixed 128 px atlas layers, then samples world materials linearly. Add actual sharp pixel-art presentation modes without breaking prior original assets or world files.

## Alternatives
1. **Selected:** preserve the 128×128 atlas/shader contract, quantize each tile's colors to 16×16 or 32×32 then upscale by nearest neighbour to 128×128; switch world GPU sampler to nearest/nearest, anisotropy=1. Share the chosen mode with UI icons. No mesh or shader rewrites.
2. Change all GPU texture arrays, mip generation and shader expectations to variable 16/32/128 sizes. More invasive; unnecessary to achieve desired on-screen pixels in present release.
3. Depend on external 16/32 ZIPs and the existing linear sampler. Already partially supported but does not give consistently sharp rendering.

## Features
- Settings option "Текстуры: 128×128 (исходные) → 16×16 → 32×32" (cyclic).
- Existing settings.cfg keys remain compatible, defaulting to original 128×128; invalid pixel-size values fall back safely.
- Selection is persisted when clicked and **takes effect on next game launch**, which the settings UI must tell the player. Do not misrepresent it as live.
- Block tiles from built-ins or active Minecraft ZIP packs are quantized per layer; neighbours never bleed into each other.
- GPU sampler uses repeat addressing, nearest magnification/minification/mipmap in 16/32 modes; original 128 continues to use the existing high-quality linear/anisotropic sampler.
- Tool/material/armor icons and cube icons display with nearest filtering in pixel modes. Keep transparency.
- UI sprites from GUI packs already use nearest and are unchanged. Save files/gameplay logic unchanged.

## Validation
Tests cover exactly 16×16 / 32×32 distinct pixels (4×4 and 8×8 in 128 output), original mode identity, each atlas layer's independence, nearest sampler selection, settings migration and persistence. GitHub Rust CI and Windows build must pass before claiming a usable EXE.
