use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
#[derive(Clone)]
pub struct Step {
    pub escalator: Entity,
    pub push_velocity: f32,
}

impl Component for Step {
    type Storage = DenseVecStorage<Self>;
}

impl Step {
    pub fn new(escalator: Entity, push_velocity: f32) -> Step {
        Step {
            escalator,
            push_velocity,
        }
    }
}
