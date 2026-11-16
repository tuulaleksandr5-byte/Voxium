//! Поведение новых блоков: взрыв динамита, цветочные горшки, опасные и вязкие блоки.
//! Сигнал редстоуна живёт в модуле `redstone`.

use crate::{
    AIR, BEDROCK, BLOCKS, BlockId, FamilyKind, TNT, World, family_of, is_fluid,
    models::{Shape, shape_of},
};

/// Радиус взрыва динамита в клетках.
pub const BLAST_RADIUS: i32 = 3;

/// Дверь, калитка или люк в нужном состоянии; `None`, если блок не открывается
/// или уже в этом состоянии.
#[must_use]
pub fn with_open(id: BlockId, open: bool) -> Option<BlockId> {
    let (first, index, kind) = family_of(id)?;
    let target = match kind {
        FamilyKind::Door | FamilyKind::Gate => {
            if shape_of(id) != Some(Shape::Door) && shape_of(id) != Some(Shape::Gate) {
                return None;
            }
            (index & !4) | (u8::from(open) * 4)
        }
        FamilyKind::Trap => u8::from(open),
        _ => return None,
    };
    (target != index).then(|| first + BlockId::from(target))
}

/// Включён ли рычаг (по его состоянию).
#[must_use]
pub fn lever_on(id: BlockId) -> Option<bool> {
    (shape_of(id) == Some(Shape::Lever)).then(|| family_of(id).is_some_and(|(_, i, _)| i >= 4))
}

/// Динамит с горящим фитилём: подпрыгивает, мигает и взрывается через `fuse` секунд.
#[derive(Clone, Debug, PartialEq)]
pub struct PrimedTnt {
    /// Стабильный номер для отрисовки.
    pub id: u64,
    /// Центр блока.
    pub position: [f32; 3],
    /// Вертикальная скорость, блоков в секунду.
    pub velocity: f32,
    /// Секунд до взрыва.
    pub fuse: f32,
}

/// Длина фитиля динамита, зажжённого игроком, огнём или редстоуном.
pub const FUSE_SECONDS: f32 = 4.0;

/// Поджигает не более `budget` блоков динамита из очереди `world.blasts`:
/// блок исчезает, вместо него появляется горящий динамит. Возвращает число зажжённых.
pub fn run_blasts(world: &mut World, budget: usize) -> usize {
    let mut lit = 0;
    while lit < budget {
        let Some(cell) = world.blasts.pop() else {
            break;
        };
        if world.block(cell) != TNT {
            continue;
        }
        world.set(cell, AIR);
        let id = world.next_entity_id();
        world.primed.push(PrimedTnt {
            id,
            position: [
                cell[0] as f32 + 0.5,
                cell[1] as f32 + 0.5,
                cell[2] as f32 + 0.5,
            ],
            velocity: 4.0,
            fuse: FUSE_SECONDS,
        });
        lit += 1;
    }
    lit
}

/// Один кадр горящего динамита. Возвращает центры взрывов этого кадра.
pub fn step_primed(world: &mut World, dt: f32) -> Vec<[f32; 3]> {
    if world.primed.is_empty() || !dt.is_finite() || dt <= 0.0 {
        return Vec::new();
    }
    let mut exploded = Vec::new();
    let mut keep = Vec::new();
    for mut tnt in std::mem::take(&mut world.primed) {
        let mut remaining = dt;
        while remaining > 0.0 {
            let h = remaining.min(0.02);
            remaining -= h;
            let cell = [
                tnt.position[0].floor() as i32,
                tnt.position[1].floor() as i32,
                tnt.position[2].floor() as i32,
            ];
            let bottom = [cell[0], (tnt.position[1] - 0.5).floor() as i32, cell[2]];
            tnt.velocity = (tnt.velocity - 16.0 * h).max(-30.0);
            tnt.position[1] += tnt.velocity * h;
            if tnt.velocity <= 0.0
                && crate::fluids::loaded(world, bottom)
                && crate::is_solid(world.block(bottom))
                && tnt.position[1] - 0.5 < (bottom[1] + 1) as f32
            {
                tnt.position[1] = bottom[1] as f32 + 1.5;
                tnt.velocity = 0.0;
            }
        }
        tnt.fuse -= dt;
        if tnt.fuse <= 0.0 {
            exploded.push(tnt.position);
        } else {
            keep.push(tnt);
        }
    }
    world.primed = keep;
    for center in &exploded {
        explode(world, *center);
    }
    exploded
}

