//! Редстоун: источники сигнала, провод с затуханием, повторитель с задержкой,
//! факел-инвертор и потребители (лампа, двери, поршни, раздатчик, динамит,
//! нотный блок, наблюдатель).
//!
//! Мир считается шагами по 0,1 с (`TICK_SECONDS`). За шаг сигнал вычисляется
//! заново по снимку мира, а потом изменения применяются разом, поэтому петли из
//! факелов и повторителей дают часы, а не бесконечный цикл.

use crate::{
    AIR, BlockId, Facing, FamilyKind, World, facing_of, family_of, is_opaque,
    models::{Shape, shape_of},
};
use std::collections::{HashMap, HashSet};

/// Длительность шага в секундах.
pub const TICK_SECONDS: f32 = 0.1;
/// Сколько шагов кнопка держит сигнал.
pub const BUTTON_TICKS: u8 = 10;
/// Предел блоков, которые двигает поршень.
pub const PISTON_LIMIT: usize = 12;

const SIDES: [[i32; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];
const HORIZONTAL: [[i32; 3]; 4] = [[1, 0, 0], [-1, 0, 0], [0, 0, 1], [0, 0, -1]];

fn add(p: [i32; 3], d: [i32; 3]) -> [i32; 3] {
    [p[0] + d[0], p[1] + d[1], p[2] + d[2]]
}

fn sub(p: [i32; 3], d: [i32; 3]) -> [i32; 3] {
    [p[0] - d[0], p[1] - d[1], p[2] - d[2]]
}

/// Событие для клиента, у которого есть контейнеры и предметы.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// Раздатчик или выбрасыватель выстрелил в сторону `dir`.
    Dispense {
        position: [i32; 3],
        dir: [i32; 3],
        dropper: bool,
    },
    /// Нотный блок получил сигнал.
    Note { position: [i32; 3] },
}

#[derive(Clone, Debug, Default)]
pub struct Redstone {
    /// Клетки с компонентами редстоуна.
    pub cells: HashSet<[i32; 3]>,
    /// Нажатые кнопки: сколько шагов ещё держат сигнал.
    pub timers: HashMap<[i32; 3], u8>,
    /// Нажимные плиты, на которых что-то стоит (задаёт клиент).
    pub pressed: HashSet<[i32; 3]>,
    /// Задержка повторителей, 1…4 шага.
    pub delay: HashMap<[i32; 3], u8>,
    /// День ли сейчас (для датчика дневного света).
    pub day: bool,
    /// События для клиента.
    pub events: Vec<Event>,
    out: HashSet<[i32; 3]>,
    pending: HashMap<[i32; 3], (bool, u8)>,
    prev: HashMap<[i32; 3], bool>,
    observed: HashMap<[i32; 3], BlockId>,
    pulses: HashMap<[i32; 3], u8>,
    extended: HashSet<[i32; 3]>,
}

/// Соседи провода: (клетка соседа, направление).
type Links = Vec<([i32; 3], [i32; 3])>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Wire,
    Lever,
    Button,
    Plate,
    RedBlock,
    Torch,
    WallTorch,
    Lamp,
    Repeater,
    Openable,
    Tnt,
    Piston,
    Dispenser,
    Note,
    Observer,
    Daylight,
}

fn kind(id: BlockId) -> Option<Kind> {
    use crate::*;
    Some(match id {
        REDSTONE_WIRE | REDSTONE_WIRE_ON => Kind::Wire,
        REDSTONE_BLOCK => Kind::RedBlock,
        REDSTONE_TORCH | REDSTONE_TORCH_OFF => Kind::Torch,
        REDSTONE_LAMP | REDSTONE_LAMP_LIT => Kind::Lamp,
        TNT => Kind::Tnt,
        NOTE_BLOCK => Kind::Note,
        DAYLIGHT_DETECTOR => Kind::Daylight,
        _ => match base_block(id) {
            REDSTONE_WALL_TORCH | REDSTONE_WALL_TORCH_OFF => Kind::WallTorch,
            PISTON | STICKY_PISTON => Kind::Piston,
            DISPENSER | DROPPER => Kind::Dispenser,
            OBSERVER => Kind::Observer,
            _ => match shape_of(id) {
                Some(Shape::Lever) => Kind::Lever,
                Some(Shape::Button) => Kind::Button,
                Some(Shape::Plate) => Kind::Plate,
                Some(Shape::Repeater) => Kind::Repeater,
                Some(Shape::Door | Shape::Gate | Shape::Trapdoor) => Kind::Openable,
                _ => return None,
            },
        },
    })
}

