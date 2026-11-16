#!/usr/bin/env python3
"""Convert the explicitly supplied Minecraft packs into native, embedded Voxium assets.

No pack scripts or expressions are executed. ZIP paths are never extracted wholesale.
Requires Pillow, numpy and fontTools. Run --help for the reproducible import command.
"""
import argparse
import ast
from functools import lru_cache
import hashlib
import io
import json
import math
from pathlib import Path
import re
import zipfile

import numpy as np
from PIL import Image


def safe_relative(value):
    path = Path(value)
    if path.is_absolute() or ".." in path.parts or "\\" in value:
        raise ValueError(f"unsafe resource path: {value}")
    return path


def faithful_name(name):
    aliases = {
        "grass_top": "grass_block_top", "grass_side": "grass_block_side",
        "water": "water_still", "lava": "lava_still", "wool": "white_wool",
        "lamp": "glowstone", "crafting_table_bottom": "oak_planks",
        "smoker_side": "smoker_side", "snow_block": "snow",
        "snow_layer": "snow", "kelp_sprout": "kelp", "bamboo_small": "bamboo_stage0",
        "bamboo_large": "bamboo_large_leaves", "farmland_moist": "farmland_moist",
        "beetroots_stage_3": "beetroots_stage3",
        "hay_bale_side": "hay_block_side", "hay_bale_top": "hay_block_top",
        "quartz_block": "quartz_block_side", "fire": "fire_0", "soul_fire": "soul_fire_0",
        "dispenser_side": "furnace_side", "sticky_piston_top": "piston_top_sticky",
        "mangrove_roots": "mangrove_roots_side", "muddy_mangrove_roots": "muddy_mangrove_roots_side",
        "basalt": "basalt_side", "polished_basalt": "polished_basalt_side",
        "daisy": "oxeye_daisy", "tall_grass": "tall_grass_top", "tall_seagrass": "tall_seagrass_top",
        "suspicious_sand": "suspicious_sand_0", "suspicious_gravel": "suspicious_gravel_0",
        "chiseled_bookshelf_front": "chiseled_bookshelf_empty",
        "wheat": "wheat_stage7", "carrots": "carrots_stage3", "potatoes": "potatoes_stage3",
        "beetroots": "beetroots_stage3", "farmland_top": "farmland", "glass_pane": "glass",
        "carved_pumpkin_front": "carved_pumpkin", "jack_o_lantern_front": "jack_o_lantern",
        "repeater_top": "repeater", "comparator_top": "comparator",
        "nether_wart": "nether_wart_stage2", "cocoa": "cocoa_stage2",
        "hopper": "hopper_outside", "quartz_ore": "nether_quartz_ore",
        "command_block": "command_block_front", "repeating_command_block": "repeating_command_block_front",
        "chain_command_block": "chain_command_block_front", "quartz_pillar_side": "quartz_pillar",
        "purpur_pillar_side": "purpur_pillar", "magma_block": "magma",
        "sunflower": "sunflower_top", "lilac": "lilac_top", "rose_bush": "rose_bush_top", "peony": "peony_top",
        "large_fern": "large_fern_top", "torchflower_crop": "torchflower_crop_stage1",
        "pitcher_crop": "pitcher_crop_top_stage4", "pitcher_plant": "pitcher_crop_top_stage4",
        "bamboo_sapling": "bamboo_stage0", "bamboo": "bamboo_stalk", "scaffolding": "scaffolding_side",
        "dried_kelp_block": "dried_kelp_side", "sniffer_egg": "sniffer_egg_not_cracked_top",
        "cartography_table_side": "cartography_table_side1", "grindstone": "grindstone_side", "lectern": "lectern_sides",
        "campfire": "campfire_log_lit", "soul_campfire": "soul_campfire_log_lit",
        "sweet_berry_bush": "sweet_berry_bush_stage3", "pointed_dripstone": "pointed_dripstone_up_tip",
        "azalea": "azalea_top", "flowering_azalea": "flowering_azalea_top", "small_dripleaf": "small_dripleaf_top",
        "frogspawn_decal": "frogspawn", "warped_hyphae": "warped_stem", "stripped_warped_hyphae": "stripped_warped_stem",
        "crimson_hyphae": "crimson_stem", "stripped_crimson_hyphae": "stripped_crimson_stem",
        "warped_nylium_top": "warped_nylium", "crimson_nylium_top": "crimson_nylium",
        "jigsaw": "jigsaw_side", "honey_block": "honey_block_side", "respawn_anchor_side": "respawn_anchor_side0",
        "beehive_top": "beehive_end", "moss_carpet": "moss_block",
        "big_dripleaf": "big_dripleaf_top", "redstone_wire": "redstone_dust_line0", "redstone_wire_on": "redstone_dust_line0",
    }
    if name in aliases:
        return aliases[name]
    if name.startswith("glass_"):
        return name.removeprefix("glass_") + "_stained_glass"
    for family in ["wool", "concrete", "concrete_powder", "stained_glass", "terracotta", "glazed_terracotta", "carpet"]:
        if name.startswith(family + "_"):
            return name[len(family)+1:] + "_" + family
    return re.sub(r"^(wheat|carrots|potatoes|beetroots)_stage_(\d)$", r"\1_stage\2", name)


