use amethyst::ecs::prelude::{Component, DenseVecStorage};
pub struct Thing {
    pub width: f32,
    pub height: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
}

impl Component for Thing {
    type Storage = DenseVecStorage<Self>;
}

impl Thing {
    pub fn new(    width: f32,
        height: f32,
        x_velocity: f32,
        y_velocity: f32,) -> Thing {
        Thing {
            width, height, x_velocity, y_velocity
        }
    }
}
