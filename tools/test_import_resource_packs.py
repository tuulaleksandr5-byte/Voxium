"""Behavioral checks for safe, geometry-preserving pack conversion."""
import importlib.util
from pathlib import Path
import unittest
from PIL import Image


def importer():
    path = Path(__file__).with_name("import_resource_packs.py")
    if not path.exists():
        raise AssertionError("resource pack importer has not been implemented")
    spec = importlib.util.spec_from_file_location("import_resource_packs", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class ConversionTests(unittest.TestCase):
    def test_powered_wire_remains_visibly_distinct_in_the_imported_atlas(self):
        import json
        root=Path(__file__).resolve().parents[1]
        manifest=json.loads((root/'assets/resource_packs/manifest.json').read_text())
        layers={entry['name']:entry['layer'] for entry in manifest['blocks']}
        atlas=Image.open(root/'assets/resource_packs/faithful_atlas.png').convert('RGBA')
        tiles=[]
        for name in ['redstone_wire','redstone_wire_on']:
            layer=layers[name];x=(layer%6)*128;y=(layer//6)*128
            tiles.append(atlas.crop((x,y,x+128,y+128)))
        self.assertTrue(tiles[0].tobytes()!=tiles[1].tobytes(),"powered and unpowered wire must use distinct colours")
        self.assertGreater(sum(p[0] for p in tiles[1].getdata()),sum(p[0] for p in tiles[0].getdata()))

    def test_rest_scales_preserve_authored_joint_rotation_after_host_wrapper_removal(self):
        m=importer()
        source={"models":[{"part":"body","submodels":[{"id":"joint","rotate":[90,0,0],"boxes":[{"coordinates":[0,0,0,4,4,4],"textureOffset":[0,0]}]}],"animations":[{"joint.rx":"0","joint.sx":"0.5"}]}]}
        model=m.convert_model(source,Image.new('RGBA',(32,32),'white'),'cow','test')
        self.assertEqual(model['parts'][1]['rotation'],[90,0,0])
        self.assertEqual(model['parts'][1]['scale'],[0.5,1,1])

    def test_cem_multiaxis_rotation_keeps_z_then_y_then_x_order(self):
        import numpy as np
        m=importer()
        point=m.rotation_matrix([90,0,90]) @ np.array([0,1,0])
        np.testing.assert_allclose(point,[0,0,1],atol=1e-10)

    def test_cem_rest_state_hides_mutually_exclusive_bodies_without_executing_code(self):
        m=importer()
        source={"textureSize":[32,32],"models":[{"part":"body","submodels":[
            {"id":"creeper_swap","boxes":[{"coordinates":[0,0,0,4,4,4],"textureOffset":[0,0]}]},
            {"id":"zombie_swap","boxes":[{"coordinates":[0,0,0,4,4,4],"textureOffset":[0,0]}]}
        ],"animations":[{"var.swap":"if(is_aggressive, 1, 0)","creeper_swap.sx":"1-var.swap","zombie_swap.sx":"var.swap"}]}]}
        model=m.convert_model(source,Image.new('RGBA',(32,32),'white'),'creeper','test')
        self.assertIn('creeper_swap',[p['name'] for p in model['parts']])
        self.assertNotIn('zombie_swap',[p['name'] for p in model['parts']])
        with self.assertRaises(ValueError):
            m.rest_expression("__import__('os').system('echo unsafe')", {})

    def test_direct_top_geometry_animates_once_around_its_joint(self):
        m=importer()
        source={"models":[{"part":"head","id":"head","translate":[0,-12,4],"boxes":[{"coordinates":[-2,10,-6,4,4,4],"textureOffset":[0,0]}]}]}
        model=m.convert_model(source,Image.new('RGBA',(32,32),'white'),'mooshroom','test')
        self.assertEqual(model['parts'][0]['role'],'head')
        self.assertEqual(model['parts'][0]['translation'][2],-0.25)
        self.assertEqual(m.rig_role('spider_leg_right1','body',False,set()),'leg_right')

    def test_anatomical_exclusion_wrappers_never_animate_as_limbs(self):
        m = importer()
        for name in ["body_noarms", "chicken_nolegs", "spider_nolegs", "spider_legs", "bow_arms"]:
            self.assertEqual(m.rig_role(name, "body", False, set()), "fixed")
        self.assertEqual(m.rig_role("cow_left_leg", "body", False, set()), "leg_left")

    def test_alpha_prunes_other_breed_boxes_without_losing_parent_transforms(self):
        m = importer()
        skin = Image.new("RGBA", (64, 32))
        skin.paste((255, 255, 255, 255), (0, 0, 16, 16))
        source = {"textureSize": [64, 32], "models": [{
            "part": "body", "translate": [0, -24, 0], "submodels": [{
                "id": "rig", "translate": [0, 12, 0], "rotate": [0, 0, 15],
                "boxes": [
                    {"coordinates": [0, 0, 0, 4, 4, 4], "textureOffset": [0, 0]},
                    {"coordinates": [8, 0, 0, 4, 4, 4], "textureOffset": [32, 0]},
                ]}]}]}
        model = m.convert_model(source, skin, "wolf", "test")
        self.assertEqual(sum(len(p["indices"]) // 36 for p in model["parts"]), 1)
        self.assertEqual(model["parts"][1]["translation"], [0, 0.75, 0])
        self.assertEqual(model["parts"][1]["rotation"], [0, 0, 15])
        self.assertEqual(model["parts"][1]["parent"], 0)

    def test_each_face_has_outward_winding_and_uv_inside_the_skin(self):
        m = importer()
        skin = Image.new("RGBA", (32, 32), "white")
        box = {"coordinates": [-2, 0, -2, 4, 4, 4], "textureOffset": [0, 0]}
        mesh = m.box_mesh(box, skin, mirror=True)
        self.assertEqual(len(mesh["positions"]), 24)
        self.assertEqual(len(mesh["indices"]), 36)
        for uv in mesh["uvs"]:
            self.assertTrue(all(0 <= coordinate <= 1 for coordinate in uv))
        for i in range(0, len(mesh["indices"]), 3):
            a, b, c = [mesh["positions"][n] for n in mesh["indices"][i:i+3]]
            ab = [b[n]-a[n] for n in range(3)]
            ac = [c[n]-a[n] for n in range(3)]
            cross = [ab[1]*ac[2]-ab[2]*ac[1], ab[2]*ac[0]-ab[0]*ac[2], ab[0]*ac[1]-ab[1]*ac[0]]
            normal = mesh["normals"][mesh["indices"][i]]
            self.assertGreater(sum(x*y for x, y in zip(cross, normal)), 0)

    def test_faithful_aliases_resolve_colours_fluids_and_legacy_names(self):
        m = importer()
        for original, expected in [("grass_top", "grass_block_top"),
                                   ("wool_red", "red_wool"),
                                   ("water", "water_still"),
                                   ("farmland_moist", "farmland_moist"),
                                   ("stone", "stone")]:
            self.assertEqual(m.faithful_name(original), expected)

    def test_resource_paths_cannot_escape_the_destination(self):
        m = importer()
        for path in ["../secret", "/etc/passwd", "a/../../b", "a\\..\\b"]:
            with self.assertRaises(ValueError):
                m.safe_relative(path)
        self.assertEqual(m.safe_relative("mobs/wolf.png"), Path("mobs/wolf.png"))


if __name__ == "__main__":
    unittest.main()
