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


# ------------------------------------------------------------ palette set

COLORS = {
    "white": "#e9ecec",
    "orange": "#f07613",
    "magenta": "#bd44b3",
    "light_blue": "#3aafd9",
    "yellow": "#f8c627",
    "lime": "#70b919",
    "pink": "#ed8dac",
    "gray": "#3e4447",
    "light_gray": "#8e8e86",
    "cyan": "#158991",
    "purple": "#792aac",
    "blue": "#35399d",
    "brown": "#724728",
    "green": "#546d1b",
    "red": "#a12722",
    "black": "#16161b",
}


def shades(color, spread=0.16):
    base = rgb(color)
    return ["#%02x%02x%02x" % tuple(int(v) for v in np.clip(base * (1 + spread * k) + 6 * k, 0, 255)) for k in (-1.0, -0.4, 0.2, 0.8)]


def dyed_wool(name, color):
    def build():
        r = rng("wool")
        image = fill(canvas(), fbm(r, ((8, 1.0), (16, 0.8))), shades(color))
        for y in range(N):
            for x in range(N):
                if (x + y * 3) % 8 == 0 or (x * 3 - y) % 11 == 0:
                    image[y, x, :3] = image[y, x, :3] * 0.88
        return image

    TILES[name] = build


def concrete(name, color):
    def build():
        r = rng("concrete")
        image = fill(canvas(), fbm(r, ((4, 1.0), (8, 0.3))), shades(color, 0.05))
        speckle(image, r, [shades(color, 0.1)[0]], 14)
        return image

    TILES[name] = build


for _color, _hex in COLORS.items():
    if _color != "white":
        dyed_wool(f"wool_{_color}", _hex)
    concrete(f"concrete_{_color}", _hex)


def rock(name, palette, speck, veins=None, layered=False):
    def raw():
        r = rng(name)
        octaves = ((2, 0.3), (32, 1.0)) if layered else ((4, 1.0), (8, 0.7), (16, 0.5))
        field = fbm(r, octaves)
        if layered:
            field = np.clip(np.repeat(field[:, ::8], 8, axis=1) * 0.8 + fbm(r) * 0.2, 0, 0.999)
        image = fill(canvas(), field, palette)
        speckle(image, r, speck, 60)
        if veins:
            for _ in range(5):
                x, y = r.integers(0, N, 2)
                for _ in range(int(r.integers(10, 22))):
                    put(image, x, y, veins)
                    x += 1
                    y += int(r.integers(-1, 2))
        return image

    def polished():
        image = raw()
        image[..., :3] = image[..., :3] * 0.5 + np.array(rgb(palette[2])) * 0.5
        for i in range(N):
            put(image, i, 0, palette[3])
            put(image, 0, i, palette[3])
            put(image, i, N - 1, palette[0])
            put(image, N - 1, i, palette[0])
        return image

    TILES[name] = raw
    TILES["polished_" + name] = polished


rock("granite", ["#8a5a4a", "#9a6756", "#a97462", "#b8826f"], ["#d9b0a0", "#5e3a30", "#e8e0d8"])
rock("marble", ["#d8d8d4", "#e2e2de", "#ecece8", "#f6f6f3"], ["#c8c8c4"], veins="#a8adb4")
rock("basalt", ["#2c2c30", "#35353a", "#3f3f45", "#494950"], ["#5a5a62", "#202024"], layered=True)
rock("slate", ["#3e4a5a", "#475466", "#505e72", "#59687e"], ["#6a7a92", "#323c4a"], layered=True)
rock("limestone", ["#b8ae90", "#c2b89a", "#ccc2a4", "#d6ccae"], ["#a89e80", "#e2dac0"])

TILES["pine_log"] = lambda: bark(rng("pine_log"), ["#2e2018", "#382820", "#423028", "#4c3830"], "#1c120c")
TILES["pine_log_top"] = lambda: log_top("pine_log_top", ["#2e2018", "#382820", "#423028"], ["#8a6a42", "#947448", "#9e7e50", "#a88858"], "#6e5230")
TILES["pine_planks"] = lambda: planks("pine_planks", ["#7a5a36", "#84633c", "#8e6c42", "#987548"], "#58401f", "#684c28")
TILES["pine_leaves"] = lambda: leaves("pine_leaves", ["#1c4a2c", "#225636", "#286240", "#30704a", "#3a7e54"], "#143820", 0.28)


@tile("sandstone_bricks")
def sandstone_bricks():
    return brick_pattern(rng("sandstone_bricks"), ["#cdb983", "#d6c38e", "#dfcd9a", "#e7d6a6"], "#a89462")


@tile("mossy_stone_bricks")
def mossy_stone_bricks():
    image = brick_pattern(rng("stone_bricks"), STONE, "#555555")
    moss = fbm(rng("moss-bricks"), ((4, 1.0), (8, 0.8), (16, 0.5)))
    green = ramp(fbm(rng("moss2"), ((16, 1.0), (32, 1.0))), ["#3f6b2a", "#4f7f33", "#5f9440"])
    image[moss > 0.6, :3] = green[moss > 0.6]
    return image


@tile("cracked_stone_bricks")
def cracked_stone_bricks():
    r = rng("cracked")
    image = brick_pattern(rng("stone_bricks"), STONE, "#555555")
    for _ in range(9):
        x, y = r.integers(0, N, 2)
        for _ in range(int(r.integers(5, 11))):
            put(image, x, y, "#3e3e3e")
            x += int(r.integers(-1, 2))
            y += 1
    return image


@tile("dark_bricks")
def dark_bricks():
    image = bricks()
    image[..., :3] = image[..., :3] * np.array([0.42, 0.44, 0.6])
    return image


@tile("stone_tiles")
def stone_tiles():
    r = rng("stone_tiles")
    image = canvas("#555555")
    tone = fbm(r, ((16, 1.0), (32, 0.6)))
    for ty in range(4):
        for tx in range(4):
            dark = (tx + ty) % 2
            for y in range(ty * 8, ty * 8 + 7):
                for x in range(tx * 8, tx * 8 + 7):
                    value = np.clip(tone[y, x] * 0.5 + (0.1 if dark else 0.5), 0, 0.999)
                    image[y, x, :3] = ramp(np.array(value), ["#5e5e5e", "#727272", "#8a8a8a", "#a0a0a0", "#b4b4b4"])
            for i in range(7):
                image[ty * 8, tx * 8 + i, :3] = np.clip(image[ty * 8, tx * 8 + i, :3] + 20, 0, 255)
    return image


# ----------------------------------------------------------------- plants


