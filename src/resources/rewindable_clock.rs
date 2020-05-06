use core::default::Default;

pub struct RewindableClock {
    pub current_time: f32,
    pub velocity: f32,
}

impl RewindableClock {
    pub fn update_clock(&mut self, velocity: f32, delta_seconds: f32) {
        self.velocity = velocity;
        self.current_time = f32::max(0., self.current_time + delta_seconds);
    }

    pub fn going_forwards(&self) -> bool {
        self.velocity > 0.
    }
}

impl Default for RewindableClock {
    fn default() -> Self {
        RewindableClock {
            current_time: 0.,
            velocity: 1.,
        }
    }
}
