//! Формы блоков, которые не являются целым кубом: ступеньки, таблички, двери,
//! люки, ограды, лестницы и т. п. Форма — набор коробок в шестнадцатых долях клетки.

use crate::{BlockId, Facing, FamilyKind, family_of};

/// Коробка внутри клетки, координаты 0…16. `aux` — рисуется дополнительным тайлом.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModelBox {
    pub min: [u8; 3],
    pub max: [u8; 3],
    pub aux: bool,
}

/// Блоки, соединяющиеся с соседями.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connect {
    Fence,
    Pane,
    Wall,
}

const fn bx(min: [u8; 3], max: [u8; 3]) -> ModelBox {
    ModelBox {
        min,
        max,
        aux: false,
    }
}

const fn ax(min: [u8; 3], max: [u8; 3]) -> ModelBox {
    ModelBox {
        min,
        max,
        aux: true,
    }
}

/// Поворачивает коробку, нарисованную для юга (стена или спина в −Z), на сторону света.
fn rotate(b: ModelBox, facing: Facing) -> ModelBox {
    let map = |x: u8, z: u8| -> (u8, u8) {
        match facing {
            Facing::South | Facing::Up | Facing::Down => (x, z),
            Facing::North => (16 - x, 16 - z),
            Facing::West => (16 - z, x),
            Facing::East => (z, 16 - x),
        }
    };
    let (x0, z0) = map(b.min[0], b.min[2]);
    let (x1, z1) = map(b.max[0], b.max[2]);
    ModelBox {
        min: [x0.min(x1), b.min[1], z0.min(z1)],
        max: [x0.max(x1), b.max[1], z0.max(z1)],
        aux: b.aux,
    }
}

fn rotated(boxes: &[ModelBox], facing: Facing) -> Vec<ModelBox> {
    boxes.iter().map(|b| rotate(*b, facing)).collect()
}

fn turned(facing: Facing) -> Facing {
    match facing {
        Facing::North => Facing::East,
        Facing::East => Facing::South,
        Facing::South => Facing::West,
        Facing::West | Facing::Up | Facing::Down => Facing::North,
    }
}

fn h4(index: u8) -> Facing {
    Facing::ALL[usize::from(index & 3)]
}

fn in_range(id: BlockId, first: BlockId, count: BlockId) -> bool {
    id >= first && id < first + count
}

/// Вид модели блока.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Stairs,
    Sign,
    WallSign,
    Hanging,
    WallHanging,
    Door,
    Trapdoor,
    Ladder,
    Lever,
    Button,
    Plate,
    Farmland,
    SnowLayer,
    Cactus,
    Cake,
    Repeater,
    Chain,
    Portal,
    Fence,
    Pane,
    Wall,
    Gate,
    Potted,
    Pot,
    EnchantingTable,
    Brewing,
    Cauldron,
    EndPortal,
    EndFrame,
    Egg,
    EnderChest,
    Hook,
    Head,
    WallHead,
    Anvil,
    Hopper,
    Daylight,
    Carpet,
    Banner,
    WallBanner,
    Rod,
    ChorusPlant,
    ChorusFlower,
    DirtPath,
}

