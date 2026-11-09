//! Pure input transitions, shared with the Bevy input system.
#[derive(Default)]
pub struct ControlInput {
    pub wheel: f32,
    pub frames: u32,
    pub frame_time: f32,
    last_jump: Option<f64>,
    f3_used: bool,
    last_forward: Option<f64>,
    forward_running: bool,
}
impl ControlInput {
    pub fn forward_tap(&mut self, now: f64) {
        if self.last_forward.is_some_and(|last| now - last <= 0.3) {
            self.forward_running = true;
            self.last_forward = None;
        } else {
            self.last_forward = Some(now);
        }
    }
    pub fn forward_sprint(&mut self, held: bool) -> bool {
        if !held {
            self.forward_running = false;
        }
        self.forward_running
    }
    pub fn clear_movement(&mut self) {
        self.forward_running = false;
        self.last_forward = None;
        self.last_jump = None;
    }

    pub fn jump_tap(&mut self, now: f64) -> bool {
        if self.last_jump.is_some_and(|last| now - last <= 0.3) {
            self.last_jump = None;
            true
        } else {
            self.last_jump = Some(now);
            false
        }
    }
    pub fn scroll(&mut self, selected: usize, delta: f32) -> usize {
        self.wheel += delta;
        let steps = self.wheel.trunc() as i32;
        self.wheel -= steps as f32;
        (selected as i32 - steps).rem_euclid(9) as usize
    }
    pub fn debug_keys(
        &mut self,
        pressed: bool,
        down: bool,
        released: bool,
        g: bool,
    ) -> (bool, bool) {
        if pressed {
            self.f3_used = false;
        }
        let grid = (down || pressed) && g;
        if grid {
            self.f3_used = true;
        }
        let debug = released && !self.f3_used;
        if released {
            self.f3_used = false;
        }
        (debug, grid)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn double_forward_runs_until_forward_is_released() {
        let mut s = ControlInput::default();
        s.forward_tap(1.);
        assert!(!s.forward_sprint(true));
        assert!(!s.forward_sprint(false));
        s.forward_tap(1.2);
        assert!(s.forward_sprint(true));
        assert!(s.forward_sprint(true));
        assert!(!s.forward_sprint(false));
        s.forward_tap(4.);
        assert!(!s.forward_sprint(true));
    }
    #[test]
    fn wheel_wraps_and_accumulates_touchpad_steps() {
        let mut s = ControlInput::default();
        assert_eq!(s.scroll(0, 1.), 8);
        assert_eq!(s.scroll(8, -1.), 0);
        assert_eq!(s.scroll(0, 0.4), 0);
        assert_eq!(s.scroll(0, 0.6), 8);
        assert_eq!(s.scroll(0, -19.), 1);
    }
    #[test]
    fn flight_requires_two_close_presses() {
        let mut s = ControlInput::default();
        assert!(!s.jump_tap(1.));
        assert!(!s.jump_tap(2.));
        assert!(s.jump_tap(2.2));
        assert!(!s.jump_tap(2.3));
    }
    #[test]
    fn combined_keys_in_one_frame_still_toggle_only_grid() {
        let mut s = ControlInput::default();
        assert_eq!(s.debug_keys(true, false, true, true), (false, true));
    }
    #[test]
    fn f3_g_does_not_toggle_debug_on_release() {
        let mut s = ControlInput::default();
        assert_eq!(s.debug_keys(true, true, false, false), (false, false));
        assert_eq!(s.debug_keys(false, true, false, true), (false, true));
        assert_eq!(s.debug_keys(false, false, true, false), (false, false));
        assert_eq!(s.debug_keys(true, true, false, false), (false, false));
        assert_eq!(s.debug_keys(false, false, true, false), (true, false));
    }
}

// После интерфейса мир получает мышь только после отпускания обеих кнопок.
#[derive(Default)]
pub struct WorldMouseGate {
    armed: bool,
}
impl WorldMouseGate {
    pub fn disarm(&mut self) {
        self.armed = false;
    }
    pub fn update(&mut self, captured: bool, left: bool, right: bool) -> bool {
        if !captured {
            self.disarm();
        } else if !left && !right {
            self.armed = true;
        }
        self.armed
    }
    pub fn allowed(&self) -> bool {
        self.armed
    }
}
#[cfg(test)]
mod mouse_gate_tests {
    use super::*;
    #[test]
    fn held_menu_click_stays_blocked_until_both_buttons_are_released() {
        let mut gate = WorldMouseGate::default();
        assert!(!gate.update(true, true, false));
        assert!(gate.update(true, false, false));
        assert!(gate.update(true, true, false));
        assert!(!gate.update(false, true, false));
        assert!(!gate.update(true, true, false));
        assert!(!gate.update(true, false, true));
        assert!(gate.update(true, false, false));
        assert!(gate.update(true, true, false));
        gate.disarm();
        assert!(!gate.update(true, true, false));
    }
}