def box_mesh(box, skin, mirror=False):
    x, y, z, w, h, d = box["coordinates"]
    add = box.get("sizeAdd", 0)
    x0, y0, z0 = [n-add for n in [x, y, z]]
    x1, y1, z1 = [x+w+add, y+h+add, z+d+add]
    u, v = box.get("textureOffset", [0, 0])
    faces = [
        ([0, 1, 0], [u+d, v, w, d], [[x0,y1,z0],[x1,y1,z0],[x1,y1,z1],[x0,y1,z1]]),
        ([0,-1, 0], [u+d+w, v, w, d], [[x0,y0,z1],[x1,y0,z1],[x1,y0,z0],[x0,y0,z0]]),
        ([-1,0,0], [u, v+d, d,h], [[x0,y1,z0],[x0,y1,z1],[x0,y0,z1],[x0,y0,z0]]),
        ([0,0,-1], [u+d,v+d,w,h], [[x1,y1,z0],[x0,y1,z0],[x0,y0,z0],[x1,y0,z0]]),
        ([1,0,0], [u+d+w,v+d,d,h], [[x1,y1,z1],[x1,y1,z0],[x1,y0,z0],[x1,y0,z1]]),
        ([0,0,1], [u+2*d+w,v+d,w,h], [[x0,y1,z1],[x1,y1,z1],[x1,y0,z1],[x0,y0,z1]]),
    ]
    result = {k: [] for k in ["positions", "normals", "uvs", "indices"]}
    alpha = skin.getchannel("A")
    tw, th = skin.size
    for normal, (rx, ry, rw, rh), corners in faces:
        if rw <= 0 or rh <= 0:
            continue
        rect = (max(0,math.floor(rx)), max(0,math.floor(ry)), min(tw,math.ceil(rx+rw)), min(th,math.ceil(ry+rh)))
        if rect[2] <= rect[0] or rect[3] <= rect[1] or alpha.crop(rect).getbbox() is None:
            continue
        start = len(result["positions"])
        result["positions"].extend([[round(c/16,7) for c in corner] for corner in corners])
        result["normals"].extend([normal]*4)
        left, right = (rx+0.01)/tw, (rx+rw-0.01)/tw
        if mirror:
            left, right = right, left
        top, bottom = (ry+0.01)/th, (ry+rh-0.01)/th
        result["uvs"].extend([[left,top],[right,top],[right,bottom],[left,bottom]])
        result["indices"].extend([start+i for i in [0,3,2,0,2,1]])
    return result


def rotation_matrix(degrees):
    x,y,z = [math.radians(n) for n in degrees]
    rx=np.array([[1,0,0],[0,math.cos(x),-math.sin(x)],[0,math.sin(x),math.cos(x)]])
    ry=np.array([[math.cos(y),0,math.sin(y)],[0,1,0],[-math.sin(y),0,math.cos(y)]])
    rz=np.array([[math.cos(z),-math.sin(z),0],[math.sin(z),math.cos(z),0],[0,0,1]])
    return rz @ ry @ rx


@lru_cache(maxsize=20000)
def expression_tree(expression):
    expression=re.sub(r"\bif\s*\(","cem_if(",str(expression))
    expression=expression.replace("&&"," and ").replace("||"," or ")
    return ast.parse(re.sub(r"!(?!=)"," not ",expression).strip(),mode="eval").body


