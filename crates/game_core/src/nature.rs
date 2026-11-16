//! Природа как в Minecraft: падающие блоки, опора растений, гибель листвы,
//! затвердевание бетонного порошка, рост травы, тростника и кактусов.
//!
//! Клетки, рядом с которыми что-то изменилось, попадают в `World::nature_active` и
//! обрабатываются небольшими порциями. Случайные «тики» растений работают только
//! вокруг игрока, как случайные тики блоков в Minecraft.

use crate::{
    AIR, ANVIL, BAMBOO, BAMBOO_LARGE, BAMBOO_SAPLING, BAMBOO_SMALL, BEETROOTS, BEETROOTS_0,
    BEETROOTS_1, BEETROOTS_2, BlockId, CACTUS, CARROTS, CARROTS_0, CARROTS_1, CARROTS_2,
    CHIPPED_ANVIL, CONCRETE_WHITE, DAMAGED_ANVIL, DIRT, DRAGON_EGG, FARMLAND, FARMLAND_MOIST,
    GRASS, GRAVEL, ICE, KELP, KELP_PLANT, KELP_SPROUT, LARGE_FERN, MELON, MELON_STEM, POPPY,
    POTATOES, POTATOES_0, POTATOES_1, POTATOES_2, PUMPKIN, PUMPKIN_STEM, RED_SAND, SAND,
    SNOW_LAYER, SUGAR_CANE, SUNFLOWER, TALL_GRASS, TNT, WATER, WHEAT, WHEAT_0, WHEAT_1, WHEAT_2,
    WHITE_CONCRETE_POWDER, World, blocks::BLOCKS, fluids::loaded, is_fire, is_fluid, is_leaves,
    is_opaque, is_plant, is_solid, is_water,
};
use std::sync::OnceLock;

/// Период шага природы в секундах.
pub const TICK_SECONDS: f32 = 0.1;
/// Сколько случайных клеток проверяется за шаг вокруг игрока.
pub const RANDOM_SAMPLES_PER_TICK: usize = 70;
/// Радиус случайных тиков вокруг игрока.
const RANDOM_RADIUS: i32 = 24;
/// Дальность, на которой лист ещё держится за бревно (как в Minecraft).
const LEAF_REACH: u32 = 4;
/// Сколько блоков могут падать одновременно.
const MAX_FALLING: usize = 512;

/// Что произошло в мире само по себе: клиент превращает это в выпавшие предметы.
#[derive(Clone, Debug, PartialEq)]
pub enum NatureEvent {
    /// Блок разрушился сам (лист опал, растение без опоры, упавший блок некуда деть).
    Broke { position: [i32; 3], block: BlockId },
    /// Падающий блок приземлился (звук, пыль).
    Landed {
        position: [i32; 3],
        block: BlockId,
        fallen: f32,
    },
}

/// Падающий блок: пока летит, он не часть мира.
#[derive(Clone, Debug, PartialEq)]
pub struct Falling {
    /// Стабильный номер для отрисовки.
    pub id: u64,
    pub block: BlockId,
    /// Центр блока.
    pub position: [f32; 3],
    /// Скорость по вертикали, блоков в секунду (вниз — отрицательная).
    pub velocity: f32,
    /// Сколько блоков уже пролетел: от этого зависит урон наковальни.
    pub fallen: f32,
    /// Наковальня уже ударила тех, кто был под ней.
    pub struck: bool,
}

/// Следующая стадия роста культуры.
#[must_use]
pub fn crop_next(id: BlockId) -> Option<BlockId> {
    Some(match id {
        WHEAT_0 => WHEAT_1,
        WHEAT_1 => WHEAT_2,
        WHEAT_2 => WHEAT,
        CARROTS_0 => CARROTS_1,
        CARROTS_1 => CARROTS_2,
        CARROTS_2 => CARROTS,
        POTATOES_0 => POTATOES_1,
        POTATOES_1 => POTATOES_2,
        POTATOES_2 => POTATOES,
        BEETROOTS_0 => BEETROOTS_1,
        BEETROOTS_1 => BEETROOTS_2,
        BEETROOTS_2 => BEETROOTS,
        _ => return None,
    })
}

/// Любая стадия пшеницы, моркови, картофеля или свёклы.
#[must_use]
pub fn is_crop(id: BlockId) -> bool {
    crop_next(id).is_some() || is_ripe_crop(id)
}

/// Созревшая культура.
#[must_use]
pub fn is_ripe_crop(id: BlockId) -> bool {
    matches!(id, WHEAT | CARROTS | POTATOES | BEETROOTS)
}

/// Грядка, на которой растут культуры.
#[must_use]
pub fn is_farmland(id: BlockId) -> bool {
    id == FARMLAND || id == FARMLAND_MOIST
}

