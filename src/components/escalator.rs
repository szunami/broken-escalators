use super::Side;
use crate::levels::Direction;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::VirtualKeyCode;

#[derive(Clone, Copy)]
pub struct Escalator {
    pub speed: i32,
    pub direction: Direction,
    pub toggle_key: VirtualKeyCode,
}

impl Component for Escalator {
    type Storage = DenseVecStorage<Self>;
}

impl Escalator {
    pub fn new(speed: i32, direction: Direction, toggle_key: VirtualKeyCode) -> Escalator {
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

    pub fn next_side(self, side: &Side) -> Side {
        match self.direction {
            Direction::CLOCKWISE => match side {
                Side::TopLeftCorner => Side::Diagonal,
                Side::Diagonal => Side::BottomRightCorner,
                Side::BottomRightCorner => Side::Bottom,
                Side::Bottom => Side::BottomLeftCorner,
                Side::BottomLeftCorner => Side::Left,
                Side::Left => Side::TopLeftCorner,
            },
            Direction::COUNTERCLOCKWISE => match side {
                Side::TopLeftCorner => Side::Left,
                Side::Left => Side::BottomLeftCorner,
                Side::BottomLeftCorner => Side::Bottom,
                Side::Bottom => Side::BottomRightCorner,
                Side::BottomRightCorner => Side::Diagonal,
                Side::Diagonal => Side::TopLeftCorner,
            },
        }
    }
}