/// Форма блока, если он не обычный куб, не растение и не плоский рисунок.
#[must_use]
pub fn shape_of(id: BlockId) -> Option<Shape> {
    #[allow(unused_imports)]
    use crate::*;
    use crate::{
        CACTUS, CAKE, CHAIN, COBBLESTONE_STAIRS, FARMLAND, GLASS_PANE, IRON_BARS, IRON_DOOR,
        LADDER, LEVER, NETHER_PORTAL, OAK_DOOR, OAK_FENCE, OAK_HANGING_SIGN, OAK_PRESSURE_PLATE,
        OAK_SIGN, OAK_STAIRS, OAK_TRAPDOOR, OAK_WALL_HANGING_SIGN, OAK_WALL_SIGN, REPEATER,
        SNOW_LAYER, STONE_BUTTON, STONE_PRESSURE_PLATE,
    };
    Some(match id {
        _ if in_range(id, OAK_STAIRS, 4)
            || in_range(id, COBBLESTONE_STAIRS, 4)
            || in_range(id, BRICK_STAIRS, 12)
            || in_range(id, NETHER_BRICK_STAIRS, 4)
            || in_range(id, SANDSTONE_STAIRS, 4)
            || in_range(id, SPRUCE_STAIRS, 12) =>
        {
            Shape::Stairs
        }
        _ if in_range(id, QUARTZ_STAIRS, 4)
            || in_range(id, ACACIA_STAIRS, 24)
            || in_range(id, PRISMARINE_STAIRS, 12)
            || in_range(id, RED_SANDSTONE_STAIRS, 4) =>
        {
            Shape::Stairs
        }
        _ if in_range(id, WHITE_STAINED_GLASS_PANE, 16) => Shape::Pane,
        _ if in_range(id, IRON_TRAPDOOR, 5) => Shape::Trapdoor,
        _ if in_range(id, WHITE_CARPET, 16) => Shape::Carpet,
        _ if in_range(id, WHITE_BANNER, 64) => Shape::Banner,
        _ if in_range(id, WHITE_WALL_BANNER, 64) => Shape::WallBanner,
        _ if in_range(id, OAK_FENCE_GATE, 8) || in_range(id, SPRUCE_FENCE_GATE, 64) => Shape::Gate,
        _ if in_range(id, SPRUCE_FENCE, 8) => Shape::Fence,
        _ if in_range(id, SPRUCE_DOOR, 128) => Shape::Door,
        _ if in_range(id, PURPUR_STAIRS, 4) => Shape::Stairs,
        END_ROD => Shape::Rod,
        CHORUS_PLANT => Shape::ChorusPlant,
        CHORUS_FLOWER => Shape::ChorusFlower,
        DIRT_PATH => Shape::DirtPath,
        COBBLESTONE_WALL | MOSSY_COBBLESTONE_WALL => Shape::Wall,
        NETHER_BRICK_FENCE => Shape::Fence,
        FLOWER_POT => Shape::Pot,
        _ if (POTTED_TORCHFLOWER..=POTTED_CACTUS).contains(&id) => Shape::Potted,
        ENCHANTING_TABLE => Shape::EnchantingTable,
        BREWING_STAND => Shape::Brewing,
        _ if in_range(id, CAULDRON, 4) => Shape::Cauldron,
        END_PORTAL => Shape::EndPortal,
        END_PORTAL_FRAME => Shape::EndFrame,
        DRAGON_EGG => Shape::Egg,
        ENDER_CHEST => Shape::EnderChest,
        _ if in_range(id, TRIPWIRE_HOOK, 4) => Shape::Hook,
        SKELETON_SKULL
        | WITHER_SKELETON_SKULL
        | ZOMBIE_HEAD
        | PLAYER_HEAD
        | CREEPER_HEAD
        | DRAGON_HEAD
        | PIGLIN_HEAD => Shape::Head,
        _ if in_range(id, SKELETON_WALL_SKULL, 4)
            || in_range(id, WITHER_SKELETON_WALL_SKULL, 4)
            || in_range(id, ZOMBIE_WALL_HEAD, 4)
            || in_range(id, PLAYER_WALL_HEAD, 4)
            || in_range(id, CREEPER_WALL_HEAD, 4)
            || in_range(id, DRAGON_WALL_HEAD, 4)
            || in_range(id, PIGLIN_WALL_HEAD, 4) =>
        {
            Shape::WallHead
        }
        _ if in_range(id, ANVIL, 6) => Shape::Anvil,
        HOPPER => Shape::Hopper,
        DAYLIGHT_DETECTOR => Shape::Daylight,
        LIGHT_WEIGHTED_PRESSURE_PLATE | HEAVY_WEIGHTED_PRESSURE_PLATE => Shape::Plate,
        _ if in_range(id, OAK_BUTTON, 36) => Shape::Button,
        _ if in_range(id, COMPARATOR, 4) => Shape::Repeater,
        _ if in_range(id, OAK_SIGN, 36) => Shape::Sign,
        _ if in_range(id, OAK_WALL_SIGN, 36) => Shape::WallSign,
        _ if in_range(id, OAK_HANGING_SIGN, 22) => Shape::Hanging,
        _ if in_range(id, OAK_WALL_HANGING_SIGN, 22) => Shape::WallHanging,
        _ if in_range(id, OAK_DOOR, 16) || in_range(id, IRON_DOOR, 16) => Shape::Door,
        _ if in_range(id, OAK_TRAPDOOR, 45) => Shape::Trapdoor,
        _ if in_range(id, LADDER, 4) => Shape::Ladder,
        _ if in_range(id, LEVER, 8) => Shape::Lever,
        _ if in_range(id, STONE_BUTTON, 4) => Shape::Button,
        STONE_PRESSURE_PLATE => Shape::Plate,
        _ if in_range(id, OAK_PRESSURE_PLATE, 9) => Shape::Plate,
        FARMLAND => Shape::Farmland,
        SNOW_LAYER => Shape::SnowLayer,
        CACTUS => Shape::Cactus,
        CAKE => Shape::Cake,
        _ if in_range(id, REPEATER, 4) => Shape::Repeater,
        CHAIN => Shape::Chain,
        _ if in_range(id, NETHER_PORTAL, 2) => Shape::Portal,
        OAK_FENCE => Shape::Fence,
        IRON_BARS | GLASS_PANE => Shape::Pane,
        _ => return None,
    })
}

