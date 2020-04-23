use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Thing {}

impl Component for Thing {
    type Storage = DenseVecStorage<Self>;
}

impl Thing {
    pub fn new() -> Thing {
        Thing {}
    }
}
