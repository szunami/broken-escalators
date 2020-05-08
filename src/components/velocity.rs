use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl<'s> Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> Velocity {
    pub fn default(x:f32, y:f32) -> Velocity {
        Velocity {
            x, y
        }
    }
}