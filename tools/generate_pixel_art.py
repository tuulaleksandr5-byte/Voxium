#!/usr/bin/env python3
"""Offline generator for Voxium's original pixel-art set.

Everything here is drawn procedurally from scratch: 32x32 block tiles, item
icons, mob skins and the box models those skins are unwrapped onto. No pixels
are copied from any third-party resource pack. The output is deterministic,
so re-running the script must leave the repository unchanged.

    python3 tools/generate_pixel_art.py [--sheet preview.png]

Outputs:
    assets/textures/pixel/32x32/<tile>.png    authoring resolution
    assets/textures/pixel/128x128/<tile>.png  4x nearest copy used by the atlas
    assets/textures/pixel/icons/<item>.png    128x128 item icons
    assets/textures/mobs/<mob>.png            mob skins
    assets/data/mob_models.ron                box models with UV origins
"""
import math
import sys
import zlib
from pathlib import Path

import numpy as np
from PIL import Image

N = 32
ROOT = Path(__file__).resolve().parent.parent
TILES = {}
ICONS = {}


def rng(name):
    return np.random.default_rng(zlib.crc32(name.encode()))


def rgb(value):
    value = value.lstrip("#")
    return np.array([int(value[i : i + 2], 16) for i in (0, 2, 4)], dtype=float)


def canvas(color="#000000", alpha=255):
    image = np.zeros((N, N, 4), dtype=float)
    image[..., :3] = rgb(color)
    image[..., 3] = alpha
    return image


def fbm(r, octaves=((4, 1.0), (8, 0.6), (16, 0.35), (32, 0.2)), size=N):
    """Tileable value noise in 0..1."""
    total = np.zeros((size, size))
    weight = 0.0
    ys, xs = np.mgrid[0:size, 0:size]
    for cells, amplitude in octaves:
        grid = r.random((cells, cells))
        fx = xs * cells / size
        fy = ys * cells / size
        x0 = np.floor(fx).astype(int) % cells
        y0 = np.floor(fy).astype(int) % cells
        x1 = (x0 + 1) % cells
        y1 = (y0 + 1) % cells
        tx = fx - np.floor(fx)
        ty = fy - np.floor(fy)
        tx = tx * tx * (3 - 2 * tx)
        ty = ty * ty * (3 - 2 * ty)
        top = grid[y0, x0] * (1 - tx) + grid[y0, x1] * tx
        bottom = grid[y1, x0] * (1 - tx) + grid[y1, x1] * tx
        total += (top * (1 - ty) + bottom * ty) * amplitude
        weight += amplitude
    total /= weight
    low, high = total.min(), total.max()
    return (total - low) / max(high - low, 1e-9)


def ramp(field, palette):
    colors = np.array([rgb(c) for c in palette])
    index = np.clip((field * len(palette)).astype(int), 0, len(palette) - 1)
    return colors[index]


def fill(image, field, palette):
    image[..., :3] = ramp(field, palette)
    return image


def speckle(image, r, colors, count):
    for _ in range(count):
        x, y = r.integers(0, N, 2)
        image[y, x, :3] = rgb(colors[r.integers(0, len(colors))])


def put(image, x, y, color, alpha=255):
    image[y % N, x % N, :3] = rgb(color)
    image[y % N, x % N, 3] = alpha


def rect(image, x0, y0, w, h, color, alpha=255):
    for y in range(y0, y0 + h):
        for x in range(x0, x0 + w):
            put(image, x, y, color, alpha)


def shade(image, amount):
    image[..., :3] = np.clip(image[..., :3] * amount, 0, 255)
    return image


def tile(name):
    def register(function):
        TILES[name] = function
        return function

    return register


# ---------------------------------------------------------------- terrain


def dirt_base(r):
    image = fill(canvas(), fbm(r), ["#5b3d26", "#6b4a2e", "#7a5636", "#8a643f"])
    speckle(image, r, ["#4a3220", "#9a7650", "#6f6a62"], 46)
    return image


@tile("dirt")
def dirt():
    return dirt_base(rng("dirt"))


def grass_field(r):
    field = fbm(r, ((8, 1.0), (16, 0.8), (32, 0.6)))
    image = fill(canvas(), field, ["#3f7a2a", "#4c8c30", "#5a9c38", "#6aad42", "#7dbd4e"])
    for _ in range(70):
        x, y = r.integers(0, N, 2)
        put(image, x, y, "#8fd05a")
        put(image, x, y + 1, "#356b24")
    return image


@tile("grass_top")
def grass_top():
    return grass_field(rng("grass_top"))


@tile("grass_side")
def grass_side():
    r = rng("grass_side")
    image = dirt_base(r)
    grass = grass_field(r)
    depth = 5 + (fbm(r, ((8, 1.0), (16, 0.7))) [0] * 5).astype(int)
    for x in range(N):
        d = int(depth[x])
        image[:d, x] = grass[:d, x]
        put(image, x, d, "#356b24")
        if r.random() < 0.3:
            put(image, x, d + 1, "#3f7a2a")
    return image


def stone_base(r, palette, cracks=5, crack_color="#5d5d5d"):
    image = fill(canvas(), fbm(r), palette)
    for _ in range(cracks):
        x, y = r.integers(0, N, 2)
        for _ in range(int(r.integers(4, 9))):
            put(image, x, y, crack_color)
            x += int(r.integers(-1, 2))
            y += int(r.integers(0, 2))
    return image


STONE = ["#6f6f6f", "#7b7b7b", "#868686", "#929292"]
DEEP = ["#2f2f38", "#3a3a45", "#454552", "#50505e"]


@tile("stone")
def stone():
    r = rng("stone")
    image = stone_base(r, STONE)
    speckle(image, r, ["#9d9d9d", "#666666"], 40)
    return image


@tile("deepslate")
def deepslate():
    r = rng("deepslate")
    image = fill(canvas(), fbm(r, ((2, 0.4), (8, 1.0), (16, 0.5))), DEEP)
    for y in range(0, N, 4):
        offset = int(r.integers(0, N))
        for x in range(int(r.integers(10, 22))):
            put(image, offset + x, y + int(r.integers(0, 2)), "#25252c")
    speckle(image, r, ["#5c5c6c"], 26)
    return image


@tile("sand")
def sand():
    r = rng("sand")
    image = fill(canvas(), fbm(r, ((4, 1.0), (16, 0.5))), ["#d9c88f", "#e0d09a", "#e7d8a6", "#eee0b2"])
    speckle(image, r, ["#c9b67c", "#f5ebc6", "#bfa96e"], 70)
    return image


