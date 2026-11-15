//! Течение воды и лавы. Состояние жидкости целиком хранится в ID блока:
//! источник и несколько уровней течения у каждой жидкости.
//!
//! Симуляция событийная: `World::set` будит клетки рядом с жидкостью, а
//! `tick` обрабатывает их порциями. Океан, созданный генератором, не считается,
//! пока рядом ничего не меняется. Нигде не читаются незагруженные чанки: жидкость
//! в них не течёт.

use crate::{
    AIR, COBBLE, FLOW_LAVA_1, FLOW_WATER_1, LAVA, OBSIDIAN, STONE, WATER, World, is_lava, is_plant,
    is_solid, valid_world_height,
};

/// Лава течёт в шесть раз реже воды.
pub const LAVA_EVERY: u64 = 6;
/// Период шага воды в секундах.
pub const TICK_SECONDS: f32 = 0.25;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FluidKind {
    Water,
    Lava,
}

impl FluidKind {
    /// Уровень источника: у воды 8 (течёт на 7 клеток), у лавы 4 (на 3 клетки).
    #[must_use]
    pub const fn source_level(self) -> u8 {
        match self {
            Self::Water => 8,
            Self::Lava => 4,
        }
    }
    /// Самый высокий уровень течения; столько получает падающая струя.
    #[must_use]
    pub const fn max_flow(self) -> u8 {
        self.source_level() - 1
    }
    /// ID блока для уровня `1..=source_level`.
    #[must_use]
    pub const fn block(self, level: u8) -> u8 {
        match (self, level) {
            (Self::Water, 8..) => WATER,
            (Self::Water, _) => FLOW_WATER_1 + level.saturating_sub(1),
            (Self::Lava, 4..) => LAVA,
            (Self::Lava, _) => FLOW_LAVA_1 + level.saturating_sub(1),
        }
    }
}

/// Вид и уровень жидкого блока.
#[must_use]
pub fn fluid_of(id: u8) -> Option<(FluidKind, u8)> {
    match id {
        WATER => Some((FluidKind::Water, 8)),
        FLOW_WATER_1..=190 => Some((FluidKind::Water, id - FLOW_WATER_1 + 1)),
        191 => Some((FluidKind::Water, 7)),
        LAVA => Some((FluidKind::Lava, 4)),
        FLOW_LAVA_1..=195 => Some((FluidKind::Lava, id - FLOW_LAVA_1 + 1)),
        _ => None,
    }
}

/// Высота поверхности блока жидкости, 0…1. Источник воды и столб под
/// такой же жидкостью — полный блок.
#[must_use]
pub fn surface_height(id: u8, above: u8) -> f32 {
    let Some((kind, level)) = fluid_of(id) else {
        return 0.0;
    };
    if fluid_of(above).is_some_and(|(other, _)| other == kind) {
        return 1.0;
    }
    f32::from(level) / f32::from(kind.source_level())
}

fn is_source(id: u8) -> bool {
    fluid_of(id).is_some_and(|(kind, level)| level == kind.source_level())
}

fn replaceable(id: u8) -> bool {
    id == AIR || is_plant(id)
}

fn loaded(world: &World, p: [i32; 3]) -> bool {
    valid_world_height(p[1])
        && world
            .chunks
            .contains_key(&[p[0].div_euclid(16), p[2].div_euclid(16)])
}

const SIDES: [[i32; 3]; 4] = [[1, 0, 0], [-1, 0, 0], [0, 0, 1], [0, 0, -1]];

fn offset(p: [i32; 3], d: [i32; 3]) -> [i32; 3] {
    [p[0] + d[0], p[1] + d[1], p[2] + d[2]]
}

/// Жидкость растекается вбок, только если под ней опора: твёрдый блок,
/// источник такой же жидкости или другая жидкость. Падающая струя не растекается.
fn can_spread(world: &World, p: [i32; 3], kind: FluidKind) -> bool {
    let below = offset(p, [0, -1, 0]);
    if !loaded(world, below) {
        return true;
    }
    let id = world.block(below);
    if replaceable(id) {
        return false;
    }
    match fluid_of(id) {
        Some((other, _)) if other == kind => is_source(id),
        _ => true,
    }
}

/// Уровень, который «должен» быть у течения в клетке: поток сверху даёт
/// максимум, соседи с опорой отдают свой уровень минус один.
fn expected_level(world: &World, p: [i32; 3], kind: FluidKind) -> u8 {
    if fluid_of(world.block(offset(p, [0, 1, 0]))).is_some_and(|(other, _)| other == kind) {
        return kind.max_flow();
    }
    let mut best = 0;
    for d in SIDES {
        let n = offset(p, d);
        if !loaded(world, n) {
            continue;
        }
        if let Some((other, level)) = fluid_of(world.block(n))
            && other == kind
            && level > 1
            && can_spread(world, n, kind)
        {
            best = best.max(level - 1);
        }
    }
    best
}