/// Рисуется ли блок коробками, а не кубом.
#[must_use]
pub fn is_model(id: BlockId) -> bool {
    id >= crate::PART2_FIRST && shape_of(id).is_some()
}

#[must_use]
pub fn connect_kind(id: BlockId) -> Option<Connect> {
    match shape_of(id)? {
        Shape::Fence => Some(Connect::Fence),
        Shape::Pane => Some(Connect::Pane),
        Shape::Wall => Some(Connect::Wall),
        _ => None,
    }
}

/// Соединяется ли ограда или панель с соседним блоком.
#[must_use]
pub fn connects_to(kind: Connect, neighbor: BlockId) -> bool {
    let same = match kind {
        Connect::Fence => shape_of(neighbor) == Some(Shape::Fence),
        Connect::Pane => shape_of(neighbor) == Some(Shape::Pane),
        Connect::Wall => shape_of(neighbor) == Some(Shape::Wall),
    };
    same || (crate::is_opaque(neighbor)
        && !crate::is_sprite(neighbor)
        && !crate::is_leaves(neighbor))
}

/// Коробки ограды или панели при заданных соединениях: север, юг, запад, восток.
#[must_use]
pub fn connected_boxes(kind: Connect, sides: [bool; 4]) -> Vec<ModelBox> {
    let mut boxes = Vec::new();
    match kind {
        Connect::Fence => {
            boxes.push(bx([6, 0, 6], [10, 16, 10]));
            for (index, on) in sides.into_iter().enumerate() {
                if on {
                    for (y0, y1) in [(6, 9), (12, 15)] {
                        // Рука для севера (−Z), затем поворот.
                        let arm = bx([7, y0, 0], [9, y1, 6]);
                        boxes.push(rotate(arm, Facing::ALL[index ^ 1]));
                    }
                }
            }
        }
        Connect::Wall => {
            boxes.push(bx([4, 0, 4], [12, 16, 12]));
            for (index, on) in sides.into_iter().enumerate() {
                if on {
                    let arm = bx([5, 0, 0], [11, 14, 4]);
                    boxes.push(rotate(arm, Facing::ALL[index ^ 1]));
                }
            }
        }
        Connect::Pane => {
            boxes.push(bx([7, 0, 7], [9, 16, 9]));
            for (index, on) in sides.into_iter().enumerate() {
                if on {
                    let arm = bx([7, 0, 0], [9, 16, 7]);
                    boxes.push(rotate(arm, Facing::ALL[index ^ 1]));
                }
            }
        }
    }
    boxes
}

