//! Огонь: горит, пока есть топливо, перекидывается на горючие блоки и гаснет.
//!
//! Как и жидкости, огонь работает событийно: клетка с огнём остаётся в
//! `World::fire_active`, пока горит. Случайность — чистая функция seed, номера шага
//! и координат, поэтому пожар повторяем и не зависит от порядка загрузки чанков.

use crate::{AIR, BlockId, SOUL_FIRE, World, flammability, fluids::loaded, is_fire, is_solid};

/// Период шага огня в секундах.
pub const TICK_SECONDS: f32 = 0.5;

const SIDES: [[i32; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

fn offset(p: [i32; 3], d: [i32; 3]) -> [i32; 3] {
    [p[0] + d[0], p[1] + d[1], p[2] + d[2]]
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

fn fuel_around(world: &World, p: [i32; 3]) -> u32 {
    SIDES
        .iter()
        .filter(|d| {
            let n = offset(p, **d);
            loaded(world, n) && flammability(world.block(n)) > 0
        })
        .count() as u32
}

fn update(world: &mut World, p: [i32; 3], tick: u64) {
    let id: BlockId = world.block(p);
    if !is_fire(id) || id == SOUL_FIRE {
        // Огонь душ ничего не жжёт и не гаснет сам.
        return;
    }
    let seed = world.seed;
    let fuel = fuel_around(world, p);
    let below = offset(p, [0, -1, 0]);
    let supported = fuel > 0 || (loaded(world, below) && is_solid(world.block(below)));
    if !supported {
        world.set(p, AIR);
        return;
    }
    // Горючие соседи занимаются: сам блок сгорает и становится огнём.
    for (index, d) in SIDES.iter().enumerate() {
        let n = offset(p, *d);
        if !loaded(world, n) {
            continue;
        }
        let flammable = u32::from(flammability(world.block(n)));
        if flammable > 0 && roll(seed, tick, p, 1 + index as u64) < flammable / 6 {
            world.set(n, crate::FIRE);
        }
    }
    // Искры: огонь прыгает в соседний воздух, если рядом есть что жечь.
    if fuel > 0 {
        let mut salt = 20;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    salt += 1;
                    let n = offset(p, [dx, dy, dz]);
                    if (dx, dy, dz) == (0, 0, 0)
                        || !loaded(world, n)
                        || world.block(n) != AIR
                        || fuel_around(world, n) == 0
                    {
                        continue;
                    }
                    if roll(seed, tick, p, salt) < 4 {
                        world.set(n, crate::FIRE);
                    }
                }
            }
        }
    }
    // Без топлива огонь быстро гаснет, с топливом держится дольше.
    let burn_out = if fuel > 0 { 6 } else { 30 };
    if roll(seed, tick, p, 99) < burn_out {
        world.set(p, AIR);
    } else {
        world.fire_active.insert(p);
    }
}

