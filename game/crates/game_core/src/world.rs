//! Compact, deterministic terrain and copy-on-write loaded chunks.

use crate::*;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

pub const SEA_LEVEL: i32 = 63;
/// Версия неизменяемого базового ландшафта Chunk::generate.
pub const GENERATOR_VERSION: u32 = 1;
pub const SECTION_VOLUME: usize = 16 * 16 * 16;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub blocks: Box<[u8; SECTION_VOLUME]>,
}
impl Default for Section {
    fn default() -> Self {
        Self {
            blocks: Box::new([AIR; SECTION_VOLUME]),
        }
    }
}
impl Section {
    /// Section-local x/y/z, each in 0..16. Invalid coordinates read as air.
    #[must_use]
    pub fn block(&self, local: [u8; 3]) -> u8 {
        if local.iter().any(|&n| n >= 16) {
            return AIR;
        }
        self.blocks[Self::index(local)]
    }
    pub fn set(&mut self, local: [u8; 3], id: u8) {
        if local.iter().all(|&n| n < 16) && usize::from(id) < BLOCKS.len() {
            self.blocks[Self::index(local)] = id;
        }
    }
    fn index([x, y, z]: [u8; 3]) -> usize {
        usize::from(x) + 16 * (usize::from(z) + 16 * usize::from(y))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    pub position: [i32; 2],
    pub sections: HashMap<i32, Section>,
}
impl Chunk {
    #[must_use]
    pub fn generate(seed: u64, position: [i32; 2]) -> Self {
        let mut chunk = Self {
            position,
            sections: HashMap::new(),
        };
        let min_x = i64::from(position[0]) * 16;
        let min_z = i64::from(position[1]) * 16;
        if min_x < i64::from(i32::MIN)
            || min_x + 15 > i64::from(i32::MAX)
            || min_z < i64::from(i32::MIN)
            || min_z + 15 > i64::from(i32::MAX)
        {
            return chunk;
        }
        // Noise is sampled once per column, not once per voxel.
        for x in min_x as i32..=(min_x + 15) as i32 {
            for z in min_z as i32..=(min_z + 15) as i32 {
                let column = terrain_column(seed, x, z);
                let cave_a = noise(seed ^ 0xCA71, f64::from(x) / 28.0, f64::from(z) / 28.0);
                let cave_b = noise(seed ^ 0xCA72, f64::from(x) / 43.0, f64::from(z) / 43.0);
                for y in WORLD_MIN_Y..=column.height.max(SEA_LEVEL) {
                    let id = if y == WORLD_MIN_Y {
                        BEDROCK
                    } else if y > column.height {
                        WATER
                    } else if y > WORLD_MIN_Y + 4
                        && y < column.height - 5
                        && ((cave_a > -0.22
                            && (f64::from(y) - (9.0 + cave_b * 23.0)).abs() < 2.4 + cave_a * 2.0)
                            || (cave_b > 0.05
                                && (f64::from(y) - (-35.0 + cave_a * 13.0)).abs() < 2.8))
                    {
                        AIR
                    } else if y < -12 {
                        DEEP_STONE
                    } else if y < column.height - 4 {
                        STONE
                    } else {
                        column.surface_block(y)
                    };
                    if id != AIR {
                        chunk.write([x, y, z], id);
                    }
                }
            }
        }
        // Every chunk evaluates neighboring structure origins too. A tree's shape
        // therefore never depends on chunk load order or mutable random state.
        for gx in (min_x - 2).div_euclid(5)..=(min_x + 17).div_euclid(5) {
            for gz in (min_z - 2).div_euclid(5)..=(min_z + 17).div_euclid(5) {
                if let Some(tree) = tree_at_cell(seed, gx, gz) {
                    chunk.plant(tree);
                }
            }
        }
        chunk
    }
    /// Read using world block coordinates. Other columns and heights are air.
    #[must_use]
    pub fn block(&self, position: [i32; 3]) -> u8 {
        if !valid_world_height(position[1])
            || [
                section_coordinate(position[0]),
                section_coordinate(position[2]),
            ] != self.position
        {
            return AIR;
        }
        self.sections
            .get(&section_coordinate(position[1]))
            .map_or(AIR, |section| section.block(position.map(local_coordinate)))
    }
    pub(crate) fn write(&mut self, position: [i32; 3], id: u8) {
        let sy = section_coordinate(position[1]);
        if id == AIR {
            if let Some(section) = self.sections.get_mut(&sy) {
                section.set(position.map(local_coordinate), AIR);
                if section.blocks.iter().all(|&id| id == AIR) {
                    self.sections.remove(&sy);
                }
            }
        } else {
            self.sections
                .entry(sy)
                .or_default()
                .set(position.map(local_coordinate), id);
        }
    }
    fn plant(&mut self, tree: Tree) {
        let top = tree.ground + tree.height;
        for dy in -1_i32..=1 {
            let radius: i32 = if dy == 1 { 1 } else { 2 };
            for dx in -radius..=radius {
                for dz in -radius..=radius {
                    if dx.abs() == 2 && dz.abs() == 2 {
                        continue;
                    }
                    let x = i64::from(tree.x) + i64::from(dx);
                    let z = i64::from(tree.z) + i64::from(dz);
                    let (Ok(x), Ok(z)) = (i32::try_from(x), i32::try_from(z)) else {
                        continue;
                    };
                    let p = [x, top + dy, z];
                    if [section_coordinate(x), section_coordinate(z)] == self.position
                        && self.block(p) == AIR
                    {
                        self.write(p, LEAVES);
                    }
                }
            }
        }
        if [section_coordinate(tree.x), section_coordinate(tree.z)] == self.position {
            for y in tree.ground + 1..=top {
                self.write([tree.x, y, tree.z], WOOD);
            }
        }
    }
}

#[derive(Debug)]
pub struct World {
    pub seed: u64,
    pub chunks: HashMap<[i32; 2], Arc<Chunk>>,
    /// Edits outlive loaded chunk data and are replayed when a chunk is inserted.
    pub edits: HashMap<[i32; 3], u8>,
    pub dirty: HashSet<[i32; 3]>,
}
impl World {
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            chunks: HashMap::new(),
            edits: HashMap::new(),
            dirty: HashSet::new(),
        }
    }
    #[must_use]
    pub fn block(&self, position: [i32; 3]) -> u8 {
        self.chunks
            .get(&[
                section_coordinate(position[0]),
                section_coordinate(position[2]),
            ])
            .map_or(AIR, |chunk| chunk.block(position))
    }
    /// Returns true only when a valid, loaded, breakable block actually changed.
    pub fn set(&mut self, position: [i32; 3], id: u8) -> bool {
        if !valid_world_height(position[1]) || usize::from(id) >= BLOCKS.len() {
            return false;
        }
        let key = [
            section_coordinate(position[0]),
            section_coordinate(position[2]),
        ];
        let Some(chunk) = self.chunks.get_mut(&key) else {
            return false;
        };
        let old = chunk.block(position);
        if old == id || old == BEDROCK {
            return false;
        }
        Arc::make_mut(chunk).write(position, id);
        self.edits.insert(position, id);
        let section = position.map(section_coordinate);
        // AO samples diagonals too: an edit at a section corner can affect
        // eight sections, not just the three face-adjacent neighbors.
        let offsets = position.map(|p| match local_coordinate(p) {
            0 => vec![0, -1],
            15 => vec![0, 1],
            _ => vec![0],
        });
        for &dx in &offsets[0] {
            for &dy in &offsets[1] {
                for &dz in &offsets[2] {
                    self.dirty
                        .insert([section[0] + dx, section[1] + dy, section[2] + dz]);
                }
            }
        }
        true
    }
    pub fn insert(&mut self, mut chunk: Chunk) {
        for (&position, &id) in &self.edits {
            if [
                section_coordinate(position[0]),
                section_coordinate(position[2]),
            ] == chunk.position
                && valid_world_height(position[1])
                && usize::from(id) < BLOCKS.len()
                && chunk.block(position) != BEDROCK
            {
                chunk.write(position, id);
            }
        }
        if let Some(old) = self.chunks.remove(&chunk.position) {
            self.dirty_chunk(&old);
        }
        self.dirty_chunk(&chunk);
        self.chunks.insert(chunk.position, Arc::new(chunk));
    }
    pub fn unload(&mut self, position: [i32; 2]) {
        if let Some(chunk) = self.chunks.remove(&position) {
            self.dirty_chunk(&chunk);
        }
    }
    fn dirty_chunk(&mut self, chunk: &Chunk) {
        for &y in chunk.sections.keys() {
            let section = [chunk.position[0], y, chunk.position[1]];
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if let (Some(x), Some(y), Some(z)) = (
                            section[0].checked_add(dx),
                            section[1].checked_add(dy),
                            section[2].checked_add(dz),
                        ) {
                            self.dirty.insert([x, y, z]);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Biome {
    Plains,
    Forest,
    Desert,
    Mountains,
    Ocean,
}

struct TerrainColumn {
    height: i32,
    biome: Biome,
}
impl TerrainColumn {
    fn surface_block(&self, y: i32) -> u8 {
        match self.biome {
            Biome::Ocean => {
                if y == self.height {
                    GRAVEL
                } else {
                    SAND
                }
            }
            Biome::Desert => {
                if y >= self.height - 2 {
                    SAND
                } else {
                    SANDSTONE
                }
            }
            Biome::Mountains if self.height > 115 => {
                if y == self.height {
                    SNOW
                } else {
                    STONE
                }
            }
            Biome::Mountains if self.height > 94 => STONE,
            _ if self.height <= SEA_LEVEL + 1 => SAND,
            _ => {
                if y == self.height {
                    GRASS
                } else {
                    DIRT
                }
            }
        }
    }
}

fn terrain_column(seed: u64, x: i32, z: i32) -> TerrainColumn {
    let (x, z) = (f64::from(x), f64::from(z));
    let continent = noise(seed ^ 0xC017, x / 360.0, z / 360.0);
    let moisture = noise(seed ^ 0xF012, x / 190.0, z / 190.0);
    let peaks = noise(seed ^ 0xB017, x / 220.0, z / 220.0);
    let detail = noise(seed ^ 0xD317, x / 38.0, z / 38.0);
    let mountain = smooth(((peaks - 0.1) / 0.7).clamp(0.0, 1.0));
    let land = smooth(((continent + 0.25) / 0.5).clamp(0.0, 1.0));
    let height = (67.0 + continent * 27.0 + detail * 5.0 + mountain * land * 65.0).floor() as i32;
    let biome = if height < SEA_LEVEL - 2 {
        Biome::Ocean
    } else if mountain > 0.28 && land > 0.45 {
        Biome::Mountains
    } else if moisture < -0.22 {
        Biome::Desert
    } else if moisture > 0.1 {
        Biome::Forest
    } else {
        Biome::Plains
    };
    TerrainColumn {
        height: height.clamp(WORLD_MIN_Y + 8, WORLD_MAX_Y - 8),
        biome,
    }
}

#[must_use]
pub fn surface_height(seed: u64, x: i32, z: i32) -> i32 {
    terrain_column(seed, x, z).height
}
#[must_use]
pub fn biome_at(seed: u64, x: i32, z: i32) -> Biome {
    terrain_column(seed, x, z).biome
}

#[derive(Clone, Copy)]
struct Tree {
    x: i32,
    z: i32,
    ground: i32,
    height: i32,
}
fn tree_at_cell(seed: u64, gx: i64, gz: i64) -> Option<Tree> {
    let hash = coordinate_hash(seed ^ 0x7EEE, gx, gz);
    let x = i32::try_from(gx * 5 + (hash % 5) as i64).ok()?;
    let z = i32::try_from(gz * 5 + ((hash >> 12) % 5) as i64).ok()?;
    let column = terrain_column(seed, x, z);
    let allowed = match column.biome {
        Biome::Forest => !hash.is_multiple_of(4),
        Biome::Plains => hash.is_multiple_of(7),
        _ => false,
    };
    (allowed && column.height > SEA_LEVEL + 1).then_some(Tree {
        x,
        z,
        ground: column.height,
        height: 4 + ((hash >> 24) % 2) as i32,
    })
}

// Integer-coordinate value noise has a stable hash and no platform-dependent RNG.
fn mix(mut value: u64) -> u64 {
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}
fn coordinate_hash(seed: u64, x: i64, z: i64) -> u64 {
    mix(seed
        ^ (x as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (z as u64).wrapping_mul(0xd1b5_4a32_d192_ed03))
}
fn smooth(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}
fn noise(seed: u64, x: f64, z: f64) -> f64 {
    let (ix, iz) = (x.floor() as i64, z.floor() as i64);
    let (tx, tz) = (smooth(x - x.floor()), smooth(z - z.floor()));
    let sample =
        |x, z| (coordinate_hash(seed, x, z) >> 11) as f64 / ((1_u64 << 53) - 1) as f64 * 2.0 - 1.0;
    let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
    lerp(
        lerp(sample(ix, iz), sample(ix + 1, iz), tx),
        lerp(sample(ix, iz + 1), sample(ix + 1, iz + 1), tx),
        tz,
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    fn empty_chunk(position: [i32; 2]) -> Chunk {
        Chunk {
            position,
            sections: HashMap::new(),
        }
    }

    #[test]
    fn noise_terrain_has_five_biomes_and_continuous_heights() {
        let mut biomes = HashSet::new();
        for x in (-5000..5000).step_by(83) {
            for z in (-5000..5000).step_by(83) {
                biomes.insert(biome_at(5, x, z));
                let height = surface_height(5, x, z);
                assert!((surface_height(5, x + 1, z) - height).abs() <= 3);
                assert!((WORLD_MIN_Y + 8..WORLD_MAX_Y - 8).contains(&height));
            }
        }
        assert_eq!(biomes.len(), 5);
    }

    #[test]
    fn water_and_caves_are_generated() {
        let chunk = Chunk::generate(3, [0, 0]);
        let mut caves = 0;
        for x in 0..16 {
            for z in 0..16 {
                for y in WORLD_MIN_Y + 5..surface_height(3, x, z) - 5 {
                    caves += usize::from(chunk.block([x, y, z]) == AIR);
                }
            }
        }
        assert!(caves > 0);
        let ocean = (-1000..1000)
            .step_by(37)
            .find_map(|x| {
                (-1000..1000)
                    .step_by(37)
                    .find_map(|z| (surface_height(3, x, z) < SEA_LEVEL - 2).then_some([x, z]))
            })
            .expect("seed has an ocean");
        let chunk = Chunk::generate(3, ocean.map(section_coordinate));
        assert_eq!(chunk.block([ocean[0], SEA_LEVEL, ocean[1]]), WATER);
    }

    #[test]
    fn trees_cross_chunk_boundaries_independent_of_generation_order() {
        let tree = (-200..200)
            .find_map(|gx| {
                (-200..200)
                    .find_map(|gz| tree_at_cell(5, gx, gz).filter(|t| local_coordinate(t.x) >= 14))
            })
            .expect("seed has a tree crossing a chunk border");
        let cx = section_coordinate(tree.x);
        let cz = section_coordinate(tree.z);
        let neighbor = Chunk::generate(5, [cx + 1, cz]);
        let trunk = Chunk::generate(5, [cx, cz]);
        assert_eq!(trunk.block([tree.x, tree.ground + 1, tree.z]), WOOD);
        let canopy_y = tree.ground + tree.height;
        assert_eq!(neighbor.block([(cx + 1) * 16, canopy_y, tree.z]), LEAVES);
        assert_eq!(neighbor, Chunk::generate(5, [cx + 1, cz]));
    }

    #[test]
    fn section_storage_is_compact_and_coordinates_do_not_alias() {
        let mut section = Section::default();
        section.set([15, 0, 0], STONE);
        section.set([0, 1, 0], GLASS);
        section.set([0, 0, 1], WOOD);
        assert_eq!(section.blocks.len(), 4096);
        assert_eq!(section.block([15, 0, 0]), STONE);
        assert_eq!(section.block([0, 1, 0]), GLASS);
        assert_eq!(section.block([0, 0, 1]), WOOD);
        assert_eq!(section.block([0, 0, 0]), AIR);
    }

    #[test]
    fn legacy_generator_matches_independent_foundation_fingerprints() {
        for (seed, pos, expected) in [
            (1, [0, 0], 0x9cdc8677c85fa51b),
            (42, [-1, 0], 0xf8cd49d7c5d1be9a),
            (3, [-1, -1], 0x864f28e2bf98744f),
        ] {
            let chunk = Chunk::generate(seed, pos);
            let mut hash = 0xcbf29ce484222325_u64;
            for y in -64..=320 {
                for z in 0..16 {
                    for x in 0..16 {
                        let id = chunk.block([pos[0] * 16 + x, y, pos[1] * 16 + z]);
                        hash = (hash ^ u64::from(id)).wrapping_mul(0x100000001b3);
                    }
                }
            }
            assert_eq!(hash, expected, "seed={seed}, pos={pos:?}");
        }
    }

    #[test]
    fn generation_is_deterministic_seeded_bounded_and_compact() {
        let a = Chunk::generate(100, [-1, -1]);
        assert_eq!(a, Chunk::generate(100, [-1, -1]));
        assert_ne!(a, Chunk::generate(101, [-1, -1]));
        assert!(!a.sections.is_empty());
        for section in a.sections.values() {
            assert!(section.blocks.iter().any(|&id| id != AIR));
            assert!(section.blocks.iter().all(|&id| id < BLOCKS.len() as u8));
        }
        for x in -16..0 {
            for z in -16..0 {
                assert_eq!(a.block([x, WORLD_MIN_Y, z]), BEDROCK);
                assert_eq!(a.block([x, WORLD_MIN_Y - 1, z]), AIR);
                assert_eq!(a.block([x, WORLD_MAX_Y + 1, z]), AIR);
            }
        }
        assert_eq!(a.block([0, WORLD_MIN_Y, -1]), AIR);
    }

    #[test]
    fn edits_at_negative_corners_dirty_only_touching_sections() {
        let mut world = World::new(10);
        world.insert(empty_chunk([-1, -1]));
        world.dirty.clear();
        assert!(world.set([-1, 15, -1], GLASS));
        assert_eq!(world.block([-1, 15, -1]), GLASS);
        assert_eq!(
            world.dirty,
            HashSet::from([
                [-1, 0, -1],
                [0, 0, -1],
                [-1, 1, -1],
                [-1, 0, 0],
                [0, 1, -1],
                [0, 0, 0],
                [-1, 1, 0],
                [0, 1, 0]
            ])
        );
        world.dirty.clear();
        assert!(world.set([-8, 8, -8], STONE));
        assert_eq!(world.dirty, HashSet::from([[-1, 0, -1]]));
    }

    #[test]
    fn boundary_edits_persist_across_unload_reload_and_arc_snapshots() {
        let mut world = World::new(7);
        world.insert(empty_chunk([0, 0]));
        let snapshot = Arc::clone(&world.chunks[&[0, 0]]);
        assert!(world.set([0, 320, 0], WOOD));
        assert_eq!(snapshot.block([0, 320, 0]), AIR);
        world.unload([0, 0]);
        assert_eq!(world.block([0, 320, 0]), AIR);
        assert_eq!(world.edits.get(&[0, 320, 0]), Some(&WOOD));
        world.insert(empty_chunk([0, 0]));
        assert_eq!(world.block([0, 320, 0]), WOOD);
        assert!(world.set([0, 320, 0], AIR));
        assert!(!world.chunks[&[0, 0]].sections.contains_key(&20));
        world.unload([0, 0]);
        world.insert(empty_chunk([0, 0]));
        assert_eq!(world.block([0, 320, 0]), AIR);
    }

    #[test]
    fn invalid_unloaded_and_bedrock_writes_are_rejected() {
        let mut world = World::new(1);
        assert!(!world.set([0, 70, 0], STONE));
        world.insert(Chunk::generate(1, [0, 0]));
        assert!(!world.set([0, -65, 0], STONE));
        assert!(!world.set([0, 321, 0], STONE));
        assert!(!world.set([0, 70, 0], 26));
        assert!(!world.set([0, -64, 0], AIR));
        assert!(!world.set([0, -64, 0], STONE));
        assert_eq!(world.block([0, -64, 0]), BEDROCK);
        assert!(world.edits.is_empty());
    }

    #[test]
    fn loading_and_unloading_dirty_neighbor_section_boundaries() {
        let mut world = World::new(5);
        let mut chunk = empty_chunk([0, 0]);
        let mut section = Section::default();
        section.set([15, 8, 0], STONE);
        chunk.sections.insert(4, section);
        world.insert(chunk);
        assert!(world.dirty.contains(&[0, 4, 0]));
        assert!(world.dirty.contains(&[1, 4, 0]));
        assert!(
            world.dirty.contains(&[1, 5, -1]),
            "loading changes diagonal AO"
        );
        world.dirty.clear();
        world.unload([0, 0]);
        assert!(world.dirty.contains(&[1, 4, 0]));
        assert!(
            world.dirty.contains(&[1, 5, -1]),
            "unloading changes diagonal AO"
        );
    }
}
