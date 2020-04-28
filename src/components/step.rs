use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Step {
    pub escalator_id: i32,
    pub width: f32,
    pub height: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub push_velocity: f32,
}

impl Component for Step {
    type Storage = DenseVecStorage<Self>;
}

impl Step {
    pub fn new(
        escalator_id: i32,
        width: f32,
        height: f32,
        x_velocity: f32,
        y_velocity: f32,
    ) -> Step {
        Step {
            escalator_id,
            width,
            height,
            x_velocity,
            y_velocity,
            push_velocity: 0.,
        }
    }
}