def cells(r, points, mortar, palette, highlight, edge=1.3):
    """Wrapped Voronoi cobbles with a lit top-left rim."""
    pts = r.random((points, 2)) * N
    tone = r.random(points)
    ys, xs = np.mgrid[0:N, 0:N]
    best = np.full((N, N), 1e9)
    second = np.full((N, N), 1e9)
    owner = np.zeros((N, N), dtype=int)
    for index, (px, py) in enumerate(pts):
        dx = np.abs(xs - px)
        dy = np.abs(ys - py)
        dx = np.minimum(dx, N - dx)
        dy = np.minimum(dy, N - dy)
        distance = np.hypot(dx, dy * 1.15)
        closer = distance < best
        second = np.where(closer, best, np.minimum(second, distance))
        owner = np.where(closer, index, owner)
        best = np.where(closer, distance, best)
    image = canvas()
    grain = fbm(r, ((16, 1.0), (32, 0.8)))
    image[..., :3] = ramp(np.clip(tone[owner] * 0.75 + grain * 0.25, 0, 0.999), palette)
    seam = second - best < edge
    image[seam, :3] = rgb(mortar)
    for y in range(N):
        for x in range(N):
            if not seam[y, x] and (seam[(y - 1) % N, x] or seam[y, (x - 1) % N]):
                image[y, x, :3] = np.clip(image[y, x, :3] + highlight, 0, 255)
    return image, seam


@tile("gravel")
def gravel():
    r = rng("gravel")
    image, _ = cells(r, 70, "#4d4944", ["#6b665f", "#7d776f", "#8f8980", "#a39c92", "#7a6f66"], 14, 0.9)
    return image


@tile("cobblestone")
def cobblestone():
    image, _ = cells(rng("cobblestone"), 17, "#4a4a4a", ["#6c6c6c", "#7a7a7a", "#888888", "#969696"], 18)
    return image


@tile("mossy_cobblestone")
def mossy_cobblestone():
    r = rng("cobblestone")
    image, seam = cells(r, 17, "#4a4a4a", ["#6c6c6c", "#7a7a7a", "#888888", "#969696"], 18)
    moss = fbm(rng("moss"), ((4, 1.0), (8, 0.8), (16, 0.5)))
    green = ramp(fbm(rng("moss2"), ((16, 1.0), (32, 1.0))), ["#3f6b2a", "#4f7f33", "#5f9440"])
    mask = (moss > 0.55) | (seam & (moss > 0.38))
    image[mask, :3] = green[mask]
    return image


@tile("snow")
def snow():
    r = rng("snow")
    image = fill(canvas(), fbm(r, ((4, 1.0), (8, 0.5))), ["#e4ecf2", "#edf3f7", "#f5f9fb", "#ffffff"])
    speckle(image, r, ["#d5e0ea", "#ffffff"], 30)
    return image


def sandstone_base(r):
    return fill(canvas(), fbm(r, ((4, 1.0), (16, 0.6))), ["#cdb983", "#d6c38e", "#dfcd9a", "#e7d6a6"])


@tile("sandstone_top")
def sandstone_top():
    r = rng("sandstone_top")
    image = sandstone_base(r)
    speckle(image, r, ["#c2ad76", "#efe2b8"], 40)
    return image


@tile("sandstone")
def sandstone():
    r = rng("sandstone")
    image = sandstone_base(r)
    for y in (0, 1):
        for x in range(N):
            put(image, x, y, "#efe2b8" if y == 0 else "#bfa96e")
    for y in (9, 10, 19, 20, 27):
        wobble = fbm(r, ((8, 1.0),))[0]
        for x in range(N):
            put(image, x, y + int(wobble[x] * 2), "#bfa96e" if y % 2 else "#c9b67c")
    for x in range(N):
        put(image, x, N - 1, "#b39d62")
    return image


@tile("clay")
def clay():
    r = rng("clay")
    image = fill(canvas(), fbm(r, ((4, 1.0), (8, 0.6))), ["#8f96a6", "#9aa1b1", "#a5acbc", "#b0b7c6"])
    speckle(image, r, ["#858c9c", "#bcc3d1"], 34)
    return image


@tile("terracotta")
def terracotta():
    r = rng("terracotta")
    image = fill(canvas(), fbm(r, ((4, 1.0), (8, 0.4))), ["#964f32", "#a25838", "#ad613f", "#b86a46"])
    speckle(image, r, ["#8a4830", "#c2744e"], 22)
    return image


@tile("obsidian")
def obsidian():
    r = rng("obsidian")
    image = fill(canvas(), fbm(r, ((4, 1.0), (8, 0.9), (16, 0.5))), ["#0e0b16", "#161022", "#1f1730", "#2a1f40"])
    for _ in range(9):
        x, y = r.integers(0, N, 2)
        for step in range(int(r.integers(2, 5))):
            put(image, x + step, y + step, "#5b4a86")
    speckle(image, r, ["#7a68ad"], 8)
    return image


