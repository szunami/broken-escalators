use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Copy, Clone)]
pub struct Thing {
    pub x_velocity: f32,
    pub y_velocity: f32,
}

impl Component for Thing {
    type Storage = DenseVecStorage<Self>;
}

impl Thing {
    pub fn new(x_velocity: f32, y_velocity: f32) -> Thing {
        Thing {
            x_velocity,
            y_velocity,
        }
    }
}
