//! Поведение новых блоков: двери от рычага, кнопки и плит, взрыв динамита,
//! цветочные горшки, опасные и вязкие блоки.

use crate::{
    AIR, BEDROCK, BLOCKS, BlockId, FamilyKind, TNT, World, family_of, is_fluid,
    models::{Shape, shape_of},
};

/// Радиус взрыва динамита в клетках.
pub const BLAST_RADIUS: i32 = 3;
/// Радиус действия рычага, кнопки и нажимной плиты вокруг неё.
pub const POWER_RADIUS: i32 = 2;

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

/// Подаёт или снимает сигнал вокруг `origin`: двери, калитки и люки открываются и
/// закрываются, динамит при сигнале ставится в очередь на взрыв. Возвращает число
/// изменённых блоков.
pub fn power_area(world: &mut World, origin: [i32; 3], powered: bool) -> usize {
    let mut changed = 0;
    for dx in -POWER_RADIUS..=POWER_RADIUS {
        for dy in -POWER_RADIUS..=POWER_RADIUS {
            for dz in -POWER_RADIUS..=POWER_RADIUS {
                let p = [origin[0] + dx, origin[1] + dy, origin[2] + dz];
                if p == origin || !crate::fluids::loaded(world, p) {
                    continue;
                }
                let id = world.block(p);
                if let Some(next) = with_open(id, powered)
                    && world.set(p, next)
                {
                    changed += 1;
                }
                if powered && id == TNT {
                    world.blasts.push(p);
                    changed += 1;
                }
            }
        }
    }
    changed
}

/// Выполняет не более `budget` взрывов из очереди. Возвращает центры взрывов, чтобы
/// клиент мог нанести урон игроку и сыграть звук.
pub fn run_blasts(world: &mut World, budget: usize) -> Vec<[i32; 3]> {
    let mut centers = Vec::new();
    while centers.len() < budget {
        let Some(center) = world.blasts.pop() else {
            break;
        };
        if world.block(center) != TNT {
            continue;
        }
        world.set(center, AIR);
        for dx in -BLAST_RADIUS..=BLAST_RADIUS {
            for dy in -BLAST_RADIUS..=BLAST_RADIUS {
                for dz in -BLAST_RADIUS..=BLAST_RADIUS {
                    if dx * dx + dy * dy + dz * dz > BLAST_RADIUS * BLAST_RADIUS {
                        continue;
                    }
                    let p = [center[0] + dx, center[1] + dy, center[2] + dz];
                    if !crate::fluids::loaded(world, p) {
                        continue;
                    }
                    let id = world.block(p);
                    if id == AIR || id == BEDROCK || is_fluid(id) {
                        continue;
                    }
                    if id == TNT {
                        world.blasts.push(p);
                        continue;
                    }
                    // Прочные блоки (обсидиан и крепче) выдерживают взрыв.
                    if BLOCKS[usize::from(id)].hardness < 50.0 {
                        world.set(p, AIR);
                    }
                }
            }
        }
        centers.push(center);
    }
    centers
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
    fn a_lever_opens_and_closes_nearby_doors_both_halves_and_primes_tnt() {
        let mut w = world();
        w.set([2, 70, 2], OAK_DOOR);
        w.set([2, 71, 2], OAK_DOOR + 8);
        w.set([1, 70, 2], OAK_TRAPDOOR);
        w.set([30, 70, 2], OAK_DOOR);
        w.set([0, 70, 1], TNT);
        assert_eq!(power_area(&mut w, [0, 70, 0], true), 4);
        assert_eq!(w.block([2, 70, 2]), OAK_DOOR + 4);
        assert_eq!(w.block([2, 71, 2]), OAK_DOOR + 12);
        assert_eq!(w.block([1, 70, 2]), OAK_TRAPDOOR + 1);
        assert_eq!(w.block([30, 70, 2]), OAK_DOOR, "далёкая дверь не тронута");
        assert_eq!(w.blasts, vec![[0, 70, 1]]);
        power_area(&mut w, [0, 70, 0], false);
        assert_eq!(w.block([2, 70, 2]), OAK_DOOR);
        assert_eq!(w.block([1, 70, 2]), OAK_TRAPDOOR);
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
        let centers = run_blasts(&mut w, 1);
        assert_eq!(centers, vec![[0, 70, 0]]);
        assert_eq!(w.block([0, 71, 0]), AIR);
        assert_eq!(w.block([0, 70, 2]), OBSIDIAN);
        assert_eq!(w.block([4, 70, 4]), STONE, "за радиусом цело");
        // Соседний динамит в зоне взрыва ждёт своей очереди.
        assert_eq!(w.block([2, 70, 0]), TNT);
        assert_eq!(w.blasts, vec![[2, 70, 0]]);
        let more = run_blasts(&mut w, 8);
        assert_eq!(more, vec![[2, 70, 0]]);
        assert_eq!(w.block([2, 70, 0]), AIR);
        assert!(run_blasts(&mut w, 8).is_empty());
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