/// Один шаг огня: не более `budget` клеток. Возвращает число обработанных.
pub fn tick(world: &mut World, tick: u64, budget: usize) -> usize {
    if world.fire_active.is_empty() {
        return 0;
    }
    let mut cells: Vec<[i32; 3]> = world.fire_active.iter().copied().collect();
    cells.sort_unstable();
    let mut done = 0;
    for p in cells {
        if done >= budget {
            break;
        }
        world.fire_active.remove(&p);
        done += 1;
        if loaded(world, p) {
            update(world, p, tick);
        }
    }
    done
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Chunk, FIRE, LEAVES, PLANKS, STONE, Section, WATER, WOOD, fluids};
    use std::collections::HashMap;

    const FLOOR: i32 = 60;

    fn flat_world() -> World {
        let mut world = World::new(7);
        for cx in -1..=1 {
            for cz in -1..=1 {
                let mut chunk = Chunk {
                    position: [cx, cz],
                    sections: HashMap::<i32, Section>::new(),
                };
                for x in 0..16 {
                    for z in 0..16 {
                        chunk.write([cx * 16 + x, FLOOR, cz * 16 + z], STONE);
                    }
                }
                world.insert(chunk);
            }
        }
        world.fire_active.clear();
        world
    }

    fn burn(world: &mut World, ticks: u64) {
        for step in 0..ticks {
            tick(world, step, 4096);
        }
    }

    fn count(world: &World, f: impl Fn(BlockId) -> bool) -> usize {
        let mut total = 0;
        for x in -20..20 {
            for z in -20..20 {
                for y in FLOOR + 1..FLOOR + 12 {
                    total += usize::from(f(world.block([x, y, z])));
                }
            }
        }
        total
    }

    #[test]
    fn flammability_covers_wood_leaves_and_plants_but_not_stone_torches_or_fire() {
        assert!(flammability(PLANKS) > 0 && flammability(LEAVES) > flammability(PLANKS));
        assert!(flammability(crate::POPPY) > 0 && flammability(crate::RED_BED) > 0);
        for id in [
            STONE,
            WATER,
            crate::LAVA,
            FIRE,
            SOUL_FIRE,
            crate::TORCH,
            crate::TNT,
            crate::WALL_TORCH_EAST,
        ] {
            assert_eq!(flammability(id), 0, "{id}");
        }
    }

    #[test]
    fn fire_on_bare_stone_dies_out_and_never_spreads() {
        let mut world = flat_world();
        world.set([0, FLOOR + 1, 0], FIRE);
        burn(&mut world, 60);
        assert_eq!(count(&world, is_fire), 0);
        assert_eq!(count(&world, |id| id != AIR), 0);
    }

    #[test]
    fn fire_spreads_through_a_wooden_wall_and_leaves_nothing_flammable_behind() {
        let mut world = flat_world();
        for x in -4..=4 {
            for y in FLOOR + 1..=FLOOR + 3 {
                world.set([x, y, 0], PLANKS);
            }
        }
        world.set([-4, FLOOR + 1, 1], FIRE);
        burn(&mut world, 600);
        assert_eq!(
            count(&world, |id| flammability(id) > 0),
            0,
            "стена сгорела целиком"
        );
        assert_eq!(count(&world, is_fire), 0, "и огонь погас");
        // Камень под стеной цел.
        assert_eq!(world.block([0, FLOOR, 0]), STONE);
    }

    #[test]
    fn fire_jumps_across_a_one_block_gap_in_leaves_but_not_a_wide_gap() {
        let mut world = flat_world();
        for y in FLOOR + 1..=FLOOR + 2 {
            world.set([0, y, 0], LEAVES);
            world.set([2, y, 0], LEAVES);
            world.set([12, y, 0], LEAVES);
        }
        world.set([-1, FLOOR + 1, 0], FIRE);
        burn(&mut world, 400);
        assert_eq!(world.block([0, FLOOR + 1, 0]), AIR);
        assert_eq!(
            world.block([2, FLOOR + 1, 0]),
            AIR,
            "огонь перепрыгнул через клетку"
        );
        assert_eq!(
            world.block([12, FLOOR + 1, 0]),
            LEAVES,
            "далёкое дерево цело"
        );
    }

    #[test]
    fn soul_fire_stays_and_burns_nothing_while_water_puts_regular_fire_out() {
        let mut world = flat_world();
        world.set([0, FLOOR + 1, 0], SOUL_FIRE);
        world.set([1, FLOOR + 1, 0], PLANKS);
        burn(&mut world, 200);
        assert_eq!(world.block([0, FLOOR + 1, 0]), SOUL_FIRE);
        assert_eq!(world.block([1, FLOOR + 1, 0]), PLANKS);
        let mut wet = flat_world();
        wet.set([5, FLOOR + 1, 5], WOOD);
        wet.set([5, FLOOR + 2, 5], FIRE);
        wet.set([5, FLOOR + 3, 5], WATER);
        for step in 0..40 {
            fluids::tick(&mut wet, step, 4096);
            tick(&mut wet, step, 4096);
        }
        assert_eq!(count(&wet, is_fire), 0);
    }

    #[test]
    fn fire_is_deterministic_and_respects_the_budget() {
        let build = || {
            let mut world = flat_world();
            for x in -3..=3 {
                world.set([x, FLOOR + 1, 0], LEAVES);
            }
            world.set([-3, FLOOR + 2, 0], FIRE);
            burn(&mut world, 80);
            let mut cells: Vec<_> = world.edits.iter().map(|(p, id)| (*p, *id)).collect();
            cells.sort_unstable();
            cells
        };
        assert_eq!(build(), build());
        let mut world = flat_world();
        for x in -5..5 {
            world.set([x * 2, FLOOR + 1, 3], FIRE);
        }
        assert_eq!(tick(&mut world, 1, 3), 3);
        assert!(!world.fire_active.is_empty());
    }
}
