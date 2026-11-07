//! Shared world-coordinate rules. Keep this crate independent of Bevy.

/// Inclusive lower bound of the overworld.
pub const WORLD_MIN_Y: i32 = -64;
/// Inclusive upper bound of the overworld.
pub const WORLD_MAX_Y: i32 = 320;
/// Horizontal chunk width and vertical section height.
pub const SECTION_SIZE: i32 = 16;

/// Converts a block coordinate to the containing section coordinate.
/// Uses Euclidean division so negative coordinates behave correctly.
#[must_use]
pub fn section_coordinate(block: i32) -> i32 {
    block.div_euclid(SECTION_SIZE)
}

/// Returns the local coordinate in a section, in `0..16`.
#[must_use]
pub fn local_coordinate(block: i32) -> u8 {
    block.rem_euclid(SECTION_SIZE) as u8
}

#[must_use]
pub fn valid_world_height(y: i32) -> bool {
    (WORLD_MIN_Y..=WORLD_MAX_Y).contains(&y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_coordinates_map_to_expected_sections() {
        assert_eq!(section_coordinate(-1), -1);
        assert_eq!(local_coordinate(-1), 15);
        assert_eq!(section_coordinate(-16), -1);
        assert_eq!(local_coordinate(-16), 0);
        assert_eq!(section_coordinate(-17), -2);
        assert_eq!(local_coordinate(-17), 15);
    }

    #[test]
    fn coordinates_round_trip() {
        for block in -1000..1000 {
            let rebuilt = section_coordinate(block) * SECTION_SIZE
                + i32::from(local_coordinate(block));
            assert_eq!(rebuilt, block);
        }
    }

    #[test]
    fn height_bounds_are_inclusive() {
        assert!(!valid_world_height(-65));
        assert!(valid_world_height(-64));
        assert!(valid_world_height(320));
        assert!(!valid_world_height(321));
    }
}
