//! Рост саженцев в деревья.
//!
//! Саженец стоит на подходящей почве и по случайному жребию превращается в дерево
//! своей породы, если места хватает. Жребий — чистая функция seed, шага и клетки.

use crate::{
    ACACIA_LEAVES, ACACIA_LOG, ACACIA_SAPLING, AIR, BIRCH_LEAVES, BIRCH_LOG, BIRCH_SAPLING,
    BlockId, CHERRY_LEAVES, CHERRY_LOG, CHERRY_SAPLING, COARSE_DIRT, DARK_OAK_LEAVES, DARK_OAK_LOG,
    DARK_OAK_SAPLING, DIRT, GRASS, JUNGLE_LEAVES, JUNGLE_LOG, JUNGLE_SAPLING, LEAVES,
    MANGROVE_LEAVES, MANGROVE_LOG, MANGROVE_PROPAGULE, MOSS_BLOCK, MUD, OAK_SAPLING, PINE_LEAVES,
    PODZOL, SPRUCE_LEAVES, SPRUCE_LOG, SPRUCE_SAPLING, WOOD, World, fluids::loaded, is_leaves,
    is_plant,
};

/// Период шага роста в секундах.
pub const TICK_SECONDS: f32 = 5.0;
/// Шанс вырасти за шаг, в процентах: в среднем дерево появляется через полминуты.
const GROW_PERCENT: u32 = 15;

#[must_use]
pub fn is_sapling(id: BlockId) -> bool {
    species(id).is_some()
}

/// Саженец, который иногда выпадает из листвы этой породы.
#[must_use]
pub fn sapling_for_leaves(id: BlockId) -> Option<BlockId> {
    Some(match id {
        LEAVES => OAK_SAPLING,
        BIRCH_LEAVES => BIRCH_SAPLING,
        PINE_LEAVES | SPRUCE_LEAVES => SPRUCE_SAPLING,
        JUNGLE_LEAVES => JUNGLE_SAPLING,
        ACACIA_LEAVES => ACACIA_SAPLING,
        DARK_OAK_LEAVES => DARK_OAK_SAPLING,
        CHERRY_LEAVES => CHERRY_SAPLING,
        MANGROVE_LEAVES => MANGROVE_PROPAGULE,
        _ => return None,
    })
}

/// Форма кроны: пары (смещение по высоте от верха ствола, радиус).
type Layers = &'static [(i32, i32)];
const ROUND: Layers = &[(-1, 2), (0, 2), (1, 1)];
const CONE: Layers = &[(-3, 2), (-2, 2), (-1, 1), (0, 1), (1, 0)];
const WIDE: Layers = &[(-1, 3), (0, 3), (1, 1)];
const FLAT: Layers = &[(0, 3), (1, 1)];

#[derive(Clone, Copy)]
struct Species {
    log: BlockId,
    leaves: BlockId,
    min_height: i32,
    extra_height: i32,
    layers: Layers,
}

fn species(sapling: BlockId) -> Option<Species> {
    let (log, leaves, min_height, extra_height, layers) = match sapling {
        OAK_SAPLING => (WOOD, LEAVES, 4, 2, ROUND),
        SPRUCE_SAPLING => (SPRUCE_LOG, SPRUCE_LEAVES, 6, 3, CONE),
        BIRCH_SAPLING => (BIRCH_LOG, BIRCH_LEAVES, 5, 2, ROUND),
        JUNGLE_SAPLING => (JUNGLE_LOG, JUNGLE_LEAVES, 7, 3, ROUND),
        ACACIA_SAPLING => (ACACIA_LOG, ACACIA_LEAVES, 5, 2, FLAT),
        CHERRY_SAPLING => (CHERRY_LOG, CHERRY_LEAVES, 5, 2, WIDE),
        DARK_OAK_SAPLING => (DARK_OAK_LOG, DARK_OAK_LEAVES, 5, 2, WIDE),
        MANGROVE_PROPAGULE => (MANGROVE_LOG, MANGROVE_LEAVES, 4, 2, ROUND),
        _ => return None,
    };
    Some(Species {
        log,
        leaves,
        min_height,
        extra_height,
        layers,
    })
}