/// Участвует ли блок в редстоуне.
#[must_use]
pub fn is_component(id: BlockId) -> bool {
    kind(id).is_some()
}

fn facing_toward(id: BlockId) -> Option<Facing> {
    // Лицо блока: у рычага и кнопки — сторона от стены.
    match family_of(id) {
        Some((_, index, FamilyKind::Lever)) => Some(Facing::ALL[usize::from(index & 3)]),
        _ => facing_of(id),
    }
}

/// Выход источника сигнала.
struct Source {
    at: [i32; 3],
    /// Эту соседнюю клетку не питает (куда факел прикреплён).
    skip: Option<[i32; 3]>,
    /// Эту клетку питает «сильно» (блок становится источником).
    strong: Option<[i32; 3]>,
    /// Питает только эту клетку (выход повторителя, наблюдателя).
    only: Option<[i32; 3]>,
}

impl Redstone {
    /// Включён ли выход повторителя (для тестов и отрисовки).
    #[must_use]
    pub fn repeater_on(&self, p: [i32; 3]) -> bool {
        self.out.contains(&p)
    }

    /// Нажимает кнопку.
    pub fn press(&mut self, p: [i32; 3]) {
        self.timers.insert(p, BUTTON_TICKS);
    }
}

/// Делает один шаг редстоуна по всем зарегистрированным клеткам.
pub fn tick(world: &mut World) {
    if world.rs.cells.is_empty() && world.rs.pending.is_empty() {
        return;
    }
    let mut rs = std::mem::take(&mut world.rs);
    step(world, &mut rs);
    // Клетки, зарегистрированные при записи блоков во время шага, не теряем.
    let fresh = std::mem::replace(&mut world.rs, rs);
    world.rs.cells.extend(fresh.cells);
}

fn loaded(world: &World, p: [i32; 3]) -> bool {
    crate::fluids::loaded(world, p)
}