/// Блок, который падает, если под ним пусто.
#[must_use]
pub fn is_falling(id: BlockId) -> bool {
    matches!(
        id,
        SAND | RED_SAND | GRAVEL | ANVIL | CHIPPED_ANVIL | DAMAGED_ANVIL | DRAGON_EGG
    ) || is_concrete_powder(id)
}

#[must_use]
pub fn is_anvil(id: BlockId) -> bool {
    matches!(id, ANVIL | CHIPPED_ANVIL | DAMAGED_ANVIL)
}

#[must_use]
pub fn is_bamboo(id: BlockId) -> bool {
    matches!(id, BAMBOO | BAMBOO_SMALL | BAMBOO_LARGE | BAMBOO_SAPLING)
}

#[must_use]
pub fn is_concrete_powder(id: BlockId) -> bool {
    (WHITE_CONCRETE_POWDER..WHITE_CONCRETE_POWDER + 16).contains(&id)
}

/// Бревно, ствол или гриб-ствол: листва держится за них.
#[must_use]
pub fn is_log(id: BlockId) -> bool {
    static TABLE: OnceLock<Vec<bool>> = OnceLock::new();
    TABLE.get_or_init(|| {
        BLOCKS
            .iter()
            .map(|b| {
                let n = b.name;
                n == "Wood"
                    || n.ends_with(" Log")
                    || n.ends_with(" Wood")
                    || n.ends_with("Crimson Stem")
                    || n.ends_with("Warped Stem")
                    || n.ends_with(" Hyphae")
            })
            .collect()
    })[usize::from(id)]
}

/// Растение, которому нужна опора снизу.
fn needs_ground(id: BlockId) -> bool {
    (POPPY..=TALL_GRASS).contains(&id)
        || (SUNFLOWER..=LARGE_FERN).contains(&id)
        || id == SUGAR_CANE
        || id == CACTUS
        || is_bamboo(id)
}

fn hole(id: BlockId) -> bool {
    id == AIR || is_fluid(id) || is_fire(id)
}

fn above(p: [i32; 3]) -> [i32; 3] {
    [p[0], p[1] + 1, p[2]]
}
fn below(p: [i32; 3]) -> [i32; 3] {
    [p[0], p[1] - 1, p[2]]
}

