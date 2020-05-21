use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl<'s> Component for Rectangle {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> Rectangle {
    pub fn new(width: i32, height: i32) -> Rectangle {
        Rectangle { width, height }
    }
}
