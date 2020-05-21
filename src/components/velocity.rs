use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone, Debug, Default)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

impl<'s> Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> Velocity {
    pub fn default() -> Velocity {
        Velocity { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Velocity {
        Velocity { x, y }
    }
}