def plant(name, petals, center, tall=False):
    def build():
        r = rng(name)
        image = np.zeros((N, N, 4))
        image[..., :3] = rgb("#4c8c30")
        stem = ["#356b24", "#4c8c30", "#6aad42"]
        if tall:
            for blade in range(11):
                x = 3 + blade * 2.5 + r.uniform(-1, 1)
                height = int(r.integers(12, 27))
                lean = r.uniform(-0.25, 0.25)
                for step in range(height):
                    put(image, int(x + lean * step), N - 1 - step, stem[min(2, step * 3 // height)])
                    if step < height * 0.6:
                        put(image, int(x + lean * step) + 1, N - 1 - step, stem[0])
            return image
        for y in range(13, N):
            put(image, 15 + (1 if y % 7 == 0 else 0), y, stem[1])
            put(image, 16 + (1 if y % 7 == 0 else 0), y, stem[0])
        for side, y0 in ((-1, 22), (1, 25)):
            for step in range(5):
                put(image, 15 + side * (step + 1), y0 - step // 2, stem[2])
                put(image, 15 + side * (step + 1), y0 - step // 2 + 1, stem[0])
        light, mid, dark = petals
        for y in range(2, 16):
            for x in range(8, 24):
                d = math.hypot(x - 15.5, (y - 8.5) * 1.15)
                angle = math.atan2(y - 8.5, x - 15.5)
                edge = 5.6 + 1.3 * math.cos(angle * 5)
                if d < edge:
                    put(image, x, y, light if (x - 15.5) + (y - 8.5) < -2 else mid if d < edge - 1.2 else dark)
                if d < 1.9:
                    put(image, x, y, center)
        return image

    TILES[name] = build


plant("poppy", ("#f0554a", "#d0302a", "#8e1c1c"), "#2a1a1a")
plant("dandelion", ("#fff07a", "#f8c627", "#c8901a"), "#e0a020")
plant("cornflower", ("#7a9cf2", "#4668d8", "#2a3f9a"), "#1c2a6e")
plant("daisy", ("#ffffff", "#e9ecec", "#b8bcc0"), "#f8c627")
plant("tall_grass", None, None, tall=True)


def dye_icon(color):
    def build():
        r = rng("dye")
        mask = ellipse(15.5, 19, 10, 6.5) | ellipse(13, 14, 5, 4.5) | ellipse(19, 15, 4.5, 4)
        palette = shades(color, 0.22)
        dark = "#%02x%02x%02x" % tuple(int(v * 0.45) for v in rgb(color))
        return sprite(mask, [dark, palette[0], palette[2], palette[3]], r, 0.7)

    return build


for _color, _hex in COLORS.items():
    ICONS[f"dye_{_color}"] = dye_icon(_hex)


# ------------------------------------------------------------- big batch

WOODS = {
    "acacia": (["#5a5148", "#665c52", "#72675c", "#7e7266"], ["#a8552e", "#b25e34", "#bc673a", "#c67040"], ["#5c7a22", "#6a8a28", "#789a2e", "#86aa36", "#94ba40"]),
    "jungle": (["#4a3f1e", "#564a24", "#62552a", "#6e6030"], ["#a07550", "#aa7e58", "#b48760", "#be9068"], ["#1f6a12", "#277a18", "#2f8a1e", "#389a26", "#44ac30"]),
    "dark_oak": (["#2a1e10", "#332514", "#3c2c18", "#45331c"], ["#3e2912", "#463016", "#4e371a", "#563e1e"], ["#1f4f14", "#265c19", "#2d691e", "#357624", "#3e842c"]),
    "mangrove": (["#4e3a2a", "#584232", "#624a3a", "#6c5242"], ["#6e2c2a", "#783230", "#823836", "#8c3e3c"], ["#3a7a1e", "#448a24", "#4e9a2a", "#58aa32", "#64ba3c"]),
    "spruce": (["#2c2218", "#362a1e", "#403224", "#4a3a2a"], ["#6e5230", "#785a36", "#82623c", "#8c6a42"], ["#2a5a34", "#306840", "#367648", "#3e8452", "#48925c"]),
    "cherry": (["#2e1c22", "#382228", "#42282e", "#4c2e34"], ["#deb0a8", "#e4b8b0", "#eac0b8", "#f0c8c0"], ["#e896b8", "#eea6c4", "#f4b6d0", "#f8c6dc", "#fcd8e8"]),
}
for _wood, (_bark, _planks, _leaf) in WOODS.items():
    TILES[f"{_wood}_log"] = (lambda b=_bark, n=_wood: bark(rng(n + "_log"), b, b[0]))
    TILES[f"{_wood}_log_top"] = (lambda b=_bark, p=_planks, n=_wood: log_top(n + "_log_top", b[:3], p, p[0]))
    TILES[f"{_wood}_planks"] = (lambda p=_planks, n=_wood: planks(n + "_planks", p, "#%02x%02x%02x" % tuple(int(v * 0.7) for v in rgb(p[0])), p[0]))
    TILES[f"{_wood}_leaves"] = (lambda l=_leaf, n=_wood: leaves(n + "_leaves", l, "#%02x%02x%02x" % tuple(int(v * 0.72) for v in rgb(l[0])), 0.3))

ore("copper_ore", ("#f0a070", "#c8683c", "#4a5a50"))
ore("emerald_ore", ("#a8ffc0", "#22c85a", "#3a5a44"))
ore("lapis_ore", ("#7a9cff", "#2a4fc8", "#3a4460"))
metal("copper_block", ("#8a4424", "#b8643a", "#d07a4a", "#e89460", "#ffc8a0"))
metal("emerald_block", ("#0e7a34", "#1ea84c", "#2ec864", "#5ee88c", "#c8ffd8"))
metal("lapis_block", ("#16286e", "#22409a", "#2c52b8", "#4a70d8", "#a8c0ff"))


def stained_glass(name, color):
    def build():
        image = canvas(color, 96)
        light = shades(color, 0.3)[3]
        dark = shades(color, 0.3)[0]
        for i in range(N):
            for edge in (0, N - 1):
                put(image, i, edge, light, 255)
                put(image, edge, i, light, 255)
            for edge in (1, N - 2):
                if 0 < i < N - 1:
                    put(image, i, edge, dark, 230)
                    put(image, edge, i, dark, 230)
        for start, length in ((5, 7), (9, 4), (20, 6)):
            for step in range(length):
                put(image, start + step, start + length - step + 2, "#ffffff", 190)
        return image

    TILES[name] = build


def glazed(name, color):
    def build():
        r = rng("terracotta")
        base = rgb(color) * 0.62 + rgb("#964f32") * 0.38
        hexc = "#%02x%02x%02x" % tuple(int(v) for v in base)
        image = fill(canvas(), fbm(r, ((4, 1.0), (8, 0.4))), shades(hexc, 0.07))
        speckle(image, r, [shades(hexc, 0.12)[0]], 18)
        return image

    TILES[name] = build


for _color, _hex in COLORS.items():
    stained_glass(f"glass_{_color}", _hex)
    glazed(f"terracotta_{_color}", _hex)


def simple(name, palette, speck=(), octaves=((4, 1.0), (8, 0.7), (16, 0.5)), count=40):
    def build():
        r = rng(name)
        image = fill(canvas(), fbm(r, octaves), palette)
        if speck:
            speckle(image, r, list(speck), count)
        return image

    TILES[name] = build


simple("mud", ["#3a3238", "#42393f", "#4a4046", "#52474d"], ["#2e282c", "#5e5258"])
simple("packed_mud", ["#8a6a52", "#94735a", "#9e7c62", "#a8856a"], ["#c8b08a", "#7a5c46"], count=60)
simple("moss_block", ["#4a6a24", "#55772a", "#608430", "#6b9136"], ["#7aa240", "#3e5a1e"], count=70)
simple("coarse_dirt", ["#5e4630", "#6b5038", "#785a40", "#856448"], ["#9a9088", "#4a3828", "#a08a70"], count=90)
simple("red_sand", ["#b05a20", "#ba6326", "#c46c2c", "#ce7532"], ["#a04e18", "#dc8a48"], ((4, 1.0), (16, 0.5)), 70)
simple("calcite", ["#d8dcd8", "#e0e4e0", "#e8ece8", "#f0f4f0"], ["#c8ccc8"], ((2, 1.0), (8, 0.5)))
simple("tuff", ["#5a5c52", "#64665c", "#6e7066", "#787a70"], ["#4a4c44", "#8a8c80"], count=80)
simple("quartz_block", ["#e4ddd4", "#eae4dc", "#f0ebe4", "#f6f2ec"], ["#dcd4c8"], ((2, 1.0), (4, 0.5)), 16)
simple("sponge", ["#b8b030", "#c4bc3a", "#d0c844", "#dcd44e"])


def _holes(name, base, color, count):
    def build():
        image = TILES[base]()
        r = rng(name)
        for _ in range(count):
            x, y = r.integers(1, N - 2, 2)
            rect(image, int(x), int(y), int(r.integers(1, 4)), int(r.integers(1, 3)), color)
        return image

    return build


_sponge = TILES["sponge"]
TILES["sponge"] = lambda: (lambda image: (
    [rect(image, int(x), int(y), 2, 2, "#8a8420") for x, y in rng("sponge-holes").integers(1, N - 3, (26, 2))], image)[1])(_sponge())


@tile("red_sandstone")
def red_sandstone():
    r = rng("red_sandstone")
    image = fill(canvas(), fbm(r, ((4, 1.0), (16, 0.6))), ["#a8521c", "#b25b22", "#bc6428", "#c66d2e"])
    for y in (0, 9, 10, 19, 20, 27, N - 1):
        for x in range(N):
            put(image, x, y, "#8e4416" if y % 2 else "#d07e3e")
    return image


@tile("mud_bricks")
def mud_bricks():
    return brick_pattern(rng("mud_bricks"), ["#8a6a52", "#94735a", "#9e7c62", "#a8856a"], "#5e4636")


@tile("amethyst_block")
def amethyst_block():
    r = rng("amethyst")
    image, _ = cells(r, 14, "#4a2a7a", ["#7a4ab8", "#8a5ac8", "#9a6cd8", "#b088e8"], 30, 1.0)
    speckle(image, r, ["#e8d8ff"], 14)
    return image


def striped(name, palette, dark, vertical=True, period=4):
    def build():
        r = rng(name)
        image = fill(canvas(), fbm(r, ((16, 1.0), (32, 0.6))), palette)
        for i in range(0, N, period):
            for j in range(N):
                if r.random() < 0.85:
                    put(image, i if vertical else j, j if vertical else i, dark)
        return image

    TILES[name] = build


striped("melon_side", ["#5a9a22", "#66a828", "#72b62e", "#7ec436"], "#3a6a16", True, 5)
simple("melon_top", ["#5a9a22", "#66a828", "#72b62e", "#7ec436"], ["#3a6a16"], count=50)
striped("pumpkin_side", ["#d07a14", "#dc861a", "#e89220", "#f29e28"], "#a85a0c", True, 6)
striped("hay_bale_side", ["#b89a1c", "#c4a622", "#d0b228", "#dcbe30"], "#8a7012", True, 3)
simple("hay_bale_top", ["#b89a1c", "#c4a622", "#d0b228", "#dcbe30"], ["#8a7012", "#ecd45a"], ((16, 1.0), (32, 1.0)), 120)
striped("bone_block_side", ["#d8d2b8", "#e0dac2", "#e8e2cc", "#f0ead6"], "#b8b298", True, 8)
simple("bone_block_top", ["#d8d2b8", "#e0dac2", "#e8e2cc", "#f0ead6"], ["#b8b298"], count=50)
simple("podzol_top", ["#5a3e1c", "#664822", "#725228", "#7e5c2e"], ["#3e2a12", "#8e6c3a", "#4a5a22"], count=90)


@tile("pumpkin_top")
def pumpkin_top():
    image = TILES["pumpkin_side"]()
    image = np.rot90(image).copy()
    rect(image, 13, 13, 6, 6, "#5a7a22")
    rect(image, 14, 14, 4, 4, "#7a9a32")
    return image


@tile("podzol_side")
def podzol_side():
    r = rng("podzol_side")
    image = dirt_base(r)
    top = TILES["podzol_top"]()
    for x in range(N):
        d = 4 + int(r.integers(0, 3))
        image[:d, x] = top[:d, x]
    return image


def gem_icon(name, palette, shape):
    def build():
        r = rng(name)
        if shape == "gem":
            mask = np.zeros((N, N), dtype=bool)
            for y in range(N):
                for x in range(N):
                    mask[y, x] = abs(x - 15.5) / 9 + abs(y - 16) / 11 <= 1
        elif shape == "ingot":
            mask = np.zeros((N, N), dtype=bool)
            for y in range(11, 22):
                shift = (y - 11) // 3
                mask[y, 4 + shift : 24 + shift] = True
        else:
            mask = ellipse(12, 14, 6, 5) | ellipse(19, 18, 7, 6) | ellipse(13, 21, 5, 4)
        return sprite(mask, palette, r)

    ICONS[name] = build


gem_icon("raw_copper", ["#6e3418", "#b8643a", "#d07a4a", "#f0a070"], "lump")
gem_icon("copper_ingot", ["#6e3418", "#b8643a", "#d88454", "#ffc8a0"], "ingot")
gem_icon("emerald", ["#0a5a26", "#1ea84c", "#3ed874", "#c8ffd8"], "gem")
gem_icon("lapis", ["#101e5a", "#22409a", "#3a60d0", "#a8c0ff"], "lump")


# ----------------------------------------------------------------- fluids


@tile("lava")
def lava():
    r = rng("lava")
    field = fbm(r, ((4, 1.0), (8, 0.8), (16, 0.4)))
    image = fill(canvas(), field, ["#8a1c04", "#b82a08", "#e0480e", "#f8751a", "#ffa72a", "#ffd25a"])
    crust = fbm(rng("lava-crust"), ((4, 1.0), (8, 0.7), (16, 0.5)))
    for y in range(N):
        for x in range(N):
            if crust[y, x] > 0.74:
                image[y, x, :3] = rgb("#3a1208" if crust[y, x] > 0.8 else "#5e1c0a")
    speckle(image, r, ["#fff0a0"], 18)
    return image


def bucket(name, fill_color=None):
    def build():
        r = rng("bucket")
        mask = np.zeros((N, N), dtype=bool)
        for y in range(10, 27):
            half = 9.5 - (y - 10) * 0.32
            mask[y, int(15.5 - half) : int(15.5 + half) + 1] = True
        body = sprite(mask, ["#3a3a40", "#8e9199", "#bcc0c8", "#e6e9ee"], r, 0.3)
        ring = ellipse(15.5, 11, 11, 7) & ~ellipse(15.5, 11, 9.6, 5.8)
        ring[11:] = False
        handle = sprite(ring, ["#3a3a40", "#8e9199", "#bcc0c8", "#e6e9ee"], r, 0.1)
        image = overlay(handle, body)
        rim = ellipse(15.5, 11, 9, 2.6)
        for y in range(N):
            for x in range(N):
                if rim[y, x]:
                    image[y, x, :3] = rgb(fill_color or "#2a2a30")
                    image[y, x, 3] = 255
        return image

    ICONS[name] = build


bucket("bucket")
bucket("water_bucket", "#3a74d8")
bucket("lava_bucket", "#ff8a1a")


# ------------------------------------------------------------- part 1 batch


def transparent():
    return np.zeros((N, N, 4), dtype=float)


for _color, _hex in COLORS.items():
    pass

rock("diorite", ["#8f8f8f", "#9a9a9a", "#a6a6a6", "#b2b2b2"], ["#e4e4e4", "#5e5e5e", "#f4f4f4", "#6e6e6e"])
rock("andesite", ["#7b7b7b", "#858585", "#8f8f8f", "#9a9a9a"], ["#6a6a6a", "#a6a6a6", "#5a5a5a"])

WOOD_PLANKS = {name: palette[1] for name, palette in WOODS.items()}
WOOD_PLANKS["oak"] = OAK_WOOD
WOOD_PLANKS["birch"] = BIRCH_WOOD


def stripped(wood):
    palette = WOOD_PLANKS[wood]

    def side():
        r = rng(f"stripped_{wood}_log")
        columns = fbm(r, ((16, 1.0), (32, 0.5)))[0]
        field = np.clip(columns[None, :] * 0.7 + fbm(r, ((2, 1.0), (8, 0.3))) * 0.3, 0, 0.999)
        image = fill(canvas(), field, palette)
        dark = shades(palette[0], 0.2)[0]
        for x in range(N):
            if columns[x] < 0.16:
                for y in range(N):
                    if r.random() < 0.7:
                        put(image, x, y, dark)
        return image

    def top():
        r = rng(f"stripped_{wood}_log_top")
        image = fill(canvas(), fbm(r, ((16, 1.0),)), palette)
        ring = shades(palette[0], 0.25)[0]
        for y in range(N):
            for x in range(N):
                radius = max(abs(x - 15.5), abs(y - 15.5))
                if radius > 14.2:
                    image[y, x, :3] = rgb(shades(palette[0], 0.2)[0])
                elif int(radius + 0.5 * math.sin(x * 0.9)) % 4 == 0:
                    image[y, x, :3] = rgb(ring)
        return image

    TILES[f"stripped_{wood}_log"] = side
    TILES[f"stripped_{wood}_log_top"] = top


for _wood in ["oak", "spruce", "birch", "jungle", "acacia", "cherry", "dark_oak", "mangrove"]:
    stripped(_wood)

BAMBOO = ["#c8b34a", "#d2bd52", "#dcc75c", "#e6d166"]
TILES["bamboo_planks"] = lambda: planks("bamboo_planks", BAMBOO, "#8a7a2a", "#a08c34")


@tile("bamboo_mosaic")
def bamboo_mosaic():
    r = rng("bamboo_mosaic")
    image = fill(canvas(), fbm(r, ((16, 1.0), (32, 0.5))), BAMBOO)
    dark = "#8a7a2a"
    for by in range(4):
        for bx in range(4):
            x0, y0 = bx * 8, by * 8
            horizontal = (bx + by) % 2 == 0
            for i in range(8):
                put(image, x0 + i, y0, dark)
                put(image, x0, y0 + i, dark)
            for k in range(2, 8, 2):
                for i in range(1, 8):
                    if horizontal:
                        put(image, x0 + i, y0 + k, shades(BAMBOO[0], 0.1)[0])
                    else:
                        put(image, x0 + k, y0 + i, shades(BAMBOO[0], 0.1)[0])
    return image


@tile("bamboo_block")
def bamboo_block():
    r = rng("bamboo_block")
    image = fill(canvas(), fbm(r, ((16, 1.0), (32, 0.4)))[:, :] * 0 + np.linspace(0.2, 0.9, N)[None, :].repeat(N, 0), ["#3e7a1a", "#4a8a20", "#589a28", "#68aa32", "#7aba3e"])
    for y in (0, 1, 15, 16, 30, 31):
        for x in range(N):
            put(image, x, y, "#2e5a12" if y in (0, 15, 30) else "#8ed04a")
    return image


@tile("bamboo_block_top")
def bamboo_block_top():
    image = canvas("#6aa82e")
    for y in range(N):
        for x in range(N):
            d = max(abs(x - 15.5), abs(y - 15.5))
            image[y, x, :3] = rgb("#3a7014" if d > 13 else "#7ac040" if d > 7 else "#d8c86a" if d > 3 else "#6a5a22")
    return image


@tile("stripped_bamboo_block")
def stripped_bamboo_block():
    image = TILES["bamboo_block"]()
    image[..., :3] = image[..., :3] * 0.4 + np.array(rgb("#d8c350")) * 0.6
    return image


@tile("stripped_bamboo_block_top")
def stripped_bamboo_block_top():
    image = canvas("#d8c350")
    for y in range(N):
        for x in range(N):
            d = max(abs(x - 15.5), abs(y - 15.5))
            image[y, x, :3] = rgb("#a89830" if d > 13 else "#e8d460" if d > 7 else "#f0e080" if d > 3 else "#8a7a28")
    return image


def sapling(name, palette, shape="round", trunk="#6a4a2a"):
    def build():
        r = rng(name)
        image = transparent()
        for y in range(20, N):
            for x in (15, 16):
                put(image, x, y, trunk)
        leaf = fill(canvas(), fbm(r, ((8, 1.0), (16, 1.0))), palette)
        dark = shades(palette[0], 0.2)[0]
        for y in range(N):
            for x in range(N):
                if shape == "round":
                    inside = math.hypot((x - 15.5) / 9, (y - 12) / 8.5) < 1
                elif shape == "cone":
                    inside = 3 <= y <= 24 and abs(x - 15.5) < (y - 2) * 0.42
                else:
                    inside = math.hypot((x - 15.5) / 6, (y - 10) / 8) < 1
                if inside:
                    image[y, x, :3] = leaf[y, x, :3]
                    image[y, x, 3] = 255
        solid = image[..., 3] > 0
        for y in range(N):
            for x in range(N):
                if solid[y, x] and (y + 1 >= N or not solid[y + 1, x] or x + 1 >= N or not solid[y, x + 1]) and y < 20:
                    image[y, x, :3] = rgb(dark)
        return image

    TILES[name] = build


sapling("oak_sapling", ["#2f6b1f", "#3a7c26", "#46902e", "#55a538"])
sapling("spruce_sapling", ["#1f4a2c", "#265a36", "#2e6a40", "#387a4a"], "cone")
sapling("birch_sapling", ["#5d8f2c", "#6fa334", "#82b53e", "#96c64c"])
sapling("jungle_sapling", ["#1f6a12", "#277a18", "#2f8a1e", "#389a26"])
sapling("acacia_sapling", ["#5c7a22", "#6a8a28", "#789a2e", "#86aa36"], "flat", "#8a7a6a")
sapling("cherry_sapling", ["#e896b8", "#eea6c4", "#f4b6d0", "#f8c6dc"])
sapling("dark_oak_sapling", ["#1f4f14", "#265c19", "#2d691e", "#357624"])


@tile("mangrove_propagule")
def mangrove_propagule():
    r = rng("propagule")
    image = transparent()
    for y in range(8, 26):
        for x in (15, 16):
            put(image, x, y, "#4a7a2a" if y < 22 else "#6a8a3a")
    for cx, cy, tilt in ((11, 8, -1), (20, 8, 1), (15, 4, 0), (9, 13, -1), (22, 13, 1)):
        for k in range(5):
            put(image, cx + tilt * k // 2, cy + k, "#5aa03a")
            put(image, cx + tilt * k // 2 + 1, cy + k, "#3e7a26")
    for y in range(26, 32):
        put(image, 15, y, "#5a3e22")
        put(image, 16, y, "#4a3018")
    return image


def suspicious(name, base):
    def build():
        image = TILES[base]()
        r = rng(name)
        for _ in range(3):
            x, y = r.integers(3, N - 6, 2)
            for i in range(int(r.integers(4, 9))):
                put(image, x + i, y + int(r.integers(0, 2)), shades("#8a7a5a", 0.1)[0])
        speckle(image, r, ["#e8dcc0", "#5a4a30"], 18)
        return image

    TILES[name] = build


suspicious("suspicious_sand", "sand")
suspicious("suspicious_gravel", "gravel")


def ore_on(name, colors, palette, host, crack, speck):
    def build():
        r = rng(host)
        image = stone_base(r, palette, crack_color=crack)
        speckle(image, r, speck, 40)
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
                        put(image, x, y, light if (x - cx) + (y - cy) < -1 else mid)
                    elif d < radius + 1.0 and (x - cx) + (y - cy) > 0:
                        put(image, x, y, dark)
        return image

    TILES[name] = build


ore_on("deepslate_coal_ore", ("#4a4a4a", "#1e1e1e", "#0e0e10"), DEEP, "deepslate", "#25252c", ["#5c5c6c"])
ore_on("deepslate_iron_ore", ("#e8c3a0", "#c9966c", "#3a3a40"), DEEP, "deepslate", "#25252c", ["#5c5c6c"])
ore_on("deepslate_gold_ore", ("#fff08a", "#f2c12e", "#3a3a40"), DEEP, "deepslate", "#25252c", ["#5c5c6c"])
ore_on("deepslate_lapis_ore", ("#7a9cff", "#2a4fc8", "#2a2a40"), DEEP, "deepslate", "#25252c", ["#5c5c6c"])
ore_on("nether_gold_ore", ("#fff08a", "#f2c12e", "#3a1414"), ["#6a2828", "#742e2e", "#7e3434", "#883a3a"], "netherrack", "#4a1818", ["#a04a4a", "#4a1818"])


def roots(name, palette, muddy):
    def build():
        r = rng(name)
        image = transparent()
        for _ in range(11):
            x, y = int(r.integers(0, N)), int(r.integers(0, N))
            for _ in range(int(r.integers(14, 28))):
                for dx in range(2):
                    for dy in range(2):
                        color = palette[int(r.integers(0, len(palette)))]
                        put(image, x + dx, y + dy, color)
                x += int(r.integers(-1, 2))
                y += int(r.integers(-1, 2))
                if r.random() < 0.55:
                    x += 1 if r.random() < 0.5 else 0
        if muddy:
            mud = fbm(rng(name + "mud"), ((8, 1.0), (16, 0.6)))
            for y in range(N):
                for x in range(N):
                    if image[y, x, 3] > 0 and mud[y, x] > 0.45:
                        image[y, x, :3] = rgb("#4a3a32")
        speckle_alpha = image[..., 3] > 0
        image[~speckle_alpha, :3] = rgb(palette[1])
        return image

    TILES[name] = build


roots("mangrove_roots", ["#4a3622", "#5a4429", "#6a5232", "#7a5e3a"], False)
roots("muddy_mangrove_roots", ["#4a3622", "#5a4429", "#6a5232", "#7a5e3a"], True)
TILES["azalea_leaves"] = lambda: leaves("azalea_leaves", ["#4a6a1a", "#567a20", "#628a26", "#6e9a2e", "#7aaa38"], "#365010", 0.3)


@tile("flowering_azalea_leaves")
def flowering_azalea_leaves():
    image = TILES["azalea_leaves"]()
    r = rng("blossom")
    for _ in range(26):
        x, y = r.integers(1, N - 1, 2)
        if image[y, x, 3] > 0:
            for dx, dy in ((0, 0), (1, 0), (0, 1)):
                put(image, x + dx, y + dy, "#f0a0d8" if (dx + dy) % 2 == 0 else "#d870b8")
    return image


@tile("wet_sponge")
def wet_sponge():
    image = TILES["sponge"]()
    image[..., :3] = image[..., :3] * np.array([0.62, 0.68, 0.45]) + np.array([10, 14, 6])
    return image


def stone_box(face, name):
    r = rng("dispenser")
    image, _ = cells(r, 17, "#4a4a4a", ["#6c6c6c", "#7a7a7a", "#888888", "#969696"], 18)
    if face == "front":
        frame(image, "#3a3a3a", "#9a9a9a")
        rect(image, 8, 8, 16, 16, "#3a3a3a")
        rect(image, 10, 10, 12, 12, "#222222")
        rect(image, 13, 13, 6, 6, "#0c0c0c")
        rect(image, 14, 14, 4, 4, "#000000")
        for i in range(2, 6):
            put(image, 16, 5 - i // 2 + 8, "#555555")
    return image


TILES["dispenser_front"] = lambda: stone_box("front", "dispenser")
TILES["dispenser_side"] = lambda: stone_box("side", "dispenser")


@tile("note_block")
def note_block():
    image = planks("note_block", ["#8a5f30", "#966a38", "#a27540", "#ae8048"], "#5e3e1a", "#6e4a22")
    frame(image, "#4a300e", "#7a5430")
    for y in range(N):
        for x in range(N):
            d = math.hypot(x - 15.5, y - 15.5)
            if d < 5.2:
                image[y, x, :3] = rgb("#2e1c0a" if d < 3.2 else "#5e3e1a")
    for dx, dy in ((-1, -1), (0, -1), (-1, 0)):
        put(image, 15 + dx, 15 + dy, "#c8a068")
    return image


@tile("chiseled_sandstone")
def chiseled_sandstone():
    image = sandstone_base(rng("chiseled"))
    for i in range(4, 28):
        for edge in (4, 27):
            put(image, i, edge, "#a89462")
            put(image, edge, i, "#a89462")
    for i in range(7, 25):
        put(image, i, 16, "#bfa96e")
        put(image, 16, i, "#bfa96e")
    rect(image, 13, 13, 6, 6, "#a89462")
    rect(image, 14, 14, 4, 4, "#d6c38e")
    for x in range(N):
        put(image, x, 0, "#efe2b8")
        put(image, x, N - 1, "#b39d62")
    return image


@tile("cut_sandstone")
def cut_sandstone():
    image = sandstone_base(rng("cut_sandstone"))
    for y in (0, 15, 16, N - 1):
        for x in range(N):
            put(image, x, y, "#bfa96e" if y in (15, N - 1) else "#efe2b8")
    for x in (0, 15, 16, N - 1):
        for y in range(N):
            put(image, x, y, "#c9b67c")
    return image


def bed(color):
    hexc = COLORS[color]

    def top():
        r = rng("bed-" + color)
        image = fill(canvas(), fbm(r, ((8, 1.0), (16, 0.6))), shades(hexc, 0.1))
        edge = shades(hexc, 0.3)[0]
        light = shades(hexc, 0.3)[3]
        for i in range(N):
            put(image, 0, i, edge)
            put(image, N - 1, i, edge)
            put(image, i, N - 1, edge)
        for y in range(14, N - 2, 5):
            for x in range(2, N - 2):
                put(image, x, y, edge)
        rect(image, 3, 2, 26, 9, "#e8ecec")
        for x in range(3, 29):
            put(image, x, 2, "#ffffff")
            put(image, x, 10, "#b8bcc0")
        for y in range(2, 11):
            put(image, 3, y, "#b8bcc0")
            put(image, 28, y, "#b8bcc0")
        return image

    def side():
        image = planks("oak_planks", OAK_WOOD, "#7a5c30", "#8f6f3d")
        r = rng("bedside-" + color)
        for y in range(0, 17):
            for x in range(N):
                image[y, x, :3] = rgb(shades(hexc, 0.1)[int(r.integers(1, 3))])
        for x in range(N):
            put(image, x, 16, shades(hexc, 0.3)[0])
            put(image, x, 0, shades(hexc, 0.3)[3])
        return image

    TILES[f"bed_top_{color}"] = top
    TILES[f"bed_side_{color}"] = side


for _color in COLORS:
    bed(_color)


def railtile(name, rail_color, tie_color, plate=None):
    def build():
        image = transparent()
        for y in range(1, N, 6):
            rect(image, 2, y, 28, 3, tie_color)
            rect(image, 2, y, 28, 1, shades(tie_color, 0.3)[3])
        for x in (6, 23):
            for y in range(N):
                put(image, x, y, rail_color)
                put(image, x + 1, y, shades(rail_color, 0.3)[3])
                put(image, x + 2, y, shades(rail_color, 0.3)[0])
        if plate:
            rect(image, 10, 8, 12, 16, plate)
            rect(image, 11, 9, 10, 14, shades(plate, 0.2)[3])
        return image

    TILES[name] = build


railtile("powered_rail", "#d8b020", "#6e4c22")
railtile("detector_rail", "#b8bcc4", "#6e4c22", "#8a8a8a")


@tile("piston_side")
def piston_side():
    # Без деревянной полосы: поршень лежит в любом направлении, поэтому боковая грань симметрична.
    r = rng("piston")
    image, _ = cells(r, 17, "#4a4a4a", ["#6c6c6c", "#7a7a7a", "#888888", "#969696"], 18)
    frame(image, "#4a4a4a", "#9a9a9a")
    return image


@tile("piston_top")
def piston_top():
    image = planks("oak_planks", OAK_WOOD, "#7a5c30", "#8f6f3d")
    frame(image, "#5e4020", "#9a7038")
    return image


@tile("sticky_piston_top")
def sticky_piston_top():
    r = rng("slime")
    image = fill(canvas(), fbm(r, ((4, 1.0), (8, 0.5))), ["#5ea83a", "#6ab845", "#78c850", "#88d860"])
    frame(image, "#3a7a22", "#a8f080")
    for x in range(6, 26, 4):
        for y in range(6, 26, 4):
            put(image, x, y, "#a8f080")
    return image


@tile("piston_bottom")
def piston_bottom():
    image, _ = cells(rng("pistonb"), 17, "#4a4a4a", ["#6c6c6c", "#7a7a7a", "#888888", "#969696"], 18)
    return image


LETTERS = {"T": ["111", "010", "010", "010", "010"], "N": ["101", "111", "111", "111", "101"]}


@tile("tnt_side")
def tnt_side():
    r = rng("tnt")
    image = fill(canvas(), fbm(r, ((8, 1.0), (16, 0.5))), ["#b8281e", "#c4301f", "#d03a24", "#dc4a2e"])
    for y in range(9, 23):
        for x in range(N):
            image[y, x, :3] = rgb("#e8e0d0" if y not in (9, 22) else "#b8b0a0")
    for index, ch in enumerate("TNT"):
        for row, bits in enumerate(LETTERS[ch]):
            for col, bit in enumerate(bits):
                if bit == "1":
                    rect(image, 5 + index * 8 + col * 2, 11 + row * 2, 2, 2, "#2a2a2a")
    for y in (0, N - 1):
        for x in range(N):
            put(image, x, y, "#8a1c14")
    return image


@tile("tnt_top")
def tnt_top():
    r = rng("tnt-top")
    image = fill(canvas(), fbm(r, ((4, 1.0), (8, 0.5))), ["#8a8a8a", "#969696", "#a2a2a2", "#aeaeae"])
    frame(image, "#5a5a5a", "#c8c8c8")
    rect(image, 14, 14, 4, 4, "#2a2a2a")
    put(image, 15, 12, "#d8a020")
    put(image, 16, 11, "#f0d040")
    return image


@tile("chiseled_bookshelf_front")
def chiseled_bookshelf_front():
    r = rng("cbs")
    image = planks("oak_planks", OAK_WOOD, "#7a5c30", "#8f6f3d")
    frame(image, "#5e4020", "#9a7038")
    covers = ["#a83232", "#2f6b9c", "#3d8a3d", "#b98a2c", "#6b3d8a", "#8a5a2c"]
    for row in range(2):
        for col in range(3):
            x0, y0 = 3 + col * 9, 3 + row * 14
            rect(image, x0, y0, 8, 12, "#2e2012")
            if r.random() < 0.8:
                for i in range(int(r.integers(2, 4))):
                    rect(image, x0 + 1 + i * 2, y0 + 3, 2, 8, covers[int(r.integers(0, len(covers)))])
    return image


@tile("chiseled_bookshelf_side")
def chiseled_bookshelf_side():
    image = planks("oak_planks", OAK_WOOD, "#7a5c30", "#8f6f3d")
    frame(image, "#5e4020", "#9a7038")
    return image


def tuft(name, palette, count, low, high, wave=0.0, seed_name=None):
    def build():
        r = rng(seed_name or name)
        image = transparent()
        for blade in range(count):
            x = 2 + blade * (N - 4) / max(count, 1) + r.uniform(-1, 1)
            height = int(r.integers(low, high))
            lean = r.uniform(-0.3, 0.3)
            for step in range(height):
                px = int(x + lean * step + wave * math.sin(step * 0.45 + blade))
                put(image, px, N - 1 - step, palette[min(len(palette) - 1, step * len(palette) // height)])
                if step < height * 0.55:
                    put(image, px + 1, N - 1 - step, palette[0])
        return image

    TILES[name] = build


tuft("grass", ["#356b24", "#4c8c30", "#6aad42", "#8fd05a"], 9, 6, 15)
tuft("seagrass", ["#1f5a3a", "#2a7048", "#369058", "#4ab070"], 7, 12, 26, 1.8)
tuft("tall_seagrass", ["#1f5a3a", "#2a7048", "#369058", "#4ab070"], 7, 22, 31, 2.4)


@tile("fern")
def fern():
    r = rng("fern")
    image = transparent()
    for stem, (lean, length) in enumerate(((-0.55, 22), (0.0, 28), (0.55, 22), (-0.25, 18), (0.3, 19))):
        for step in range(length):
            x = 15.5 + lean * step * 0.7
            y = N - 1 - step
            put(image, int(x), y, "#2f6a26")
            if step > 4 and step % 2 == 0:
                for side in (-1, 1):
                    for k in range(1, 4):
                        put(image, int(x + side * k), y - k // 2, "#4c9a3a" if k < 3 else "#3a7a2c")
    return image


@tile("dead_bush")
def dead_bush():
    r = rng("dead_bush")
    image = transparent()
    def branch(x, y, angle, length):
        for step in range(length):
            put(image, int(x), int(y), "#7a5a32" if step % 3 else "#5e4422")
            x += math.cos(angle) * 1.0
            y -= math.sin(angle) * 1.0
            if step in (length // 3, length * 2 // 3) and length > 5:
                branch(x, y, angle + r.uniform(-0.8, 0.8), length // 2)
    for angle in (1.2, 1.57, 1.95, 0.9, 2.2):
        branch(15.5, N - 2, angle, int(r.integers(10, 17)))
    return image


@tile("cobweb")
def cobweb():
    image = transparent()
    for angle in range(0, 360, 30):
        dx, dy = math.cos(math.radians(angle)), math.sin(math.radians(angle))
        for t in range(0, 17):
            put(image, int(15.5 + dx * t), int(15.5 + dy * t), "#e8ecec")
    for radius in (4, 8, 12, 15):
        for angle in range(0, 360, 5):
            put(image, int(15.5 + math.cos(math.radians(angle)) * radius), int(15.5 + math.sin(math.radians(angle)) * radius), "#d0d4d4")
    return image


def blossom(name, petals, center, shape="star"):
    plant(name, petals, center)


plant("blue_orchid", ("#8ae8ff", "#2ab4e8", "#14709a"), "#ffffff")
plant("azure_bluet", ("#ffffff", "#d8e0f8", "#98a8d8"), "#f8e060")
plant("wither_rose", ("#4a4a50", "#26262a", "#101012"), "#0a0a0a")
plant("torchflower", ("#ffc070", "#f08020", "#c04a10"), "#ffe080")


def tulip(name, color):
    def build():
        image = transparent()
        for y in range(13, N):
            put(image, 15, y, "#356b24")
            put(image, 16, y, "#4c8c30")
        for side, y0 in ((-1, 22), (1, 25)):
            for step in range(6):
                put(image, 15 + side * (step + 1), y0 - step // 2, "#4c8c30")
        light, mid, dark = shades(color, 0.25)[3], color, shades(color, 0.25)[0]
        for y in range(4, 15):
            half = 4 - abs(y - 10) // 2 if y > 7 else 2 + (y - 4)
            for x in range(int(15.5 - half), int(16.5 + half)):
                put(image, x, y, light if x < 14 else mid if x < 17 else dark)
        for x in (12, 15, 18):
            put(image, x, 4, mid)
            put(image, x, 3, mid)
        return image

    TILES[name] = build


tulip("red_tulip", "#d83020")
tulip("orange_tulip", "#f08a20")
tulip("white_tulip", "#f0f0f0")
tulip("pink_tulip", "#f090b0")


@tile("allium")
def allium():
    r = rng("allium")
    image = transparent()
    for y in range(13, N):
        put(image, 15, y, "#356b24")
        put(image, 16, y, "#4c8c30")
    for y in range(1, 16):
        for x in range(8, 24):
            d = math.hypot(x - 15.5, y - 7.5)
            if d < 7.2 and r.random() < 0.85:
                put(image, x, y, ["#b86ad8", "#a050c8", "#d890f0", "#8a3ab0"][int(r.integers(0, 4))])
    return image


@tile("lily_of_the_valley")
def lily_of_the_valley():
    image = transparent()
    for y in range(5, N):
        put(image, 15, y, "#356b24")
        put(image, 16, y, "#4c8c30")
    for side, y0 in ((-1, 20), (1, 24)):
        for step in range(8):
            put(image, 15 + side * (step + 1), y0 - step // 2, "#4c8c30")
            put(image, 15 + side * (step + 1), y0 - step // 2 + 1, "#356b24")
    for k, y in enumerate(range(3, 14, 3)):
        x = 17 + (k % 2) * 2
        for dx, dy in ((0, 0), (1, 0), (0, 1), (1, 1), (0, 2)):
            put(image, x + dx, y + dy, "#f4f4f4" if dx + dy < 2 else "#c8c8d0")
    return image


def mushroom(name, cap, spot):
    def build():
        r = rng(name)
        image = transparent()
        for y in range(18, N):
            for x in (14, 15, 16, 17):
                put(image, x, y, "#d8d0b8" if x < 16 else "#b8b098")
        for y in range(8, 20):
            for x in range(6, 26):
                d = math.hypot((x - 15.5) / 10, (y - 19) / 11)
                if d < 1 and y < 19:
                    put(image, x, y, shades(cap, 0.2)[3] if y < 12 else cap if y < 16 else shades(cap, 0.2)[0])
        if spot:
            for x, y in ((11, 13), (16, 11), (20, 15), (14, 16), (22, 17)):
                rect(image, x, y, 2, 2, spot)
        return image

    TILES[name] = build


mushroom("brown_mushroom", "#8a5c3a", None)
mushroom("red_mushroom", "#d02828", "#f4f0e8")


@tile("torch")
def torch():
    image = transparent()
    for y in range(13, N):
        for x in (14, 15, 16, 17):
            put(image, x, y, "#9a6a34" if x < 16 else "#7a5224")
    for y in range(13, 17):
        for x in (14, 15, 16, 17):
            put(image, x, y, "#3a2a1a" if x < 16 else "#2a1c10")
    for y in range(4, 13):
        half = 3 if 6 < y < 11 else 2
        for x in range(int(15.5 - half), int(16.5 + half)):
            put(image, x, y, "#fff3a0" if y > 7 and abs(x - 15.5) < 2 else "#ffb02e" if y > 5 else "#e2501a")
    return image


def fire(name, palette):
    def build():
        r = rng(name)
        image = transparent()
        for x in range(2, N - 2):
            height = int(8 + 14 * abs(math.sin(x * 0.55 + 1.2)) + r.integers(0, 6))
            for step in range(height):
                y = N - 1 - step
                t = step / max(height, 1)
                put(image, x, y, palette[0] if t < 0.3 else palette[1] if t < 0.62 else palette[2])
        return image

    TILES[name] = build


fire("fire", ["#fff3a0", "#ffb02e", "#e2501a"])
fire("soul_fire", ["#d8ffff", "#4ac8f0", "#1a6ac8"])


# ------------------------------------------------------- functional blocks


def barrel(face):
    image = planks("barrel", ["#8a5f30", "#966a38", "#a27540", "#ae8048"], "#5e3e1a", "#6e4a22")
    if face == "top":
        frame(image, "#3e3e44", "#6a6a72")
        rect(image, 6, 6, 20, 20, "#5e3e1a")
        rect(image, 7, 7, 18, 18, "#966a38")
        rect(image, 14, 14, 4, 4, "#3e3e44")
    else:
        image = np.rot90(image).copy()
        for y in (5, 6, 25, 26):
            rect(image, 0, y, N, 1, "#3e3e44" if y in (6, 26) else "#6a6a72")
    return image


TILES["barrel_side"] = lambda: barrel("side")
TILES["barrel_top"] = lambda: barrel("top")


def smoker(face, lit=False):
    image = furnace_shell("furnace_side")
    rect(image, 0, 0, N, 5, "#7a5630")
    rect(image, 0, 5, N, 1, "#4a300e")
    rect(image, 0, N - 5, N, 4, "#7a5630")
    rect(image, 0, N - 6, N, 1, "#4a300e")
    if face == "front":
        rect(image, 6, 9, 20, 14, "#3b3b3b")
        rect(image, 8, 11, 16, 10, "#120d0a")
        for x in range(9, 24, 3):
            rect(image, x, 11, 1, 10, "#4a4a4a")
        if lit:
            for x in range(8, 24):
                if (x - 9) % 3:
                    height = 3 + (x * 7) % 4
                    for step in range(height):
                        put(image, x, 20 - step, "#fff3a0" if step < 1 else "#ffb02e" if step < 3 else "#e2501a")
    return image


TILES["smoker_side"] = lambda: smoker("side")
TILES["smoker_front"] = lambda: smoker("front")
TILES["smoker_front_on"] = lambda: smoker("front", True)


def blast_furnace(face, lit=False):
    r = rng("blast_furnace")
    image = fill(canvas(), fbm(r, ((4, 1.0), (16, 0.4))), ["#55585e", "#606369", "#6b6e75", "#767981"])
    frame(image, "#2e3034", "#8e9199")
    for x, y in ((4, 4), (27, 4), (4, 27), (27, 27)):
        put(image, x, y, "#c6c9d0")
        put(image, x + 1, y + 1, "#2e3034")
    rect(image, 2, 15, N - 4, 2, "#2e3034")
    if face == "front":
        rect(image, 8, 5, 16, 8, "#2e3034")
        for x in range(10, 23, 3):
            rect(image, x, 7, 2, 4, "#e2501a" if lit else "#101114")
        rect(image, 7, 18, 18, 10, "#2e3034")
        rect(image, 9, 20, 14, 7, "#101114")
        if lit:
            for x in range(9, 23):
                height = 2 + (x * 5) % 5
                for step in range(height):
                    put(image, x, 26 - step, "#fff3a0" if step < 2 else "#ffb02e" if step < 4 else "#e2501a")
    return image


TILES["blast_furnace_side"] = lambda: blast_furnace("side")
TILES["blast_furnace_front"] = lambda: blast_furnace("front")
TILES["blast_furnace_front_on"] = lambda: blast_furnace("front", True)


@tile("stonecutter_side")
def stonecutter_side():
    image = brick_pattern(rng("stonecutter"), STONE, "#555555")
    rect(image, 0, 0, N, 4, "#9a9a9a")
    rect(image, 0, 4, N, 1, "#4a4a4a")
    return image


@tile("stonecutter_top")
def stonecutter_top():
    r = rng("stonecutter_top")
    image = fill(canvas(), fbm(r, ((4, 1.0), (16, 0.3))), ["#8a8a8a", "#939393", "#9c9c9c", "#a5a5a5"])
    frame(image, "#5a5a5a", "#b8b8b8")
    for y in range(N):
        for x in range(N):
            d = math.hypot(x - 15.5, y - 15.5)
            if d < 10:
                image[y, x, :3] = rgb("#d0d4da" if d < 8 else "#7d828a")
                angle = math.atan2(y - 15.5, x - 15.5)
                if 8 <= d and int(angle * 6 / math.pi) % 2 == 0:
                    image[y, x, :3] = rgb("#eef1f5")
            if d < 2.5:
                image[y, x, :3] = rgb("#4a4a4a")
    return image


def vault(face):
    r = rng("vault")
    image = fill(canvas(), fbm(r, ((4, 1.0), (8, 0.6))), ["#1c2a3a", "#223246", "#283a52", "#2e425e"])
    frame(image, "#0e1620", "#4fd8cc")
    if face != "top":
        rect(image, 2, 11, N - 4, 2, "#0e1620")
    if face == "front":
        rect(image, 12, 8, 8, 9, "#36bdb3")
        rect(image, 12, 8, 8, 1, "#dcfffa")
        rect(image, 15, 11, 2, 4, "#0e1620")
    if face == "top":
        rect(image, 12, 12, 8, 8, "#36bdb3")
        rect(image, 14, 14, 4, 4, "#dcfffa")
    return image


TILES["vault_front"] = lambda: vault("front")
TILES["vault_side"] = lambda: vault("side")
TILES["vault_top"] = lambda: vault("top")


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


# ------------------------------------------------------------ part 2 tiles


def load_tile(name):
    if name in TILES:
        return TILES[name]()
    path = ROOT / "assets/textures/pixel/32x32" / f"{name}.png"
    return np.array(Image.open(path).convert("RGBA"), dtype=float)


def border(image, color, width=1):
    for i in range(width):
        for k in range(N):
            for x, y in ((k, i), (k, N - 1 - i), (i, k), (N - 1 - i, k)):
                put(image, x, y, color)
    return image


def noise_tile(name, palette, scale=None, specks=None, count=40):
    def build():
        r = rng(name)
        image = fill(canvas(), fbm(r, scale) if scale else fbm(r), palette)
        if specks:
            speckle(image, r, specks, count)
        return image

    TILES[name] = build


noise_tile("netherrack", ["#6a2828", "#742e2e", "#7e3434", "#883a3a", "#5a2020"], specks=["#a04a4a", "#4a1818"], count=60)
noise_tile("soul_sand", ["#4a3a30", "#54433a", "#5e4c42", "#68554a"], ((6, 1.0), (16, 0.6)), ["#2e2018", "#7a6658"], 50)
noise_tile("soul_soil", ["#3e3028", "#483a30", "#524238", "#5c4a40"], specks=["#2a1e18", "#6a5848"], count=40)


@tile("glowstone")
def glowstone():
    r = rng("glowstone")
    image = fill(canvas(), fbm(r, ((4, 1.0), (8, 0.9), (16, 0.6))), ["#a8742a", "#c8902e", "#e0aa3c", "#f4c85a", "#fff0a0"])
    speckle(image, r, ["#fff6c8", "#8a5a1e"], 70)
    return image


@tile("spawner")
def spawner():
    image = stone_base(rng("spawner"), DEEP, cracks=3, crack_color="#14141a")
    for k in range(4, 28):
        for a, b in ((4, k), (27, k), (k, 4), (k, 27)):
            put(image, a, b, "#1c2433")
    for k in range(5, 27, 4):
        for a, b in ((k, k), (k, 31 - k)):
            put(image, a, b, "#2b3a55")
            put(image, a + 1, b, "#2b3a55")
    rect(image, 11, 11, 10, 10, "#0b0b12")
    rect(image, 13, 13, 6, 6, "#3a1a5a")
    rect(image, 15, 15, 2, 2, "#b070ff")
    return image


def redstone_ore(name, host_tile, deep):
    def build():
        r = rng(name)
        image = load_tile(host_tile)
        o = rng(name + "_ore")
        for cx, cy in ((7, 7), (22, 8), (13, 17), (5, 24), (24, 24)):
            cx += int(o.integers(-2, 3))
            cy += int(o.integers(-2, 3))
            for y in range(cy - 3, cy + 4):
                for x in range(cx - 3, cx + 4):
                    d = math.hypot(x - cx, y - cy) + o.uniform(-0.5, 0.5)
                    if d < 2.6:
                        put(image, x, y, "#ff3a3a" if (x + y) % 3 else "#c81818")
                    elif d < 3.4 and x + y > cx + cy:
                        put(image, x, y, "#6a0c0c")
        return image

    TILES[name] = build


redstone_ore("redstone_ore", "stone", False)
redstone_ore("deepslate_redstone_ore", "deepslate", True)


def _deep_diamond():
    r = rng("deepslate")
    image = stone_base(r, DEEP, crack_color="#25252c")
    speckle(image, r, ["#5c5c6c"], 40)
    o = rng("deepslate_diamond_ore")
    for cx, cy in ((7, 7), (22, 6), (14, 16), (5, 23), (24, 24), (16, 27)):
        cx += int(o.integers(-2, 3))
        cy += int(o.integers(-2, 3))
        radius = float(o.uniform(2.0, 3.4))
        for y in range(cy - 4, cy + 5):
            for x in range(cx - 4, cx + 5):
                d = math.hypot(x - cx, (y - cy) * 1.2) + o.uniform(-0.6, 0.6)
                if d < radius:
                    put(image, x, y, "#e0fffa" if (x - cx) + (y - cy) < -1 else "#3ad8d0")
                elif d < radius + 1.0 and (x - cx) + (y - cy) > 0:
                    put(image, x, y, "#1a6a68")
    return image


TILES["deepslate_diamond_ore"] = _deep_diamond


@tile("chiseled_stone_bricks")
def chiseled_stone_bricks():
    image = load_tile("stone_bricks")
    border(image, "#4a4a4a", 2)
    rect(image, 8, 8, 16, 16, "#6e6e6e")
    rect(image, 10, 10, 12, 12, "#8a8a8a")
    rect(image, 13, 13, 6, 6, "#5a5a5a")
    return image


@tile("farmland_top")
def farmland_top():
    r = rng("farmland_top")
    image = fill(canvas(), fbm(r), ["#4a2e1a", "#563620", "#623f26"])
    for y in range(2, N, 6):
        rect(image, 0, y, N, 2, "#3a2314")
    speckle(image, r, ["#6b4a2e"], 24)
    return image


@tile("cactus_side")
def cactus_side():
    r = rng("cactus_side")
    image = fill(canvas(), fbm(r), ["#1f6a2a", "#267a30", "#2e8a38"])
    for x in (3, 10, 17, 24):
        rect(image, x, 0, 2, N, "#175a22")
        rect(image, x + 1, 0, 1, N, "#3aa046")
    for y in range(2, N, 7):
        for x in (6, 13, 20, 27):
            put(image, x, y, "#e8e0b8")
    return image


@tile("cactus_top")
def cactus_top():
    image = canvas("#267a30")
    border(image, "#175a22", 2)
    rect(image, 6, 6, 20, 20, "#2e8a38")
    rect(image, 12, 12, 8, 8, "#3aa046")
    return image


@tile("jukebox_side")
def jukebox_side():
    image = planks("jukebox_side", ["#6a4a2a", "#74522e", "#7e5a34", "#88623a"], "#3e2a14", "#52381c")
    rect(image, 10, 12, 12, 10, "#1a1a1a")
    rect(image, 12, 14, 8, 6, "#2b2b2b")
    return image


@tile("jukebox_top")
def jukebox_top():
    image = planks("jukebox_top", ["#6a4a2a", "#74522e", "#7e5a34", "#88623a"], "#3e2a14", "#52381c")
    rect(image, 8, 8, 16, 16, "#1a1a1a")
    rect(image, 14, 14, 4, 4, "#8a8a8a")
    return image


def pumpkin_face(name, lit):
    def build():
        image = load_tile("pumpkin_side")
        eye = "#ffd23a" if lit else "#2a1608"
        mouth = "#ff9c1a" if lit else "#2a1608"
        rect(image, 7, 9, 6, 6, eye)
        rect(image, 19, 9, 6, 6, eye)
        rect(image, 9, 20, 14, 4, mouth)
        rect(image, 12, 24, 8, 2, mouth)
        if lit:
            rect(image, 9, 11, 2, 2, "#fff6b0")
            rect(image, 21, 11, 2, 2, "#fff6b0")
        return image

    TILES[name] = build


pumpkin_face("carved_pumpkin_front", False)
pumpkin_face("jack_o_lantern_front", True)


@tile("cake_top")
def cake_top():
    image = canvas("#f4ecd8")
    border(image, "#d8c8a8", 2)
    for x, y in ((8, 8), (22, 12), (14, 22), (24, 24)):
        rect(image, x, y, 4, 4, "#d8242c")
    return image


@tile("cake_side")
def cake_side():
    image = canvas("#7a4a26")
    rect(image, 0, 0, N, 8, "#f4ecd8")
    rect(image, 0, 8, N, 4, "#c8242c")
    rect(image, 0, 12, N, 6, "#fff4e0")
    rect(image, 0, 18, N, 14, "#7a4a26")
    return image


@tile("repeater_top")
def repeater_top():
    image = canvas("#9a9a98")
    border(image, "#6a6a68", 2)
    rect(image, 6, 14, 20, 4, "#5a2a2a")
    rect(image, 14, 6, 4, 6, "#c81818")
    rect(image, 14, 20, 4, 6, "#7a1010")
    return image


def mushroom_block(name, cap, spot):
    def build():
        r = rng(name)
        image = fill(canvas(), fbm(r), cap)
        if spot:
            for cx, cy in ((7, 7), (21, 9), (12, 21), (26, 24)):
                for y in range(cy - 3, cy + 4):
                    for x in range(cx - 3, cx + 4):
                        if math.hypot(x - cx, y - cy) < 3.2:
                            put(image, x, y, spot)
        return image

    TILES[name] = build


mushroom_block("brown_mushroom_block", ["#8a6444", "#946c4a", "#9e7450"], "#c8a880")
mushroom_block("red_mushroom_block", ["#b02a22", "#bc342a", "#c83e32"], "#f0ece0")
mushroom_block("mushroom_stem", ["#d8d0c0", "#e2dac8", "#ece4d2"], None)


@tile("crimson_planks")
def crimson_planks():
    return planks("crimson_planks", ["#6a2840", "#742e48", "#7e3450", "#883a58"], "#4a1a2c", "#5a2036")


@tile("warped_planks")
def warped_planks():
    return planks("warped_planks", ["#1e6a6a", "#247474", "#2a7e7e", "#308888"], "#124646", "#165454")


def door_tile(name, base, upper, iron):
    def build():
        if iron:
            image = load_tile("iron_block")
            border(image, "#6a6a6a", 1)
            rect(image, 4, 4, 24, 10, "#a8a8a8") if not upper else None
            for y in ((10, 20) if not upper else (24,)):
                rect(image, 4, y, 24, 2, "#7a7a7a")
            rect(image, 24, 15 if not upper else 6, 3, 3, "#303030")
        else:
            image = load_tile(base)
            border(image, "#4a3418", 1)
            if upper:
                for k in range(3):
                    rect(image, 5 + k * 8, 6, 6, 5, "#8ab8c8" if k != 3 else "#a8d0e0")
                rect(image, 3, 4, 26, 1, "#4a3418")
                rect(image, 3, 12, 26, 1, "#4a3418")
            else:
                rect(image, 4, 6, 24, 1, "#4a3418")
                rect(image, 4, 25, 24, 1, "#4a3418")
                rect(image, 24, 15, 3, 3, "#303030")
        return image

    TILES[name] = build


door_tile("oak_door_top", "oak_planks", True, False)
door_tile("oak_door_bottom", "oak_planks", False, False)
door_tile("iron_door_top", "iron_block", True, True)
door_tile("iron_door_bottom", "iron_block", False, True)

for _wood in ("oak", "spruce", "birch", "jungle", "acacia", "cherry", "dark_oak", "mangrove", "bamboo"):
    def _trapdoor(wood=_wood):
        def build():
            image = load_tile(f"{wood}_planks")
            border(image, "#2e2210", 2)
            for k in (8, 16, 24):
                rect(image, 2, k, 28, 2, "#2e2210")
            rect(image, 2, 2, 28, 4, "#2e2210")
            rect(image, 13, 25, 6, 3, "#3a3a3a")
            return image

        return build

    TILES[f"{_wood}_trapdoor"] = _trapdoor()


@tile("lever")
def lever():
    image = load_tile("cobblestone")
    rect(image, 14, 2, 4, 28, "#6a4a22")
    rect(image, 15, 2, 2, 28, "#8a6a34")
    return image


# ---- cutout tiles (прозрачные текселы)


@tile("wheat")
def wheat():
    image = transparent()
    r = rng("wheat")
    for x in range(2, 31, 4):
        height = int(14 + r.integers(0, 8))
        for y in range(N - height, N):
            put(image, x, y, "#7a9a2a")
            put(image, x + 1, y, "#8aaa34")
        for y in range(N - height, N - height + 7):
            put(image, x, y, "#d8b648")
            put(image, x + 1, y, "#e8c85a")
            put(image, x - 1, y + 2, "#c8a63a")
    return image


@tile("sugar_cane")
def sugar_cane():
    image = transparent()
    for x in (4, 12, 21, 27):
        for y in range(0, N):
            put(image, x, y, "#8aca58")
            put(image, x + 1, y, "#6ea840")
            if y % 8 == 0:
                put(image, x, y, "#4a7a2a")
                put(image, x + 1, y, "#4a7a2a")
        for k in range(1, 5):
            put(image, x - k, 6 + k, "#7aba48")
            put(image, x + 1 + k, 4 + k, "#7aba48")
    return image


@tile("redstone_wire")
def redstone_wire():
    image = transparent()
    for k in range(N):
        for w in (15, 16):
            put(image, w, k, "#a01010")
            put(image, k, w, "#a01010")
    rect(image, 13, 13, 6, 6, "#d01818")
    rect(image, 15, 15, 2, 2, "#ff5a4a")
    return image


def lit_torch(name, flame, glow):
    def build():
        image = transparent()
        for y in range(11, N):
            for x in (14, 15, 16, 17):
                put(image, x, y, "#8a5a2a" if x < 16 else "#6a4420")
        for y in range(6, 12):
            for x in range(13, 19):
                put(image, x, y, flame)
        rect(image, 14, 7, 4, 3, glow)
        return image

    TILES[name] = build


lit_torch("redstone_torch", "#c81818", "#ff6a5a")
lit_torch("soul_torch", "#2aa8c8", "#a8f0ff")


@tile("ladder")
def ladder():
    image = transparent()
    rect(image, 5, 0, 3, N, "#8a6a34")
    rect(image, 24, 0, 3, N, "#8a6a34")
    for y in range(3, N, 8):
        rect(image, 5, y, 22, 3, "#a07c40")
        rect(image, 5, y + 2, 22, 1, "#6a4e22")
    return image


@tile("rail")
def rail():
    image = transparent()
    for y in range(N):
        rect(image, 7, y, 3, 1, "#a8a8a8")
        rect(image, 22, y, 3, 1, "#a8a8a8")
    for y in range(2, N, 8):
        rect(image, 5, y, 22, 3, "#7a5a2a")
        rect(image, 5, y, 22, 1, "#946e34")
    return image


@tile("nether_portal")
def nether_portal():
    image = transparent()
    r = rng("nether_portal")
    field = fbm(r, ((4, 1.0), (8, 0.7)))
    for y in range(N):
        for x in range(N):
            v = field[y, x]
            if (x + y) % 2 == 0 or v > 0.55:
                image[y, x, :3] = ramp(np.array([[v]]), ["#4a0f8a", "#6a1ab8", "#9a3ae0", "#d08aff"])[0, 0]
                image[y, x, 3] = 200
    return image


@tile("iron_bars")
def iron_bars():
    image = transparent()
    for x in range(N):
        if x % 16 in (7, 8):
            rect(image, x, 0, 1, N, "#b0b0b4" if x % 16 == 7 else "#707076")
    rect(image, 0, 14, N, 2, "#8a8a90")
    return image


@tile("chain")
def chain():
    image = transparent()
    for y in range(0, N, 8):
        rect(image, 12, y, 8, 2, "#7a7a82")
        rect(image, 12, y, 2, 7, "#7a7a82")
        rect(image, 18, y, 2, 7, "#7a7a82")
        rect(image, 12, y + 6, 8, 2, "#5a5a62")
        rect(image, 14, y + 2, 4, 4, "#9a9aa2")
    return image


@tile("glass_pane")
def glass_pane():
    image = transparent()
    for x in range(N):
        if x % 16 in (7, 8):
            rect(image, x, 0, 1, N, "#cfeaf2")
    rect(image, 0, 0, N, 1, "#cfeaf2")
    rect(image, 0, N - 1, N, 1, "#cfeaf2")
    for k in range(4):
        put(image, 4 + k, 20 - k, "#ffffff", 200)
    return image



# ------------------------------------------------------------ part 3 tiles


noise_tile("end_stone", ["#d9d99a", "#e2e2a6", "#eaeab4", "#c8c88a"], specks=["#a8a870", "#f4f4c8"], count=50)
noise_tile("quartz_ore", ["#7a4040", "#844848", "#8e5050", "#984a4a"], specks=["#4a2020", "#c8b0a0"], count=30)
noise_tile("powder_snow", ["#f4f8fa", "#e8f0f4", "#dde8ee"], specks=["#ffffff", "#c8d8e0"], count=30)
noise_tile("redstone_block", ["#a81410", "#b81c14", "#c82418", "#981008"], specks=["#e84030", "#780800"], count=40)
noise_tile("command_block", ["#a8663a", "#b27040", "#bc7a46"], specks=["#7a4422", "#d49060"], count=40)


@tile("mycelium_top")
def mycelium_top():
    r = rng("mycelium_top")
    image = fill(canvas(), fbm(r), ["#6e5e78", "#7a6a84", "#867690", "#92829c"])
    speckle(image, r, ["#b8a8c4", "#5a4a64", "#e0d8e8"], 60)
    return image


@tile("mycelium_side")
def mycelium_side():
    image = load_tile("dirt")
    top = mycelium_top()
    for y in range(0, 7):
        for x in range(N):
            if y < 5 or (x * 7 + y * 3) % 5 < 2:
                image[y, x, :] = top[y, x, :]
    return image


@tile("nether_bricks")
def nether_bricks():
    image = canvas("#2e1618")
    for row in range(4):
        y = row * 8
        rect(image, 0, y + 7, N, 1, "#1a0a0c")
        off = 0 if row % 2 == 0 else 8
        for x in range(off, N + 8, 16):
            rect(image, x % N, y, 1, 7, "#1a0a0c")
    r = rng("nether_bricks")
    speckle(image, r, ["#3e1e20", "#4a2426"], 80)
    return image


@tile("enchanting_table_top")
def enchanting_table_top():
    image = canvas("#7a1830")
    border(image, "#2a0a16", 2)
    rect(image, 8, 8, 16, 16, "#5a1024")
    rect(image, 12, 12, 8, 8, "#40d8c8")
    rect(image, 14, 14, 4, 4, "#b8fff0")
    return image


@tile("enchanting_table_side")
def enchanting_table_side():
    image = canvas("#201830")
    rect(image, 0, 0, N, 5, "#7a1830")
    rect(image, 0, 27, N, 5, "#1a1228")
    for x in (6, 15, 24):
        rect(image, x, 12, 3, 3, "#40d8c8")
    return image


@tile("brewing_stand")
def brewing_stand():
    image = transparent()
    rect(image, 14, 4, 4, 20, "#c8b890")
    rect(image, 2, 26, 28, 4, "#8a8a8a")
    for x in (3, 25):
        rect(image, x, 16, 4, 10, "#d8e8f0", 180)
    rect(image, 14, 2, 4, 4, "#f0a020")
    return image


@tile("cauldron_side")
def cauldron_side():
    image = canvas("#3a3a42")
    border(image, "#22222a", 2)
    rect(image, 0, 0, N, 4, "#505058")
    r = rng("cauldron_side")
    speckle(image, r, ["#2a2a32", "#4a4a52"], 40)
    return image


@tile("cauldron_top")
def cauldron_top():
    image = canvas("#22222a")
    border(image, "#4a4a52", 3)
    return image


@tile("end_portal")
def end_portal():
    r = rng("end_portal")
    image = fill(canvas(), fbm(r), ["#050810", "#0a1a2a", "#103a4a", "#1a6a6a"])
    speckle(image, r, ["#e0fff0", "#8affc8"], 40)
    return image


@tile("end_portal_frame_top")
def end_portal_frame_top():
    image = canvas("#4a7a5a")
    border(image, "#d8d898", 2)
    rect(image, 8, 8, 16, 16, "#2a5a3a")
    rect(image, 11, 11, 10, 10, "#0e2a1e")
    return image


@tile("end_portal_frame_side")
def end_portal_frame_side():
    image = load_tile("end_stone")
    rect(image, 0, 0, N, 8, "#4a7a5a")
    rect(image, 0, 8, N, 1, "#2a5a3a")
    return image


@tile("dragon_egg")
def dragon_egg():
    r = rng("dragon_egg")
    image = fill(canvas(), fbm(r), ["#0c0614", "#140a22", "#1c0e30"])
    speckle(image, r, ["#b040e0", "#e070ff", "#5a1a8a"], 90)
    return image


@tile("redstone_lamp")
def redstone_lamp():
    image = canvas("#a0602a")
    for y in range(0, N, 8):
        rect(image, 0, y, N, 1, "#6a3a14")
    for x in range(0, N, 8):
        rect(image, x, 0, 1, N, "#6a3a14")
    rect(image, 4, 4, 24, 24, "#e8b050")
    rect(image, 8, 8, 16, 16, "#f8d880")
    rect(image, 12, 12, 8, 8, "#fff4c0")
    return image


@tile("deepslate_emerald_ore")
def deepslate_emerald_ore():
    r = rng("deepslate")
    image = stone_base(r, DEEP, crack_color="#25252c")
    speckle(image, r, ["#5c5c6c"], 40)
    o = rng("deepslate_emerald_ore")
    for cx, cy in ((8, 8), (22, 12), (13, 23), (25, 25)):
        for y in range(cy - 3, cy + 4):
            for x in range(cx - 3, cx + 4):
                d = math.hypot(x - cx, y - cy) + o.uniform(-0.5, 0.5)
                if d < 2.6:
                    put(image, x, y, "#7affa0" if (x - cx) + (y - cy) < 0 else "#20b050")
                elif d < 3.4 and x + y > cx + cy:
                    put(image, x, y, "#0a5a2a")
    return image


@tile("ender_chest")
def ender_chest():
    image = canvas("#143a3a")
    border(image, "#0a1e1e", 2)
    rect(image, 0, 12, N, 3, "#0a1e1e")
    rect(image, 13, 10, 6, 7, "#9affd8")
    rect(image, 14, 12, 4, 3, "#2a5a5a")
    r = rng("ender_chest")
    speckle(image, r, ["#1e5a5a", "#0e2a2a"], 50)
    return image


@tile("tripwire_hook")
def tripwire_hook():
    image = load_tile("oak_planks")
    rect(image, 12, 2, 8, 12, "#4a4a4a")
    rect(image, 14, 14, 4, 14, "#8a6a34")
    return image


@tile("tripwire")
def tripwire():
    image = transparent()
    rect(image, 0, 15, N, 2, "#e8e8e8", 220)
    return image


@tile("beacon")
def beacon():
    image = canvas("#58e8e0")
    border(image, "#1a8a88", 2)
    rect(image, 6, 6, 20, 20, "#1a1a28")
    rect(image, 9, 9, 14, 14, "#6a4a9a")
    rect(image, 13, 13, 6, 6, "#e8f0ff")
    return image


@tile("flower_pot")
def flower_pot():
    image = canvas("#8a4a32")
    border(image, "#5a2c1c", 2)
    for y in range(4, 30, 6):
        rect(image, 2, y, 28, 1, "#6a3820")
    return image


@tile("daylight_detector_top")
def daylight_detector_top():
    image = canvas("#7a6a50")
    border(image, "#4a3e2e", 2)
    rect(image, 4, 4, 24, 24, "#3a4a8a")
    for k in range(6, 26, 5):
        rect(image, k, 6, 2, 20, "#7a8ad0")
    return image


@tile("daylight_detector_side")
def daylight_detector_side():
    image = canvas("#9a8860")
    rect(image, 0, 0, N, 6, "#3a4a8a")
    rect(image, 0, 6, N, 1, "#4a3e2e")
    return image


@tile("anvil")
def anvil():
    r = rng("anvil")
    image = fill(canvas(), fbm(r), ["#2a2a30", "#34343c", "#3e3e46"])
    rect(image, 0, 0, N, 3, "#52525c")
    return image


@tile("hopper")
def hopper():
    image = canvas("#4a4a52")
    border(image, "#2a2a30", 2)
    rect(image, 10, 10, 12, 12, "#1a1a1e")
    return image


@tile("chiseled_quartz_block")
def chiseled_quartz_block():
    image = load_tile("quartz_block")
    border(image, "#c8c0b8", 2)
    rect(image, 8, 14, 16, 4, "#b8b0a8")
    rect(image, 14, 8, 4, 16, "#b8b0a8")
    return image


@tile("comparator_top")
def comparator_top():
    image = canvas("#9a9a98")
    border(image, "#6a6a68", 2)
    rect(image, 6, 20, 5, 6, "#c81818")
    rect(image, 21, 20, 5, 6, "#c81818")
    rect(image, 13, 6, 6, 6, "#e8c8c0")
    return image


def head(name, skin, accent, eyes):
    def build():
        r = rng(name)
        image = fill(canvas(), fbm(r), skin)
        rect(image, 6, 11, 6, 5, eyes)
        rect(image, 20, 11, 6, 5, eyes)
        rect(image, 8, 22, 16, 3, accent)
        return image

    TILES[name] = build


head("skeleton_skull", ["#c8c8c0", "#d4d4cc", "#dcdcd4"], "#6a6a66", "#18181a")
head("wither_skull", ["#2a2a2c", "#343436", "#3c3c3e"], "#0a0a0a", "#c0c0c8")
head("zombie_head", ["#5a8a4a", "#649454", "#6e9e5e"], "#2a4a2a", "#101820")
head("player_head", ["#c89070", "#d49c7a", "#dea884"], "#6a4a38", "#3a5acd")
head("creeper_head", ["#5ab04a", "#66bc56", "#72c862"], "#143a14", "#0a0a0a")
head("dragon_head", ["#1c1c24", "#24242c", "#2c2c34"], "#8a3ae0", "#d070ff")
head("piglin_head", ["#d89a8a", "#e0a496", "#e8aea0"], "#7a3a30", "#2a1a14")


@tile("vine")
def vine():
    image = transparent()
    r = rng("vine")
    for x in range(1, N, 3):
        length = int(8 + r.integers(0, 22))
        for y in range(length):
            put(image, x, y, "#2e7a2a" if y % 3 else "#3a9a34")
            if y % 5 == 2:
                put(image, x + 1, y, "#3a9a34")
    return image


@tile("glow_lichen")
def glow_lichen():
    image = transparent()
    r = rng("glow_lichen")
    for _ in range(90):
        x, y = r.integers(0, N, 2)
        put(image, x, y, "#9ad8b8", 230)
        if r.random() < 0.3:
            put(image, x + 1, y, "#d8fff0", 255)
    return image


@tile("lily_pad")
def lily_pad():
    image = transparent()
    for y in range(N):
        for x in range(N):
            d = math.hypot(x - 15.5, y - 15.5)
            if d < 13 and not (x > 15 and 13 < y < 17):
                put(image, x, y, "#2a7a32" if (x + y) % 5 else "#34903c")
    return image


@tile("nether_wart")
def nether_wart():
    image = transparent()
    r = rng("nether_wart")
    for x in range(3, 29, 6):
        h = int(10 + r.integers(0, 8))
        for y in range(N - h, N):
            rect(image, x, y, 3, 1, "#a8202a" if y % 3 else "#c83038")
    return image


@tile("cocoa")
def cocoa():
    image = transparent()
    for y in range(8, 26):
        w = 5 if y < 20 else 3
        rect(image, 16 - w, y, w * 2, 1, "#8a4a1e" if y % 4 else "#a85a26")
    rect(image, 14, 4, 4, 4, "#3a7a2a")
    return image


def crop(name, stem, head_color):
    def build():
        image = transparent()
        for x in range(2, 30, 7):
            for y in range(12, N):
                rect(image, x, y, 2, 1, stem)
            rect(image, x - 2, 8, 6, 5, head_color)
            rect(image, x - 1, 6, 4, 2, "#4a8a3a")
        return image

    TILES[name] = build


crop("carrots", "#3a8a30", "#e88a1a")
crop("potatoes", "#3a8a30", "#c8a060")


def stem(name, color, attached=None):
    def build():
        image = transparent()
        for y in range(10, N):
            rect(image, 14, y, 3, 1, color)
        if attached:
            for x in range(17, 28):
                rect(image, x, 14 + (x - 17) // 3, 1, 3, color)
            rect(image, 24, 16, 7, 7, attached)
        return image

    TILES[name] = build


stem("pumpkin_stem", "#7a9a3a")
stem("melon_stem", "#6a9a4a")
stem("attached_pumpkin_stem", "#8a8a3a", "#d8841c")
stem("attached_melon_stem", "#7a9a4a", "#5aa032")



# ------------------------------------------------------------ part 4 tiles


@tile("quartz_pillar_side")
def quartz_pillar_side():
    image = load_tile("quartz_block")
    for x in (0, 8, 16, 24):
        rect(image, x, 0, 1, N, "#c8c0b8")
        rect(image, x + 1, 0, 1, N, "#f4eee8")
    return image


@tile("quartz_pillar_top")
def quartz_pillar_top():
    image = load_tile("quartz_block")
    border(image, "#c8c0b8", 2)
    rect(image, 6, 6, 20, 20, "#e8e2da")
    return image


@tile("dropper_front")
def dropper_front():
    image = load_tile("dispenser_front")
    rect(image, 12, 12, 8, 8, "#14141a")
    rect(image, 14, 14, 4, 4, "#8a8a94")
    return image


@tile("slime_block")
def slime_block():
    r = rng("slime_block")
    image = fill(canvas(), fbm(r), ["#5aa83a", "#68b844", "#78c850", "#88d85e"])
    border(image, "#3a7a28", 2)
    rect(image, 8, 8, 16, 16, "#7ed05a")
    speckle(image, r, ["#c8ffa8", "#4a8a30"], 30)
    return image


@tile("iron_trapdoor")
def iron_trapdoor():
    image = load_tile("iron_block")
    border(image, "#5a5a5e", 2)
    for k in (10, 21):
        rect(image, 2, k, 28, 2, "#7a7a7e")
        rect(image, k, 2, 2, 28, "#7a7a7e")
    rect(image, 13, 26, 6, 3, "#303034")
    return image


noise_tile("prismarine", ["#4a9a8a", "#58a898", "#66b6a6", "#42887a"], specks=["#8adac8", "#2e6a5e"], count=50)
noise_tile("dark_prismarine", ["#2a5a4e", "#336656", "#3c7260", "#244e44"], specks=["#4a8a74", "#16382e"], count=40)
noise_tile("packed_ice", ["#7aa8e8", "#88b4f0", "#98c0f4", "#6a98d8"], specks=["#d8ecff", "#5a88c8"], count=40)


@tile("prismarine_bricks")
def prismarine_bricks():
    image = canvas("#4aa090")
    for row in range(4):
        y = row * 8
        rect(image, 0, y + 7, N, 1, "#2a6a5e")
        off = 0 if row % 2 == 0 else 8
        for x in range(off, N + 8, 16):
            rect(image, x % N, y, 1, 7, "#2a6a5e")
    r = rng("prismarine_bricks")
    speckle(image, r, ["#68c0ae", "#3a8a7a"], 80)
    return image


@tile("sea_lantern")
def sea_lantern():
    image = canvas("#cfe8e4")
    for k in (0, 16):
        rect(image, k, 0, 2, N, "#6aa8a8")
        rect(image, 0, k, N, 2, "#6aa8a8")
    rect(image, 6, 6, 4, 4, "#fffff0")
    rect(image, 22, 22, 4, 4, "#fffff0")
    rect(image, 22, 6, 4, 4, "#e0fffa")
    rect(image, 6, 22, 4, 4, "#e0fffa")
    return image


@tile("chiseled_red_sandstone")
def chiseled_red_sandstone():
    image = load_tile("red_sandstone")
    border(image, "#8a4a22", 2)
    rect(image, 8, 8, 16, 16, "#b0652e")
    rect(image, 12, 12, 8, 8, "#8a4a22")
    return image


@tile("cut_red_sandstone")
def cut_red_sandstone():
    image = load_tile("red_sandstone")
    for y in (7, 15, 23):
        rect(image, 0, y, N, 1, "#8a4a22")
    return image


@tile("activator_rail")
def activator_rail():
    image = load_tile("rail")
    for y in range(N):
        rect(image, 15, y, 2, 1, "#c82a2a")
    return image


@tile("barrier")
def barrier():
    image = transparent()
    for y in range(N):
        for x in range(N):
            d = math.hypot(x - 15.5, y - 15.5)
            if 10 < d < 14 or (abs((x - 15.5) - (y - 15.5)) < 1.6 and d < 14):
                put(image, x, y, "#d82020")
    return image


@tile("light")
def light():
    image = transparent()
    for y in range(N):
        for x in range(N):
            d = math.hypot(x - 15.5, y - 13.5)
            if d < 8:
                put(image, x, y, "#ffe45a" if d < 5 else "#f8b830")
    rect(image, 12, 22, 8, 3, "#8a8a92")
    rect(image, 13, 25, 6, 2, "#8a8a92")
    return image


def tall_flower(name, petal, center, stem="#3a8a2a", petals=True):
    def build():
        image = transparent()
        rect(image, 15, 10, 2, 22, stem)
        rect(image, 11, 20, 4, 2, stem)
        rect(image, 17, 24, 4, 2, stem)
        if petals:
            for dx, dy in ((-5, 0), (5, 0), (0, -5), (0, 5), (-4, -4), (4, 4), (-4, 4), (4, -4)):
                rect(image, 14 + dx, 6 + dy, 4, 4, petal)
        rect(image, 14, 6, 4, 4, center)
        return image

    TILES[name] = build


tall_flower("sunflower", "#f8d020", "#6a4a1a")
tall_flower("lilac", "#b888d8", "#e8d0f4")
tall_flower("rose_bush", "#d82838", "#8a1420")
tall_flower("peony", "#f0a8c8", "#fff0f6")


@tile("large_fern")
def large_fern():
    image = transparent()
    r = rng("large_fern")
    for k in range(7):
        x = 4 + k * 4
        h = int(14 + r.integers(0, 14))
        for y in range(N - h, N):
            put(image, x + (N - y) // 6 * (1 if k % 2 else -1), y, "#2e7a2a" if y % 3 else "#3a9a34")
            put(image, x + 1 + (N - y) // 6 * (1 if k % 2 else -1), y, "#256a24")
    return image



# ------------------------------------------------------------ part 5 tiles

for _w in ("spruce", "birch", "jungle", "acacia", "cherry", "dark_oak", "mangrove", "bamboo"):
    door_tile(f"{_w}_door_top", f"{_w}_planks", True, False)
    door_tile(f"{_w}_door_bottom", f"{_w}_planks", False, False)

for _c, _hex in COLORS.items():
    def _powder(c=_c, hexv=_hex):
        def build():
            r = rng("powder_" + c)
            image = load_tile(f"concrete_{c}")
            image[..., :3] = np.clip(image[..., :3] * 0.92 + 14, 0, 255)
            speckle(image, r, [hexv, "#ffffff", "#000000"], 120)
            return image

        return build

    TILES[f"{_c}_concrete_powder"] = _powder()

    def _glazed(c=_c, hexv=_hex):
        def build():
            base = rgb(hexv)
            light = "#%02x%02x%02x" % tuple(int(v) for v in np.clip(base * 1.35 + 30, 0, 255))
            dark = "#%02x%02x%02x" % tuple(int(v) for v in np.clip(base * 0.6, 0, 255))
            image = canvas(hexv)
            for k in range(N):
                for x, y in ((k, 0), (k, N - 1), (0, k), (N - 1, k)):
                    put(image, x, y, dark)
            for y in range(N):
                for x in range(N):
                    d = abs(x - 15.5) + abs(y - 15.5)
                    if 9 < d < 12:
                        put(image, x, y, light)
                    elif d <= 9 and (x + y) % 4 == 0:
                        put(image, x, y, dark)
                    elif d >= 22:
                        put(image, x, y, light if (x + y) % 6 < 2 else hexv)
            return image

        return build

    TILES[f"{_c}_glazed_terracotta"] = _glazed()

    def _shulker(c=_c, hexv=_hex):
        def build():
            r = rng("shulker_" + c)
            image = fill(canvas(), fbm(r, ((8, 1.0), (16, 0.6))), shades(hexv, 0.12))
            base = rgb(hexv)
            dark = "#%02x%02x%02x" % tuple(int(v) for v in np.clip(base * 0.55, 0, 255))
            rect(image, 0, 13, N, 3, dark)
            rect(image, 13, 12, 6, 5, "#e8e0c0")
            rect(image, 14, 14, 4, 2, dark)
            border(image, dark, 1)
            return image

        return build

    TILES[f"shulker_box_{_c}"] = _shulker()


def _plain_shulker():
    r = rng("shulker_plain")
    image = fill(canvas(), fbm(r, ((8, 1.0), (16, 0.6))), ["#7a4e7a", "#86588a", "#92628f", "#9e6c9a"])
    rect(image, 0, 13, N, 3, "#4a2a4a")
    rect(image, 13, 12, 6, 5, "#e8e0c0")
    rect(image, 14, 14, 4, 2, "#4a2a4a")
    border(image, "#4a2a4a", 1)
    return image


TILES["shulker_box"] = _plain_shulker

noise_tile("purpur_block", ["#9a6a9a", "#a574a5", "#b07eb0", "#8a5a8a"], specks=["#c898c8", "#6a3a6a"], count=40)
noise_tile("magma_block", ["#3a1408", "#5a1c0a", "#7a240c", "#481608"], specks=["#ff7a1a", "#ffb030", "#200a04"], count=70)
noise_tile("nether_wart_block", ["#8a1a22", "#9a2028", "#aa262e", "#7a161c"], specks=["#c83a40", "#5a1014"], count=50)
noise_tile("red_nether_bricks", ["#5a1218", "#661820", "#721e26", "#4e0e14"], specks=["#8a2a30", "#34080c"], count=50)
noise_tile("dirt_path_top", ["#8a6a3a", "#96743f", "#a07e46", "#7a5c30"], specks=["#6a4a24", "#b89458"], count=40)


@tile("purpur_pillar_side")
def purpur_pillar_side():
    image = load_tile("purpur_block")
    for x in (0, 8, 16, 24):
        rect(image, x, 0, 1, N, "#6a3a6a")
        rect(image, x + 1, 0, 1, N, "#c898c8")
    return image


@tile("purpur_pillar_top")
def purpur_pillar_top():
    image = load_tile("purpur_block")
    border(image, "#6a3a6a", 2)
    rect(image, 8, 8, 16, 16, "#a070a0")
    return image


@tile("end_stone_bricks")
def end_stone_bricks():
    image = load_tile("end_stone")
    for row in range(4):
        y = row * 8
        rect(image, 0, y + 7, N, 1, "#a8a870")
        off = 0 if row % 2 == 0 else 8
        for x in range(off, N + 8, 16):
            rect(image, x % N, y, 1, 7, "#a8a870")
    return image


@tile("end_rod")
def end_rod():
    image = canvas("#f8f4e8")
    for y in range(0, N, 6):
        rect(image, 12, y, 8, 2, "#c8c4b8")
    return image


@tile("chorus_plant")
def chorus_plant():
    r = rng("chorus_plant")
    image = fill(canvas(), fbm(r), ["#6a466a", "#745074", "#7e5a7e", "#5e3c5e"])
    speckle(image, r, ["#8a648a", "#4a2c4a"], 40)
    return image


@tile("chorus_flower")
def chorus_flower():
    image = canvas("#9a7aa8")
    border(image, "#5a3a68", 2)
    rect(image, 8, 8, 16, 16, "#b898c4")
    rect(image, 12, 12, 8, 8, "#d8c0e0")
    return image


def command_variant(name, accent):
    def build():
        image = load_tile("command_block")
        border(image, accent, 2)
        rect(image, 12, 12, 8, 8, accent)
        return image

    TILES[name] = build


command_variant("repeating_command_block", "#7a5ac8")
command_variant("chain_command_block", "#4aa070")


@tile("observer_front")
def observer_front():
    image = canvas("#6a6a6e")
    border(image, "#3a3a3e", 2)
    rect(image, 8, 12, 6, 6, "#1a1a1e")
    rect(image, 18, 12, 6, 6, "#1a1a1e")
    rect(image, 10, 22, 12, 3, "#a82a2a")
    return image


@tile("torchflower_crop")
def torchflower_crop():
    image = transparent()
    rect(image, 15, 12, 2, 20, "#4a9a34")
    rect(image, 13, 8, 6, 5, "#f08020")
    rect(image, 14, 6, 4, 3, "#ffb050")
    return image


@tile("pitcher_crop")
def pitcher_crop():
    image = transparent()
    rect(image, 15, 14, 2, 18, "#5aaa6a")
    rect(image, 10, 16, 5, 2, "#5aaa6a")
    rect(image, 17, 20, 5, 2, "#5aaa6a")
    return image


@tile("pitcher_plant")
def pitcher_plant():
    image = transparent()
    rect(image, 15, 14, 2, 18, "#4a9a7a")
    rect(image, 11, 4, 10, 10, "#c8c0e8")
    rect(image, 12, 6, 8, 6, "#8a7ac8")
    rect(image, 10, 20, 5, 2, "#4a9a7a")
    return image


@tile("beetroots")
def beetroots():
    image = transparent()
    for x in range(2, 30, 7):
        for y in range(10, N):
            rect(image, x + 1, y, 2, 1, "#3a8a3a")
        rect(image, x - 1, 6, 6, 6, "#7a1a3a")
        rect(image, x, 4, 4, 3, "#3a8a3a")
    return image


@tile("structure_void")
def structure_void():
    image = transparent()
    rect(image, 12, 12, 8, 8, "#d0d0e8")
    rect(image, 14, 14, 4, 4, "#8a8aa8")
    return image



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
