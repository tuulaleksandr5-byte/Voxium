//! A cadence gate driven by actual horizontal travel, not held movement keys.
#[derive(Default)]
pub struct StepCadence {
    elapsed: f32,
    distance: f32,
}
impl StepCadence {
    pub fn reset(&mut self) {
        self.elapsed = 0.;
        self.distance = 0.;
    }
    pub fn advance(
        &mut self,
        dt: f32,
        distance: f32,
        grounded: bool,
        flying: bool,
        running: bool,
    ) -> bool {
        if !grounded
            || flying
            || !dt.is_finite()
            || !distance.is_finite()
            || dt <= 0.
            || distance <= 0.001
        {
            self.reset();
            return false;
        }
        self.elapsed += dt;
        self.distance += distance;
        let interval = if running { 0.25 } else { 0.4 };
        if self.elapsed + f32::EPSILON >= interval && self.distance >= 0.3 {
            self.reset();
            true
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn walking_is_limited_to_one_step_per_four_tenths() {
        let mut cadence = StepCadence::default();
        for _ in 0..3 {
            assert!(!cadence.advance(0.1, 0.4, true, false, false));
        }
        assert!(cadence.advance(0.1, 0.4, true, false, false));
        assert!(!cadence.advance(0.1, 0.4, true, false, false));
    }
    #[test]
    fn running_has_a_shorter_interval_and_does_not_burst_after_a_long_frame() {
        let mut cadence = StepCadence::default();
        assert!(!cadence.advance(0.125, 0.8, true, false, true));
        assert!(cadence.advance(0.125, 0.8, true, false, true));
        assert!(cadence.advance(2., 1., true, false, true));
        assert!(!cadence.advance(0.01, 0.08, true, false, true));
    }
    #[test]
    fn stationary_airborne_and_flight_never_make_steps() {
        let mut cadence = StepCadence::default();
        for (distance, grounded, flying) in [
            (0., true, false),
            (0.001, true, false),
            (1., false, false),
            (1., true, true),
        ] {
            assert!(!cadence.advance(1., distance, grounded, flying, false));
        }
        assert!(!cadence.advance(0.1, 0.4, true, false, false));
        cadence.reset();
        assert!(!cadence.advance(0.3, 0.4, true, false, false));
    }
}
