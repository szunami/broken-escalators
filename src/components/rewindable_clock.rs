use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct RewindableClock {
    pub current_time: f32
}

impl Component for RewindableClock {
    type Storage = DenseVecStorage<Self>;
}

impl RewindableClock {
    pub fn new() -> RewindableClock {
        RewindableClock {
            current_time: 0.,
        }
    }

    pub fn update_clock(&mut self, delta_seconds: f32) {
        self.current_time = f32::max(0., self.current_time + delta_seconds);
    }
}