/// Коробки блока с фиксированной формой.
#[must_use]
pub fn model_boxes(id: BlockId) -> Vec<ModelBox> {
    let Some(shape) = shape_of(id) else {
        return Vec::new();
    };
    let state = family_of(id).map_or(0, |(_, index, _)| index);
    match shape {
        Shape::Stairs => rotated(
            &[bx([0, 0, 0], [16, 8, 16]), bx([0, 8, 0], [16, 16, 8])],
            h4(state),
        ),
        Shape::Sign => rotated(
            &[bx([0, 8, 7], [16, 16, 9]), bx([7, 0, 7], [9, 8, 9])],
            h4(state),
        ),
        Shape::WallSign => rotated(&[bx([0, 4, 0], [16, 12, 2])], h4(state)),
        Shape::Hanging => rotated(
            &[
                bx([1, 0, 7], [15, 10, 9]),
                ax([3, 10, 7], [5, 16, 9]),
                ax([11, 10, 7], [13, 16, 9]),
            ],
            if state == 0 {
                Facing::South
            } else {
                Facing::West
            },
        ),
        Shape::WallHanging => rotated(
            &[
                bx([0, 14, 6], [16, 16, 10]),
                bx([1, 2, 7], [15, 10, 9]),
                ax([3, 10, 7], [5, 14, 9]),
                ax([11, 10, 7], [13, 14, 9]),
            ],
            if state == 0 {
                Facing::South
            } else {
                Facing::West
            },
        ),
        Shape::Door => {
            let facing = h4(state);
            let facing = if (state / 4) % 2 == 1 {
                turned(facing)
            } else {
                facing
            };
            rotated(&[bx([0, 0, 0], [16, 16, 3])], facing)
        }
        Shape::Trapdoor => {
            if state == 0 {
                vec![bx([0, 0, 0], [16, 3, 16])]
            } else {
                rotated(&[bx([0, 0, 0], [16, 16, 3])], h4(state - 1))
            }
        }
        Shape::Ladder => rotated(&[bx([0, 0, 0], [16, 16, 2])], h4(state)),
        Shape::Lever => {
            let on = state >= 4;
            let handle = if on {
                bx([7, 8, 2], [9, 14, 4])
            } else {
                bx([7, 2, 2], [9, 8, 4])
            };
            rotated(&[ax([5, 4, 0], [11, 12, 2]), handle], h4(state))
        }
        Shape::Button => rotated(&[bx([5, 6, 0], [11, 10, 2])], h4(state)),
        Shape::Plate => vec![bx([1, 0, 1], [15, 1, 15])],
        Shape::Farmland => vec![bx([0, 0, 0], [16, 15, 16])],
        Shape::SnowLayer => vec![bx([0, 0, 0], [16, 2, 16])],
        Shape::Cactus => vec![bx([1, 0, 1], [15, 16, 15])],
        Shape::Cake => vec![bx([1, 0, 1], [15, 8, 15])],
        Shape::Repeater => rotated(
            &[
                bx([0, 0, 0], [16, 2, 16]),
                ax([7, 2, 2], [9, 7, 4]),
                ax([7, 2, 9], [9, 5, 11]),
            ],
            h4(state),
        ),
        Shape::Chain => vec![bx([7, 0, 7], [9, 16, 9])],
        Shape::Portal => rotated(
            &[bx([0, 0, 7], [16, 16, 9])],
            if state == 0 {
                Facing::South
            } else {
                Facing::West
            },
        ),
        Shape::Gate => {
            let facing = h4(state);
            let mut boxes = vec![bx([0, 5, 6], [2, 16, 10]), bx([14, 5, 6], [16, 16, 10])];
            if state < 4 {
                boxes.push(bx([2, 6, 7], [14, 9, 9]));
                boxes.push(bx([2, 12, 7], [14, 15, 9]));
            } else {
                boxes.push(bx([0, 6, 10], [2, 15, 16]));
                boxes.push(bx([14, 6, 10], [16, 15, 16]));
            }
            rotated(&boxes, facing)
        }
        Shape::Pot | Shape::Potted => vec![bx([5, 0, 5], [11, 6, 11])],
        Shape::EnchantingTable => vec![bx([0, 0, 0], [16, 12, 16])],
        Shape::Brewing => vec![bx([1, 0, 1], [15, 2, 15]), bx([7, 2, 7], [9, 14, 9])],
        Shape::Cauldron => {
            let mut boxes = vec![
                bx([0, 0, 0], [16, 4, 16]),
                bx([0, 4, 0], [16, 16, 2]),
                bx([0, 4, 14], [16, 16, 16]),
                bx([0, 4, 2], [2, 16, 14]),
                bx([14, 4, 2], [16, 16, 14]),
            ];
            if id != crate::CAULDRON {
                boxes.push(ax([2, 4, 2], [14, 13, 14]));
            }
            boxes
        }
        Shape::EndPortal => vec![bx([0, 0, 0], [16, 12, 16])],
        Shape::EndFrame => vec![bx([0, 0, 0], [16, 13, 16])],
        Shape::Egg => vec![bx([1, 0, 1], [15, 16, 15])],
        Shape::EnderChest => vec![bx([1, 0, 1], [15, 14, 15])],
        Shape::Hook => rotated(&[bx([6, 2, 0], [10, 14, 3])], h4(state)),
        Shape::Head => vec![bx([4, 0, 4], [12, 8, 12])],
        Shape::WallHead => rotated(&[bx([4, 4, 0], [12, 12, 8])], h4(state)),
        Shape::Anvil => rotated(
            &[
                bx([2, 0, 2], [14, 4, 14]),
                bx([4, 4, 5], [12, 10, 11]),
                bx([0, 10, 3], [16, 16, 13]),
            ],
            if state == 0 {
                Facing::South
            } else {
                Facing::West
            },
        ),
        Shape::Hopper => vec![
            bx([0, 10, 0], [16, 16, 16]),
            bx([4, 4, 4], [12, 10, 12]),
            bx([6, 0, 6], [10, 4, 10]),
        ],
        Shape::Daylight => vec![bx([0, 0, 0], [16, 6, 16])],
        Shape::Carpet => vec![bx([0, 0, 0], [16, 1, 16])],
        Shape::Rod => vec![bx([6, 0, 6], [10, 16, 10])],
        Shape::ChorusPlant => vec![bx([4, 0, 4], [12, 16, 12])],
        Shape::ChorusFlower => vec![bx([2, 0, 2], [14, 14, 14])],
        Shape::DirtPath => vec![bx([0, 0, 0], [16, 15, 16])],
        Shape::Banner => rotated(
            &[ax([7, 0, 7], [9, 16, 9]), bx([2, 2, 9], [14, 15, 10])],
            h4(state),
        ),
        Shape::WallBanner => rotated(
            &[ax([1, 14, 0], [15, 16, 2]), bx([3, 0, 1], [13, 14, 3])],
            h4(state),
        ),
        Shape::Fence | Shape::Pane | Shape::Wall => Vec::new(),
    }
}

