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
                Side::TOPPER_LEFT_CORNER => Side::DIAGONAL,
                Side::DIAGONAL => Side::LOWER_RIGHT_CORNER,
                Side::LOWER_RIGHT_CORNER => Side::HORIZONTAL,
                Side::HORIZONTAL => Side::LOWER_LEFT_CORNER,
                Side::LOWER_LEFT_CORNER => Side::VERTICAL,
                Side::VERTICAL => Side::TOPPER_LEFT_CORNER,
            },
            Direction::COUNTERCLOCKWISE => match side {
                Side::TOPPER_LEFT_CORNER => Side::VERTICAL,
                Side::VERTICAL => Side::LOWER_LEFT_CORNER,
                Side::LOWER_LEFT_CORNER => Side::HORIZONTAL,
                Side::HORIZONTAL => Side::LOWER_RIGHT_CORNER,
                Side::LOWER_RIGHT_CORNER => Side::DIAGONAL,
                Side::DIAGONAL => Side::TOPPER_LEFT_CORNER,
            },
        }
    }
}
