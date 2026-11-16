#!/usr/bin/env python3
"""Читает справочник мобов docs/MOBS_REFERENCE.txt и создаёт таблицы:
  crates/game_core/src/mob_table.rs  — виды мобов и их характеристики;
  apps/client/src/mob_skins.rs       — встроенные текстуры мобов;
  tools/mobs.json                    — данные для рисования моделей и текстур.
Запуск: python3 tools/generate_mobs.py
"""
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
text = (ROOT / "docs/MOBS_REFERENCE.txt").read_text(encoding="utf-8")
blocks = re.split(r"\n(?=\d\d\. )", text)[1:]
items = (ROOT / "assets/data/items.ron").read_text(encoding="utf-8")
by_name = {}
for m in re.finditer(r'id: "([^"]+)", name: "([^"]*)"', items):
    by_name[m.group(2).lower()] = m.group(1)
ALIAS = {
    "баранина": "raw_mutton", "гнилая плоть": "rotten_meat", "говядина": "raw_beef", "курятина": "raw_chicken",
    "свинина": "raw_pork", "железные слитки": "iron_ingot", "золотой самородок": "gold_nugget", "кости": "bone",
    "кристаллы призмарина": "prismarine_crystals", "призмариновые осколки": "prismarine_shard", "лосось": "salmon",
    "треска": "cod", "снежки": "snowball", "шерсть": "wool", "нить": "string", "мак": "poppy",
    "перо": "feather", "кожа": "leather", "порох": "gunpowder", "уголь": "coal", "кость": "bone",
    "железный слиток": "iron_ingot", "изумруд": "emerald", "огненный стержень": "blaze_rod",
    "слеза гаста": "ghast_tear", "эндер-жемчуг": "ender_pearl", "тотем бессмертия": "totem_of_undying",
    "иглобрюх": "pufferfish", "тропическая рыба": "tropical_fish", "кроличья шкурка": "rabbit_hide",
    "сырая крольчатина": "rabbit", "бамбук": "bamboo", "морская трава": "seagrass", "седло": "saddle",
    "паучий глаз": "spider_eye", "чернильный мешок": "ink_sac", "светящийся чернильный мешок": "glow_ink_sac",
    "скалковый катализатор": "sculk_catalyst", "звезда незера": "nether_star", "стрела": "arrow",
    "кроличья лапка": "rabbit_foot", "светокаменная пыль": "glowstone_dust", "сахар": "sugar",
    "редстоун": "redstone", "бутылочка": "glass_bottle", "палка": "stick",
}
OVERRIDES = {  # строки, которые не разобрать по шаблону
    "witch": [("glowstone_dust", 0, 2), ("sugar", 0, 2), ("redstone", 0, 2), ("spider_eye", 0, 2), ("glass_bottle", 0, 2), ("gunpowder", 0, 2), ("stick", 0, 2)],
    "magma_cube": [("magma_cream", 0, 1)], "slime": [("slime_ball", 0, 2)], "shulker": [("shulker_shell", 0, 1)],
    "phantom": [("phantom_membrane", 0, 1)], "evoker": [("totem_of_undying", 1, 1), ("emerald", 0, 1)],
    "vindicator": [("emerald", 0, 1)], "polar_bear": [("cod", 0, 2)], "sheep": [("raw_mutton", 1, 2), ("wool", 1, 1)],
    "elder_guardian": [("prismarine_shard", 0, 2), ("cod", 0, 1), ("prismarine_crystals", 0, 1)],
    "guardian": [("prismarine_shard", 0, 2), ("cod", 0, 1), ("prismarine_crystals", 0, 1)],
    "ender_dragon": [], "giant": [], "chicken": [("feather", 0, 2), ("raw_chicken", 1, 1)],
    "iron_golem": [("iron_ingot", 3, 5), ("poppy", 0, 2)], "wither": [("nether_star", 1, 1)],
    "zombified_piglin": [("rotten_meat", 0, 1), ("gold_nugget", 0, 1)], "turtle": [("seagrass", 0, 2)],
    "wither_skeleton": [("coal", 0, 1), ("bone", 0, 2)],
}
# (поведение, архетип, основной цвет, второй цвет, глаза)
ART = {
    "allay": ("flyer", "#58c8f0", "#a8e8ff", "#101018"), "axolotl": ("quad", "#f0a8c0", "#d87898", "#101018"),
    "bat": ("flyer", "#4a3a2a", "#6a5238", "#ff4040"), "bee": ("flyer", "#f0c030", "#2a2010", "#101010"),
    "blaze": ("flyer", "#f0a820", "#d86010", "#fff0a0"), "camel": ("quad", "#d8b070", "#b88a48", "#101010"),
    "cat": ("quad", "#d09858", "#f0e0c0", "#58c850"), "cave_spider": ("quad", "#1a5a6a", "#2a7a8a", "#e03030"),
    "chicken": ("quad", "#f2eee6", "#e0a030", "#101010"), "cod": ("fish", "#b8946a", "#e0d0a8", "#101010"),
    "cow": ("quad", "#4a3020", "#e8e4dc", "#101010"), "creeper": ("blob", "#5ab04a", "#2a6a2a", "#101010"),
    "dolphin": ("fish", "#7a8a9a", "#d8e0e8", "#101010"), "donkey": ("quad", "#7a6a5a", "#a89888", "#101010"),
    "drowned": ("biped", "#4a8a7a", "#6ab0a0", "#a0f0f0"), "elder_guardian": ("blob", "#8a8a78", "#c8a898", "#e0e0e0"),
    "ender_dragon": ("flyer", "#1a1a22", "#6a2a8a", "#e060ff"), "enderman": ("biped", "#16101c", "#3a2a48", "#e060ff"),
    "endermite": ("blob", "#3a2a48", "#6a4a88", "#e060ff"), "evoker": ("biped", "#8a8a88", "#4a4a48", "#d8d8d0"),
    "fox": ("quad", "#d8782a", "#f4f0e8", "#101010"), "frog": ("quad", "#5a8a48", "#c8d890", "#101010"),
    "ghast": ("blob", "#f0f0f0", "#d8d8d8", "#202020"), "giant": ("biped", "#5a8a48", "#3a5ab0", "#101010"),
    "glow_squid": ("fish", "#2a8a8a", "#58e8d0", "#e0ffff"), "goat": ("quad", "#e8e4dc", "#b8b0a0", "#101010"),
    "guardian": ("blob", "#5a8a7a", "#d8a878", "#f0f0f0"), "hoglin": ("quad", "#a8685a", "#d8b0a0", "#101010"),
    "horse": ("quad", "#8a5a3a", "#3a2a1a", "#101010"), "husk": ("biped", "#8a7858", "#a89878", "#101010"),
    "illusioner": ("biped", "#4a5a8a", "#8a98c8", "#d8d8d0"), "iron_golem": ("biped", "#c8c8c0", "#7a8a5a", "#c83030"),
    "llama": ("quad", "#d8c8a8", "#a88868", "#101010"), "magma_cube": ("blob", "#4a1a0a", "#f08020", "#ffe080"),
    "mooshroom": ("quad", "#b83028", "#e8e4dc", "#101010"), "mule": ("quad", "#5a4030", "#8a6a50", "#101010"),
    "ocelot": ("quad", "#e0c060", "#8a6a20", "#58c850"), "panda": ("quad", "#f4f4f4", "#1a1a1a", "#101010"),
    "parrot": ("flyer", "#d82828", "#2858d8", "#101010"), "phantom": ("flyer", "#3a4a8a", "#5a78c8", "#58f0a0"),
    "pig": ("quad", "#f0a8a0", "#d88880", "#101010"), "piglin": ("biped", "#d8946a", "#f0c8a0", "#f0e0a0"),
    "piglin_brute": ("biped", "#b87848", "#d8a070", "#f0e0a0"), "pillager": ("biped", "#6a6a68", "#8a8a88", "#d8d8d0"),
    "polar_bear": ("quad", "#f2f2ee", "#d0d0c8", "#101010"), "pufferfish": ("blob", "#e8c83a", "#f4e8a0", "#101010"),
    "rabbit": ("quad", "#9a7a5a", "#d8c8a8", "#101010"), "ravager": ("quad", "#5a5258", "#8a8288", "#e0e0e0"),
    "salmon": ("fish", "#a8584a", "#e8d0c0", "#101010"), "sheep": ("quad", "#e8e4dc", "#c8b8a0", "#101010"),
    "shulker": ("blob", "#8a5a8a", "#b898b8", "#f0f0a0"), "silverfish": ("blob", "#7a7a7e", "#a8a8ac", "#101010"),
    "skeleton": ("biped", "#d8d8d0", "#a8a8a0", "#101010"), "skeleton_horse": ("quad", "#d0d0c8", "#a0a098", "#101010"),
    "slime": ("blob", "#68c050", "#98e078", "#101010"), "sniffer": ("quad", "#a83a30", "#58a050", "#101010"),
    "snow_golem": ("biped", "#f4f8f8", "#d88020", "#101010"), "spider": ("quad", "#3a2a28", "#7a2a2a", "#e03030"),
    "squid": ("fish", "#2a4a8a", "#5a78b8", "#e0f0ff"), "stray": ("biped", "#b8c8d0", "#7a98b0", "#a0f0ff"),
    "strider": ("quad", "#a83030", "#d86050", "#101010"), "tadpole": ("fish", "#3a3a2a", "#5a5a42", "#101010"),
    "trader_llama": ("quad", "#d8c8a8", "#2858a8", "#101010"), "tropical_fish": ("fish", "#f08a2a", "#f4f0e8", "#101010"),
    "turtle": ("quad", "#4a8a48", "#c8c088", "#101010"), "vex": ("flyer", "#9ab0d0", "#c8d8f0", "#e03030"),
    "villager": ("biped", "#c8a078", "#7a5a38", "#58a858"), "vindicator": ("biped", "#8a8a88", "#4a4a48", "#d8d8d0"),
    "wandering_trader": ("biped", "#2858a8", "#d8c8a8", "#101010"), "warden": ("biped", "#1a3a4a", "#58e0c8", "#58f0e0"),
    "witch": ("biped", "#4a2a6a", "#58a048", "#101010"), "wither": ("flyer", "#1a1a1a", "#4a4a4e", "#e0e0e0"),
    "wither_skeleton": ("biped", "#2a2a2c", "#4a4a4e", "#c0c0c8"), "wolf": ("quad", "#c8c4bc", "#a8a49c", "#101010"),
    "zoglin": ("quad", "#8a6a5a", "#a84a4a", "#e03030"), "zombie": ("biped", "#4a8a48", "#3a5ab0", "#101010"),
    "zombie_horse": ("quad", "#5a7a48", "#8a9a68", "#101010"), "zombie_villager": ("biped", "#5a8a58", "#7a5a38", "#101010"),
    "zombified_piglin": ("biped", "#c08868", "#7a9a58", "#101010"),
}
FLIES = {"allay", "bat", "bee", "blaze", "ender_dragon", "ghast", "parrot", "phantom", "vex", "wither"}
AQUATIC = {"axolotl", "cod", "dolphin", "elder_guardian", "glow_squid", "guardian", "pufferfish", "salmon", "squid", "tadpole", "tropical_fish"}
RANGED = {"skeleton": (12.0, 2.0), "stray": (12.0, 2.0), "pillager": (12.0, 3.0), "illusioner": (12.0, 3.0), "blaze": (14.0, 3.0), "ghast": (20.0, 5.0), "witch": (10.0, 3.0), "wither": (20.0, 6.0), "evoker": (10.0, 4.0), "guardian": (10.0, 3.0), "elder_guardian": (12.0, 4.0), "shulker": (12.0, 3.0), "wither_skeleton": (0.0, 0.0)}
BURNS = {"zombie", "skeleton", "zombie_villager", "phantom", "drowned"}
ATTACK = {"creeper": 0.0, "enderman": 7.0, "iron_golem": 7.0, "polar_bear": 6.0, "wolf": 3.0, "bee": 2.0, "goat": 2.0, "llama": 1.0, "trader_llama": 1.0, "panda": 2.0, "fox": 2.0, "dolphin": 2.0, "piglin": 4.0, "zombified_piglin": 5.0, "piglin_brute": 7.0, "warden": 12.0, "ravager": 8.0, "vindicator": 6.0, "wither_skeleton": 5.0, "hoglin": 6.0, "zoglin": 6.0, "husk": 3.0, "drowned": 3.0, "zombie": 3.0, "zombie_villager": 3.0, "spider": 2.0, "cave_spider": 2.0, "silverfish": 1.0, "endermite": 2.0, "slime": 2.0, "magma_cube": 3.0, "vex": 4.0, "phantom": 3.0, "ender_dragon": 8.0, "giant": 0.0, "pufferfish": 0.0}
SPEED = {"horse": 4.6, "donkey": 3.6, "mule": 3.6, "camel": 3.2, "llama": 2.6, "trader_llama": 2.6, "wolf": 3.2, "fox": 3.4, "cat": 3.2, "ocelot": 3.2, "rabbit": 3.2, "enderman": 3.4, "creeper": 2.5, "spider": 3.1, "cave_spider": 3.2, "zombie": 2.4, "husk": 2.4, "drowned": 2.4, "zombified_piglin": 2.8, "skeleton": 2.5, "stray": 2.5, "warden": 2.4, "ravager": 2.6, "iron_golem": 2.2, "bat": 3.0, "bee": 3.0, "parrot": 3.0, "phantom": 4.0, "blaze": 2.6, "ghast": 2.0, "vex": 4.0, "allay": 3.4, "ender_dragon": 5.0, "wither": 3.5, "dolphin": 4.0, "cod": 2.5, "salmon": 2.8, "tropical_fish": 2.2, "pufferfish": 1.8, "squid": 1.6, "glow_squid": 1.6, "axolotl": 1.6, "tadpole": 1.4, "turtle": 0.9, "sniffer": 1.2, "slime": 2.0, "magma_cube": 2.0, "silverfish": 2.8, "endermite": 2.6, "shulker": 0.0}
NATURAL_PASSIVE = ["cow", "pig", "sheep", "chicken", "rabbit", "horse", "donkey", "llama", "fox", "wolf", "cat", "goat"]
NATURAL_HOSTILE = ["zombie", "skeleton", "spider", "creeper", "enderman", "witch", "husk", "cave_spider"]
mobs = []
unresolved = set()
for block in blocks:
    head = re.match(r"\d\d\. (.+?) \[(\w+)\]", block)
    name_ru, ident = head.group(1), head.group(2)
    hp_line = re.search(r"Здоровье: (.*?) HP", block).group(1)
    if " / " in hp_line and "дикий" not in hp_line:
        hp = float(hp_line.split(" / ")[1])
    else:
        nums = [float(n) for n in re.findall(r"\d+(?:\.\d+)?", hp_line.split("/")[0])]
        hp = sum(nums[:2]) / len(nums[:2])
    behavior = re.search(r"Поведение: (.*)", block).group(1)
    size = re.search(r"Размер: ([\d.]+) × ([\d.]+)", block)
    w, h = float(size.group(1)), float(size.group(2))
    drop_line = re.search(r"Дроп: (.*)", block).group(1)
    drops = []
    if ident in OVERRIDES:
        drops = OVERRIDES[ident]
    elif not drop_line.startswith(("Нет", "Обычных", "В 1.20")):
        for part in drop_line.split(";"):
            part = re.sub(r"\(.*?\)", "", part).strip()
            m = re.match(r"(.+?)\s+(\d+)(?:–(\d+))?\s*(?:при.*)?$", part)
            if not m:
                unresolved.add(part)
                continue
            key = m.group(1).strip().lower()
            lo = int(m.group(2))
            hi = int(m.group(3) or m.group(2))
            drops.append((key, lo, hi))
    resolved = []
    for key, lo, hi in drops:
        item = ALIAS.get(key, key)
        found = None
        for candidate in (f"voxium:{item}", f"survival:{item}", f"builder:{item}", f"flora:{item}", f"mine:{item}", f"palette:{item}", f"maker:{item}"):
            if f'id: "{candidate}"' in items:
                found = candidate
                break
        if not found and key in by_name:
            found = by_name[key]
        if not found:
            unresolved.add(key)
            continue
        resolved.append((found, lo, hi))
    neutral = behavior.startswith("Нейтральный")
    hostile = behavior.startswith(("Враждебный", "Босс")) or neutral
    if behavior.startswith("Только команды"):
        hostile = False
    if ident == "spider":
        hostile, neutral = True, False
    arch, c1, c2, eye = ART[ident]
    ranged, ranged_damage = RANGED.get(ident, (0.0, 0.0))
    attack = ATTACK.get(ident, 3.0 if hostile else 0.0)
    if ranged_damage:
        attack = ranged_damage
    natural = 1 if ident in NATURAL_PASSIVE else 2 if ident in NATURAL_HOSTILE else 0
    mobs.append(dict(id=ident, name=name_ru, hp=hp, hostile=hostile, neutral=neutral, w=w, h=h, speed=SPEED.get(ident, 2.5 if hostile else 1.6),
                     attack=attack, drops=resolved, arch=arch, c1=c1, c2=c2, eye=eye, flies=ident in FLIES, aquatic=ident in AQUATIC,
                     ranged=ranged, burns=ident in BURNS, explodes=ident == "creeper", natural=natural, boss=behavior.startswith("Босс")))
