use macroquad::prelude::*;

pub struct Timer {
    pub start_time: f64,
    pub duration: f64,
    pub already_enabled: bool,
}

impl Timer {
    pub fn new(duration: f64, already_enabled: bool) -> Self {
        Timer {
            start_time: get_time(),
            duration,
            already_enabled,
        }
    }

    pub fn reset(&mut self) {
        self.start_time = get_time();
    }

    pub fn is_finished(&self) -> bool {
        get_time() - self.start_time >= self.duration
    }

    pub fn elapsed(&self) -> f64 {
        get_time() - self.start_time
    }

    pub fn remaining(&self) -> f64 {
        (self.duration - (get_time() - self.start_time)).max(0.0)
    }
}
