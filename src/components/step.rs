use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};


pub struct Step {
    pub width: f32,
    pub height: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
}

impl Component for Step {
    type Storage = DenseVecStorage<Self>;
}

impl Step {
    pub fn new(width: f32, height: f32, x_velocity: f32, y_velocity: f32) -> Step {
        Step { width, height, x_velocity, y_velocity }
    }
}
