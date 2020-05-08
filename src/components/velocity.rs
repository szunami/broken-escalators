use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl<'s> Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> Velocity {
    pub fn default() -> Velocity {
        Velocity { x: 0., y: 0. }
    }

    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity { x, y }
    }
}