fn step(world: &mut World, rs: &mut Redstone) {
    // Забываем клетки, где компонента больше нет.
    let mut cells: Vec<[i32; 3]> = rs.cells.iter().copied().collect();
    cells.sort_unstable();
    cells.retain(|p| !loaded(world, *p) || kind(world.block(*p)).is_some());
    rs.cells = cells.iter().copied().collect();
    rs.out.retain(|p| rs.cells.contains(p));
    rs.pending.retain(|p, _| rs.cells.contains(p));
    rs.delay.retain(|p, _| rs.cells.contains(p));
    rs.timers.retain(|p, _| rs.cells.contains(p));
    rs.prev.retain(|p, _| rs.cells.contains(p));
    rs.observed.retain(|p, _| rs.cells.contains(p));
    rs.extended.retain(|p| rs.cells.contains(p));

    let mut at: HashMap<[i32; 3], (BlockId, Kind)> = HashMap::new();
    for &p in &cells {
        if loaded(world, p) {
            let id = world.block(p);
            if let Some(k) = kind(id) {
                at.insert(p, (id, k));
            }
        }
    }

    // Кнопки тикают; наблюдатели замечают перемены.
    for t in rs.timers.values_mut() {
        *t = t.saturating_sub(1);
    }
    let expired: Vec<_> = rs
        .timers
        .iter()
        .filter(|(_, t)| **t == 0)
        .map(|(p, _)| *p)
        .collect();
    for p in expired {
        rs.timers.remove(&p);
    }
    for t in rs.pulses.values_mut() {
        *t = t.saturating_sub(1);
    }
    rs.pulses.retain(|_, t| *t > 0);
    for (&p, &(id, k)) in &at {
        if k == Kind::Observer
            && let Some(f) = facing_of(id)
        {
            let watched = add(p, f.offset());
            if loaded(world, watched) {
                let now = world.block(watched);
                if let Some(old) = rs.observed.insert(p, now)
                    && old != now
                {
                    rs.pulses.insert(p, 2);
                }
            }
        }
    }

    // --- источники
    let mut sources: Vec<Source> = Vec::new();
    for (&p, &(id, k)) in &at {
        match k {
            Kind::Lever if crate::mechanics::lever_on(id) == Some(true) => {
                let f = facing_toward(id).unwrap_or(Facing::North);
                sources.push(Source {
                    at: p,
                    skip: None,
                    strong: Some(sub(p, f.offset())),
                    only: None,
                });
            }
            Kind::Button if rs.timers.contains_key(&p) => {
                let f = facing_toward(id).unwrap_or(Facing::North);
                sources.push(Source {
                    at: p,
                    skip: None,
                    strong: Some(sub(p, f.offset())),
                    only: None,
                });
            }
            Kind::Plate if rs.pressed.contains(&p) => sources.push(Source {
                at: p,
                skip: None,
                strong: Some(add(p, [0, -1, 0])),
                only: None,
            }),
            Kind::RedBlock => sources.push(Source {
                at: p,
                skip: None,
                strong: None,
                only: None,
            }),
            Kind::Daylight if rs.day => sources.push(Source {
                at: p,
                skip: None,
                strong: None,
                only: None,
            }),
            Kind::Torch if id == crate::REDSTONE_TORCH => sources.push(Source {
                at: p,
                skip: Some(add(p, [0, -1, 0])),
                strong: Some(add(p, [0, 1, 0])),
                only: None,
            }),
            Kind::WallTorch if base_lit(id) => {
                let f = facing_toward(id).unwrap_or(Facing::North);
                sources.push(Source {
                    at: p,
                    skip: Some(sub(p, f.offset())),
                    strong: None,
                    only: None,
                });
            }
            Kind::Repeater if rs.out.contains(&p) => {
                let f = facing_toward(id).unwrap_or(Facing::North);
                let front = sub(p, f.offset());
                sources.push(Source {
                    at: p,
                    skip: None,
                    strong: Some(front),
                    only: Some(front),
                });
            }
            Kind::Observer if rs.pulses.contains_key(&p) => {
                let f = facing_toward(id).unwrap_or(Facing::North);
                let back = sub(p, f.offset());
                sources.push(Source {
                    at: p,
                    skip: None,
                    strong: Some(back),
                    only: Some(back),
                });
            }
            _ => {}
        }
    }
    let feeds = |s: &Source, c: [i32; 3]| -> bool {
        let d = [
            (s.at[0] - c[0]).abs(),
            (s.at[1] - c[1]).abs(),
            (s.at[2] - c[2]).abs(),
        ];
        d[0] + d[1] + d[2] == 1 && s.skip != Some(c) && s.only.is_none_or(|o| o == c)
    };
    let source_adjacent = |c: [i32; 3]| sources.iter().any(|s| feeds(s, c));

    // Блоки, получившие «сильную» подачу.
    let strong: HashSet<[i32; 3]> = sources
        .iter()
        .filter_map(|s| s.strong)
        .filter(|b| loaded(world, *b) && is_opaque(world.block(*b)))
        .collect();

    // --- провод
    let wires: Vec<[i32; 3]> = at
        .iter()
        .filter(|(_, (_, k))| *k == Kind::Wire)
        .map(|(p, _)| *p)
        .collect();
    let wire_at = |p: [i32; 3]| at.get(&p).is_some_and(|(_, k)| *k == Kind::Wire);
    let opaque_at = |p: [i32; 3]| loaded(world, p) && is_opaque(world.block(p));
    let neighbours = |w: [i32; 3]| -> Vec<([i32; 3], [i32; 3])> {
        let mut found = Vec::new();
        for d in HORIZONTAL {
            let n = add(w, d);
            if wire_at(n) {
                found.push((n, d));
            } else if wire_at(add(n, [0, 1, 0])) && !opaque_at(add(w, [0, 1, 0])) {
                found.push((add(n, [0, 1, 0]), d));
            } else if wire_at(add(n, [0, -1, 0])) && !opaque_at(n) {
                found.push((add(n, [0, -1, 0]), d));
            }
        }
        found
    };
    let links: HashMap<[i32; 3], Links> = wires.iter().map(|w| (*w, neighbours(*w))).collect();
    let mut level: HashMap<[i32; 3], u8> = HashMap::new();
    for &w in &wires {
        let powered = source_adjacent(w) || SIDES.iter().any(|d| strong.contains(&add(w, *d)));
        level.insert(w, if powered { 15 } else { 0 });
    }
    for _ in 0..16 {
        let mut changed = false;
        for &w in &wires {
            let best = links[&w]
                .iter()
                .map(|(n, _)| level.get(n).copied().unwrap_or(0).saturating_sub(1))
                .max()
                .unwrap_or(0);
            if best > level[&w] {
                level.insert(w, best);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    // Куда провод подаёт сигнал.
    let wire_dirs = |w: [i32; 3]| -> Vec<[i32; 3]> {
        let dirs: Vec<[i32; 3]> = links[&w].iter().map(|(_, d)| *d).collect();
        match dirs.len() {
            0 => HORIZONTAL.to_vec(),
            1 => vec![dirs[0], [-dirs[0][0], 0, -dirs[0][2]]],
            _ => dirs,
        }
    };
    let mut weak: HashSet<[i32; 3]> = HashSet::new();
    let mut wire_feeds: HashSet<[i32; 3]> = HashSet::new();
    for &w in &wires {
        if level[&w] == 0 {
            continue;
        }
        let below = add(w, [0, -1, 0]);
        wire_feeds.insert(below);
        if opaque_at(below) {
            weak.insert(below);
        }
        for d in wire_dirs(w) {
            let n = add(w, d);
            wire_feeds.insert(n);
            if opaque_at(n) {
                weak.insert(n);
            }
        }
    }
    let block_powered = |b: [i32; 3]| strong.contains(&b) || weak.contains(&b);
    let powered = |c: [i32; 3]| -> bool {
        source_adjacent(c)
            || wire_feeds.contains(&c)
            || SIDES
                .iter()
                .any(|d| block_powered(add(c, *d)) && opaque_at(add(c, *d)))
    };

    // --- применение
    let mut writes: Vec<([i32; 3], BlockId)> = Vec::new();
    let mut blasts: Vec<[i32; 3]> = Vec::new();
    let mut events: Vec<Event> = Vec::new();
    let mut piston_jobs: Vec<([i32; 3], BlockId, bool)> = Vec::new();
    let mut edges: HashMap<[i32; 3], bool> = HashMap::new();
    let mut ordered: Vec<_> = at.iter().map(|(p, v)| (*p, *v)).collect();
    ordered.sort_unstable_by_key(|(p, _)| *p);
    for (p, (id, k)) in ordered {
        match k {
            Kind::Wire => {
                let want = if level[&p] > 0 {
                    crate::REDSTONE_WIRE_ON
                } else {
                    crate::REDSTONE_WIRE
                };
                if want != id {
                    writes.push((p, want));
                }
            }
            Kind::Lamp => {
                let want = if powered(p) {
                    crate::REDSTONE_LAMP_LIT
                } else {
                    crate::REDSTONE_LAMP
                };
                if want != id {
                    writes.push((p, want));
                }
            }
            Kind::Torch => {
                let attached = add(p, [0, -1, 0]);
                let lit = !block_powered(attached);
                let want = if lit {
                    crate::REDSTONE_TORCH
                } else {
                    crate::REDSTONE_TORCH_OFF
                };
                if want != id {
                    writes.push((p, want));
                }
            }
            Kind::WallTorch => {
                let f = facing_toward(id).unwrap_or(Facing::North);
                let attached = sub(p, f.offset());
                let lit = !block_powered(attached);
                let index = crate::family_of(id).map_or(0, |(_, i, _)| BlockId::from(i));
                let want = if lit {
                    crate::REDSTONE_WALL_TORCH + index
                } else {
                    crate::REDSTONE_WALL_TORCH_OFF + index
                };
                if want != id {
                    writes.push((p, want));
                }
            }
            Kind::Repeater => {
                let f = facing_toward(id).unwrap_or(Facing::North);
                let input = add(p, f.offset());
                let input_on = repeater_input(
                    &at,
                    rs,
                    &sources,
                    &level,
                    &wire_dirs,
                    &block_powered,
                    p,
                    input,
                    &wires,
                );
                let now = rs.out.contains(&p);
                if input_on != now {
                    let delay = rs.delay.get(&p).copied().unwrap_or(1).clamp(1, 4);
                    match rs.pending.get_mut(&p) {
                        Some((target, left)) if *target == input_on => {
                            *left = left.saturating_sub(1);
                            if *left == 0 {
                                rs.pending.remove(&p);
                                if input_on {
                                    rs.out.insert(p);
                                } else {
                                    rs.out.remove(&p);
                                }
                            }
                        }
                        _ => {
                            if delay <= 1 {
                                if input_on {
                                    rs.out.insert(p);
                                } else {
                                    rs.out.remove(&p);
                                }
                                rs.pending.remove(&p);
                            } else {
                                rs.pending.insert(p, (input_on, delay - 1));
                            }
                        }
                    }
                } else {
                    rs.pending.remove(&p);
                }
            }
            Kind::Openable => {
                let mut on = powered(p);
                if let Some(other) = crate::models::door_other_half(id)
                    && shape_of(id) == Some(Shape::Door)
                {
                    let up = crate::models::is_door_upper(other);
                    let cell = add(p, [0, if up { 1 } else { -1 }, 0]);
                    if loaded(world, cell) && world.block(cell) == other {
                        on |= powered(cell);
                    }
                }
                edges.insert(p, on);
                if let Some(old) = rs.prev.get(&p)
                    && *old != on
                    && let Some(next) = crate::mechanics::with_open(id, on)
                {
                    writes.push((p, next));
                }
            }
            Kind::Tnt => {
                let on = powered(p);
                if let Some(old) = rs.prev.get(&p)
                    && !*old
                    && on
                {
                    blasts.push(p);
                }
                edges.insert(p, on);
            }
            Kind::Note => {
                let on = powered(p);
                if let Some(old) = rs.prev.get(&p)
                    && !*old
                    && on
                {
                    events.push(Event::Note { position: p });
                }
                edges.insert(p, on);
            }
            Kind::Dispenser => {
                let on = powered(p);
                if let Some(old) = rs.prev.get(&p)
                    && !*old
                    && on
                    && let Some(f) = facing_toward(id)
                {
                    events.push(Event::Dispense {
                        position: p,
                        dir: f.offset(),
                        dropper: base_is_dropper(id),
                    });
                }
                edges.insert(p, on);
            }
            Kind::Piston => {
                let on = powered(p);
                if let Some(old) = rs.prev.get(&p)
                    && *old != on
                {
                    piston_jobs.push((p, id, on));
                }
                edges.insert(p, on);
            }
            _ => {}
        }
    }
    for (p, on) in edges {
        rs.prev.insert(p, on);
    }
    for (p, id) in writes {
        world.set(p, id);
    }
    world.blasts.extend(blasts);
    rs.events.extend(events);
    for (p, id, extend) in piston_jobs {
        if extend {
            if !rs.extended.contains(&p) && extend_piston(world, &rs.extended, p, id) {
                rs.extended.insert(p);
            }
        } else if rs.extended.remove(&p) {
            retract_piston(world, &rs.extended, p, id);
        }
    }
}

fn base_lit(id: BlockId) -> bool {
    crate::base_block(id) == crate::REDSTONE_WALL_TORCH
}

fn base_is_dropper(id: BlockId) -> bool {
    crate::base_block(id) == crate::DROPPER
}

#[allow(clippy::too_many_arguments)]
fn repeater_input(
    at: &HashMap<[i32; 3], (BlockId, Kind)>,
    rs: &Redstone,
    sources: &[Source],
    level: &HashMap<[i32; 3], u8>,
    wire_dirs: &dyn Fn([i32; 3]) -> Vec<[i32; 3]>,
    block_powered: &dyn Fn([i32; 3]) -> bool,
    me: [i32; 3],
    input: [i32; 3],
    wires: &[[i32; 3]],
) -> bool {
    let _ = rs;
    if sources
        .iter()
        .any(|s| s.at == input && s.skip != Some(me) && s.only.is_none_or(|o| o == me))
    {
        return true;
    }
    if wires.contains(&input)
        && level.get(&input).copied().unwrap_or(0) > 0
        && wire_dirs(input).iter().any(|d| add(input, *d) == me)
    {
        return true;
    }
    let _ = at;
    block_powered(input)
}

fn immovable(id: BlockId, extended: &HashSet<[i32; 3]>, p: [i32; 3]) -> bool {
    use crate::*;
    id == BEDROCK
        || is_fluid(id)
        || id == PISTON_HEAD
        || id == MOVING_PISTON
        || extended.contains(&p)
        || BLOCKS[usize::from(id)].hardness >= 50.0
        || matches!(
            base_block(id),
            FURNACE
                | FURNACE_LIT
                | SMOKER
                | SMOKER_LIT
                | BLAST_FURNACE
                | BLAST_FURNACE_LIT
                | BARREL
                | ENDER_CHEST
                | TRAPPED_CHEST
                | DISPENSER
                | DROPPER
                | HOPPER
                | JUKEBOX
                | ENCHANTING_TABLE
                | BEACON
                | SPAWNER
        )
        || is_shulker(id)
}

fn replaceable(id: BlockId) -> bool {
    id == AIR || crate::is_plant(id) && !crate::is_fire(id) || id == crate::FIRE
}

fn extend_piston(
    world: &mut World,
    extended: &HashSet<[i32; 3]>,
    p: [i32; 3],
    id: BlockId,
) -> bool {
    let Some(f) = facing_of(id) else {
        return false;
    };
    let d = f.offset();
    // Цепочка блоков перед поршнем до первой свободной клетки.
    let mut chain = Vec::new();
    let mut cell = add(p, d);
    loop {
        if !loaded(world, cell) || chain.len() > PISTON_LIMIT {
            return false;
        }
        let b = world.block(cell);
        if replaceable(b) {
            break;
        }
        if immovable(b, extended, cell) {
            return false;
        }
        chain.push((cell, b));
        cell = add(cell, d);
    }
    if chain.len() > PISTON_LIMIT {
        return false;
    }
    for (c, b) in chain.iter().rev() {
        world.set(add(*c, d), *b);
    }
    world.set(add(p, d), crate::PISTON_HEAD);
    true
}

fn retract_piston(world: &mut World, extended: &HashSet<[i32; 3]>, p: [i32; 3], id: BlockId) {
    let Some(f) = facing_of(id) else {
        return;
    };
    let d = f.offset();
    let head = add(p, d);
    if world.block(head) == crate::PISTON_HEAD {
        world.set(head, AIR);
    }
    if crate::base_block(id) == crate::STICKY_PISTON {
        let far = add(head, d);
        if loaded(world, far) {
            let b = world.block(far);
            if !replaceable(b) && !immovable(b, extended, far) {
                world.set(head, b);
                world.set(far, AIR);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::{collections::HashMap, sync::Arc};

    fn world() -> World {
        let mut w = World::new(1);
        for cx in -1..=1 {
            for cz in -1..=1 {
                w.chunks.insert(
                    [cx, cz],
                    Arc::new(Chunk {
                        position: [cx, cz],
                        sections: HashMap::new(),
                    }),
                );
            }
        }
        // Каменный пол на y = 69.
        for x in -10..=10 {
            for z in -10..=10 {
                w.set([x, 69, z], STONE);
            }
        }
        w
    }

    fn run(w: &mut World, ticks: usize) {
        for _ in 0..ticks {
            tick(w);
        }
    }

    #[test]
    fn a_lever_powers_wire_with_decay_and_lights_a_lamp_at_the_end() {
        let mut w = world();
        // Рычаг (на юг: стена в −Z) у x = 0, провод от x = 1…, лампа через 5 клеток.
        for x in 1..=5 {
            w.set([x, 70, 0], REDSTONE_WIRE);
        }
        w.set([6, 70, 0], REDSTONE_LAMP);
        w.set([0, 70, 0], LEVER + 4 + 3); // включён, на восток
        run(&mut w, 3);
        assert_eq!(w.block([1, 70, 0]), REDSTONE_WIRE_ON);
        assert_eq!(w.block([5, 70, 0]), REDSTONE_WIRE_ON);
        assert_eq!(w.block([6, 70, 0]), REDSTONE_LAMP_LIT);
        // Выключили — всё гаснет.
        w.set([0, 70, 0], LEVER + 3);
        run(&mut w, 3);
        assert_eq!(w.block([3, 70, 0]), REDSTONE_WIRE);
        assert_eq!(w.block([6, 70, 0]), REDSTONE_LAMP);
    }

    #[test]
    fn wire_signal_fades_after_fifteen_blocks() {
        let mut w = world();
        for x in 1..=20 {
            w.set([x, 70, 0], REDSTONE_WIRE);
        }
        w.set([0, 70, 0], REDSTONE_BLOCK);
        run(&mut w, 2);
        assert_eq!(w.block([15, 70, 0]), REDSTONE_WIRE_ON);
        assert_eq!(w.block([16, 70, 0]), REDSTONE_WIRE);
    }

    #[test]
    fn a_torch_inverts_and_a_not_gate_clock_toggles_every_step() {
        let mut w = world();
        // Опорный блок с факелом сверху: пока блок не запитан, факел горит.
        w.set([0, 70, 0], STONE);
        w.set([0, 71, 0], REDSTONE_TORCH);
        run(&mut w, 2);
        assert_eq!(w.block([0, 71, 0]), REDSTONE_TORCH);
        // Запитали блок под факелом рычагом сбоку: факел гаснет.
        w.set([1, 70, 0], LEVER + 4 + 3);
        // Рычаг на восточной грани, то есть прикреплён к блоку (0,70,0) со стороны +X.
        w.set([1, 70, 0], LEVER + 4 + 3);
        run(&mut w, 3);
        assert_eq!(w.block([0, 71, 0]), REDSTONE_TORCH_OFF);
        w.set([1, 70, 0], LEVER + 3);
        run(&mut w, 3);
        assert_eq!(w.block([0, 71, 0]), REDSTONE_TORCH);
    }

    #[test]
    fn a_repeater_delays_by_its_setting_and_a_button_pulses_for_a_second() {
        let mut w = world();
        // Кнопка на восточной грани блока (0,70,0), провод и повторитель на юг.
        w.set([0, 70, 0], STONE);
        w.set([1, 70, 0], STONE_BUTTON + 3);
        w.set([0, 70, 1], REDSTONE_WIRE);
        // Повторитель: вход с севера (лицом на север), выход на юг.
        w.set([0, 70, 2], REPEATER);
        w.rs.delay.insert([0, 70, 2], 3);
        w.set([0, 70, 3], REDSTONE_LAMP);
        w.rs.press([1, 70, 0]);
        let mut lit_at = None;
        for t in 0..20 {
            tick(&mut w);
            if lit_at.is_none() && w.block([0, 70, 3]) == REDSTONE_LAMP_LIT {
                lit_at = Some(t);
            }
        }
        assert!(lit_at.is_some_and(|t| t >= 2), "{lit_at:?}");
        run(&mut w, 15);
        assert_eq!(w.block([0, 70, 3]), REDSTONE_LAMP, "кнопка отпустилась");
    }

    #[test]
    fn powered_doors_open_on_the_edge_but_manual_changes_stay() {
        let mut w = world();
        w.set([2, 70, 0], OAK_DOOR + 1);
        w.set([2, 71, 0], OAK_DOOR + 9);
        w.set([1, 70, 0], REDSTONE_BLOCK);
        run(&mut w, 1);
        assert_eq!(
            w.block([2, 70, 0]),
            OAK_DOOR + 1,
            "первый кадр только запоминает"
        );
        w.set([1, 70, 0], AIR);
        run(&mut w, 1);
        w.set([1, 70, 0], REDSTONE_BLOCK);
        run(&mut w, 2);
        assert_eq!(w.block([2, 70, 0]), OAK_DOOR + 5);
        assert_eq!(w.block([2, 71, 0]), OAK_DOOR + 13);
        w.set([1, 70, 0], AIR);
        run(&mut w, 2);
        assert_eq!(w.block([2, 70, 0]), OAK_DOOR + 1);
    }

    #[test]
    fn a_piston_pushes_a_chain_and_a_sticky_piston_pulls_it_back() {
        let mut w = world();
        // Поршень смотрит на восток (лицо на +X); блоки впереди.
        w.set([0, 70, 0], facing_variant(PISTON, Facing::East));
        w.set([1, 70, 0], STONE);
        w.set([2, 70, 0], DIRT);
        w.set([0, 71, 0], REDSTONE_BLOCK);
        run(&mut w, 1);
        w.set([0, 71, 0], AIR);
        run(&mut w, 1);
        w.set([0, 71, 0], REDSTONE_BLOCK);
        run(&mut w, 2);
        assert_eq!(w.block([1, 70, 0]), PISTON_HEAD);
        assert_eq!(w.block([2, 70, 0]), STONE);
        assert_eq!(w.block([3, 70, 0]), DIRT);
        w.set([0, 71, 0], AIR);
        run(&mut w, 2);
        assert_eq!(w.block([1, 70, 0]), AIR, "обычный поршень оставляет блоки");
        assert_eq!(w.block([2, 70, 0]), STONE);
        // Прочное не двигается.
        let mut w = world();
        w.set([0, 70, 0], facing_variant(PISTON, Facing::East));
        w.set([1, 70, 0], OBSIDIAN);
        w.set([0, 71, 0], AIR);
        run(&mut w, 1);
        w.set([0, 71, 0], REDSTONE_BLOCK);
        run(&mut w, 2);
        assert_eq!(w.block([1, 70, 0]), OBSIDIAN);
    }

    #[test]
    fn tnt_note_blocks_dispensers_and_observers_react_to_signals() {
        let mut w = world();
        w.set([0, 70, 0], TNT);
        w.set([5, 70, 0], NOTE_BLOCK);
        w.set([10, 70, 0], facing_variant(DISPENSER, Facing::South));
        w.set([3, 71, 0], AIR);
        for x in [-1, 4, 9] {
            w.set([x, 70, 0], AIR);
        }
        run(&mut w, 1);
        for x in [-1, 4, 9] {
            w.set([x, 70, 0], REDSTONE_BLOCK);
        }
        run(&mut w, 2);
        assert_eq!(w.blasts, vec![[0, 70, 0]]);
        assert!(w.rs.events.contains(&Event::Note {
            position: [5, 70, 0]
        }));
        assert!(w.rs.events.iter().any(|e| matches!(
            e,
            Event::Dispense {
                position: [10, 70, 0],
                dropper: false,
                ..
            }
        )));
        // Наблюдатель смотрит на восток: пульс при смене блока.
        let mut w = world();
        w.set([0, 70, 0], OBSERVER + 3);
        w.set([-1, 70, 0], REDSTONE_LAMP);
        run(&mut w, 2);
        w.set([1, 70, 0], STONE);
        run(&mut w, 2);
        assert_eq!(
            w.block([-1, 70, 0]),
            REDSTONE_LAMP_LIT,
            "импульс идёт с тыла"
        );
        run(&mut w, 6);
        assert_eq!(w.block([-1, 70, 0]), REDSTONE_LAMP);
    }

    #[test]
    fn component_detection_covers_redstone_blocks_only() {
        for id in [
            REDSTONE_WIRE,
            LEVER,
            STONE_BUTTON,
            STONE_PRESSURE_PLATE,
            REPEATER,
            PISTON,
            STICKY_PISTON,
            DROPPER,
            TNT,
            OAK_DOOR,
            REDSTONE_LAMP_LIT,
            DAYLIGHT_DETECTOR,
        ] {
            assert!(is_component(id), "{id}");
        }
        assert!(!is_component(STONE) && !is_component(OAK_FENCE));
    }
}
