use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct GridLocation {
    pub x: i32,
    pub y: i32
}

impl<'s> Component for GridLocation {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> GridLocation {
    pub fn default() -> GridLocation {
        GridLocation {
            x: 0, y: 0
        }
    }

    pub fn new(x: i32, y: i32) -> GridLocation {
        GridLocation {
            x, y
        }
    }
}