def rest_expression(expression,context):
    """Interpret CEM numbers and booleans without executing Python from a pack."""
    functions={"sin":math.sin,"cos":math.cos,"torad":math.radians,"todeg":math.degrees,
        "min":min,"max":max,"pow":pow,"sqrt":lambda x:math.sqrt(max(0,x)),
        "acos":lambda x:math.acos(max(-1,min(1,x))),"atan2":math.atan2,
        "clamp":lambda x,a,b:max(a,min(b,x)),"random":lambda *a:0.5,"lerp":lambda t,a,b:a+t*(b-a)}
    def path(n):
        if isinstance(n,ast.Name):return n.id
        if isinstance(n,ast.Attribute):return path(n.value)+"."+n.attr
        raise ValueError("unsupported CEM variable")
    def visit(n):
        if isinstance(n,ast.Constant) and isinstance(n.value,(int,float,bool)):return n.value
        if isinstance(n,(ast.Name,ast.Attribute)):
            name=path(n);return {"true":True,"false":False,"pi":math.pi}.get(name,context.get(name,0))
        if isinstance(n,ast.UnaryOp):
            v=visit(n.operand)
            if isinstance(n.op,ast.Not):return not v
            if isinstance(n.op,ast.USub):return -v
            if isinstance(n.op,ast.UAdd):return v
        if isinstance(n,ast.BinOp):
            a,b=visit(n.left),visit(n.right)
            if isinstance(n.op,ast.Add):return a+b
            if isinstance(n.op,ast.Sub):return a-b
            if isinstance(n.op,ast.Mult):return a*b
            if isinstance(n.op,ast.Div):return a/b if b else 0
            if isinstance(n.op,ast.Mod):return a%b if b else 0
        if isinstance(n,ast.BoolOp):return all(visit(x) for x in n.values) if isinstance(n.op,ast.And) else any(visit(x) for x in n.values)
        if isinstance(n,ast.Compare):
            a=visit(n.left)
            for op,right in zip(n.ops,n.comparators):
                b=visit(right)
                if isinstance(op,ast.Eq):valid=a==b
                elif isinstance(op,ast.NotEq):valid=a!=b
                elif isinstance(op,ast.Gt):valid=a>b
                elif isinstance(op,ast.GtE):valid=a>=b
                elif isinstance(op,ast.Lt):valid=a<b
                elif isinstance(op,ast.LtE):valid=a<=b
                else:raise ValueError("unsupported CEM comparison")
                if not valid:return False
                a=b
            return True
        if isinstance(n,ast.Call) and isinstance(n.func,ast.Name):
            name=n.func.id
            if name=="cem_if":
                for i in range(0,len(n.args)-1,2):
                    if visit(n.args[i]):return visit(n.args[i+1])
                return visit(n.args[-1])
            if name in functions:return functions[name](*[visit(x) for x in n.args])
        raise ValueError("unsupported CEM expression")
    result=visit(expression_tree(expression))
    if not math.isfinite(result):raise ValueError("non-finite CEM rest state")
    return result


def rest_state(source):
    context={"is_alive":True,"is_on_ground":True,"health":20,"max_health":20,
        "frame_time":0,"age":0,"limb_swing":0,"limb_speed":0,"is_child":False,
        "is_aggressive":False,"is_sitting":False,"death_time":0,"hurt_time":0}
    equations=[]
    def collect(n):
        name=n.get("id",n.get("part",""));t=n.get("translate",[0,0,0]);r=n.get("rotate",[0,0,0])
        for i,axis in enumerate("xyz"):
            context[f"{name}.t{axis}"]=t[i]*(-1 if i<2 else 1)
            context[f"{name}.r{axis}"]=math.radians(r[i])*(-1 if i<2 else 1)
            context[f"{name}.s{axis}"]=1
        for group in n.get("animations",[]):equations.extend(group.items())
        for child in n.get("submodels",[]):collect(child)
    for n in source["models"]:collect(n)
    overrides={}
    for _ in range(3):
        for key,expression in equations:
            value=rest_expression(expression,context);context[key]=value;overrides[key]=value
    return overrides


