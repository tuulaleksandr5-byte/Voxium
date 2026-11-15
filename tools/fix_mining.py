#!/usr/bin/env python3
"""Приводит твёрдость/инструмент/уровень добычи блоков к Minecraft 1.20.1 (assets/data/mining.ron)."""
import csv, re
rows = list(csv.DictReader(open("docs/BLOCKS.csv", encoding="utf-8-sig"), delimiter=";"))
SUF = re.compile(r"_(north|south|east|west|x|z|lit|open|closed|on|off|powered|\d+|lower|upper|head|foot|left|right|top|bottom)$")
def base(n):
    for _ in range(6):
        m = SUF.sub("", n)
        if m == n: break
        n = m
    return n
ids = {}   # base name -> [ids]
for r in rows:
    n = r["identifier"].split(":")[1]
    ids.setdefault(base(n), []).append(int(r["id"]))
# base -> (hardness, tool, tier)
FIX = {}
def put(names, h, tool, tier):
    for n in names.split():
        FIX[n] = (h, tool, tier)
put("basalt polished_basalt", 1.25, "Pickaxe", 1)
put("mossy_stone_bricks cracked_stone_bricks chiseled_nether_bricks", 1.5, "Pickaxe", 1)
put("emerald_block", 5.0, "Pickaxe", 3)
put("nether_gold_ore", 3.0, "Pickaxe", 1)
put("redstone_ore", 3.0, "Pickaxe", 3)
put("deepslate_redstone_ore deepslate_emerald_ore", 4.5, "Pickaxe", 3)
put("deepslate_copper_ore", 4.5, "Pickaxe", 2)
put("vine glow_lichen sculk sculk_vein", 0.2, "Axe", 0)
put("sculk", 0.2, "Hoe", 0); put("sculk_vein glow_lichen", 0.2, "Hoe", 0)
put("ender_chest", 22.5, "Pickaxe", 1)
put("oxidized_copper weathered_copper exposed_copper oxidized_cut_copper weathered_cut_copper exposed_cut_copper cut_copper waxed_copper_block waxed_weathered_copper waxed_exposed_copper waxed_oxidized_copper waxed_oxidized_cut_copper waxed_weathered_cut_copper waxed_exposed_cut_copper waxed_cut_copper", 3.0, "Pickaxe", 2)
put("raw_iron_block raw_copper_block", 5.0, "Pickaxe", 2)
put("raw_gold_block", 5.0, "Pickaxe", 3)
put("crying_obsidian respawn_anchor", 50.0, "Pickaxe", 4)
put("nether_wart_block", 1.0, "Hoe", 0)
# ступени, плиты, стены: уровень добычи как у основного блока (нужна кирка)
BASEH = {}
for n, ids_ in ids.items():
    m = re.match(r"(.*)_(stairs|slab|wall)$", n)
    if not m: continue
    b, kind = m.groups()
    if re.match(r"^(oak|spruce|birch|jungle|acacia|dark_oak|mangrove|cherry|bamboo|crimson|warped|petrified_oak)", b): 
        if kind != "wall":
            put(n, 2.0, "Axe", 0)
        continue
    for c in (b, b + "s", b + "_block", b.replace("_brick", "_bricks"), b.replace("_tile", "_tiles")):
        if c in ids:
            bh, btool, btier = FIX.get(c, (None, None, None))
            BASEH[n] = (c, kind); break
rules = open("assets/data/mining.ron", encoding="utf-8").read()
cur = {}
for m in re.finditer(r"\(block: (\d+), hardness: ([\d.]+), tool: (\w+), harvest_tier: (\d+), seconds: (None|Some\([^)]*\))\)", rules):
    cur[int(m.group(1))] = (float(m.group(2)), m.group(3), int(m.group(4)))
STONE_SLAB_2 = {"cobblestone", "smooth_stone", "stone", "stone_brick", "brick", "sandstone", "cut_sandstone", "red_sandstone", "cut_red_sandstone", "quartz", "nether_brick", "purpur", "mud_brick", "bricks", "stone_bricks"}
def rule_for(n):
    if n in FIX: return FIX[n]
    if n in BASEH:
        c, kind = BASEH[n]
        h, tool, tier = FIX.get(c) or (None, None, None)
        if h is None:
            h0, t0, tr0 = cur[ids[c][0]]
            h, tier = h0, max(tr0, 1)
        if kind == "slab" and re.sub(r"_slab$", "", n) in STONE_SLAB_2:
            h = 2.0
        return (h, "Pickaxe", tier)
    return None
changed = {}
for n, ids_ in ids.items():
    r = rule_for(n)
    if not r: continue
    for i in ids_:
        changed[i] = r
def sub(m):
    i = int(m.group(1))
    if i not in changed: return m.group(0)
    h, tool, tier = changed[i]
    return f"(block: {i}, hardness: {h}, tool: {tool}, harvest_tier: {tier}, seconds: None)"
new = re.sub(r"\(block: (\d+), hardness: ([\d.]+), tool: (\w+), harvest_tier: (\d+), seconds: (None|Some\([^)]*\))\)", sub, rules)
open("assets/data/mining.ron", "w", encoding="utf-8").write(new)
print("changed", sum(1 for i in changed if cur.get(i) != (changed[i][0], changed[i][1], changed[i][2])), "of", len(changed))
