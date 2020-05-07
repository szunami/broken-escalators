use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Platform {
    width: f32,
    height: f32,
}

impl<'s> Component for Platform {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> Platform {
    pub fn default(width: f32, height: f32) -> Platform {
        Platform {
            width, height
        }
    }
}