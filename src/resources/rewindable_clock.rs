use core::default::Default;

pub struct RewindableClock {
    pub velocity: i32,
}

impl RewindableClock {
    pub fn update_clock(&mut self, velocity: i32) {
        self.velocity = velocity;
    }

    pub fn going_forwards(&self) -> bool {
        self.velocity > 0
    }
}

impl Default for RewindableClock {
    fn default() -> Self {
        RewindableClock {
            velocity: 0,
        }
    }
}