@tile("ice")
def ice():
    r = rng("ice")
    image = fill(canvas(alpha=205), fbm(r, ((4, 1.0), (8, 0.5))), ["#8fbcf0", "#9cc6f4", "#a9d0f8", "#b8dafb"])
    for _ in range(7):
        x, y = r.integers(0, N, 2)
        direction = 1 if r.random() < 0.5 else -1
        for step in range(int(r.integers(4, 10))):
            put(image, x + step * direction, y + step // 2, "#e4f3ff", 235)
    for i in range(N):
        put(image, i, 0, "#d2e8fc", 230)
        put(image, 0, i, "#d2e8fc", 230)
    return image


@tile("water")
def water():
    r = rng("water")
    field = fbm(r, ((4, 1.0), (8, 0.7)))
    image = fill(canvas(alpha=170), field, ["#2a5fc4", "#3069cf", "#3874d9", "#4180e2"])
    for y in range(2, N, 6):
        wobble = fbm(r, ((4, 1.0), (8, 0.5)))[0]
        start = int(r.integers(0, N))
        for x in range(int(r.integers(8, 15))):
            put(image, start + x, y + int(wobble[(start + x) % N] * 3), "#7fb2f5", 200)
    return image


@tile("glass")
def glass():
    image = canvas("#d9f1f7", 34)
    for i in range(N):
        for edge in (0, N - 1):
            put(image, i, edge, "#e8f8fb", 255)
            put(image, edge, i, "#e8f8fb", 255)
        for edge in (1, N - 2):
            if 0 < i < N - 1:
                put(image, i, edge, "#a9d3dd", 255)
                put(image, edge, i, "#a9d3dd", 255)
    for start, length in ((5, 7), (9, 4), (20, 6)):
        for step in range(length):
            put(image, start + step, start + length - step + 2, "#ffffff", 210)
    return image


# ------------------------------------------------------------------- wood


def bark(r, palette, crack):
    columns = fbm(r, ((16, 1.0), (32, 0.6)))[0]
    rows = fbm(r, ((2, 1.0), (8, 0.3)))
    field = np.clip(columns[None, :] * 0.75 + rows * 0.25, 0, 0.999)
    image = fill(canvas(), field, palette)
    for x in range(N):
        if columns[x] < 0.22:
            for y in range(N):
                if r.random() < 0.8:
                    put(image, x, y, crack)
    return image


OAK_BARK = ["#4a3520", "#574027", "#644a2e", "#725535"]
OAK_WOOD = ["#a6814b", "#b08b53", "#ba955b", "#c49f64"]
BIRCH_WOOD = ["#cdbd8a", "#d5c694", "#ddcf9f", "#e5d8aa"]


@tile("oak_log")
def oak_log():
    return bark(rng("oak_log"), OAK_BARK, "#33240f")


def log_top(name, bark_palette, wood_palette, ring):
    r = rng(name)
    image = fill(canvas(), fbm(r, ((16, 1.0),)), wood_palette)
    for y in range(N):
        for x in range(N):
            radius = max(abs(x - 15.5), abs(y - 15.5))
            if radius > 13:
                image[y, x, :3] = rgb(bark_palette[int(r.integers(0, len(bark_palette)))])
            elif int(radius + 0.5 * math.sin(x * 0.9) ) % 4 == 0:
                image[y, x, :3] = rgb(ring)
    for x, y in ((15, 15), (16, 15), (15, 16), (16, 16)):
        put(image, x, y, ring)
    return image


@tile("oak_log_top")
def oak_log_top():
    return log_top("oak_log_top", OAK_BARK, OAK_WOOD, "#8a6836")


@tile("birch_log")
def birch_log():
    r = rng("birch_log")
    image = fill(canvas(), fbm(r, ((4, 1.0), (16, 0.4))), ["#d8d8d0", "#e2e2da", "#ebebe4", "#f4f4ee"])
    for _ in range(15):
        x, y = r.integers(0, N, 2)
        width = int(r.integers(3, 9))
        for step in range(width):
            put(image, x + step, y, "#2e2a26")
            if 0 < step < width - 1 and r.random() < 0.5:
                put(image, x + step, y + 1, "#4a443d")
    speckle(image, r, ["#c9c9bf"], 26)
    return image


@tile("birch_log_top")
def birch_log_top():
    return log_top("birch_log_top", ["#d8d8d0", "#e2e2da", "#c9c9bf"], BIRCH_WOOD, "#b3a16d")


def planks(name, palette, seam, nail):
    r = rng(name)
    grain = fbm(r, ((2, 0.5), (16, 1.0), (32, 0.4)))
    stretched = np.repeat(grain[:, ::4], 4, axis=1)
    image = fill(canvas(), np.clip(stretched * 0.7 + grain * 0.3, 0, 0.999), palette)
    for row in range(4):
        y = row * 8
        for x in range(N):
            put(image, x, y + 7, seam)
            image[y, x, :3] = np.clip(image[y, x, :3] + 12, 0, 255)
        joint = (row * 11 + 5) % N
        for dy in range(7):
            put(image, joint, y + dy, seam)
        put(image, joint + 2, y + 2, nail)
        put(image, joint - 2, y + 4, nail)
    return image


@tile("oak_planks")
def oak_planks():
    return planks("oak_planks", OAK_WOOD, "#7a5c30", "#8f6f3d")


@tile("birch_planks")
def birch_planks():
    return planks("birch_planks", BIRCH_WOOD, "#a8966a", "#b9a878")


def leaves(name, palette, outline, holes):
    r = rng(name)
    cluster = fbm(r, ((8, 1.0), (16, 1.0), (32, 0.7)))
    tone = fbm(r, ((16, 1.0), (32, 1.0)))
    image = fill(canvas(), tone, palette)
    solid = cluster > holes
    for y in range(N):
        for x in range(N):
            if not solid[y, x]:
                image[y, x, 3] = 0
            elif not solid[(y + 1) % N, x] or not solid[y, (x + 1) % N]:
                image[y, x, :3] = rgb(outline)
    # Transparent texels keep a leaf colour so mip filtering never bleeds black.
    image[~solid, :3] = rgb(palette[1])
    return image


@tile("oak_leaves")
def oak_leaves():
    return leaves("oak_leaves", ["#2f6b1f", "#3a7c26", "#46902e", "#55a538", "#67b844"], "#234f17", 0.3)


@tile("birch_leaves")
def birch_leaves():
    return leaves("birch_leaves", ["#5d8f2c", "#6fa334", "#82b53e", "#96c64c", "#abd35e"], "#4a7323", 0.33)


# ------------------------------------------------------------------- ores


def ore(name, colors):
    def build():
        r = rng("stone")
        image = stone_base(r, STONE)
        speckle(image, r, ["#9d9d9d", "#666666"], 40)
        o = rng(name)
        light, mid, dark = colors
        for cx, cy in ((7, 7), (22, 6), (14, 16), (5, 23), (24, 24), (16, 27)):
            cx += int(o.integers(-2, 3))
            cy += int(o.integers(-2, 3))
            radius = float(o.uniform(2.0, 3.4))
            for y in range(cy - 4, cy + 5):
                for x in range(cx - 4, cx + 5):
                    d = math.hypot(x - cx, (y - cy) * 1.2) + o.uniform(-0.6, 0.6)
                    if d < radius:
                        color = light if (x - cx) + (y - cy) < -1 else mid
                        put(image, x, y, color)
                    elif d < radius + 1.0 and (x - cx) + (y - cy) > 0:
                        put(image, x, y, dark)
        return image

    TILES[name] = build


ore("coal_ore", ("#3a3a3a", "#1e1e1e", "#4f4f4f"))
ore("iron_ore", ("#e8c3a0", "#c9966c", "#59544f"))
ore("gold_ore", ("#fff08a", "#f2c12e", "#5a5548"))
ore("diamond_ore", ("#c8fff6", "#4fd8cc", "#4d5a5a"))


def metal(name, palette):
    def build():
        r = rng(name)
        dark, low, mid, high, shine = palette
        image = fill(canvas(), fbm(r, ((2, 1.0), (4, 0.4))), [low, mid, mid, high])
        for i in range(N):
            for edge, color in ((0, shine), (1, high)):
                put(image, i, edge, color)
                put(image, edge, i, color)
            for edge, color in ((N - 1, dark), (N - 2, low)):
                put(image, i, edge, color)
                put(image, edge, i, color)
        for inset in (6,):
            for i in range(inset, N - inset):
                put(image, i, inset, low)
                put(image, inset, i, low)
                put(image, i, N - 1 - inset, high)
                put(image, N - 1 - inset, i, high)
        for step in range(5):
            put(image, 10 + step, 14 - step, shine)
            put(image, 17 + step, 21 - step, shine)
        return image

    TILES[name] = build


metal("iron_block", ("#8f8f8f", "#bdbdbd", "#d4d4d4", "#e8e8e8", "#ffffff"))
metal("gold_block", ("#a8760f", "#d9a61c", "#f2c832", "#fbe35e", "#fff7b0"))
metal("diamond_block", ("#1e8f88", "#36bdb3", "#58d9cf", "#8cf0e6", "#dcfffa"))


@tile("coal_block")
def coal_block():
    r = rng("coal_block")
    image, _ = cells(r, 12, "#0c0c0c", ["#161616", "#1f1f1f", "#292929", "#333333"], 22, 1.0)
    speckle(image, r, ["#4a4a4a"], 12)
    return image


# ------------------------------------------------------------- masonry


@tile("bricks")
def bricks():
    r = rng("bricks")
    image = canvas("#b9b1a4")
    tone = fbm(r, ((16, 1.0), (32, 0.8)))
    for row in range(4):
        y0 = row * 8
        offset = 0 if row % 2 == 0 else 8
        for brick in range(2):
            x0 = offset + brick * 16
            base = r.uniform(0.0, 0.45)
            for y in range(y0, y0 + 7):
                for x in range(x0, x0 + 15):
                    value = np.clip(base + tone[y % N, x % N] * 0.5, 0, 0.999)
                    color = ramp(np.array(value), ["#8e3f2c", "#9c4832", "#aa5138", "#b85c40"])
                    image[y % N, x % N, :3] = color
            for x in range(x0, x0 + 15):
                image[y0 % N, x % N, :3] = np.clip(image[y0 % N, x % N, :3] + 22, 0, 255)
                image[(y0 + 6) % N, x % N, :3] = image[(y0 + 6) % N, x % N, :3] * 0.82
    speckle(image, r, ["#a69e91"], 10)
    return image


def brick_pattern(r, palette, mortar):
    image = canvas(mortar)
    tone = fbm(r, ((8, 1.0), (32, 0.6)))
    for row in range(2):
        y0 = row * 16
        offset = 0 if row == 0 else 16
        for block in range(2):
            x0 = offset + block * 16 if row == 0 else (8 + block * 16)
            base = r.uniform(0.0, 0.35)
            for y in range(y0, y0 + 15):
                for x in range(x0, x0 + 15):
                    value = np.clip(base + tone[y % N, x % N] * 0.6, 0, 0.999)
                    image[y % N, x % N, :3] = ramp(np.array(value), palette)
            for i in range(15):
                image[y0 % N, (x0 + i) % N, :3] = np.clip(image[y0 % N, (x0 + i) % N, :3] + 24, 0, 255)
                image[(y0 + i) % N, x0 % N, :3] = np.clip(image[(y0 + i) % N, x0 % N, :3] + 24, 0, 255)
                image[(y0 + 14) % N, (x0 + i) % N, :3] *= 0.8
                image[(y0 + i) % N, (x0 + 14) % N, :3] *= 0.8
    return image


@tile("stone_bricks")
def stone_bricks():
    r = rng("stone_bricks")
    image = brick_pattern(r, STONE, "#555555")
    speckle(image, r, ["#666666"], 14)
    return image


@tile("smooth_stone")
def smooth_stone():
    r = rng("smooth_stone")
    image = fill(canvas(), fbm(r, ((4, 1.0), (16, 0.3))), ["#9a9a9a", "#a3a3a3", "#acacac", "#b5b5b5"])
    for i in range(N):
        put(image, i, 0, "#c6c6c6")
        put(image, 0, i, "#c6c6c6")
        put(image, i, N - 1, "#7d7d7d")
        put(image, N - 1, i, "#7d7d7d")
    return image


def furnace_shell(name):
    r = rng(name)
    image = brick_pattern(r, ["#6a6a6a", "#767676", "#828282", "#8e8e8e"], "#454545")
    return image


@tile("furnace_side")
def furnace_side():
    return furnace_shell("furnace_side")


@tile("furnace_top")
def furnace_top():
    image = furnace_shell("furnace_top")
    rect(image, 11, 11, 10, 10, "#3a3a3a")
    rect(image, 13, 13, 6, 6, "#151515")
    return image


def furnace_front(lit):
    image = furnace_shell("furnace_side")
    rect(image, 6, 4, 20, 6, "#3b3b3b")
    for x in range(8, 24, 3):
        rect(image, x, 5, 2, 4, "#141414")
    rect(image, 5, 13, 22, 15, "#3b3b3b")
    rect(image, 7, 15, 18, 12, "#120d0a")
    if lit:
        r = rng("fire")
        for x in range(7, 25):
            height = int(4 + 5 * abs(math.sin(x * 0.9)) + r.integers(0, 3))
            for step in range(height):
                y = 26 - step
                color = "#fff3a0" if step < height * 0.3 else "#ffb02e" if step < height * 0.65 else "#e2501a"
                put(image, x, y, color)
        for x in range(8, 24, 3):
            rect(image, x, 5, 2, 4, "#e2501a")
            put(image, x, 7, "#ffb02e")
    else:
        for x in range(8, 24, 2):
            put(image, x, 26, "#2c2420")
            put(image, x + 1, 25, "#3a302a")
    return image


TILES["furnace_front"] = lambda: furnace_front(False)
TILES["furnace_front_on"] = lambda: furnace_front(True)


# ------------------------------------------------------------ carpentry


def frame(image, dark, light):
    for i in range(N):
        for edge in (0, 1):
            put(image, i, edge, light if edge == 0 else dark)
            put(image, edge, i, light if edge == 0 else dark)
        put(image, i, N - 1, dark)
        put(image, N - 1, i, dark)
        put(image, i, N - 2, dark)
        put(image, N - 2, i, dark)


@tile("crafting_table_top")
def crafting_table_top():
    image = planks("table_top", ["#b98a4e", "#c39456", "#cd9e5f", "#d7a868"], "#8a6230", "#9a7038")
    frame(image, "#5e4020", "#7a5630")
    for line in (11, 20):
        for i in range(3, N - 3):
            put(image, i, line, "#6e4c26")
            put(image, line, i, "#6e4c26")
    return image


@tile("crafting_table_bottom")
def crafting_table_bottom():
    return planks("oak_planks", OAK_WOOD, "#7a5c30", "#8f6f3d")


def table_side(tools):
    image = planks("table_side", OAK_WOOD, "#7a5c30", "#8f6f3d")
    rect(image, 0, 0, N, 5, "#c39456")
    rect(image, 0, 5, N, 1, "#5e4020")
    rect(image, 0, 6, 3, N - 6, "#6e4c26")
    rect(image, N - 3, 6, 3, N - 6, "#6e4c26")
    if tools == "front":
        rect(image, 7, 10, 2, 14, "#8a6230")  # hammer handle
        rect(image, 4, 9, 8, 4, "#9a9a9a")
        rect(image, 4, 9, 8, 1, "#c8c8c8")
        for step in range(12):  # saw blade
            put(image, 17 + step // 2, 11 + step, "#c8c8c8")
            put(image, 18 + step // 2, 11 + step, "#9a9a9a")
            if step % 2 == 0:
                put(image, 19 + step // 2, 11 + step, "#7d7d7d")
        rect(image, 15, 8, 5, 4, "#5e4020")
    else:
        rect(image, 6, 10, 20, 8, "#8a6230")
        rect(image, 7, 11, 18, 6, "#a6814b")
        rect(image, 14, 13, 4, 2, "#5e4020")
        rect(image, 6, 20, 20, 8, "#8a6230")
        rect(image, 7, 21, 18, 6, "#a6814b")
        rect(image, 14, 23, 4, 2, "#5e4020")
    return image


TILES["crafting_table_front"] = lambda: table_side("front")
TILES["crafting_table_side"] = lambda: table_side("side")


def chest(face):
    image = planks("chest", ["#9a6a2c", "#a67434", "#b27e3c", "#be8844"], "#6a4618", "#7a5220")
    frame(image, "#4a300e", "#6a4618")
    if face != "top":
        rect(image, 2, 10, N - 4, 2, "#4a300e")
    if face == "front":
        rect(image, 13, 8, 6, 8, "#b9b9b9")
        rect(image, 13, 8, 6, 1, "#e8e8e8")
        rect(image, 15, 11, 2, 3, "#3a3a3a")
    return image


TILES["chest_front"] = lambda: chest("front")
TILES["chest_side"] = lambda: chest("side")
TILES["chest_top"] = lambda: chest("top")


@tile("bookshelf")
def bookshelf():
    r = rng("bookshelf")
    image = planks("oak_planks", OAK_WOOD, "#7a5c30", "#8f6f3d")
    covers = ["#a83232", "#2f6b9c", "#3d8a3d", "#b98a2c", "#6b3d8a", "#8a5a2c", "#2c8a80"]
    for shelf_y in (3, 18):
        rect(image, 1, shelf_y, N - 2, 12, "#2e2012")
        x = 2
        while x < N - 3:
            width = int(r.integers(2, 5))
            height = int(r.integers(8, 12))
            if x + width > N - 2:
                width = N - 2 - x
            color = covers[int(r.integers(0, len(covers)))]
            rect(image, x, shelf_y + 12 - height, width, height, color)
            for y in range(shelf_y + 12 - height, shelf_y + 12):
                image[y, x, :3] = np.clip(image[y, x, :3] + 35, 0, 255)
            put(image, x + width // 2, shelf_y + 14 - height, "#e8d8a0")
            x += width + (1 if r.random() < 0.2 else 0)
    return image


@tile("lamp")
def lamp():
    r = rng("lamp")
    glow = fbm(r, ((4, 1.0), (8, 0.6)))
    image = fill(canvas(), glow, ["#f2a62e", "#f8bf45", "#fcd563", "#ffe98a", "#fff7c4"])
    frame(image, "#5a3a14", "#8a5a20")
    for line in (10, 21):
        for i in range(2, N - 2):
            put(image, i, line, "#8a5a20")
            put(image, line, i, "#8a5a20")
    for cx, cy in ((6, 6), (16, 6), (26, 6), (6, 16), (16, 16), (26, 16), (6, 26), (16, 26), (26, 26)):
        put(image, cx - 1, cy - 1, "#ffffff")
    return image


@tile("wool")
def wool():
    r = rng("wool")
    image = fill(canvas(), fbm(r, ((8, 1.0), (16, 0.8))), ["#dcdcd6", "#e6e6e0", "#efefea", "#f8f8f4"])
    for y in range(N):
        for x in range(N):
            if (x + y * 3) % 8 == 0 or (x * 3 - y) % 11 == 0:
                image[y, x, :3] = image[y, x, :3] * 0.9
    return image


# ------------------------------------------------------------------ icons


def icon(name):
    def register(function):
        ICONS[name] = function
        return function

    return register


def sprite(mask, palette, r, grain=0.5):
    """Shade a boolean mask from the top-left and add a dark outline."""
    dark, low, mid, high = [rgb(c) for c in palette]
    image = np.zeros((N, N, 4))
    ys, xs = np.nonzero(mask)
    if len(xs) == 0:
        return image
    span = max(xs.max() - xs.min() + ys.max() - ys.min(), 1)
    noise = fbm(r, ((16, 1.0), (32, 1.0)))
    for y, x in zip(ys, xs):
        light = 1 - ((x - xs.min()) + (y - ys.min())) / span
        light = light * (1 - grain * 0.5) + noise[y, x] * grain * 0.5
        color = high if light > 0.66 else mid if light > 0.36 else low
        image[y, x, :3] = color
        image[y, x, 3] = 255
    padded = np.pad(mask, 1)
    for y in range(N):
        for x in range(N):
            if mask[y, x]:
                neighbours = (padded[y, x + 1], padded[y + 2, x + 1], padded[y + 1, x], padded[y + 1, x + 2])
                if not all(neighbours):
                    image[y, x, :3] = dark
    return image


def ellipse(cx, cy, rx, ry, angle=0.0):
    ys, xs = np.mgrid[0:N, 0:N]
    c, s = math.cos(angle), math.sin(angle)
    dx, dy = xs - cx, ys - cy
    u = dx * c + dy * s
    v = -dx * s + dy * c
    return (u / rx) ** 2 + (v / ry) ** 2 <= 1.0


def line_mask(x0, y0, x1, y1, width):
    ys, xs = np.mgrid[0:N, 0:N]
    px, py = x1 - x0, y1 - y0
    t = np.clip(((xs - x0) * px + (ys - y0) * py) / (px * px + py * py), 0, 1)
    return np.hypot(xs - (x0 + t * px), ys - (y0 + t * py)) <= width


def overlay(base, top):
    mask = top[..., 3] > 0
    base[mask] = top[mask]
    return base


@icon("stick")
def stick_icon():
    return sprite(line_mask(7, 25, 24, 6, 1.6), ["#3d2a12", "#6e4c22", "#8f6a34", "#b08a4a"], rng("stick"))


@icon("egg")
def egg_icon():
    return sprite(ellipse(15.5, 17, 7.5, 9.5) | ellipse(15.5, 13, 6, 7), ["#8a7a5c", "#d9cba8", "#efe4c6", "#fffaf0"], rng("egg"), 0.2)


@icon("clay_ball")
def clay_ball_icon():
    return sprite(ellipse(15.5, 16, 9, 8.5), ["#5c6272", "#8f96a6", "#a5acbc", "#c4cbd8"], rng("clay_ball"))


@icon("brick")
def brick_icon():
    mask = np.zeros((N, N), dtype=bool)
    for y in range(10, 23):
        shift = (y - 10) // 4
        mask[y, 5 + shift : 25 + shift] = True
    return sprite(mask, ["#5e2416", "#8e3f2c", "#aa5138", "#c96a4c"], rng("brick"))


@icon("string")
def string_icon():
    mask = np.zeros((N, N), dtype=bool)
    for t in np.linspace(0, 1, 160):
        x = 6 + t * 20
        y = 16 + math.sin(t * math.tau * 1.5) * 8 * (1 - t * 0.3)
        mask |= ellipse(x, y, 1.3, 1.3)
    return sprite(mask, ["#8a8a84", "#c9c9c2", "#e6e6e0", "#ffffff"], rng("string"), 0.2)


@icon("feather")
def feather_icon():
    mask = ellipse(17, 14, 4.5, 11, -0.6) | line_mask(6, 27, 14, 19, 0.8)
    image = sprite(mask, ["#8a8a84", "#c9c9c2", "#e6e6e0", "#ffffff"], rng("feather"), 0.3)
    for step in range(16):
        x, y = 9 + step, 24 - step
        if image[y, x, 3] > 0:
            image[y, x, :3] = rgb("#a8a8a0")
    return image


def steak(name, palette, fat, marks):
    r = rng(name)
    mask = ellipse(15, 16, 10.5, 7.5, 0.35) | ellipse(21, 12, 5, 4.5)
    image = sprite(mask, palette, r)
    rim = ellipse(15, 16, 10.5, 7.5, 0.35) & ~ellipse(15.6, 16.6, 9.2, 6.2, 0.35)
    for y in range(N):
        for x in range(N):
            if rim[y, x] and x + y < 30 and image[y, x, 3] > 0 and not np.array_equal(image[y, x, :3], rgb(palette[0])):
                image[y, x, :3] = rgb(fat)
    if marks:
        for offset in (-5, 0, 5):
            for step in range(-5, 6):
                x, y = 15 + step + offset, 16 - step + offset // 2
                if ellipse(15, 16, 8.5, 5.5, 0.35)[y, x]:
                    image[y, x, :3] = rgb(marks)
    else:
        for _ in range(12):
            x, y = r.integers(8, 24, 2)
            if ellipse(15, 16, 8.5, 5.5, 0.35)[y, x]:
                image[y, x, :3] = rgb(fat)
    return image


ICONS["raw_beef"] = lambda: steak("raw_beef", ["#5e1418", "#a82830", "#c93c44", "#e25a60"], "#f2d8d0", None)
ICONS["cooked_beef"] = lambda: steak("cooked_beef", ["#3a1c0e", "#6e3a1c", "#8a4c26", "#a66234"], "#c9a070", "#3a1c0e")
ICONS["raw_pork"] = lambda: steak("raw_pork", ["#8a4048", "#d98890", "#eaa4aa", "#f7c4c8"], "#fff0ec", None)
ICONS["cooked_pork"] = lambda: steak("cooked_pork", ["#5a3a1c", "#b08a52", "#c9a268", "#e0bd84"], "#f0dcae", "#7a5228")
ICONS["raw_mutton"] = lambda: steak("raw_mutton", ["#6e1c24", "#b83a44", "#d0525c", "#e67078"], "#f4e0d8", None)
ICONS["cooked_mutton"] = lambda: steak("cooked_mutton", ["#402010", "#7a4826", "#946034", "#ae7844"], "#d0a878", "#402010")
ICONS["rotten_meat"] = lambda: steak("rotten_meat", ["#2e3a1c", "#5c6a34", "#74804a", "#8e9a62"], "#a84a3a", None)


def drumstick(name, palette):
    r = rng(name)
    meat = sprite(ellipse(13, 13, 8, 6.5, 0.7), palette, r)
    bone = sprite(line_mask(18, 19, 25, 26, 1.4) | ellipse(25, 24, 2, 2) | ellipse(23, 27, 2, 2), ["#8a8068", "#d8d0b8", "#ece6d2", "#ffffff"], r, 0.1)
    return overlay(bone, meat)


ICONS["raw_chicken"] = lambda: drumstick("raw_chicken", ["#9a5a52", "#e0a69c", "#efc0b6", "#fbdcd4"])
ICONS["cooked_chicken"] = lambda: drumstick("cooked_chicken", ["#5a3410", "#b0742c", "#cc8e3c", "#e6aa54"])


# ------------------------------------------------------------------- mobs


class Part:
    def __init__(self, name, pivot, start, size, palette, parent=None, paint=None, grain=0.5):
        self.name, self.pivot, self.start, self.size = name, pivot, start, size
        self.palette, self.parent, self.paint, self.grain = palette, parent, paint, grain
        self.uv = (0, 0)

    def faces(self):
        w, h, d = (math.ceil(n) for n in self.size)
        u, v = self.uv
        return {
            "top": (u + d, v, w, d),
            "bottom": (u + d + w, v, w, d),
            "west": (u, v + d, d, h),
            "front": (u + d, v + d, w, h),
            "east": (u + d + w, v + d, d, h),
            "back": (u + d + w + d, v + d, w, h),
        }

    def footprint(self):
        w, h, d = (math.ceil(n) for n in self.size)
        return 2 * (w + d), d + h


class Painter:
    def __init__(self, image, rect_):
        self.image, (self.x, self.y, self.w, self.h) = image, rect_

    def px(self, x, y, color):
        if 0 <= x < self.w and 0 <= y < self.h:
            self.image[self.y + y, self.x + x, :3] = rgb(color)
            self.image[self.y + y, self.x + x, 3] = 255

    def rect(self, x0, y0, w, h, color):
        for y in range(y0, y0 + h):
            for x in range(x0, x0 + w):
                self.px(x, y, color)


def pack(parts, width=64):
    x = y = shelf = 0
    for part in sorted(parts, key=lambda p: -p.footprint()[1]):
        w, h = part.footprint()
        if x + w > width:
            x, y, shelf = 0, y + shelf, 0
        part.uv = (x, y)
        x += w
        shelf = max(shelf, h)
    height = y + shelf
    return width, max(16, 1 << (height - 1).bit_length())


def paint_mob(name, parts):
    width, height = pack(parts)
    image = np.zeros((height, width, 4))
    r = rng("mob:" + name)
    for part in parts:
        colors = [rgb(c) for c in part.palette]
        for face, (x0, y0, w, h) in part.faces().items():
            for y in range(h):
                for x in range(w):
                    roll = r.random()
                    index = 1
                    if roll < 0.22 * part.grain * 2:
                        index = 0
                    elif roll > 1 - 0.22 * part.grain * 2:
                        index = 2
                    color = colors[index].copy()
                    if face == "bottom":
                        color *= 0.7
                    elif face in ("west", "east", "front", "back") and y >= h - 1 and h > 3:
                        color *= 0.86
                    image[y0 + y, x0 + x, :3] = color
                    image[y0 + y, x0 + x, 3] = 255
            if part.paint:
                part.paint(face, Painter(image, (x0, y0, w, h)), r)
    return image.astype(np.uint8), (width, height)


def eyes(p, y, left, right, white="#ffffff", pupil="#101010", size=2):
    for x in (left, right):
        p.rect(x, y, size, size, white)
    p.px(left + size - 1, y + size - 1, pupil)
    p.px(right, y + size - 1, pupil)


def hooves(color, rows=2):
    def paint(face, p, r):
        if face in ("west", "east", "front", "back"):
            p.rect(0, p.h - rows, p.w, rows, color)
        if face == "bottom":
            p.rect(0, 0, p.w, p.h, color)

    return paint


def patches(color, threshold, seed):
    field = fbm(rng(seed), ((4, 1.0), (8, 0.7)), size=64)

    def paint(face, p, r):
        if face == "bottom":
            return
        for y in range(p.h):
            for x in range(p.w):
                if field[(p.y + y) % 64, (p.x + x) % 64] > threshold:
                    p.px(x, y, color)

    return paint


def cow():
    hide = ["#6a3e24", "#7c4a2c", "#8c5634"]
    cream = "#ece2c8"
    body_paint = patches(cream, 0.6, "cow-hide")

    def head(face, p, r):
        if face == "front":
            p.rect(3, 0, 2, 5, cream)
            eyes(p, 2, 1, 5)
            p.rect(1, 5, 6, 3, "#d9a8a0")
            p.px(2, 6, "#5a3030")
            p.px(5, 6, "#5a3030")
        elif face == "top":
            p.rect(3, 0, 2, p.h, cream)

    def tail(face, p, r):
        if face != "top":
            p.rect(0, p.h - 2, p.w, 2, "#2e2018")

    leg = lambda n, x, z: Part(n, (x, 11, z), (-2, -11, -2), (4, 11, 4), hide, paint=hooves("#2e2018"))
    return [
        Part("body", (0, 16, 0), (-6, -5, -9), (12, 10, 18), hide, paint=body_paint),
        Part("head", (0, 19, -9), (-4, -4, -6), (8, 8, 6), hide, paint=head),
        Part("horn_l", (0, 19, -9), (4, 3, -4), (1, 3, 1), ["#cfc8b4", "#dcd6c4", "#e8e2d2"], parent="head"),
        Part("horn_r", (0, 19, -9), (-5, 3, -4), (1, 3, 1), ["#cfc8b4", "#dcd6c4", "#e8e2d2"], parent="head"),
        Part("tail", (0, 20, 9), (-0.5, -8, 0), (1, 8, 1), hide, paint=tail),
        leg("leg_fl", 4, -7),
        leg("leg_fr", -4, -7),
        leg("leg_bl", 4, 7),
        leg("leg_br", -4, 7),
    ]


def pig():
    skin = ["#de949c", "#e8a2aa", "#f0b0b6"]

    def head(face, p, r):
        if face == "front":
            eyes(p, 2, 1, 5)
            p.rect(0, 0, 2, 2, "#c97880")
            p.rect(6, 0, 2, 2, "#c97880")

    def snout(face, p, r):
        p.rect(0, 0, p.w, p.h, "#d07c86")
        if face == "front":
            p.px(0, 1, "#7a3840")
            p.px(3, 1, "#7a3840")

    leg = lambda n, x, z: Part(n, (x, 6, z), (-2, -6, -2), (4, 6, 4), skin, paint=hooves("#a86068", 1))
    return [
        Part("body", (0, 10, 0), (-5, -4, -7), (10, 8, 14), skin, paint=patches("#d0848e", 0.72, "pig-mud")),
        Part("head", (0, 10, -7), (-4, -4, -7), (8, 8, 7), skin, paint=head),
        Part("snout", (0, 10, -7), (-2, -3, -8), (4, 3, 1), skin, parent="head", paint=snout),
        Part("tail", (0, 12, 7), (-1, -1, 0), (2, 2, 1), ["#d07c86", "#d98890", "#e09098"]),
        leg("leg_fl", 3, -5),
        leg("leg_fr", -3, -5),
        leg("leg_bl", 3, 5),
        leg("leg_br", -3, 5),
    ]


def sheep():
    fleece = ["#d6d0c2", "#ebe6da", "#faf7f0"]
    face_tone = ["#4e3c2e", "#5a4636", "#665040"]

    def head(face, p, r):
        if face == "front":
            eyes(p, 2, 0, 4)
            p.rect(2, 4, 2, 1, "#2e2018")
        if face == "top":
            p.rect(0, 2, p.w, p.h - 2, "#ebe6da")
        if face in ("west", "east", "back"):
            p.rect(0, 0, p.w, 2, "#ebe6da")

    leg = lambda n, x, z: Part(n, (x, 9, z), (-1.5, -9, -1.5), (3, 9, 3), face_tone, paint=hooves("#2e2018", 1))
    return [
        Part("body", (0, 13, 0), (-5.5, -5, -8), (11, 10, 16), fleece, grain=1.0),
        Part("head", (0, 16, -8), (-3, -3, -6), (6, 6, 7), face_tone, paint=head),
        leg("leg_fl", 3, -5),
        leg("leg_fr", -3, -5),
        leg("leg_bl", 3, 5),
        leg("leg_br", -3, 5),
    ]


def chicken():
    plume = ["#e2ded2", "#f4f1ea", "#ffffff"]
    amber = ["#c8862a", "#e0a030", "#f0b648"]

    def head(face, p, r):
        if face == "west":
            p.px(1, 1, "#101010")
        if face == "east":
            p.px(p.w - 2, 1, "#101010")

    def foot(face, p, r):
        pass

    return [
        Part("body", (0, 8, 0), (-3, -3, -4), (6, 6, 8), plume),
        Part("head", (0, 10, -3), (-2, 0, -3), (4, 6, 3), plume, paint=head),
        Part("beak", (0, 10, -3), (-1, 2, -5), (2, 1, 2), amber, parent="head"),
        Part("comb", (0, 10, -3), (-0.5, 6, -3), (1, 2, 3), ["#a82424", "#c83030", "#dc4444"], parent="head"),
        Part("wattle", (0, 10, -3), (-0.5, 0, -4), (1, 2, 1), ["#a82424", "#c83030", "#dc4444"], parent="head"),
        Part("wing_l", (3, 10, 0), (0, -4, -3), (1, 4, 6), ["#cfcbbf", "#dedad0", "#ebe8e0"]),
        Part("wing_r", (-3, 10, 0), (-1, -4, -3), (1, 4, 6), ["#cfcbbf", "#dedad0", "#ebe8e0"]),
        Part("tail", (0, 10, 4), (-2, -1, 0), (4, 4, 2), ["#cfcbbf", "#dedad0", "#ebe8e0"]),
        Part("leg_l", (1.5, 5, 0), (-0.5, -5, -0.5), (1, 5, 1), amber, paint=foot),
        Part("leg_r", (-1.5, 5, 0), (-0.5, -5, -0.5), (1, 5, 1), amber, paint=foot),
        Part("foot_l", (1.5, 5, 0), (-1.5, -5, -2.5), (3, 0.5, 3), amber, parent="leg_l"),
        Part("foot_r", (-1.5, 5, 0), (-1.5, -5, -2.5), (3, 0.5, 3), amber, parent="leg_r"),
    ]


def ghoul():
    skin = ["#7c728a", "#8a8098", "#988ea6"]
    cloth = ["#3e3028", "#4a3a30", "#564438"]
    trousers = ["#26222e", "#2e2a38", "#363242"]

    def head(face, p, r):
        if face == "front":
            p.rect(1, 2, 2, 2, "#1c1622")
            p.rect(5, 2, 2, 2, "#1c1622")
            p.px(2, 3, "#ffb020")
            p.px(5, 3, "#ffb020")
            p.rect(2, 5, 4, 1, "#1c1622")
            p.px(3, 5, "#d8d0c0")
            p.px(5, 5, "#d8d0c0")
        if face == "top":
            for x in range(p.w):
                for y in range(p.h):
                    if (x * 3 + y * 5) % 4:
                        p.px(x, y, "#2c2634")
        if face in ("west", "east", "back", "front"):
            for x in range(p.w):
                p.px(x, 0, "#2c2634")
                if (x * 7) % 3 == 0:
                    p.px(x, 1, "#2c2634")

    def torso(face, p, r):
        if face in ("front", "back"):
            for x, y in ((1, 2), (2, 3), (6, 6), (7, 7), (3, 8)):
                p.px(x, y, skin[1])
            p.rect(0, p.h - 2, p.w, 1, "#8a6a2c")
            p.px(p.w // 2, p.h - 2, "#d8b040")
            for x in range(0, p.w, 2):
                p.px(x, p.h - 1, skin[0])

    def arm(face, p, r):
        if face in ("west", "east", "front", "back"):
            p.rect(0, 0, p.w, 4, cloth[1])
            for x in range(p.w):
                if (x + p.x) % 2:
                    p.px(x, 4, cloth[0])
            p.rect(0, p.h - 2, p.w, 2, "#6a6078")
        if face == "top":
            p.rect(0, 0, p.w, p.h, cloth[1])

    def leg(face, p, r):
        if face in ("west", "east", "front", "back"):
            for x in range(p.w):
                if (x * 5 + p.x) % 3 == 0:
                    p.px(x, p.h - 3, skin[0])
            p.rect(0, p.h - 2, p.w, 2, skin[0])

    return [
        Part("body", (0, 10, 0), (-4.5, 0, -2.5), (9, 11, 5), cloth, paint=torso),
        Part("head", (0, 21, 0), (-4, 0, -4), (8, 7, 8), skin, parent="body", paint=head),
        Part("arm_l", (6, 20, 0), (-1.5, -12, -1.5), (3, 13, 3), skin, parent="body", paint=arm),
        Part("arm_r", (-6, 20, 0), (-1.5, -12, -1.5), (3, 13, 3), skin, parent="body", paint=arm),
        Part("leg_l", (2.25, 10, 0), (-2, -10, -2), (4, 10, 4), trousers, paint=leg),
        Part("leg_r", (-2.25, 10, 0), (-2, -10, -2), (4, 10, 4), trousers, paint=leg),
    ]


def spider():
    shell = ["#241e2a", "#2e2634", "#3a3042"]

    def head(face, p, r):
        if face == "front":
            p.rect(1, 1, 2, 2, "#e02828")
            p.rect(5, 1, 2, 2, "#e02828")
            p.px(1, 1, "#ff8a8a")
            p.px(5, 1, "#ff8a8a")
            for x in (0, 3, 4, 7):
                p.px(x, 0, "#b01c1c")
            p.px(2, 5, "#d8d0c0")
            p.px(5, 5, "#d8d0c0")

    def abdomen(face, p, r):
        if face == "top":
            for y in range(p.h):
                half = 2 if y in (2, 3, 7, 8) else 1 if y in (4, 6) else 0
                if 2 <= y <= 8:
                    p.rect(p.w // 2 - 1 - half, y, 2 + half * 2, 1, "#c85a20")

    def leg(face, p, r):
        if face in ("front", "back", "top"):
            for x in (0, 4, 5, p.w - 1):
                p.rect(x, 0, 1, p.h, "#5a4a66")

    parts = [
        Part("body", (0, 8, 1), (-5, -3, 0), (10, 7, 11), shell, paint=abdomen),
        Part("head", (0, 8, 1), (-4, -3, -7), (8, 6, 7), shell, paint=head),
    ]
    for index, z in enumerate((-4.5, -2, 0.5, 3)):
        parts.append(Part(f"leg_l{index}", (4, 8, z), (0, -1, -1), (11, 2, 2), shell, paint=leg))
        parts.append(Part(f"leg_r{index}", (-4, 8, z), (-11, -1, -1), (11, 2, 2), shell, paint=leg))
    return parts


MOBS = {"cow": cow, "pig": pig, "sheep": sheep, "chicken": chicken, "ghoul": ghoul, "spider": spider}


def number(value):
    return f"{float(value):g}" if float(value) != int(value) else f"{int(value)}.0"


def triple(values):
    return "(" + ", ".join(number(v) for v in values) + ")"


def write_mobs():
    lines = [
        "// Generated by tools/generate_pixel_art.py. Units are 1/16 of a block.",
        "// Models face -Z with feet on y = 0; `from` is relative to `pivot`.",
        "[",
    ]
    out = ROOT / "assets/textures/mobs"
    out.mkdir(parents=True, exist_ok=True)
    sheets = {}
    for name, build in MOBS.items():
        parts = build()
        image, (width, height) = paint_mob(name, parts)
        Image.fromarray(image, "RGBA").save(out / f"{name}.png", optimize=True)
        sheets[name] = image
        lines.append(f'    (kind: "{name}", texture: ({width}, {height}), parts: [')
        for part in parts:
            parent = f'Some("{part.parent}")' if part.parent else "None"
            lines.append(
                f'        (name: "{part.name}", parent: {parent}, pivot: {triple(part.pivot)}, '
                f"from: {triple(part.start)}, size: {triple(part.size)}, uv: ({part.uv[0]}, {part.uv[1]})),"
            )
        lines.append("    ]),")
    lines.append("]")
    (ROOT / "assets/data/mob_models.ron").write_text("\n".join(lines) + "\n", encoding="utf-8")
    return sheets


def save(image, path, scale):
    data = np.clip(np.rint(image), 0, 255).astype(np.uint8)
    picture = Image.fromarray(data, "RGBA")
    if scale != 1:
        picture = picture.resize((N * scale, N * scale), Image.NEAREST)
    path.parent.mkdir(parents=True, exist_ok=True)
    picture.save(path, optimize=True)
    return picture


def main():
    pixel = ROOT / "assets/textures/pixel"
    tiles = {}
    for name in sorted(TILES):
        image = TILES[name]()
        save(image, pixel / "32x32" / f"{name}.png", 1)
        tiles[name] = save(image, pixel / "128x128" / f"{name}.png", 4)
    icons = {}
    for name in sorted(ICONS):
        icons[name] = save(ICONS[name](), pixel / "icons" / f"{name}.png", 4)
    sheets = write_mobs()
    print(f"{len(tiles)} tiles, {len(icons)} icons, {len(sheets)} mobs")
    if "--sheet" in sys.argv:
        target = sys.argv[sys.argv.index("--sheet") + 1]
        everything = list(tiles.items()) + list(icons.items())
        columns = 10
        rows = -(-len(everything) // columns)
        sheet = Image.new("RGBA", (columns * 132, rows * 132 + 270), (60, 60, 70, 255))
        for index, (_, picture) in enumerate(everything):
            sheet.paste(picture, ((index % columns) * 132 + 2, (index // columns) * 132 + 2), picture)
        x = 2
        for image in sheets.values():
            picture = Image.fromarray(image, "RGBA")
            picture = picture.resize((picture.width * 4, picture.height * 4), Image.NEAREST)
            sheet.paste(picture, (x, rows * 132 + 4), picture)
            x += picture.width + 6
        sheet.save(target)


if __name__ == "__main__":
    main()