/// Число 0…99, зависящее только от seed, шага, клетки и «соли».
fn roll(seed: u64, tick: u64, p: [i32; 3], salt: u64) -> u32 {
    let mut value = seed
        ^ tick.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ i64::from(p[0])
            .cast_unsigned()
            .wrapping_mul(0xd1b5_4a32_d192_ed03)
        ^ i64::from(p[1])
            .cast_unsigned()
            .wrapping_mul(0x8cb9_2ba7_2f3d_8dd7)
        ^ i64::from(p[2])
            .cast_unsigned()
            .wrapping_mul(0xc2b2_ae3d_27d4_eb4f)
        ^ salt.wrapping_mul(0x1656_67b1_9e37_79f9);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    ((value ^ (value >> 31)) % 100) as u32
}

/// Выпал ли жребий с шансом `percent` для клетки `p` при «семени» `salt`.
#[must_use]
pub fn chance(seed: u64, p: [i32; 3], salt: u64, percent: u32) -> bool {
    roll(seed, salt, p, 0xC0FFEE) < percent
}

fn soil(id: BlockId) -> bool {
    matches!(id, GRASS | DIRT | PODZOL | COARSE_DIRT | MUD | MOSS_BLOCK)
}

/// Клетка, куда дерево может вырасти: воздух, растение или чужая листва.
fn free(id: BlockId) -> bool {
    id == AIR || is_plant(id) || is_leaves(id)
}

/// Клетки ствола и кроны дерева, выросшего из саженца в `p`.
fn plan(p: [i32; 3], species: Species, height: i32) -> (Vec<[i32; 3]>, Vec<[i32; 3]>) {
    let top = p[1] + height - 1;
    let trunk: Vec<[i32; 3]> = (p[1]..=top).map(|y| [p[0], y, p[2]]).collect();
    let mut crown = Vec::new();
    for (dy, radius) in species.layers {
        for dx in -radius..=*radius {
            for dz in -radius..=*radius {
                if *radius >= 2 && dx.abs() == *radius && dz.abs() == *radius {
                    continue;
                }
                let cell = [p[0] + dx, top + dy, p[2] + dz];
                if dx == 0 && dz == 0 && cell[1] <= top {
                    continue;
                }
                crown.push(cell);
            }
        }
    }
    (trunk, crown)
}

/// Пытается вырастить дерево; `false`, если нет почвы, места или чанк не загружен.
fn grow(world: &mut World, p: [i32; 3], sapling: BlockId, tick: u64) -> bool {
    let Some(species) = species(sapling) else {
        return false;
    };
    let height = species.min_height
        + (roll(world.seed, tick, p, 7) % (species.extra_height as u32 + 1)) as i32;
    let below = [p[0], p[1] - 1, p[2]];
    if !loaded(world, below) || !soil(world.block(below)) {
        return false;
    }
    let (trunk, crown) = plan(p, species, height);
    for cell in trunk.iter().chain(&crown) {
        if !loaded(world, *cell) || !free(world.block(*cell)) {
            return false;
        }
    }
    for cell in &trunk {
        world.set(*cell, species.log);
    }
    for cell in &crown {
        if world.block(*cell) == AIR || is_plant(world.block(*cell)) {
            world.set(*cell, species.leaves);
        }
    }
    true
}