/// Дверь, лестница, люк, рычаг: блок после «использования» (ПКМ).
/// `facing` — куда смотрит игрок; нужен, чтобы открыть люк на нужную сторону.
#[must_use]
pub fn toggled(id: BlockId, facing: Facing) -> Option<BlockId> {
    let (first, index, kind) = family_of(id)?;
    match kind {
        FamilyKind::Door if shape_of(id) == Some(Shape::Door) => {
            Some(first + BlockId::from(index ^ 4))
        }
        FamilyKind::Lever | FamilyKind::Gate => Some(first + BlockId::from(index ^ 4)),
        FamilyKind::Trap => {
            if index == 0 {
                let side = Facing::ALL
                    .iter()
                    .position(|f| *f == facing)
                    .filter(|i| *i < 4)
                    .unwrap_or(0);
                Some(first + 1 + side as BlockId)
            } else {
                Some(first)
            }
        }
        _ => None,
    }
}

/// Верхняя половина двери: предмет не выпадает, ломается вместе с нижней.
#[must_use]
pub fn is_door_upper(id: BlockId) -> bool {
    shape_of(id) == Some(Shape::Door) && family_of(id).is_some_and(|(_, index, _)| index >= 8)
}

#[must_use]
pub fn is_door(id: BlockId) -> bool {
    shape_of(id) == Some(Shape::Door)
}

/// Вторая половина двери: нижняя ↔ верхняя (тот же поворот и открытость).
#[must_use]
pub fn door_other_half(id: BlockId) -> Option<BlockId> {
    let (first, index, _) = family_of(id).filter(|_| is_door(id))?;
    Some(first + BlockId::from(index ^ 8))
}