/// Разрушает шар вокруг `center`; соседний динамит загорается с коротким фитилём.
fn explode(world: &mut World, center: [f32; 3]) {
    let cell = center.map(|v| v.floor() as i32);
    for dx in -BLAST_RADIUS..=BLAST_RADIUS {
        for dy in -BLAST_RADIUS..=BLAST_RADIUS {
            for dz in -BLAST_RADIUS..=BLAST_RADIUS {
                if dx * dx + dy * dy + dz * dz > BLAST_RADIUS * BLAST_RADIUS {
                    continue;
                }
                let p = [cell[0] + dx, cell[1] + dy, cell[2] + dz];
                if !crate::fluids::loaded(world, p) {
                    continue;
                }
                let id = world.block(p);
                if id == AIR || id == BEDROCK || is_fluid(id) {
                    continue;
                }
                if id == TNT {
                    world.set(p, AIR);
                    let jitter =
                        ((p[0].wrapping_mul(31) ^ p[1].wrapping_mul(17) ^ p[2]) & 7) as f32;
                    let id = world.next_entity_id();
                    world.primed.push(PrimedTnt {
                        id,
                        position: [p[0] as f32 + 0.5, p[1] as f32 + 0.5, p[2] as f32 + 0.5],
                        velocity: 3.0,
                        fuse: 0.5 + jitter / 8.0,
                    });
                    continue;
                }
                // Прочные блоки (обсидиан и крепче) выдерживают взрыв.
                if BLOCKS[usize::from(id)].hardness < 50.0 {
                    world.set(p, AIR);
                }
            }
        }
    }
}

/// Урон игроку от взрыва на расстоянии `distance` клеток: до 10 у центра, 0 дальше
/// двойного радиуса.
#[must_use]
pub fn blast_damage(distance: f32) -> f32 {
    let reach = 2.0 * BLAST_RADIUS as f32;
    if distance >= reach {
        0.0
    } else {
        10.0 * (1.0 - distance / reach)
    }
}

/// Цветочный горшок с этим растением; `None`, если растение в горшок не садится.
#[must_use]
pub fn potted_for(plant: BlockId) -> Option<BlockId> {
    use crate::*;
    Some(match plant {
        TORCHFLOWER => POTTED_TORCHFLOWER,
        OAK_SAPLING => POTTED_OAK_SAPLING,
        SPRUCE_SAPLING => POTTED_SPRUCE_SAPLING,
        BIRCH_SAPLING => POTTED_BIRCH_SAPLING,
        JUNGLE_SAPLING => POTTED_JUNGLE_SAPLING,
        ACACIA_SAPLING => POTTED_ACACIA_SAPLING,
        CHERRY_SAPLING => POTTED_CHERRY_SAPLING,
        DARK_OAK_SAPLING => POTTED_DARK_OAK_SAPLING,
        MANGROVE_PROPAGULE => POTTED_MANGROVE_PROPAGULE,
        FERN => POTTED_FERN,
        DANDELION => POTTED_DANDELION,
        POPPY => POTTED_POPPY,
        BLUE_ORCHID => POTTED_BLUE_ORCHID,
        ALLIUM => POTTED_ALLIUM,
        AZURE_BLUET => POTTED_AZURE_BLUET,
        RED_TULIP => POTTED_RED_TULIP,
        ORANGE_TULIP => POTTED_ORANGE_TULIP,
        WHITE_TULIP => POTTED_WHITE_TULIP,
        PINK_TULIP => POTTED_PINK_TULIP,
        DAISY => POTTED_OXEYE_DAISY,
        CORNFLOWER => POTTED_CORNFLOWER,
        LILY_OF_THE_VALLEY => POTTED_LILY_OF_THE_VALLEY,
        WITHER_ROSE => POTTED_WITHER_ROSE,
        RED_MUSHROOM => POTTED_RED_MUSHROOM,
        BROWN_MUSHROOM => POTTED_BROWN_MUSHROOM,
        DEAD_BUSH => POTTED_DEAD_BUSH,
        CACTUS => POTTED_CACTUS,
        CRIMSON_FUNGUS => POTTED_CRIMSON_FUNGUS,
        WARPED_FUNGUS => POTTED_WARPED_FUNGUS,
        CRIMSON_ROOTS => POTTED_CRIMSON_ROOTS,
        WARPED_ROOTS => POTTED_WARPED_ROOTS,
        AZALEA => POTTED_AZALEA_BUSH,
        FLOWERING_AZALEA => POTTED_FLOWERING_AZALEA_BUSH,
        BAMBOO => POTTED_BAMBOO,
        _ => return None,
    })
}

/// Урон за контакт: (урон за полсекунды, поджигает ли).
#[must_use]
pub fn contact_damage(id: BlockId) -> Option<f32> {
    use crate::*;
    match id {
        CACTUS | POTTED_CACTUS => Some(1.0),
        MAGMA_BLOCK => Some(1.0),
        SWEET_BERRY_BUSH => Some(0.5),
        _ => None,
    }
}

/// Огонь костра жжёт так же, как обычный.
#[must_use]
pub fn is_campfire_flame(id: BlockId) -> bool {
    matches!(id, crate::CAMPFIRE | crate::SOUL_CAMPFIRE)
}

