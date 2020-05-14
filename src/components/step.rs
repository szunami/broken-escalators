use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

#[derive(Clone, PartialEq, Debug)]
pub enum Side {
    VERTICAL,
    HORIZONTAL,
    DIAGONAL,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Step {
    pub escalator: Entity,
    pub push_velocity: f32,
    pub side: Side
}

impl Component for Step {
    type Storage = DenseVecStorage<Self>;
}

impl Step {
    pub fn new(escalator: Entity, push_velocity: f32, side: Side) -> Step {
        Step {
            escalator,
            push_velocity,
            side,
        }
    }
}
