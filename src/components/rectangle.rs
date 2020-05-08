use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl<'s> Component for Rectangle {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> Rectangle {
    pub fn default(width: f32, height: f32) -> Rectangle {
        Rectangle {
            width, height
        }
    }
}