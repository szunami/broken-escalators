use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone, Copy)]
pub enum Direction {
    CLOCKWISE,
    COUNTERCLOCKWISE,
}
pub struct Escalator {
    pub id: i32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub direction: Direction,
}

impl Component for Escalator {
    type Storage = DenseVecStorage<Self>;
}

impl Escalator {
    pub fn new(id: i32, width: f32, height: f32, speed: f32, direction: Direction) -> Escalator {
        Escalator {
            id,
            width,
            height,
            speed,
            direction,
        }
    }
}
