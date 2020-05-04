use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct RewindableClock {}

impl Component for RewindableClock {
    type Storage = DenseVecStorage<Self>;
}

impl RewindableClock {
    pub fn new() -> RewindableClock {
        RewindableClock {}
    }
}