/// Лестница-ступеньки: полуразмерная «ступенька» поднимает персонажа сама.
#[must_use]
pub fn is_climbable(id: BlockId) -> bool {
    shape_of(id) == Some(Shape::Ladder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn every_part2_family_resolves_to_a_shape_or_a_known_cube_kind() {
        for id in PART2_FIRST..=PART2_LAST {
            let boxes = model_boxes(id);
            match shape_of(id) {
                Some(Shape::Fence | Shape::Pane) => assert!(boxes.is_empty()),
                Some(_) => {
                    assert!(!boxes.is_empty(), "{id}");
                    for b in boxes {
                        assert!(
                            (0..3).all(|a| b.min[a] < b.max[a] && b.max[a] <= 16),
                            "{id}"
                        );
                    }
                }
                None => assert!(model_boxes(id).is_empty()),
            }
        }
    }

    #[test]
    fn stairs_ascend_away_from_the_front_and_rotate_with_it() {
        let south = model_boxes(OAK_STAIRS + 1);
        assert_eq!(south[1].min, [0, 8, 0]);
        assert_eq!(south[1].max, [16, 16, 8]);
        let north = model_boxes(OAK_STAIRS);
        assert_eq!(north[1].min, [0, 8, 8]);
        let west = model_boxes(OAK_STAIRS + 2);
        assert_eq!(west[1].min, [8, 8, 0]);
        let east = model_boxes(OAK_STAIRS + 3);
        assert_eq!(east[1].max, [8, 16, 16]);
    }

    #[test]
    fn doors_toggle_open_and_closed_for_both_halves_and_keep_their_facing() {
        let lower_closed = OAK_DOOR + 2;
        let opened = toggled(lower_closed, Facing::North).unwrap();
        assert_ne!(opened, lower_closed);
        assert_eq!(toggled(opened, Facing::North), Some(lower_closed));
        let upper = door_other_half(lower_closed).unwrap();
        assert!(is_door_upper(upper) && !is_door_upper(lower_closed));
        assert_eq!(door_other_half(upper), Some(lower_closed));
        assert_eq!(
            toggled(upper, Facing::North).unwrap(),
            door_other_half(opened).unwrap()
        );
        // Закрытая дверь тонкая у одной стены, открытая — у соседней.
        assert_ne!(model_boxes(lower_closed), model_boxes(opened));
        assert!(BLOCKS[usize::from(lower_closed)].collision);
        assert!(!BLOCKS[usize::from(opened)].collision);
    }

    #[test]
    fn trapdoors_open_toward_the_player_and_levers_flip() {
        let closed = OAK_TRAPDOOR;
        let opened = toggled(closed, Facing::West).unwrap();
        assert_eq!(opened, OAK_TRAPDOOR + 3);
        assert_eq!(toggled(opened, Facing::East), Some(closed));
        assert!(BLOCKS[usize::from(closed)].collision && !BLOCKS[usize::from(opened)].collision);
        assert_eq!(toggled(LEVER, Facing::North), Some(LEVER + 4));
        assert_eq!(toggled(LEVER + 5, Facing::North), Some(LEVER + 1));
        assert_eq!(toggled(STONE, Facing::North), None);
    }

    #[test]
    fn fences_connect_to_fences_and_solid_blocks_but_not_to_air_or_plants() {
        assert!(connects_to(Connect::Fence, OAK_FENCE));
        assert!(connects_to(Connect::Fence, STONE));
        assert!(!connects_to(Connect::Fence, AIR));
        assert!(!connects_to(Connect::Fence, POPPY));
        assert!(!connects_to(Connect::Fence, IRON_BARS));
        assert!(connects_to(Connect::Pane, IRON_BARS) && connects_to(Connect::Pane, GLASS_PANE));
        let none = connected_boxes(Connect::Fence, [false; 4]);
        assert_eq!(none.len(), 1);
        let two = connected_boxes(Connect::Fence, [true, false, false, true]);
        assert_eq!(two.len(), 5);
        let north = &two[1];
        assert_eq!(north.min[2], 0, "рука на север уходит к z = 0");
        let east = &two[3];
        assert_eq!(east.max[0], 16, "рука на восток уходит к x = 16");
    }

    #[test]
    fn wall_attached_models_hug_the_wall_on_the_correct_side() {
        // Лицо на юг: стена в −Z, значит доска у z = 0.
        assert_eq!(model_boxes(OAK_WALL_SIGN + 1)[0].min[2], 0);
        // Лицо на север: стена в +Z.
        assert_eq!(model_boxes(OAK_WALL_SIGN)[0].max[2], 16);
        assert_eq!(
            model_boxes(LADDER + 2)[0].min[0],
            14,
            "лицо на запад: стена в +X"
        );
        assert_eq!(model_boxes(LADDER + 2)[0].max[0], 16);
        assert!(is_climbable(LADDER + 3) && !is_climbable(OAK_SIGN));
    }
}