/// Два источника воды рядом при опоре снизу порождают третий.
fn infinite_source(world: &World, p: [i32; 3]) -> bool {
    let sources = SIDES
        .iter()
        .filter(|d| world.block(offset(p, **d)) == WATER)
        .count();
    let below = world.block(offset(p, [0, -1, 0]));
    sources >= 2 && (is_solid(below) || below == WATER)
}

/// Встреча воды с лавой: источник лавы превращается в обсидиан, течение — в булыжник;
/// лава, дотекшая до воды, застывает камнем. Возвращает `true`, если реакция была.
fn react(world: &mut World, pusher: FluidKind, target: [i32; 3], target_id: u8) -> bool {
    match (pusher, fluid_of(target_id)) {
        (FluidKind::Water, Some((FluidKind::Lava, level))) => {
            let result = if level == FluidKind::Lava.source_level() {
                OBSIDIAN
            } else {
                COBBLE
            };
            world.set(target, result);
            true
        }
        (FluidKind::Lava, Some((FluidKind::Water, _))) => {
            world.set(target, STONE);
            true
        }
        _ => false,
    }
}

fn push(world: &mut World, p: [i32; 3], kind: FluidKind, level: u8) {
    let below = offset(p, [0, -1, 0]);
    if loaded(world, below) {
        let id = world.block(below);
        if replaceable(id) {
            world.set(below, kind.block(kind.max_flow()));
            return;
        }
        if let Some((other, below_level)) = fluid_of(id) {
            if other != kind {
                react(world, kind, below, id);
                return;
            }
            if !is_source(id) {
                if below_level < kind.max_flow() {
                    world.set(below, kind.block(kind.max_flow()));
                }
                return;
            }
        }
    }
    if level <= 1 {
        return;
    }
    let spread = level - 1;
    for d in SIDES {
        let n = offset(p, d);
        if !loaded(world, n) {
            continue;
        }
        let id = world.block(n);
        if replaceable(id) {
            world.set(n, kind.block(spread));
        } else if let Some((other, neighbor_level)) = fluid_of(id) {
            if other != kind {
                react(world, kind, n, id);
            } else if neighbor_level < spread && !is_source(id) {
                world.set(n, kind.block(spread));
            }
        }
    }
}

fn update(world: &mut World, p: [i32; 3], id: u8) {
    let Some((kind, level)) = fluid_of(id) else {
        return;
    };
    if level < kind.source_level() {
        if kind == FluidKind::Water && infinite_source(world, p) {
            world.set(p, WATER);
            return;
        }
        let expected = expected_level(world, p, kind);
        if expected == 0 {
            world.set(p, AIR);
            return;
        }
        if expected != level {
            world.set(p, kind.block(expected));
            return;
        }
    }
    push(world, p, kind, level);
}

/// Один шаг жидкостей: обрабатывает не более `budget` разбуженных клеток.
/// `tick` — номер шага; клетки лавы ждут, пока он не кратен `LAVA_EVERY`.
/// Возвращает число обработанных клеток.
pub fn tick(world: &mut World, tick: u64, budget: usize) -> usize {
    if world.fluid_active.is_empty() {
        return 0;
    }
    let mut cells: Vec<[i32; 3]> = world.fluid_active.iter().copied().collect();
    cells.sort_unstable();
    let mut done = 0;
    for p in cells {
        if done >= budget {
            break;
        }
        let id = world.block(p);
        if is_lava(id) && !tick.is_multiple_of(LAVA_EVERY) {
            continue;
        }
        world.fluid_active.remove(&p);
        done += 1;
        if loaded(world, p) {
            update(world, p, id);
        }
    }
    done
}

/// Где находится тело игрока относительно жидкостей.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BodyFluid {
    /// Ноги или середина тела в воде.
    pub water: bool,
    /// Глаза под водой.
    pub head: bool,
    /// Любая часть тела в лаве.
    pub lava: bool,
}

impl BodyFluid {
    #[must_use]
    pub fn any(self) -> bool {
        self.water || self.lava
    }
}

/// Проверяет клетки у ног, посередине тела и на уровне глаз.
#[must_use]
pub fn body_fluid(feet: [f32; 3], height: f32, sample: impl Fn([i32; 3]) -> u8) -> BodyFluid {
    let cell = |dy: f32| {
        sample([
            feet[0].floor() as i32,
            (feet[1] + dy).floor() as i32,
            feet[2].floor() as i32,
        ])
    };
    let (low, mid, eye) = (cell(0.05), cell(height * 0.5), cell(height - 0.15));
    BodyFluid {
        water: crate::is_water(low) || crate::is_water(mid),
        head: crate::is_water(eye),
        lava: is_lava(low) || is_lava(mid) || is_lava(eye),
    }
}