const NEIGHBORS: [[i32; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

fn near(p: [i32; 3], d: [i32; 3]) -> [i32; 3] {
    [p[0] + d[0], p[1] + d[1], p[2] + d[2]]
}

fn hash(seed: u64, a: u64, b: u64) -> u64 {
    let mut v =
        seed ^ a.wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ b.wrapping_mul(0xd1b5_4a32_d192_ed03);
    v ^= v >> 30;
    v = v.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    v ^= v >> 27;
    v = v.wrapping_mul(0x94d0_49bb_1331_11eb);
    v ^ (v >> 31)
}

fn cell_key(p: [i32; 3]) -> u64 {
    (i64::from(p[0]).cast_unsigned() << 40)
        ^ (i64::from(p[1]).cast_unsigned() << 20)
        ^ i64::from(p[2]).cast_unsigned()
}

/// Лист держится, если до бревна не больше `LEAF_REACH` шагов по листве.
fn leaf_supported(world: &World, start: [i32; 3]) -> bool {
    let mut frontier = vec![start];
    let mut seen = vec![start];
    for _ in 0..LEAF_REACH {
        let mut next = Vec::new();
        for p in &frontier {
            for d in NEIGHBORS {
                let q = near(*p, d);
                if seen.contains(&q) || !loaded(world, q) {
                    continue;
                }
                let id = world.block(q);
                if is_log(id) {
                    return true;
                }
                if is_leaves(id) {
                    seen.push(q);
                    next.push(q);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    false
}

fn water_near(world: &World, p: [i32; 3]) -> bool {
    (-4..=4).any(|dx| {
        (-4..=4)
            .any(|dz| (0..=1).any(|dy| is_water(world.block([p[0] + dx, p[1] + dy, p[2] + dz]))))
    })
}

fn broke(world: &mut World, p: [i32; 3], block: BlockId) {
    world.set(p, AIR);
    world
        .nature_events
        .push(NatureEvent::Broke { position: p, block });
}

/// Обработка одной клетки. Возвращает true, если блок изменился.
fn update(world: &mut World, p: [i32; 3], tick: u64) -> bool {
    let id = world.block(p);
    if is_falling(id) {
        let under = below(p);
        if loaded(world, under)
            && (hole(world.block(under)) || is_plant(world.block(under)))
            && world.falling.len() < MAX_FALLING
        {
            world.set(p, AIR);
            let entity = world.next_entity_id();
            world.falling.push(Falling {
                id: entity,
                block: id,
                position: [p[0] as f32 + 0.5, p[1] as f32 + 0.5, p[2] as f32 + 0.5],
                velocity: 0.0,
                fallen: 0.0,
                struck: false,
            });
            return true;
        }
    }
    if is_concrete_powder(id) && NEIGHBORS.iter().any(|d| is_water(world.block(near(p, *d)))) {
        world.set(p, CONCRETE_WHITE + (id - WHITE_CONCRETE_POWDER));
        return true;
    }
    if is_crop(id) {
        let under = below(p);
        if loaded(world, under) && !is_farmland(world.block(under)) {
            broke(world, p, id);
            return true;
        }
    } else if needs_ground(id) {
        let under = below(p);
        if loaded(world, under) && hole(world.block(under)) {
            broke(world, p, id);
            return true;
        }
    }
    if id == FARMLAND && water_near(world, p) {
        world.set(p, FARMLAND_MOIST);
        return true;
    }
    if is_leaves(id)
        && world.edits.get(&p) != Some(&id)
        && hash(world.seed, tick, cell_key(p)) % 100 < 6
        && !leaf_supported(world, p)
    {
        broke(world, p, id);
        return true;
    }
    false
}

/// Один шаг: не более `budget` клеток из очереди. Возвращает число обработанных.
pub fn tick(world: &mut World, tick: u64, budget: usize) -> usize {
    if world.nature_active.is_empty() {
        return 0;
    }
    let mut cells: Vec<[i32; 3]> = world.nature_active.iter().copied().collect();
    cells.sort_unstable();
    cells.truncate(budget);
    for p in &cells {
        world.nature_active.remove(p);
    }
    for p in &cells {
        if !loaded(world, *p) {
            continue;
        }
        let id = world.block(*p);
        if update(world, *p, tick) {
            // Упавший блок и соседи проверяются снова.
            continue;
        }
        // Лист, который ещё может опасть, остаётся в очереди до следующего шага.
        if is_leaves(id) && world.edits.get(p) != Some(&id) && !leaf_supported(world, *p) {
            world.nature_active.insert(*p);
        }
    }
    cells.len()
}

/// Случайные тики вокруг `center`: трава, тростник, кактусы.
pub fn random_tick(world: &mut World, center: [i32; 3], tick: u64, samples: usize) {
    let span = (2 * RANDOM_RADIUS + 1) as u64;
    for i in 0..samples {
        let h = hash(world.seed, tick, i as u64);
        let p = [
            center[0] - RANDOM_RADIUS + (h % span) as i32,
            center[1] - 10 + ((h >> 20) % 21) as i32,
            center[2] - RANDOM_RADIUS + ((h >> 40) % span) as i32,
        ];
        if !loaded(world, p) {
            continue;
        }
        let id = world.block(p);
        let roll = ((h >> 48) % 100) as u32;
        match id {
            GRASS => grass(world, p, h),
            SUGAR_CANE | CACTUS => grow_column(world, p, id, h),
            FARMLAND | FARMLAND_MOIST => farmland(world, p, id),
            BAMBOO | BAMBOO_SMALL | BAMBOO_LARGE | BAMBOO_SAPLING => {
                grow_bamboo(world, p, id, roll);
            }
            KELP | KELP_SPROUT => grow_kelp(world, p, id, roll),
            ICE | SNOW_LAYER => melt(world, p, id, roll),
            MELON_STEM | PUMPKIN_STEM => grow_fruit(world, p, id, h, roll),
            _ => {
                if let Some(next) = crop_next(id) {
                    let moist = world.block(below(p)) == FARMLAND_MOIST;
                    if roll < if moist { 40 } else { 15 } {
                        world.set(p, next);
                    }
                }
            }
        }
    }
}

fn grass(world: &mut World, p: [i32; 3], h: u64) {
    let cover = world.block(above(p));
    if is_opaque(cover) || is_water(cover) {
        world.set(p, DIRT);
        return;
    }
    let d = [
        ((h >> 8) % 3) as i32 - 1,
        ((h >> 12) % 3) as i32 - 1,
        ((h >> 16) % 3) as i32 - 1,
    ];
    let target = near(p, d);
    if loaded(world, target)
        && world.block(target) == DIRT
        && !is_opaque(world.block(above(target)))
        && !is_water(world.block(above(target)))
    {
        world.set(target, GRASS);
    }
}

fn grow_column(world: &mut World, p: [i32; 3], id: BlockId, h: u64) {
    if !(h >> 56).is_multiple_of(7) {
        return;
    }
    let top = above(p);
    if world.block(top) != AIR || !loaded(world, top) {
        return;
    }
    // Не выше трёх блоков.
    let mut height = 1;
    let mut cell = below(p);
    while world.block(cell) == id {
        height += 1;
        cell = below(cell);
    }
    if height >= 3 {
        return;
    }
    if id == SUGAR_CANE {
        // Тростнику нужна вода рядом с основанием.
        let base = cell;
        let wet = [[1, 0], [-1, 0], [0, 1], [0, -1]]
            .iter()
            .any(|d| is_water(world.block([base[0] + d[0], base[1], base[2] + d[1]])));
        if !wet {
            return;
        }
    } else if [[1, 0], [-1, 0], [0, 1], [0, -1]]
        .iter()
        .any(|d| is_solid(world.block([top[0] + d[0], top[1], top[2] + d[1]])))
    {
        // Кактус не растёт вплотную к стене.
        return;
    }
    world.set(top, id);
}

fn farmland(world: &mut World, p: [i32; 3], id: BlockId) {
    let cover = world.block(above(p));
    if is_opaque(cover) || is_solid(cover) && !is_crop(cover) {
        world.set(p, DIRT);
        return;
    }
    let wet = water_near(world, p);
    if wet && id == FARMLAND {
        world.set(p, FARMLAND_MOIST);
    } else if !wet {
        if id == FARMLAND_MOIST {
            world.set(p, FARMLAND);
        } else if !is_crop(cover) {
            world.set(p, DIRT);
        }
    }
}

fn column_height(world: &World, p: [i32; 3], id: BlockId) -> i32 {
    let mut height = 1;
    let mut cell = below(p);
    while world.block(cell) == id {
        height += 1;
        cell = below(cell);
    }
    height
}

fn bamboo_stalk(world: &World, p: [i32; 3]) -> i32 {
    let mut height = 0;
    let mut cell = p;
    while matches!(world.block(cell), BAMBOO | BAMBOO_SMALL | BAMBOO_LARGE) {
        height += 1;
        cell = below(cell);
    }
    height
}

/// Бамбук растёт вверх: росток даёт стебель с маленькими листьями, с высоты 3
/// наверху большие листья, под ними маленькие, ниже голый стебель.
fn grow_bamboo(world: &mut World, p: [i32; 3], id: BlockId, roll: u32) {
    let top = above(p);
    if roll >= 30 || world.block(top) != AIR || !loaded(world, top) {
        return;
    }
    if id == BAMBOO_SAPLING {
        world.set(p, BAMBOO);
        world.set(top, BAMBOO_SMALL);
        return;
    }
    let height = bamboo_stalk(world, p) + 1;
    if height >= 12 {
        return;
    }
    let tall = height + 1 >= 3;
    world.set(top, if tall { BAMBOO_LARGE } else { BAMBOO_SMALL });
    world.set(p, if tall { BAMBOO_SMALL } else { BAMBOO });
    let under = below(p);
    if matches!(world.block(under), BAMBOO_SMALL | BAMBOO_LARGE) {
        world.set(under, BAMBOO);
    }
}

/// Ламинария: ростки взрослеют, а взрослая верхушка выращивает новый росток.
fn grow_kelp(world: &mut World, p: [i32; 3], id: BlockId, roll: u32) {
    if id == KELP_SPROUT {
        if roll < 40 {
            world.set(p, KELP);
        }
        return;
    }
    let top = above(p);
    if roll < 25
        && world.block(top) == WATER
        && loaded(world, top)
        && column_height(world, p, KELP_PLANT) < 20
    {
        world.set(p, KELP_PLANT);
        world.set(top, KELP_SPROUT);
    }
}

/// Вытаптывание: грядка под приземлившимся становится землёй, посадка ломается.
pub fn trample(world: &mut World, cell: [i32; 3]) -> bool {
    if !is_farmland(world.block(cell)) {
        return false;
    }
    let up = above(cell);
    let crop = world.block(up);
    if is_crop(crop) || crop == MELON_STEM || crop == PUMPKIN_STEM {
        broke(world, up, crop);
    }
    world.set(cell, DIRT);
    true
}

/// Приземляет всё, что ещё в воздухе: перед сохранением мира блоки не должны пропасть.
pub fn land_all(world: &mut World) {
    for _ in 0..4000 {
        if world.falling.is_empty() {
            break;
        }
        step_falling(world, 0.05);
    }
    for f in std::mem::take(&mut world.falling) {
        let cell = f.position.map(|v| v.floor() as i32);
        if world.block(cell) == AIR {
            world.set(cell, f.block);
        }
    }
    for tnt in std::mem::take(&mut world.primed) {
        let cell = tnt.position.map(|v| v.floor() as i32);
        if world.block(cell) == AIR {
            world.set(cell, TNT);
        }
    }
}

fn melt(world: &mut World, p: [i32; 3], id: BlockId, roll: u32) {
    let warm = std::iter::once([0, 0, 0])
        .chain(NEIGHBORS)
        .any(|d| BLOCKS[usize::from(world.block(near(p, d)))].light_emission > 11);
    if warm && roll < 60 {
        world.set(p, if id == ICE { WATER } else { AIR });
    }
}

fn grow_fruit(world: &mut World, p: [i32; 3], stem: BlockId, h: u64, roll: u32) {
    let fruit = if stem == MELON_STEM { MELON } else { PUMPKIN };
    if roll >= 25 {
        return;
    }
    let sides = [[1, 0, 0], [-1, 0, 0], [0, 0, 1], [0, 0, -1]];
    if sides.iter().any(|d| world.block(near(p, *d)) == fruit) {
        return;
    }
    let d = sides[((h >> 8) % 4) as usize];
    let target = near(p, d);
    let ground = world.block(below(target));
    if loaded(world, target)
        && world.block(target) == AIR
        && (matches!(ground, GRASS | DIRT) || is_farmland(ground))
    {
        world.set(target, fruit);
    }
}

/// Один кадр падающих блоков. Возвращает число приземлившихся.
pub fn step_falling(world: &mut World, dt: f32) -> usize {
    if world.falling.is_empty() || !dt.is_finite() || dt <= 0.0 {
        return 0;
    }
    let mut landed = 0;
    let items = std::mem::take(&mut world.falling);
    let mut keep = Vec::with_capacity(items.len());
    for mut f in items {
        let cell = [
            f.position[0].floor() as i32,
            f.position[1].floor() as i32,
            f.position[2].floor() as i32,
        ];
        let mut remaining = dt;
        let mut done = false;
        while remaining > 0.0 {
            let h = remaining.min(0.01);
            remaining -= h;
            let y = f.position[1] + (f.velocity - 32.0 * h).max(-55.0) * h;
            let bottom = (y - 0.5).floor() as i32;
            let under = [cell[0], bottom, cell[2]];
            if !loaded(world, under) {
                break;
            }
            f.velocity = (f.velocity - 32.0 * h).max(-55.0);
            f.fallen += (f.position[1] - y).max(0.0);
            let block = world.block(under);
            if is_plant(block) {
                broke(world, under, block);
            } else if !hole(block) {
                let target = above(under);
                let occupant = world.block(target);
                if hole(occupant) || is_plant(occupant) {
                    let mut placed = f.block;
                    // Наковальня при ударе может треснуть, а треснувшая — разбиться.
                    if is_anvil(placed) && hash(world.seed, f.id, cell_key(target)) % 100 < 12 {
                        placed = match placed {
                            ANVIL => CHIPPED_ANVIL,
                            CHIPPED_ANVIL => DAMAGED_ANVIL,
                            _ => AIR,
                        };
                    }
                    if placed != AIR {
                        world.set(target, placed);
                    }
                    world.nature_events.push(NatureEvent::Landed {
                        position: target,
                        block: f.block,
                        fallen: f.fallen,
                    });
                } else {
                    world.nature_events.push(NatureEvent::Broke {
                        position: target,
                        block: f.block,
                    });
                }
                landed += 1;
                done = true;
                break;
            }
            f.position[1] = y;
            if y < -70.0 {
                done = true;
                break;
            }
        }
        if !done {
            keep.push(f);
        }
    }
    keep.extend(std::mem::take(&mut world.falling));
    world.falling = keep;
    landed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Chunk, STONE, Section};
    use std::collections::HashMap;

    const GROUND: i32 = 60;

    fn world() -> World {
        let mut world = World::new(5);
        for cx in -1..=1 {
            for cz in -1..=1 {
                let mut chunk = Chunk {
                    position: [cx, cz],
                    sections: HashMap::<i32, Section>::new(),
                };
                for x in 0..16 {
                    for z in 0..16 {
                        chunk.write([cx * 16 + x, GROUND, cz * 16 + z], STONE);
                    }
                }
                world.insert(chunk);
            }
        }
        world.nature_active.clear();
        world
    }

    fn settle(world: &mut World) {
        for step in 0..120 {
            tick(world, step, 256);
            step_falling(world, 0.05);
        }
    }

    #[test]
    fn sand_and_gravel_fall_to_the_ground_and_anvils_too() {
        let mut w = world();
        w.set([0, GROUND + 5, 0], SAND);
        w.set([2, GROUND + 3, 0], GRAVEL);
        w.set([4, GROUND + 4, 0], ANVIL);
        settle(&mut w);
        assert_eq!(w.block([0, GROUND + 1, 0]), SAND);
        assert_eq!(w.block([2, GROUND + 1, 0]), GRAVEL);
        assert!(is_anvil(w.block([4, GROUND + 1, 0])));
        assert_eq!(w.block([0, GROUND + 5, 0]), AIR);
    }

    #[test]
    fn removing_the_support_drops_a_whole_column() {
        let mut w = world();
        w.set([0, GROUND + 1, 0], DIRT);
        for y in 2..=4 {
            w.set([0, GROUND + y, 0], SAND);
        }
        w.set([0, GROUND + 1, 0], AIR);
        settle(&mut w);
        assert_eq!(w.block([0, GROUND + 1, 0]), SAND);
        assert_eq!(w.block([0, GROUND + 3, 0]), SAND);
        assert_eq!(w.block([0, GROUND + 4, 0]), AIR);
    }

    #[test]
    fn plants_break_without_ground_and_stay_with_it() {
        let mut w = world();
        w.set([0, GROUND + 1, 0], DIRT);
        w.set([0, GROUND + 2, 0], POPPY);
        w.set([3, GROUND + 1, 0], POPPY);
        w.set([0, GROUND + 1, 0], AIR);
        settle(&mut w);
        assert_eq!(w.block([0, GROUND + 2, 0]), AIR);
        assert_eq!(w.block([3, GROUND + 1, 0]), POPPY);
    }

    #[test]
    fn powder_hardens_next_to_water() {
        let mut w = world();
        w.set([0, GROUND + 1, 0], crate::WATER);
        w.set([1, GROUND + 1, 0], WHITE_CONCRETE_POWDER + 3);
        w.set([5, GROUND + 1, 0], WHITE_CONCRETE_POWDER);
        settle(&mut w);
        assert_eq!(w.block([1, GROUND + 1, 0]), CONCRETE_WHITE + 3);
        assert_eq!(w.block([5, GROUND + 1, 0]), WHITE_CONCRETE_POWDER);
    }

    #[test]
    fn concrete_ids_follow_powder_ids() {
        for i in 0..16u16 {
            let powder = BLOCKS[usize::from(WHITE_CONCRETE_POWDER + i)].name;
            let concrete = BLOCKS[usize::from(CONCRETE_WHITE + i)].name;
            assert_eq!(
                powder.replace(" Powder", "").replace(' ', "_"),
                concrete.replace(' ', "_"),
                "{powder} / {concrete}"
            );
        }
    }

    #[test]
    fn worldgen_leaves_decay_after_the_logs_go_but_placed_leaves_stay() {
        let mut w = world();
        // «Природная» крона: пишем в чанк напрямую, как это делает генератор.
        let chunk = std::sync::Arc::make_mut(w.chunks.get_mut(&[0, 0]).unwrap());
        chunk.write([5, GROUND + 1, 5], crate::WOOD);
        chunk.write([5, GROUND + 2, 5], crate::WOOD);
        chunk.write([5, GROUND + 3, 5], crate::LEAVES);
        chunk.write([6, GROUND + 3, 5], crate::LEAVES);
        w.set([10, GROUND + 3, 5], crate::LEAVES);
        w.set([5, GROUND + 2, 5], AIR);
        w.set([5, GROUND + 1, 5], AIR);
        w.nature_active.insert([5, GROUND + 3, 5]);
        for step in 0..2000 {
            tick(&mut w, step, 256);
        }
        assert_eq!(w.block([5, GROUND + 3, 5]), AIR);
        assert_eq!(w.block([6, GROUND + 3, 5]), AIR);
        assert_eq!(w.block([10, GROUND + 3, 5]), crate::LEAVES);
    }

    #[test]
    fn leaves_next_to_a_log_never_decay() {
        let mut w = world();
        let chunk = std::sync::Arc::make_mut(w.chunks.get_mut(&[0, 0]).unwrap());
        chunk.write([5, GROUND + 1, 5], crate::WOOD);
        chunk.write([5, GROUND + 2, 5], crate::LEAVES);
        w.nature_active.insert([5, GROUND + 2, 5]);
        for step in 0..200 {
            tick(&mut w, step, 256);
        }
        assert_eq!(w.block([5, GROUND + 2, 5]), crate::LEAVES);
    }

    #[test]
    fn grass_dies_under_blocks_and_spreads_to_open_dirt() {
        let mut w = world();
        w.set([0, GROUND + 1, 0], GRASS);
        w.set([0, GROUND + 2, 0], STONE);
        w.set([1, GROUND + 1, 0], GRASS);
        w.set([2, GROUND + 1, 0], DIRT);
        for step in 0..20000 {
            random_tick(&mut w, [0, GROUND, 0], step, 70);
        }
        assert_eq!(w.block([0, GROUND + 1, 0]), DIRT);
        assert_eq!(w.block([2, GROUND + 1, 0]), GRASS);
    }

    #[test]
    fn sugar_cane_needs_water_and_stops_at_three() {
        let mut w = world();
        w.set([0, GROUND + 1, 0], SAND);
        w.set([1, GROUND + 1, 0], crate::WATER);
        w.set([0, GROUND + 2, 0], SUGAR_CANE);
        w.set([6, GROUND + 1, 0], SAND);
        w.set([6, GROUND + 2, 0], SUGAR_CANE);
        for step in 0..60000 {
            random_tick(&mut w, [0, GROUND, 0], step, 70);
        }
        assert_eq!(w.block([0, GROUND + 4, 0]), SUGAR_CANE);
        assert_eq!(w.block([0, GROUND + 5, 0]), AIR);
        assert_eq!(w.block([6, GROUND + 3, 0]), AIR);
    }

    #[test]
    fn falling_blocks_accelerate_and_leave_the_world_while_in_flight() {
        let mut w = world();
        w.set([0, GROUND + 20, 0], SAND);
        tick(&mut w, 0, 16);
        assert_eq!(w.block([0, GROUND + 20, 0]), AIR);
        assert_eq!(w.falling.len(), 1);
        step_falling(&mut w, 0.1);
        let early = w.falling[0].position[1];
        step_falling(&mut w, 0.1);
        let second_step = early - w.falling[0].position[1];
        assert!(second_step > (GROUND as f32 + 20.5 - early), "ускоряется");
        for _ in 0..100 {
            step_falling(&mut w, 0.05);
        }
        assert!(w.falling.is_empty());
        assert_eq!(w.block([0, GROUND + 1, 0]), SAND);
    }

    #[test]
    fn a_falling_block_with_no_room_becomes_an_event() {
        let mut w = world();
        w.set([0, GROUND + 3, 0], SAND);
        tick(&mut w, 0, 16);
        w.set([0, GROUND + 1, 0], STONE);
        w.set([0, GROUND + 2, 0], STONE);
        for _ in 0..40 {
            step_falling(&mut w, 0.05);
        }
        assert!(w.falling.is_empty());
    }

    #[test]
    fn broken_plants_and_decayed_leaves_report_events() {
        let mut w = world();
        w.set([0, GROUND + 1, 0], DIRT);
        w.set([0, GROUND + 2, 0], POPPY);
        w.set([0, GROUND + 1, 0], AIR);
        settle(&mut w);
        assert!(w.nature_events.iter().any(|e| matches!(
            e,
            NatureEvent::Broke { block, .. } if *block == POPPY
        )));
    }

    fn farm(w: &mut World, crop: BlockId) {
        w.set([0, GROUND + 1, 0], FARMLAND);
        w.set([0, GROUND + 2, 0], crop);
    }

    #[test]
    fn wheat_grows_through_every_stage_on_farmland_only() {
        let mut w = world();
        farm(&mut w, WHEAT_0);
        w.set([6, GROUND + 1, 0], DIRT);
        w.set([6, GROUND + 2, 0], WHEAT_0);
        settle(&mut w);
        assert_eq!(
            w.block([6, GROUND + 2, 0]),
            AIR,
            "на земле без грядки не растёт"
        );
        for step in 0..30000 {
            random_tick(&mut w, [0, GROUND, 0], step, 70);
            tick(&mut w, step, 64);
        }
        assert_eq!(w.block([0, GROUND + 2, 0]), WHEAT);
    }

    #[test]
    fn every_crop_has_three_stages_then_the_ripe_block() {
        for (first, ripe) in [
            (WHEAT_0, WHEAT),
            (CARROTS_0, CARROTS),
            (POTATOES_0, POTATOES),
            (BEETROOTS_0, BEETROOTS),
        ] {
            let mut id = first;
            let mut stages = 0;
            while let Some(next) = crop_next(id) {
                id = next;
                stages += 1;
            }
            assert_eq!((id, stages), (ripe, 3));
        }
    }

    #[test]
    fn water_moistens_farmland_and_dry_empty_farmland_turns_to_dirt() {
        let mut w = world();
        w.set([0, GROUND + 1, 0], FARMLAND);
        w.set([3, GROUND + 1, 0], WATER);
        w.nature_active.insert([0, GROUND + 1, 0]);
        settle(&mut w);
        assert_eq!(w.block([0, GROUND + 1, 0]), FARMLAND_MOIST);
        w.set([10, GROUND + 1, 10], FARMLAND);
        for step in 0..30000 {
            random_tick(&mut w, [0, GROUND, 0], step, 70);
        }
        assert_eq!(w.block([10, GROUND + 1, 10]), DIRT);
    }

    #[test]
    fn bamboo_and_kelp_grow_and_ice_melts_next_to_light() {
        let mut w = world();
        w.set([0, GROUND + 1, 0], DIRT);
        w.set([0, GROUND + 2, 0], BAMBOO);
        w.set([8, GROUND, 8], WATER);
        w.set([8, GROUND + 1, 8], KELP);
        w.set([8, GROUND + 2, 8], WATER);
        w.set([8, GROUND + 3, 8], WATER);
        w.set([8, GROUND + 4, 8], WATER);
        w.set([12, GROUND + 1, 0], ICE);
        w.set([13, GROUND + 1, 0], crate::TORCH);
        w.set([12, GROUND + 1, 4], ICE);
        for step in 0..60000 {
            random_tick(&mut w, [4, GROUND, 4], step, 70);
        }
        assert!(is_bamboo(w.block([0, GROUND + 4, 0])), "бамбук вырос");
        assert!(matches!(
            w.block([8, GROUND + 2, 8]),
            KELP | KELP_PLANT | KELP_SPROUT
        ));
        assert_eq!(w.block([12, GROUND + 1, 0]), WATER, "лёд у факела растаял");
        assert_eq!(w.block([12, GROUND + 1, 4]), ICE, "лёд без света цел");
    }

    #[test]
    fn trampling_turns_farmland_to_dirt_and_breaks_the_crop() {
        let mut w = world();
        farm(&mut w, WHEAT_1);
        assert!(trample(&mut w, [0, GROUND + 1, 0]));
        assert_eq!(w.block([0, GROUND + 1, 0]), DIRT);
        assert_eq!(w.block([0, GROUND + 2, 0]), AIR);
        assert!(w.nature_events.iter().any(|e| matches!(
            e,
            NatureEvent::Broke { block, .. } if *block == WHEAT_1
        )));
        assert!(
            !trample(&mut w, [3, GROUND + 1, 0]),
            "не грядка — не топчется"
        );
    }

    #[test]
    fn anvil_remembers_its_fall_and_reports_the_landing() {
        let mut w = world();
        w.set([0, GROUND + 12, 0], ANVIL);
        tick(&mut w, 0, 16);
        for _ in 0..100 {
            step_falling(&mut w, 0.02);
            if w.falling.is_empty() {
                break;
            }
            assert!(!w.falling[0].struck);
        }
        let landed = w.nature_events.iter().find_map(|e| match e {
            NatureEvent::Landed { block, fallen, .. } => Some((*block, *fallen)),
            _ => None,
        });
        let (block, fallen) = landed.expect("приземление");
        assert_eq!(block, ANVIL);
        assert!((fallen - 10.5).abs() < 1.0, "{fallen}");
        assert!(is_anvil(w.block([0, GROUND + 1, 0])) || w.block([0, GROUND + 1, 0]) == AIR);
    }

    #[test]
    fn land_all_puts_every_flying_block_and_primed_tnt_back_into_the_world() {
        let mut w = world();
        w.set([0, GROUND + 30, 0], SAND);
        w.set([4, GROUND + 1, 4], TNT);
        w.blasts.push([4, GROUND + 1, 4]);
        crate::mechanics::run_blasts(&mut w, 4);
        tick(&mut w, 0, 16);
        assert_eq!((w.falling.len(), w.primed.len()), (1, 1));
        land_all(&mut w);
        assert!(w.falling.is_empty() && w.primed.is_empty());
        assert_eq!(w.block([0, GROUND + 1, 0]), SAND);
        assert_eq!(w.block([4, GROUND + 1, 4]), TNT);
    }

    #[test]
    fn bamboo_grows_leaf_stages_from_sapling_to_tall_stalk() {
        let mut w = world();
        w.set([0, GROUND + 1, 0], DIRT);
        w.set([0, GROUND + 2, 0], BAMBOO_SAPLING);
        for step in 0..120000 {
            random_tick(&mut w, [0, GROUND, 0], step, 70);
            if w.block([0, GROUND + 6, 0]) != AIR {
                break;
            }
        }
        assert_eq!(
            w.block([0, GROUND + 2, 0]),
            BAMBOO,
            "основание — голый стебель"
        );
        let top = (2..=13)
            .map(|y| w.block([0, GROUND + y, 0]))
            .rfind(|b| is_bamboo(*b))
            .unwrap();
        assert_eq!(top, BAMBOO_LARGE);
        let smalls = (2..=13)
            .filter(|y| w.block([0, GROUND + y, 0]) == BAMBOO_SMALL)
            .count();
        assert_eq!(smalls, 1, "под большой кроной один маленький ярус");
    }

    #[test]
    fn a_new_kelp_tip_starts_as_a_sprout_and_matures() {
        let mut w = world();
        w.set([8, GROUND, 8], WATER);
        w.set([8, GROUND + 1, 8], KELP);
        for dy in 2..=6 {
            w.set([8, GROUND + dy, 8], WATER);
        }
        let mut saw_sprout = false;
        for step in 0..120000 {
            random_tick(&mut w, [8, GROUND, 8], step, 70);
            saw_sprout |= (1..=6).any(|dy| w.block([8, GROUND + dy, 8]) == KELP_SPROUT);
            if saw_sprout && w.block([8, GROUND + 3, 8]) == KELP {
                break;
            }
        }
        assert!(saw_sprout);
        assert_eq!(w.block([8, GROUND + 1, 8]), KELP_PLANT);
    }
}