/// Во сколько раз блок замедляет проходящего (1 — не замедляет).
#[must_use]
pub fn movement_factor(id: BlockId) -> f32 {
    use crate::*;
    match id {
        SOUL_SAND => 0.6,
        HONEY_BLOCK => 0.4,
        SWEET_BERRY_BUSH => 0.5,
        POWDER_SNOW => 0.3,
        _ => 1.0,
    }
}

/// Лазить можно по лестнице, лозам и подмосткам.
#[must_use]
pub fn climbable(id: BlockId) -> bool {
    use crate::*;
    crate::models::is_climbable(id)
        || matches!(
            id,
            SCAFFOLDING
                | VINE
                | WEEPING_VINES
                | WEEPING_VINES_PLANT
                | TWISTING_VINES
                | TWISTING_VINES_PLANT
                | CAVE_VINES
                | CAVE_VINES_PLANT
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::{collections::HashMap, sync::Arc};

    fn world() -> World {
        let mut world = World::new(1);
        for cx in -1..=1 {
            for cz in -1..=1 {
                world.chunks.insert(
                    [cx, cz],
                    Arc::new(Chunk {
                        position: [cx, cz],
                        sections: HashMap::new(),
                    }),
                );
            }
        }
        world
    }

    #[test]
    fn doors_gates_and_trapdoors_follow_the_requested_state_and_ignore_other_blocks() {
        let closed = OAK_DOOR + 1;
        let open = with_open(closed, true).unwrap();
        assert_eq!(open, OAK_DOOR + 5);
        assert_eq!(with_open(open, true), None);
        assert_eq!(with_open(open, false), Some(closed));
        assert_eq!(
            with_open(OAK_FENCE_GATE + 2, true),
            Some(OAK_FENCE_GATE + 6)
        );
        assert_eq!(with_open(OAK_TRAPDOOR, true), Some(OAK_TRAPDOOR + 1));
        assert_eq!(with_open(OAK_TRAPDOOR + 3, false), Some(OAK_TRAPDOOR));
        assert_eq!(with_open(STONE, true), None);
        assert_eq!(with_open(OAK_STAIRS, true), None);
        assert_eq!(lever_on(LEVER), Some(false));
        assert_eq!(lever_on(LEVER + 5), Some(true));
        assert_eq!(lever_on(STONE), None);
    }

    #[test]
    fn tnt_destroys_a_sphere_keeps_bedrock_and_obsidian_and_chains_to_neighbours() {
        let mut w = world();
        for x in -4..=4 {
            for y in 66..=74 {
                for z in -4..=4 {
                    w.set([x, y, z], STONE);
                }
            }
        }
        w.set([0, 70, 0], TNT);
        w.set([2, 70, 0], TNT);
        w.set([0, 70, 2], OBSIDIAN);
        w.blasts.push([0, 70, 0]);
        assert_eq!(run_blasts(&mut w, 8), 1);
        // Фитиль горит: блок исчез, но взрыва ещё нет.
        assert_eq!(w.block([0, 70, 0]), AIR);
        assert_eq!(w.primed.len(), 1);
        assert!(step_primed(&mut w, 1.0).is_empty());
        assert_eq!(w.block([0, 71, 0]), STONE);
        let mut booms = Vec::new();
        for _ in 0..80 {
            booms.extend(step_primed(&mut w, 0.1));
        }
        assert_eq!(booms.len(), 2, "второй динамит загорается от первого");
        assert_eq!(w.block([0, 71, 0]), AIR);
        assert_eq!(w.block([0, 70, 2]), OBSIDIAN);
        assert_eq!(w.block([4, 70, 4]), STONE, "за радиусом цело");
        assert_eq!(w.block([2, 70, 0]), AIR);
        assert!(w.primed.is_empty());
        assert!(blast_damage(0.0) > 9.0 && blast_damage(3.0) < blast_damage(1.0));
        assert_eq!(blast_damage(6.0), 0.0);
    }

    #[test]
    fn pots_hazards_slowing_and_climbing_cover_the_documented_blocks() {
        assert_eq!(potted_for(POPPY), Some(POTTED_POPPY));
        assert_eq!(potted_for(BAMBOO), Some(POTTED_BAMBOO));
        assert_eq!(potted_for(STONE), None);
        assert!(contact_damage(CACTUS).is_some() && contact_damage(STONE).is_none());
        assert!(movement_factor(SOUL_SAND) < 1.0 && movement_factor(STONE) == 1.0);
        for id in [LADDER, SCAFFOLDING, VINE, CAVE_VINES_PLANT] {
            assert!(climbable(id), "{id}");
        }
        assert!(!climbable(OAK_FENCE));
        assert!(is_campfire_flame(SOUL_CAMPFIRE) && !is_campfire_flame(TORCH));
    }
}
