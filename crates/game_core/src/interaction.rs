//! Renderer-independent voxel selection and player collision.

const PLAYER_HALF_WIDTH: f64 = 0.3;
// Ignore rounding noise where f32 player positions touch a voxel face.
const CONTACT_EPSILON: f64 = 0.000001;
const GROUND_PROBE: f64 = 0.0001;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayHit {
    pub block: [i32; 3],
    pub previous: [i32; 3],
    pub distance: f32,
}

/// Selects the first non-air block with an Amanatides-Woo 3D DDA.
///
/// Distance is measured in world units regardless of direction length. An
/// occupied starting cell has distance zero and is also its own `previous`.
/// Exact edge ties step X, then Y, then Z, keeping `previous` face-adjacent.
/// Invalid numbers, a zero direction, and coordinates outside i32 return None.
/// Static creative water blocks are selectable and removable, but not solid.
#[must_use]
pub fn raycast(
    origin: [f32; 3],
    direction: [f32; 3],
    max_distance: f32,
    sample: impl Fn([i32; 3]) -> u8,
) -> Option<RayHit> {
    if !origin.into_iter().all(f32::is_finite)
        || !direction.into_iter().all(f32::is_finite)
        || !max_distance.is_finite()
        || max_distance < 0.0
    {
        return None;
    }

    // f64 also avoids overflow/underflow when normalizing extreme f32 input.
    let origin = origin.map(f64::from);
    let mut direction = direction.map(f64::from);
    let length = direction.into_iter().map(|v| v * v).sum::<f64>().sqrt();
    if length == 0.0 {
        return None;
    }
    direction.iter_mut().for_each(|v| *v /= length);
    let mut block = [
        floor_coordinate(origin[0])?,
        floor_coordinate(origin[1])?,
        floor_coordinate(origin[2])?,
    ];
    let mut step = [0; 3];
    let mut next = [f64::INFINITY; 3];
    let mut interval = [f64::INFINITY; 3];
    for axis in 0..3 {
        if direction[axis] > 0.0 {
            step[axis] = 1;
            next[axis] = (f64::from(block[axis]) + 1.0 - origin[axis]) / direction[axis];
            interval[axis] = 1.0 / direction[axis];
        } else if direction[axis] < 0.0 {
            step[axis] = -1;
            next[axis] = (f64::from(block[axis]) - origin[axis]) / direction[axis];
            interval[axis] = -1.0 / direction[axis];
        }
    }

    let mut previous = block;
    let mut distance = 0.0;
    loop {
        if sample(block) != crate::AIR {
            return Some(RayHit {
                block,
                previous,
                distance: distance as f32,
            });
        }
        let axis = if next[0] <= next[1] && next[0] <= next[2] {
            0
        } else if next[1] <= next[2] {
            1
        } else {
            2
        };
        distance = next[axis];
        if distance > f64::from(max_distance) {
            return None;
        }
        previous = block;
        block[axis] = block[axis].checked_add(step[axis])?;
        next[axis] += interval[axis];
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Movement {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub grounded: bool,
}

/// Whether a unit voxel overlaps the player's body; touching faces are allowed.
#[must_use]
pub fn player_intersects_block(feet: [f32; 3], height: f32, block: [i32; 3]) -> bool {
    if !valid_body(feet, height) {
        return false;
    }
    let (min, max) = body_bounds(feet.map(f64::from), f64::from(height));
    (0..3).all(|axis| {
        max[axis] > f64::from(block[axis]) + CONTACT_EPSILON
            && min[axis] < f64::from(block[axis]) + 1.0 - CONTACT_EPSILON
    })
}

/// Whether the full 0.6-wide, 1.8-high standing body fits at the player's feet.
/// Uses registry collision for every overlapping voxel, including the body's
/// outer edges and top. Invalid or out-of-grid coordinates cannot stand.
#[must_use]
pub fn can_stand(feet: [f32; 3], sample: impl Fn([i32; 3]) -> u8) -> bool {
    body_cells(feet.map(f64::from), f64::from(1.8_f32))
        .is_some_and(|cells| !cells_have_solid(cells, &sample))
}

/// Moves a 0.6-wide AABB using swept voxel planes on each axis.
///
/// Horizontal axes resolve before Y so the player can slide along walls. Every
/// crossed plane is tested, including during long falls or unusually large dt;
/// velocity is cleared on blocked axes. Gravity and jumping belong to callers.
/// Invalid motion leaves the position unchanged and clears velocity.
#[must_use]
pub fn move_player(
    feet: [f32; 3],
    velocity: [f32; 3],
    dt: f32,
    height: f32,
    sample: impl Fn([i32; 3]) -> u8,
) -> Movement {
    if !valid_body(feet, height)
        || !velocity.into_iter().all(f32::is_finite)
        || !dt.is_finite()
        || dt < 0.0
    {
        return Movement {
            position: feet,
            velocity: [0.0; 3],
            grounded: false,
        };
    }

    let mut position = feet.map(f64::from);
    let mut velocity = velocity;
    let height = f64::from(height);
    for axis in [0, 2, 1] {
        let motion = f64::from(velocity[axis]) * f64::from(dt);
        // Sweep the position that the caller will actually receive. Rounding
        // an unchecked f64 endpoint to f32 can otherwise move it into a voxel.
        let target = (position[axis] + motion) as f32;
        let delta = f64::from(target) - position[axis];
        if delta != 0.0 {
            let (allowed, blocked) = sweep_axis(position, height, axis, delta, &sample);
            position[axis] += allowed;
            if blocked {
                // The exact contact may lie between f32 values. Choose the
                // representable position outside the obstacle, and use it for
                // subsequent axes as well as the returned player position.
                position[axis] = round_away_from_contact(position[axis], delta > 0.0);
                velocity[axis] = 0.0;
            }
        }
    }

    let mut probe = position;
    probe[1] -= GROUND_PROBE;
    let grounded = velocity[1] <= 0.0 && body_collides(probe, height, &sample);
    Movement {
        position: position.map(|v| v as f32),
        velocity,
        grounded,
    }
}

fn valid_body(feet: [f32; 3], height: f32) -> bool {
    feet.into_iter().all(f32::is_finite) && height.is_finite() && height > 0.0
}

fn round_away_from_contact(value: f64, moving_positive: bool) -> f64 {
    let rounded = value as f32;
    f64::from(if moving_positive && f64::from(rounded) > value {
        rounded.next_down()
    } else if !moving_positive && f64::from(rounded) < value {
        rounded.next_up()
    } else {
        rounded
    })
}

fn floor_coordinate(value: f64) -> Option<i32> {
    let value = value.floor();
    (value >= f64::from(i32::MIN) && value <= f64::from(i32::MAX)).then_some(value as i32)
}

fn cell_before(value: f64) -> Option<i32> {
    floor_coordinate(value.ceil() - 1.0)
}

fn body_bounds(position: [f64; 3], height: f64) -> ([f64; 3], [f64; 3]) {
    (
        [
            position[0] - PLAYER_HALF_WIDTH,
            position[1],
            position[2] - PLAYER_HALF_WIDTH,
        ],
        [
            position[0] + PLAYER_HALF_WIDTH,
            position[1] + height,
            position[2] + PLAYER_HALF_WIDTH,
        ],
    )
}

fn body_cells(position: [f64; 3], height: f64) -> Option<[[i32; 2]; 3]> {
    let (min, max) = body_bounds(position, height);
    let mut cells = [[0; 2]; 3];
    for axis in 0..3 {
        cells[axis] = [
            floor_coordinate(min[axis] + CONTACT_EPSILON)?,
            floor_coordinate(max[axis] - CONTACT_EPSILON)?,
        ];
    }
    Some(cells)
}

fn cells_have_solid(cells: [[i32; 2]; 3], sample: &impl Fn([i32; 3]) -> u8) -> bool {
    for y in cells[1][0]..=cells[1][1] {
        for z in cells[2][0]..=cells[2][1] {
            for x in cells[0][0]..=cells[0][1] {
                if crate::is_solid(sample([x, y, z])) {
                    return true;
                }
            }
        }
    }
    false
}

fn body_collides(position: [f64; 3], height: f64, sample: &impl Fn([i32; 3]) -> u8) -> bool {
    body_cells(position, height).is_some_and(|cells| cells_have_solid(cells, sample))
}

fn sweep_axis(
    position: [f64; 3],
    height: f64,
    axis: usize,
    delta: f64,
    sample: &impl Fn([i32; 3]) -> u8,
) -> (f64, bool) {
    let (min, max) = body_bounds(position, height);
    let positive = delta > 0.0;
    let leading = if positive { max[axis] } else { min[axis] };
    let coordinate = if positive {
        floor_coordinate
    } else {
        cell_before
    };
    let Some(first) = coordinate(leading) else {
        return (0.0, true);
    };
    let Some(last) = coordinate(leading + delta) else {
        return (0.0, true);
    };
    let Some(mut cells) = body_cells(position, height) else {
        return (0.0, true);
    };

    let step = if positive { 1 } else { -1 };
    let mut row = first;
    loop {
        cells[axis] = [row; 2];
        if cells_have_solid(cells, sample) {
            let face = f64::from(row) + if positive { 0.0 } else { 1.0 };
            let contact = face - leading;
            let allowed = if positive {
                contact.clamp(0.0, delta)
            } else {
                contact.clamp(delta, 0.0)
            };
            return (allowed, true);
        }
        if row == last {
            return (delta, false);
        }
        let Some(next) = row.checked_add(step) else {
            return (0.0, true);
        };
        row = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.0001, "{actual} != {expected}");
    }

    #[test]
    fn ray_normalizes_direction_and_returns_the_placement_face() {
        let hit = raycast([0.5, 1.5, 0.5], [7.0, 0.0, 0.0], 3.0, |p| {
            u8::from(p == [3, 1, 0])
        })
        .unwrap();
        assert_eq!(hit.block, [3, 1, 0]);
        assert_eq!(hit.previous, [2, 1, 0]);
        close(hit.distance, 2.5);
    }

    #[test]
    fn ray_crosses_negative_coordinates_and_exact_negative_boundaries() {
        let hit = raycast([-0.1, 2.5, -0.5], [-4.0, 0.0, 0.0], 3.0, |p| {
            u8::from(p == [-3, 2, -1])
        })
        .unwrap();
        assert_eq!(hit.block, [-3, 2, -1]);
        assert_eq!(hit.previous, [-2, 2, -1]);
        close(hit.distance, 1.9);

        let boundary = raycast([0.0, 2.5, 0.5], [-1.0, 0.0, 0.0], 1.0, |p| {
            u8::from(p == [-1, 2, 0])
        })
        .unwrap();
        assert_eq!(boundary.block, [-1, 2, 0]);
        assert_eq!(boundary.previous, [0, 2, 0]);
        close(boundary.distance, 0.0);
    }

    #[test]
    fn ray_targets_static_creative_water_and_respects_the_distance_limit() {
        let blocks = |p: [i32; 3]| match p {
            [3, 0, 0] => 12,
            [5, 0, 0] => 1,
            _ => 0,
        };
        assert!(raycast([0.5; 3], [1.0, 0.0, 0.0], 2.49, blocks).is_none());
        let hit = raycast([0.5; 3], [1.0, 0.0, 0.0], 2.5, blocks).unwrap();
        assert_eq!(hit.block, [3, 0, 0]);
        assert_eq!(hit.previous, [2, 0, 0]);
        close(hit.distance, 2.5);
    }

    #[test]
    fn ray_handles_vertical_axes_and_an_occupied_origin() {
        let hit = raycast([0.5, 2.5, 0.5], [0.0, -2.0, 0.0], 2.0, |p| {
            u8::from(p == [0, 0, 0])
        })
        .unwrap();
        assert_eq!(hit.previous, [0, 1, 0]);
        close(hit.distance, 1.5);
        let start = raycast([0.5; 3], [1.0, 0.0, 0.0], 0.0, |_| 1).unwrap();
        assert_eq!(start.block, start.previous);
        close(start.distance, 0.0);
    }

    #[test]
    fn ray_rejects_invalid_input_without_sampling() {
        let never_sample = |_| panic!("invalid ray sampled the world");
        for (origin, direction, distance) in [
            ([0.5; 3], [0.0; 3], 5.0),
            ([f32::INFINITY, 0.0, 0.0], [1.0, 0.0, 0.0], 5.0),
            ([0.5; 3], [f32::NAN, 0.0, 0.0], 5.0),
            ([0.5; 3], [1.0, 0.0, 0.0], f32::NAN),
            ([0.5; 3], [1.0, 0.0, 0.0], -1.0),
            ([f32::MAX; 3], [1.0, 0.0, 0.0], 5.0),
        ] {
            assert!(raycast(origin, direction, distance, never_sample).is_none());
        }
    }

    #[test]
    fn placement_checks_the_full_body_but_allows_touching_faces() {
        assert!(player_intersects_block([0.5, 1.0, 0.5], 1.8, [0, 1, 0]));
        assert!(player_intersects_block([0.5, 1.0, 0.5], 1.8, [0, 2, 0]));
        assert!(!player_intersects_block([0.5, 1.0, 0.5], 1.8, [0, 0, 0]));
        assert!(!player_intersects_block([0.7, 1.0, 0.5], 1.8, [1, 1, 0]));
        assert!(!player_intersects_block([1.3, 1.0, 0.5], 1.8, [0, 1, 0]));
        assert!(!player_intersects_block([0.5, 1.0, 0.5], 1.0, [0, 2, 0]));
    }

    #[test]
    fn standing_checks_the_top_of_the_full_body() {
        let ceiling = |p: [i32; 3]| u8::from(p == [0, 3, 0]);
        assert!(!can_stand([0.5, 1.22, 0.5], ceiling));
        assert!(can_stand([0.5, 1.2, 0.5], ceiling));
    }

    #[test]
    fn standing_checks_the_outer_lateral_body_band() {
        for axis in [0, 2] {
            for (center, side) in [(0.705, 1), (0.295, -1)] {
                let mut feet = [0.5, 1.5, 0.5];
                feet[axis] = center;
                let mut block = [0, 3, 0];
                block[axis] = side;
                assert!(!can_stand(feet, |p| u8::from(p == block)));
            }
        }
    }

    #[test]
    fn standing_uses_registry_collision_and_rejects_invalid_coordinates() {
        assert!(can_stand([0.5, 1.0, 0.5], |_| 0));
        assert!(can_stand([0.5, 1.0, 0.5], |_| 12));
        assert!(!can_stand([0.5, 1.0, 0.5], |_| 8));
        let never_sample = |_| panic!("invalid standing body sampled the world");
        assert!(!can_stand([f32::NAN, 1.0, 0.5], never_sample));
        assert!(!can_stand([f32::MAX, 1.0, 0.5], never_sample));
    }

    #[test]
    fn swept_movement_cannot_tunnel_through_a_wall() {
        let movement = move_player([0.5, 1.0, 0.5], [100.0, 0.0, 0.0], 1.0, 1.8, |p| {
            u8::from(p[0] == 5 && (1..=2).contains(&p[1]))
        });
        close(movement.position[0], 4.7);
        assert_eq!(movement.velocity[0], 0.0);
        assert!(!movement.grounded);
    }

    #[test]
    fn swept_movement_handles_negative_walls_and_slides_along_them() {
        let movement = move_player([0.5, 1.0, 0.5], [-100.0, 0.0, 2.0], 0.5, 1.8, |p| {
            u8::from(p[0] == -2 && (1..=2).contains(&p[1]))
        });
        close(movement.position[0], -0.7);
        close(movement.position[2], 1.5);
        assert_eq!(movement.velocity, [0.0, 0.0, 2.0]);
    }

    #[test]
    fn wall_contacts_remain_nonpenetrating_after_f32_rounding() {
        for start in [256.0_f32, -256.0, 4096.0, -4096.0, 65_536.0, -65_536.0] {
            for sign in [-1.0_f32, 1.0] {
                for axis in [0, 2] {
                    let wall_cell = start as i32 + if sign > 0.0 { 1 } else { -2 };
                    let wall = |p: [i32; 3]| u8::from(p[axis] == wall_cell);
                    let mut feet = [0.5, 70.0, 0.5];
                    feet[axis] = start;
                    let mut velocity = [0.0; 3];
                    velocity[axis] = sign * 4.0;
                    let hit = move_player(feet, velocity, 0.3, 1.8, wall);
                    let edge = f64::from(hit.position[axis]) + f64::from(sign) * PLAYER_HALF_WIDTH;
                    let face = f64::from(wall_cell) + if sign > 0.0 { 0.0 } else { 1.0 };
                    assert!(
                        if sign > 0.0 {
                            edge <= face
                        } else {
                            edge >= face
                        },
                        "body crossed wall: {hit:?}, axis {axis}, face {face}"
                    );

                    let slide_axis = 2 - axis;
                    let mut slide = [0.0, -4.0, 0.0];
                    slide[slide_axis] = 4.0;
                    let next = move_player(hit.position, slide, 0.3, 1.8, wall);
                    close(next.position[1], 68.8);
                    close(next.position[slide_axis], 1.7);
                    assert_eq!(next.velocity, slide);
                    assert!(!next.grounded, "wall incorrectly supports player: {next:?}");
                }
            }
        }
    }

    #[test]
    fn rounding_an_unblocked_target_cannot_push_the_body_into_a_wall() {
        let wall = |p: [i32; 3]| u8::from(p[0] == 257);
        // In f64 this movement ends just before contact; nearest-f32 rounding
        // crosses the wall unless the representable target is also swept.
        let hit = move_player([256.0, 70.0, 0.5], [4.0, 0.0, 0.0], 0.175, 1.8, wall);
        assert!(f64::from(hit.position[0]) + PLAYER_HALF_WIDTH <= 257.0);
        let next = move_player(hit.position, [0.0, -4.0, 4.0], 0.3, 1.8, wall);
        assert_eq!(next.velocity, [0.0, -4.0, 4.0]);
        assert!(!next.grounded);
    }

    #[test]
    fn ceiling_contact_stays_outside_the_block_after_f32_rounding() {
        let ceiling = |p: [i32; 3]| u8::from(p[1] == 259);
        let hit = move_player([0.5, 256.0, 0.5], [0.0, 4.0, 0.0], 0.5, 1.8, ceiling);
        assert!(f64::from(hit.position[1]) + f64::from(1.8_f32) <= 259.0);
        let next = move_player(hit.position, [4.0, -4.0, 0.0], 0.3, 1.8, ceiling);
        assert_eq!(next.velocity, [4.0, -4.0, 0.0]);
        assert!(!next.grounded);
    }

    #[test]
    fn falling_lands_on_the_floor_without_tunneling() {
        let movement = move_player([0.5, 20.0, 0.5], [0.0, -100.0, 0.0], 0.5, 1.8, |p| {
            u8::from(p[1] == 0)
        });
        close(movement.position[1], 1.0);
        assert_eq!(movement.velocity[1], 0.0);
        assert!(movement.grounded);
        let resting = move_player(movement.position, [0.0; 3], 0.1, 1.8, |p| {
            u8::from(p[1] == 0)
        });
        assert!(resting.grounded);
    }

    #[test]
    fn jumping_stops_at_the_ceiling_without_grounding() {
        let movement = move_player([0.5, 1.0, 0.5], [0.0, 100.0, 0.0], 0.5, 1.8, |p| {
            u8::from(p[1] == 3)
        });
        close(movement.position[1], 1.2);
        assert_eq!(movement.velocity[1], 0.0);
        assert!(!movement.grounded);
    }

    #[test]
    fn water_is_passable_and_leaves_use_registry_collision() {
        let water = move_player([0.5, 1.0, 0.5], [10.0, 0.0, 0.0], 0.5, 1.8, |_| 12);
        close(water.position[0], 5.5);
        let leaves = move_player([0.5, 1.0, 0.5], [10.0, 0.0, 0.0], 0.5, 1.8, |p| {
            if p[0] == 2 { 8 } else { 0 }
        });
        close(leaves.position[0], 1.7);
    }
}