/// Куда течение сносит тело в клетке `p`: единичный вектор (x, z) или нули.
/// Вода стекает в соседние клетки с меньшим уровнем и в свободное место.
#[must_use]
pub fn flow_direction(p: [i32; 3], sample: impl Fn([i32; 3]) -> u8) -> [f32; 2] {
    let Some((kind, level)) = fluid_of(sample(p)) else {
        return [0.0; 2];
    };
    let mut push = [0.0_f32; 2];
    for d in SIDES {
        let id = sample(offset(p, d));
        let neighbor = match fluid_of(id) {
            Some((other, l)) if other == kind => l,
            Some(_) => continue,
            None if replaceable(id) => 0,
            None => continue,
        };
        let drop = f32::from(level) - f32::from(neighbor);
        if drop > 0.0 {
            push[0] += d[0] as f32 * drop;
            push[1] += d[2] as f32 * drop;
        }
    }
    let length = (push[0] * push[0] + push[1] * push[1]).sqrt();
    if length < 1e-4 {
        [0.0; 2]
    } else {
        [push[0] / length, push[1] / length]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BLOCKS, Chunk, FLOW_LAVA_3, FLOW_WATER_7, Section};
    use std::collections::HashMap;

    const FLOOR: i32 = 60;

    /// Плоский каменный пол на y = 60 в 3×3 чанках вокруг начала координат.
    fn flat_world() -> World {
        let mut world = World::new(1);
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
        world.fluid_active.clear();
        world
    }

    fn settle(world: &mut World, ticks: u64) {
        for step in 0..ticks {
            tick(world, step, 4096);
        }
    }

    fn count(world: &World, f: impl Fn(u8) -> bool) -> usize {
        let mut total = 0;
        for x in -24..24 {
            for z in -24..24 {
                for y in FLOOR - 8..FLOOR + 8 {
                    total += usize::from(f(world.block([x, y, z])));
                }
            }
        }
        total
    }

    #[test]
    fn levels_round_trip_and_cover_every_fluid_id() {
        for kind in [FluidKind::Water, FluidKind::Lava] {
            for level in 1..=kind.source_level() {
                assert_eq!(fluid_of(kind.block(level)), Some((kind, level)));
            }
        }
        let fluids = (0..BLOCKS.len() as u8)
            .filter(|id| fluid_of(*id).is_some())
            .count();
        assert_eq!(fluids, 12);
        assert_eq!(FluidKind::Water.block(7), FLOW_WATER_7);
        assert_eq!(FluidKind::Lava.block(3), FLOW_LAVA_3);
        assert!(!crate::is_solid(WATER) && !crate::is_solid(LAVA));
        assert_eq!(surface_height(WATER, AIR), 1.0);
        assert_eq!(surface_height(FluidKind::Water.block(4), AIR), 0.5);
        assert_eq!(surface_height(FluidKind::Water.block(4), WATER), 1.0);
        assert_eq!(surface_height(FluidKind::Lava.block(2), AIR), 0.5);
    }

    #[test]
    fn water_spreads_seven_blocks_with_falling_levels_and_stops() {
        let mut world = flat_world();
        world.set([0, FLOOR + 1, 0], WATER);
        settle(&mut world, 40);
        assert_eq!(world.block([7, FLOOR + 1, 0]), FluidKind::Water.block(1));
        assert_eq!(world.block([3, FLOOR + 1, 0]), FluidKind::Water.block(5));
        assert_eq!(world.block([8, FLOOR + 1, 0]), AIR);
        assert_eq!(
            world.block([5, FLOOR + 1, 3]),
            AIR,
            "диагональ дальше ромба"
        );
        assert_eq!(world.block([4, FLOOR + 1, 3]), FluidKind::Water.block(1));
        assert_eq!(count(&world, is_lava), 0);
        // Повторные шаги ничего не меняют: поле устойчиво.
        let before = world.edits.clone();
        settle(&mut world, 10);
        assert_eq!(world.edits, before);
    }

    #[test]
    fn water_falls_down_a_shaft_and_spreads_on_the_bottom() {
        let mut world = flat_world();
        for y in FLOOR + 1..FLOOR + 6 {
            for d in SIDES {
                world.set(offset([0, y, 0], d), STONE);
            }
        }
        world.set([0, FLOOR + 6, 0], WATER);
        settle(&mut world, 60);
        for y in FLOOR + 1..=FLOOR + 5 {
            assert_eq!(world.block([0, y, 0]), FluidKind::Water.block(7), "y={y}");
        }
        // Источник над падающей струёй сам вбок не течёт: сначала вниз.
        assert_eq!(world.block([1, FLOOR + 6, 0]), AIR);
        assert_eq!(world.block([0, FLOOR + 6, 0]), WATER);
    }

    #[test]
    fn removing_the_source_drains_every_flowing_block() {
        let mut world = flat_world();
        world.set([0, FLOOR + 1, 0], WATER);
        settle(&mut world, 40);
        assert!(count(&world, crate::is_water) > 30);
        world.set([0, FLOOR + 1, 0], AIR);
        settle(&mut world, 80);
        assert_eq!(count(&world, crate::is_water), 0);
    }

    #[test]
    fn two_sources_make_a_third_but_one_does_not() {
        let mut world = flat_world();
        world.set([0, FLOOR + 1, 0], WATER);
        world.set([2, FLOOR + 1, 0], WATER);
        settle(&mut world, 20);
        assert_eq!(world.block([1, FLOOR + 1, 0]), WATER);
        // Одиночный источник сам себя не размножает.
        let mut single = flat_world();
        single.set([0, FLOOR + 1, 0], WATER);
        settle(&mut single, 40);
        assert_eq!(count(&single, |id| id == WATER), 1);
    }

    #[test]
    fn lava_is_slower_shorter_and_water_turns_it_to_stone() {
        let mut world = flat_world();
        world.set([0, FLOOR + 1, 0], LAVA);
        for step in 0..LAVA_EVERY {
            tick(&mut world, step, 4096);
        }
        assert_eq!(world.block([1, FLOOR + 1, 0]), FluidKind::Lava.block(3));
        assert_eq!(
            world.block([3, FLOOR + 1, 0]),
            AIR,
            "до третьей клетки за один шаг лавы не дотечь"
        );
        settle(&mut world, 100);
        assert_eq!(world.block([3, FLOOR + 1, 0]), FluidKind::Lava.block(1));
        assert_eq!(world.block([4, FLOOR + 1, 0]), AIR);
        // Вода рядом с источником лавы даёт обсидиан, с течением — булыжник.
        world.set([0, FLOOR + 2, 0], WATER);
        settle(&mut world, 60);
        assert_eq!(world.block([0, FLOOR + 1, 0]), OBSIDIAN);
        assert!(count(&world, |id| id == COBBLE) > 0);
    }

    #[test]
    fn lava_reaching_water_makes_stone_and_no_lava_enters_it() {
        let mut world = flat_world();
        world.set([3, FLOOR + 1, 0], WATER);
        world.set([0, FLOOR + 1, 0], LAVA);
        settle(&mut world, 120);
        assert!(
            count(&world, |id| id == STONE) > 17 * 17 / 4 + 1,
            "камень застыл на границе"
        );
        assert!(count(&world, |id| id == OBSIDIAN || id == COBBLE || id == STONE) > 0);
    }

    #[test]
    fn nothing_flows_into_unloaded_chunks_and_the_budget_is_respected() {
        let mut world = flat_world();
        world.unload([1, 0]);
        world.set([14, FLOOR + 1, 0], WATER);
        settle(&mut world, 30);
        assert!(count(&world, crate::is_water) > 1);
        assert!(!world.chunks.contains_key(&[1, 0]));
        assert_eq!(world.block([16, FLOOR + 1, 0]), AIR);
        // Бюджет ограничивает работу за шаг.
        let mut busy = flat_world();
        for x in -6..6 {
            busy.set([x * 2, FLOOR + 1, 0], WATER);
        }
        let processed = tick(&mut busy, 1, 5);
        assert_eq!(processed, 5);
        assert!(!busy.fluid_active.is_empty());
    }

    #[test]
    fn body_probe_and_current_follow_levels() {
        let mut world = flat_world();
        world.set([0, FLOOR + 1, 0], WATER);
        settle(&mut world, 40);
        let sample = |p| world.block(p);
        let swimmer = body_fluid([0.5, FLOOR as f32 + 1.0, 0.5], 1.8, sample);
        assert!(swimmer.water && !swimmer.head && !swimmer.lava);
        let diver = body_fluid([0.5, FLOOR as f32 + 0.5, 0.5], 1.8, |p| {
            if p[1] > FLOOR { WATER } else { STONE }
        });
        assert!(diver.water && diver.head);
        assert!(!body_fluid([9.5, FLOOR as f32 + 1.0, 9.5], 1.8, sample).any());
        let lava = body_fluid([0.5, 3.0, 0.5], 1.8, |_| LAVA);
        assert!(lava.lava && lava.any());
        // Течение сносит от центра: в клетке с уровнем 6 вектор смотрит наружу.
        let push = flow_direction([1, FLOOR + 1, 0], sample);
        assert!(push[0] > 0.9 && push[1].abs() < 0.1, "{push:?}");
        assert_eq!(flow_direction([0, FLOOR + 1, 0], |_| AIR), [0.0; 2]);
    }
}
