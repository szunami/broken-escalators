use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::VirtualKeyCode;

#[derive(Clone, Copy)]
pub enum Direction {
    CLOCKWISE,
    COUNTERCLOCKWISE,
}

#[derive(Clone, Copy)]
pub struct Escalator {
    pub speed: f32,
    pub direction: Direction,
    pub toggle_key: VirtualKeyCode,
}

impl Component for Escalator {
    type Storage = DenseVecStorage<Self>;
}

impl Escalator {
    pub fn new(
        speed: f32,
        direction: Direction,
        toggle_key: VirtualKeyCode,
    ) -> Escalator {
        Escalator {
            speed,
            direction,
            toggle_key,
        }
    }

    pub fn toggle_direction(&mut self) {
        match self.direction {
            Direction::CLOCKWISE => self.direction = Direction::COUNTERCLOCKWISE,
            Direction::COUNTERCLOCKWISE => self.direction = Direction::CLOCKWISE,
        }
    }
}
