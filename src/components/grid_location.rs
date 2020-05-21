use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone, Debug)]
pub struct GridLocation {
    pub x: i32,
    pub y: i32,
}

impl<'s> Component for GridLocation {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> GridLocation {
    pub fn new(x: i32, y: i32) -> GridLocation {
        GridLocation { x, y }
    }
}