/// Один шаг роста: не более `budget` саженцев. Возвращает число обработанных.
pub fn tick(world: &mut World, tick: u64, budget: usize) -> usize {
    if world.grow_active.is_empty() {
        return 0;
    }
    let mut cells: Vec<[i32; 3]> = world.grow_active.iter().copied().collect();
    cells.sort_unstable();
    let mut done = 0;
    for p in cells {
        if done >= budget {
            break;
        }
        done += 1;
        let id = if loaded(world, p) {
            world.block(p)
        } else {
            AIR
        };
        if !is_sapling(id) {
            world.grow_active.remove(&p);
            continue;
        }
        if roll(world.seed, tick, p, 3) < GROW_PERCENT {
            // Ушедший ввысь саженец перестаёт быть саженцем сам: ствол займёт его клетку.
            if grow(world, p, id, tick) {
                world.grow_active.remove(&p);
            }
        }
    }
    done
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Chunk, STONE, Section, is_leaves};
    use std::collections::HashMap;

    const GROUND: i32 = 60;

    fn world_with(soil_block: BlockId) -> World {
        let mut world = World::new(3);
        for cx in -1..=1 {
            for cz in -1..=1 {
                let mut chunk = Chunk {
                    position: [cx, cz],
                    sections: HashMap::<i32, Section>::new(),
                };
                for x in 0..16 {
                    for z in 0..16 {
                        chunk.write([cx * 16 + x, GROUND, cz * 16 + z], soil_block);
                    }
                }
                world.insert(chunk);
            }
        }
        world.grow_active.clear();
        world
    }

    fn grow_for(world: &mut World, ticks: u64) {
        for step in 0..ticks {
            tick(world, step, 64);
        }
    }

    fn count(world: &World, f: impl Fn(BlockId) -> bool) -> usize {
        let mut total = 0;
        for x in -8..8 {
            for z in -8..8 {
                for y in GROUND + 1..GROUND + 16 {
                    total += usize::from(f(world.block([x, y, z])));
                }
            }
        }
        total
    }

    #[test]
    fn every_sapling_grows_into_its_own_species_on_grass() {
        for (sapling, log, leaves) in [
            (OAK_SAPLING, WOOD, LEAVES),
            (SPRUCE_SAPLING, SPRUCE_LOG, SPRUCE_LEAVES),
            (BIRCH_SAPLING, BIRCH_LOG, BIRCH_LEAVES),
            (JUNGLE_SAPLING, JUNGLE_LOG, JUNGLE_LEAVES),
            (ACACIA_SAPLING, ACACIA_LOG, ACACIA_LEAVES),
            (CHERRY_SAPLING, CHERRY_LOG, CHERRY_LEAVES),
            (DARK_OAK_SAPLING, DARK_OAK_LOG, DARK_OAK_LEAVES),
            (MANGROVE_PROPAGULE, MANGROVE_LOG, MANGROVE_LEAVES),
        ] {
            let mut world = world_with(GRASS);
            world.set([0, GROUND + 1, 0], sapling);
            assert!(world.grow_active.contains(&[0, GROUND + 1, 0]));
            grow_for(&mut world, 400);
            let trunk = count(&world, |id| id == log);
            assert!((4..=10).contains(&trunk), "{sapling}: ствол {trunk}");
            assert!(count(&world, |id| id == leaves) >= 8, "{sapling}");
            assert_eq!(count(&world, |id| id == sapling), 0);
            assert!(world.grow_active.is_empty());
            // Ствол стоит ровно на клетке саженца, а крона над землёй не касается её.
            assert_eq!(world.block([0, GROUND + 1, 0]), log);
            assert_eq!(world.block([0, GROUND, 0]), GRASS);
        }
    }

    #[test]
    fn saplings_need_soil_and_free_space_and_wait_instead_of_giving_up() {
        let mut stone = world_with(STONE);
        stone.set([0, GROUND + 1, 0], OAK_SAPLING);
        grow_for(&mut stone, 400);
        assert_eq!(count(&stone, |id| id == WOOD), 0);
        assert_eq!(stone.block([0, GROUND + 1, 0]), OAK_SAPLING);
        // Потолок над саженцем мешает; убрали потолок — дерево выросло.
        let mut covered = world_with(DIRT);
        covered.set([0, GROUND + 4, 0], STONE);
        covered.set([0, GROUND + 1, 0], OAK_SAPLING);
        grow_for(&mut covered, 300);
        assert_eq!(covered.block([0, GROUND + 1, 0]), OAK_SAPLING);
        covered.set([0, GROUND + 4, 0], AIR);
        grow_for(&mut covered, 600);
        assert!(count(&covered, |id| id == WOOD) >= 4);
        assert!(!is_leaves(covered.block([0, GROUND + 1, 0])));
    }

    #[test]
    fn growth_is_deterministic_and_leaves_drop_the_matching_sapling() {
        let run = || {
            let mut world = world_with(GRASS);
            world.set([3, GROUND + 1, -2], BIRCH_SAPLING);
            grow_for(&mut world, 500);
            let mut cells: Vec<_> = world.edits.iter().map(|(p, id)| (*p, *id)).collect();
            cells.sort_unstable();
            cells
        };
        assert_eq!(run(), run());
        assert_eq!(sapling_for_leaves(LEAVES), Some(OAK_SAPLING));
        assert_eq!(sapling_for_leaves(PINE_LEAVES), Some(SPRUCE_SAPLING));
        assert_eq!(
            sapling_for_leaves(MANGROVE_LEAVES),
            Some(MANGROVE_PROPAGULE)
        );
        assert_eq!(sapling_for_leaves(crate::AZALEA_LEAVES), None);
        assert!((0..2000).filter(|i| chance(1, [*i, 70, 0], 5, 8)).count() < 400);
        assert!(!is_sapling(WOOD) && is_sapling(DARK_OAK_SAPLING));
    }
}