def rig_role(name, source_part, top, inherited):
    if top:
        return "fixed"
    n = name.lower()
    if any(word in n for word in ["noarms", "nolegs", "disarm", "_legs", "_arms"]):
        return "fixed"
    if "head" not in inherited and ("head" in n or source_part == "head") and "overlay" not in n:
        return "head"
    if "leg" not in inherited and ("leg" in n or source_part in ["leg1","leg2","leg3","leg4","left_leg","right_leg"]):
        if "right" in n+source_part:return "leg_right"
        return "leg_left" if ("left" in n or "left" in source_part or source_part in ["leg1","leg4"] or n.endswith("1") or n.endswith("4")) else "leg_right"
    if "arm" not in inherited and "arm" in n:
        return "arm_left" if "left" in n else "arm_right"
    if "wing" not in inherited and ("wing" in n or "wing" in source_part):
        return "wing_left" if "left" in n+source_part else "wing_right"
    if "tail" not in inherited and "tail" in n:
        return "tail"
    return "fixed"


def convert_model(source, skin, kind, variant):
    """Export Blockbench's Y-up CEM rest hierarchy, removing transparent alternatives.

    The top CEM wrapper compensates for the host Minecraft bone. Voxium has no such
    host: drop that wrapper's transform, retaining each actual model bone below it.
    Coordinates in these Blockbench packs use invertAxis=xy. Other conventions are
    rejected rather than silently displayed with the wrong handedness.
    """
    parts=[]
    matrices=[]
    bounds=[]
    state=rest_state(source)
    def add(node, parent, host_part, inherited, top=False, parent_shift=None):
        axis=node.get("invertAxis", "xy")
        if axis != "xy":
            raise ValueError(f"unsupported CEM axis convention {axis!r}")
        name=node.get("id",node.get("part",f"bone{len(parts)}"))
        scale=[state.get(f"{name}.s{axis}",1) for axis in "xyz"]
        if min(abs(v) for v in scale)<0.001 or not state.get(f"{name}.visible",True):return False
        children=node.get("submodels", [])
        mesh={k:[] for k in ["positions","normals","uvs","indices"]}
        for box in node.get("boxes", []):
            one=box_mesh(box,skin,"u" in node.get("mirrorTexture",""))
            offset=len(mesh["positions"])
            for key in ["positions","normals","uvs"]: mesh[key].extend(one[key])
            mesh["indices"].extend(i+offset for i in one["indices"])
        # Prune after recursion so transparent parents still carry child transforms.
        index=len(parts)
        direct_top=top and bool(mesh["positions"])
        role=rig_role(name,host_part,top and not direct_top,inherited)
        spider_leg=re.fullmatch(r"spider_leg_(left|right)([1-4])",name)
        if kind=="spider" and role.startswith("leg_") and spider_leg:
            role=f"spider_{spider_leg[1]}_{int(spider_leg[2])-1}"
        group=role.split("_")[0]
        if group=="spider":group="leg"
        translate=[0,0,0] if top else [v/16 for v in node.get("translate",[0,0,0])]
        # CEM animation rotations compensate for host Minecraft bones. Those hosts
        # do not exist here: preserve authored local transforms and only evaluate
        # rest visibility/scales. Native semantic animation supplies movement.
        rotate=[0,0,0] if top else list(node.get("rotate",[0,0,0]))
        joint=None
        if direct_top:
            raw=node.get("translate",[0,0,0]);joint=np.array([raw[0],-raw[1],-raw[2]])/16
            translate=list(joint)
            mesh["positions"]=[[float(v[i]-joint[i]) for i in range(3)] for v in mesh["positions"]]
        if parent_shift is not None:translate=[float(translate[i]-parent_shift[i]) for i in range(3)]
        transform=np.eye(4)
        transform[:3,:3]=rotation_matrix(rotate) @ np.diag(scale)
        transform[:3,3]=translate
        world=transform if parent is None else matrices[parent] @ transform
        parts.append(dict(name=name,parent=parent,translation=translate,rotation=rotate,scale=scale,role=role,**mesh))
        matrices.append(world)
        for vertex in mesh["positions"]:
            bounds.append((world @ np.array([*vertex,1]))[:3])
        active=bool(mesh["indices"])
        for child in children:
            active=add(child,index,host_part,inherited | ({group} if role!="fixed" else set()),parent_shift=joint) or active
        parts[index]["active"]=active
        return active
    for model in source["models"]:
        add(model,None,model.get("part",""),set(),True)
    if not bounds:
        raise ValueError(f"no visible geometry for {kind}/{variant}")
    remap={old:new for new,(old,p) in enumerate((i,p) for i,p in enumerate(parts) if p["active"])}
    kept=[]
    for i,p in enumerate(parts):
        if not p.pop("active"): continue
        if p["parent"] is not None: p["parent"]=remap[p["parent"]]
        kept.append(p)
    # Put the actual feet, rather than a Minecraft model origin, on the ground.
    low=min(v[1] for v in bounds)
    high=max(v[1] for v in bounds)
    for p in kept:
        if p["parent"] is None: p["translation"][1]-=float(low)
    return dict(kind=kind,variant=variant,texture=list(skin.size),height=float(high-low),parts=kept)


