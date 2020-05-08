use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
#[derive(Clone)]
pub struct Step {
    pub escalator: Entity,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub push_velocity: f32,
}

impl Component for Step {
    type Storage = DenseVecStorage<Self>;
}

impl Step {
    pub fn new(escalator: Entity, x_velocity: f32, y_velocity: f32, push_velocity: f32) -> Step {
        Step {
            escalator,
            x_velocity,
            y_velocity,
            push_velocity,
        }
    }
}
