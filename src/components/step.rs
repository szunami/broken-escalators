use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::utils::UpdateFrom;

#[derive(Clone)]
pub struct Step {
    pub escalator_id: i32,
    pub width: f32,
    pub height: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub push_velocity: f32,
}

impl UpdateFrom<Step> for Step {
    fn update_from(&mut self, other: Step) {
        self.x_velocity = other.x_velocity;
        self.y_velocity = other.y_velocity;
        self.push_velocity = other.y_velocity;
    }
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
        push_velocity: f32,
    ) -> Step {
        Step {
            escalator_id,
            width,
            height,
            x_velocity,
            y_velocity,
            push_velocity,
        }
    }
}