print("мобов:", len(mobs), "неразобрано:", sorted(unresolved))
(ROOT / "tools/mobs.json").write_text(json.dumps(mobs, ensure_ascii=False, indent=1), encoding="utf-8")


def camel(ident):
    return "".join(part.capitalize() for part in ident.split("_"))


def f(x):
    return f"{x:.2f}"


ARCH = {"quad": "Quad", "biped": "Biped", "flyer": "Flyer", "fish": "Fish", "blob": "Blob"}
rs = ["//! Таблица мобов (создаётся `tools/generate_mobs.py` из docs/MOBS_REFERENCE.txt).", "use super::{Archetype, MobSpec};", "",
      "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]", "pub enum MobKind {"]
rs += [f"    {camel(m['id'])}," for m in mobs]
rs += ["}", "", "impl MobKind {", f"    pub const ALL: [Self; {len(mobs)}] = ["]
rs += [f"        Self::{camel(m['id'])}," for m in mobs]
rs += ["    ];", f"    pub(super) const NATURAL_PASSIVE: [Self; {len(NATURAL_PASSIVE)}] = ["]
rs += [f"        Self::{camel(i)}," for i in NATURAL_PASSIVE]
rs += ["    ];", f"    pub(super) const NATURAL_HOSTILE: [Self; {len(NATURAL_HOSTILE)}] = ["]
rs += [f"        Self::{camel(i)}," for i in NATURAL_HOSTILE]
rs += ["    ];", "", "    #[must_use]", "    pub fn spec(self) -> &'static MobSpec {", "        match self {"]
for m in mobs:
    drops = ", ".join(f'("{i}", {lo}, {hi})' for i, lo, hi in m["drops"])
    rs += [f"            Self::{camel(m['id'])} => &MobSpec {{",
           f'                id: "{m["id"]}",', f'                name: "{m["name"]}",',
           f"                hostile: {str(m['hostile']).lower()},", f"                neutral: {str(m['neutral']).lower()},",
           f"                max_health: {f(m['hp'])},", f"                speed: {f(m['speed'])},",
           f"                height: {f(min(m['h'], 3.0))},", f"                half_width: {f(min(m['w'] / 2, 0.9))},",
           f"                attack: {f(m['attack'])},", f"                drops: &[{drops}],",
           f"                flies: {str(m['flies']).lower()},", f"                aquatic: {str(m['aquatic']).lower()},",
           f"                ranged: {f(m['ranged'])},", f"                burns: {str(m['burns']).lower()},",
           f"                explodes: {str(m['explodes']).lower()},", f"                archetype: Archetype::{ARCH[m['arch']]},",
           f"                size: [{f(m['w'])}, {f(m['h'])}],", "            },"]
rs += ["        }", "    }", "}", ""]
(ROOT / "crates/game_core/src/mob_table.rs").write_text("\n".join(rs), encoding="utf-8")
sk = ["//! Встроенные текстуры мобов (создаётся `tools/generate_mobs.py`).", "use game_core::mobs::MobKind;", "",
      "pub(crate) fn skin_bytes(kind: MobKind) -> &'static [u8] {", "    macro_rules! skin {", "        ($name:literal) => {",
      '            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/textures/mobs/", $name, ".png"))', "        };", "    }", "    match kind {"]
sk += [f'        MobKind::{camel(m["id"])} => skin!("{m["id"]}"),' for m in mobs]
sk += ["    }", "}", ""]
(ROOT / "apps/client/src/mob_skins.rs").write_text("\n".join(sk), encoding="utf-8")