class Importer:
    def __init__(self, root, source):
        self.root=root
        self.dest=root/"assets/resource_packs"
        self.source=source
        self.files={}
        self.manifest={"packs":{},"models":[],"blocks":[],"missing_blocks":[],"notes":[]}

    def archive(self,name):
        candidates=list(self.source.rglob(name))
        if len(candidates)!=1: raise ValueError(f"expected one {name}, found {len(candidates)}")
        path=candidates[0]
        self.manifest["packs"][name]=hashlib.sha256(path.read_bytes()).hexdigest()
        archive=zipfile.ZipFile(path)
        for entry in archive.namelist():
            if "/" not in entry and (entry=="pack.mcmeta" or any(word in entry.lower() for word in ["license","licence","credits"])):
                self.write(f"sources/packs/{Path(name).stem}/{entry}",archive.read(entry),False)
        return archive

    def write(self,name,data,embedded=True):
        path=self.dest/safe_relative(name)
        path.parent.mkdir(parents=True,exist_ok=True)
        path.write_bytes(data)
        if embedded: self.files[name]=path
        return name

    def png(self,name,image):
        buffer=io.BytesIO();image.save(buffer,format="PNG")
        return self.write(name,buffer.getvalue())

    def model(self,z,jem,kind,variant,skin_path=None):
        raw=z.read(jem)
        source=json.loads(raw.decode("utf-8-sig"))
        skin_path=skin_path or source.get("texture")
        if not skin_path: raise ValueError(f"missing skin for {jem}")
        if skin_path.startswith("textures/"): skin_path="assets/minecraft/"+skin_path
        if "/" not in skin_path: skin_path=str(Path(jem).parent/skin_path)
        raw_skin=z.read(skin_path)
        skin=Image.open(io.BytesIO(raw_skin)).convert("RGBA")
        if tuple(source.get("textureSize",skin.size))!=skin.size:
            raise ValueError(f"skin size mismatch in {jem}")
        model=convert_model(source,skin,kind,variant)
        model["skin"]=self.write(f"mobs/{kind}/{variant}.png",raw_skin)
        e=skin_path.removesuffix(".png")+"_e.png"
        model["emissive"]=self.write(f"mobs/{kind}/{variant}_e.png",z.read(e)) if e in z.namelist() else None
        self.write(f"sources/{kind}/{variant}.jem",raw,False)
        self.manifest["models"].append({"kind":kind,"variant":variant,"source":jem,"skin":skin_path,"parts":len(model["parts"]),"triangles":sum(len(p["indices"])//3 for p in model["parts"])})
        return model

    def mobs(self):
        models=[]
        dogs=self.archive("Better_Dogs_V0.41.zip")
        for number in [1,6,10,13,17,19,24,26,30,35,38,41]:
            path="assets/minecraft/textures/entity/wolf/wolf.png" if number==1 else f"assets/minecraft/optifine/mob/wolf/wolf_tame{number}.png"
            if path not in dogs.namelist(): path=f"assets/minecraft/optifine/mob/wolf/wolf{number}.png"
            models.append(self.model(dogs,"assets/minecraft/optifine/cem/wolf.jem","wolf",str(number),path))
        bosses=self.archive("boss-refreshed-v2-1.19-1.21.zip")
        for model,kind,skin in [("dragon","ender_dragon","enderdragon/dragon"),("wither","wither","wither/wither")]:
            models.append(self.model(bosses,f"assets/minecraft/optifine/cem/{model}.jem",kind,"1",f"assets/minecraft/textures/entity/{skin}.png"))
        skeletons=self.archive("Blues_Better_Skeletons_v0.01.zip")
        for name in ["skeleton","skeleton3","skeleton4","skeleton5"]:
            models.append(self.model(skeletons,f"assets/minecraft/optifine/cem/{name}.jem","skeleton",name))
        farm=self.archive("Better_Farm_Animals_v0.08.zip")
        for kind,skin in [("cow","cow/cow"),("pig","pig/pig"),("sheep","sheep/sheep"),("chicken","chicken"),("rabbit","rabbit/brown"),("mooshroom","cow/red_mooshroom")]:
            models.append(self.model(farm,f"assets/minecraft/optifine/cem/{kind}.jem",kind,"1",f"assets/minecraft/textures/entity/{skin}.png"))
        monsters=self.archive("Blues-Better-Monsters-v0.12.zip")
        for kind,names in {
            "zombie":["zombie","zombie2","zombie10","zombie13","zombie17"],
            "creeper":["creeper","creeper2","creeper3","creeper4","creeper5","creeper6"],
            "husk":["husk","husk2"],"drowned":["drowned","drowned2"],
            "enderman":["enderman"],"spider":["spider","spider2","spider3","spider4","spider5"],
            "witch":["witch"],"wither_skeleton":["wither_skeleton"],"zombie_villager":["zombie_villager","zombie_villager2"],
        }.items():
            for name in names:
                models.append(self.model(monsters,f"assets/minecraft/optifine/cem/{name}.jem",kind,name))
        self.write("models.json",json.dumps(models,separators=(",",":"),ensure_ascii=False).encode())
        self.manifest["notes"].append("CEM rest geometry and UV are converted; native semantic animation replaces OptiFine expressions, name/biome conditions and Minecraft-specific states.")

    def blocks(self,faithful):
        text=(self.root/"crates/game_render/src/atlas.rs").read_text()
        def names(section):
            body=text.split("const "+section,1)[1].split("];",1)[0]
            return re.findall(r'pixel_tile!\("([^\"]+)"\)',body)
        extras=names("EXTRA_TILES")
        layers={i:n for i,n in enumerate(names("PIXEL_SOURCE_TILES"))}
        layers.update({i+18:n for i,n in enumerate(names("PIXEL_ADDITIONAL_TILES"))})
        layers.update({i+36:n for i,n in enumerate(extras)})
        rows=math.ceil((36+len(extras))/6)
        atlas=Image.new("RGBA",(768,rows*128))
        for layer,name in layers.items():
            native=self.root/f"assets/textures/pixel/128x128/{name}.png"
            tile=Image.open(native).convert("RGBA")
            faithful_path=f"assets/minecraft/textures/block/{faithful_name(name)}.png"
            if faithful_path in faithful.namelist():
                tile=Image.open(io.BytesIO(faithful.read(faithful_path))).convert("RGBA")
                tile=tile.crop((0,0,tile.width,tile.width))
                if name in ["redstone_wire","redstone_wire_on"]:
                    color=[255,51,0] if name.endswith("_on") else [77,0,0]
                    a=np.asarray(tile).copy();a[:,:,:3]=(a[:,:,:3].astype(np.uint16)*np.array(color,dtype=np.uint16)//255).astype(np.uint8);tile=Image.fromarray(a)
                if name=="grass_side":
                    overlay=Image.open(io.BytesIO(faithful.read("assets/minecraft/textures/block/grass_block_side_overlay.png"))).convert("RGBA")
                    a=np.asarray(overlay).copy();a[:,:,:3]=(a[:,:,:3].astype(np.uint16)*np.array([118,173,69],dtype=np.uint16)//255).astype(np.uint8)
                    tile.alpha_composite(Image.fromarray(a))
                # Minecraft supplies these biome colours in addition to its grey PNGs.
                tinted_leaves=name.endswith("_leaves") and name not in ["cherry_leaves","azalea_leaves","flowering_azalea_leaves"]
                if name in ["grass_top","grass_side"] or tinted_leaves or name in ["vine","fern","grass","tall_grass"]:
                    color=(118,173,69)
                    if name=="grass_side": color=None
                    if color:
                        a=np.asarray(tile).copy();a[:,:,:3]=(a[:,:,:3].astype(np.uint16)*np.array(color,dtype=np.uint16)//255).astype(np.uint8);tile=Image.fromarray(a)
                if name=="water":
                    a=np.asarray(tile).copy();a[:,:,:3]=(a[:,:,:3].astype(np.uint16)*np.array([65,120,235],dtype=np.uint16)//255).astype(np.uint8);a[:,:,3]=np.minimum(a[:,:,3],180);tile=Image.fromarray(a)
                tile=tile.resize((128,128),Image.Resampling.NEAREST)
                self.manifest["blocks"].append({"layer":layer,"name":name,"source":faithful_path})
            else:
                self.manifest["missing_blocks"].append(name)
            atlas.paste(tile,((layer%6)*128,(layer//6)*128))
        self.png("faithful_atlas.png",atlas)

    def icons(self,faithful):
        eggs=self.archive("Spawned_Eggs_1.16-26.2_r1.zip")
        for name in eggs.namelist():
            if name.startswith("assets/minecraft/textures/item/") and name.endswith("_spawn_egg.png"):
                self.write("eggs/"+Path(name).name,eggs.read(name))
        icons=self.archive("Icons-v.1.9.17-4.zip")
        for name in ["main_menu","options","singleplayer","escape_menu","video_settings","music_sound_options","controls"]:
            path=f"assets/icons/textures/menu/{name}.png"
            self.write(f"gui/icons_{name}.png",icons.read(path))
        # Faithful item art supplements the user's dedicated egg pack.
        known=set(re.findall(r'icon!\("[^\"]+", "([^\"]+)"\)',(self.root/"apps/client/src/pixel_icons.rs").read_text()))
        for name in sorted(known):
            base=name.removeprefix("item_")
            p=f"assets/minecraft/textures/item/{base}.png"
            if p in faithful.namelist() and not base.endswith("_spawn_egg"):
                self.write(f"items/{base}.png",faithful.read(p))

    def gui(self):
        z=self.archive("lColourful-Containers8.zip")
        for name in ["inventory","furnace","blast_furnace","smoker","crafting_table","stonecutter","generic_54","chests/chest","chests/barrel","chests/ender_chest"]:
            path=f"assets/minecraft/textures/gui/container/{name}.png"
            self.write("gui/"+name.replace("/","_")+".png",z.read(path))
        inventory=Image.open(io.BytesIO(z.read("assets/minecraft/textures/gui/container/inventory.png"))).convert("RGBA")
        self.png("gui/slot.png",inventory.crop((7,83,25,101)))
        chest=Image.open(io.BytesIO(z.read("assets/minecraft/textures/gui/container/generic_54.png"))).convert("RGBA")
        self.png("gui/chest_slot.png",chest.crop((7,17,25,35)))
        for label,top in [("chest",chest), ("barrel",Image.open(io.BytesIO(z.read("assets/minecraft/textures/gui/container/chests/barrel.png"))).convert("RGBA"))]:
            compact=Image.new("RGBA",(176,166))
            compact.paste(top.crop((0,0,176,77)),(0,0))
            compact.paste(inventory.crop((0,77,176,166)),(0,77))
            self.png(f"gui/{label}_compact.png",compact)
        # A blank centre plus original outside border makes a nine-slice panel;
        # baked Minecraft item slots are deliberately excluded from this frame.
        frame=Image.new("RGBA",(18,18),(72,65,49,250))
        frame.paste(chest.crop((0,0,7,7)),(0,0));frame.paste(chest.crop((169,0,176,7)),(11,0))
        frame.paste(chest.crop((0,215,7,222)),(0,11));frame.paste(chest.crop((169,215,176,222)),(11,11))
        self.png("gui/panel.png",frame)

    def font(self,faithful):
        from fontTools.fontBuilder import FontBuilder
        from fontTools.pens.ttGlyphPen import TTGlyphPen
        mappings=json.loads((self.root/"tools/faithful_font_mapping.json").read_text())
        glyphs={};metrics={};cmap={}
        for provider in mappings:
            path="assets/minecraft/textures/"+provider["file"].split(":",1)[1]
            if path not in faithful.namelist(): continue
            sheet=Image.open(io.BytesIO(faithful.read(path))).convert("RGBA")
            rows=provider["chars"];cw=sheet.width//len(rows[0]);ch=sheet.height//len(rows)
            baseline=provider.get("ascent",7)*ch/provider.get("height",8)
            for row,line in enumerate(rows):
                for col,char in enumerate(line):
                    cp=ord(char)
                    if cp==0 or cp in cmap:continue
                    bitmap=sheet.crop((col*cw,row*ch,(col+1)*cw,(row+1)*ch)).getchannel("A")
                    pen=TTGlyphPen(None);bbox=bitmap.getbbox()
                    if bbox:
                        for y in range(ch):
                            x=0
                            while x<cw:
                                if bitmap.getpixel((x,y))<128:x+=1;continue
                                start=x
                                while x<cw and bitmap.getpixel((x,y))>=128:x+=1
                                x0,x1=start*64,x*64;y0,y1=round((baseline-y-1)*64),round((baseline-y)*64)
                                pen.moveTo((x0,y0));pen.lineTo((x1,y0));pen.lineTo((x1,y1));pen.lineTo((x0,y1));pen.closePath()
                    name=f"uni{cp:04X}";glyphs[name]=pen.glyph();metrics[name]=((bbox[2]+2)*64 if bbox else 384,0);cmap[cp]=name
        pen=TTGlyphPen(None);glyphs[".notdef"]=pen.glyph();metrics[".notdef"]=(512,0)
        glyphs["space"]=pen.glyph();metrics["space"]=(384,0);cmap[32]="space"
        assert all(cp in cmap for cp in range(0x410,0x450)),"Faithful Cyrillic mapping missing"
        # A few native controls use symbols absent from the supplied bitmap sheets.
        # Retain just those outlines from the already bundled fallback font.
        from fontTools.ttLib import TTFont
        from fontTools.pens.recordingPen import DecomposingRecordingPen
        from fontTools.pens.transformPen import TransformPen
        fallback=TTFont(self.root/"apps/client/assets/fonts/DejaVuSansMono.ttf")
        fallback_map=fallback.getBestCmap();glyph_set=fallback.getGlyphSet()
        scale=1024/fallback["head"].unitsPerEm
        for char in "↻▏✕":
            cp=ord(char)
            if cp in cmap or cp not in fallback_map:continue
            original=fallback_map[cp];record=DecomposingRecordingPen(glyph_set);glyph_set[original].draw(record)
            pen=TTGlyphPen(None);record.replay(TransformPen(pen,(scale,0,0,scale,0,0)))
            name=f"fallback{cp:04X}";glyphs[name]=pen.glyph();metrics[name]=(round(fallback["hmtx"][original][0]*scale),0);cmap[cp]=name
        fb=FontBuilder(1024,isTTF=True)
        fb.setupGlyphOrder([".notdef",*sorted(n for n in glyphs if n!=".notdef")]);fb.setupCharacterMap(cmap)
        fb.setupGlyf(glyphs);fb.setupHorizontalMetrics(metrics);fb.setupHorizontalHeader(ascent=1024,descent=-320)
        fb.setupNameTable({"familyName":"Faithful Voxium","styleName":"Regular","uniqueFontIdentifier":"FaithfulVoxium-32","fullName":"Faithful Voxium Regular","psName":"FaithfulVoxium-Regular"})
        fb.setupOS2(sTypoAscender=1024,sTypoDescender=-320,usWinAscent=1600,usWinDescent=500)
        fb.setupPost();fb.setupMaxp()
        output=io.BytesIO();fb.save(output);self.write("faithful.ttf",output.getvalue())
        self.manifest["font_glyphs"]=len(cmap)

    def index(self):
        self.write("manifest.json",json.dumps(self.manifest,ensure_ascii=False,indent=2).encode())
        lines=['//! Generated by tools/import_resource_packs.py. Do not edit by hand.',
               'pub(crate) const MODELS: &str = include_str!("../../../assets/resource_packs/models.json");',
               'pub(crate) fn file(name: &str) -> Option<&\'static [u8]> {', '    match name {']
        for name in sorted(self.files):
            lines.append(f'        {json.dumps(name)} => Some(include_bytes!("../../../assets/resource_packs/{name}")),')
        lines.extend(['        _ => None,','    }','}'])
        (self.root/"apps/client/src/pack_assets.rs").write_text("\n".join(lines)+"\n")

    def run(self):
        self.mobs();faithful=self.archive("Faithful-32x-1.20.1 (1).zip")
        self.blocks(faithful);self.icons(faithful);self.gui();self.font(faithful);self.index()
        print(json.dumps({"models":len(self.manifest["models"]),"blocks_replaced":len(self.manifest["blocks"]),"blocks_fallback":len(self.manifest["missing_blocks"]),"files":len(self.files),"font_glyphs":self.manifest["font_glyphs"]}))


if __name__=="__main__":
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir",type=Path,required=True)
    parser.add_argument("--root",type=Path,default=Path(__file__).resolve().parents[1])
    args=parser.parse_args();Importer(args.root,args.source_dir).run()
