use crate::components::{Escalator, Side, Step, Velocity};
use crate::{levels::Direction, resources::RewindableClock};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct IntrinsicStepVelocitySystem;

impl<'s> System<'s> for IntrinsicStepVelocitySystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        ReadStorage<'s, Escalator>,
        WriteStorage<'s, Step>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (clock, escalators, mut steps, mut velocities): Self::SystemData) {
        if !clock.going_forwards() {
            return;
        }
        for (step, step_velocity) in (&mut steps, &mut velocities).join() {
            let escalator = escalators.get(step.escalator).unwrap();
            step_velocity.intrinsic[0] = x_velocity_for_side_and_direction(&step.side, &escalator);
            step_velocity.intrinsic[1] = y_velocity_for_side(&step.side, &escalator);
            debug!("Step velocity: {:?}", step_velocity);
        }
    }
}

pub fn x_velocity_for_side_and_direction(side: &Side, escalator: &Escalator) -> i32 {
    escalator.speed
        * match (escalator.direction, side) {
            (Direction::CLOCKWISE, Side::TopLeftCorner) => 1,
            (Direction::CLOCKWISE, Side::Diagonal) => 1,
            (Direction::CLOCKWISE, Side::BottomRightCorner) => -1,
            (Direction::CLOCKWISE, Side::Bottom) => -1,
            (Direction::CLOCKWISE, Side::BottomLeftCorner) => 0,
            (Direction::CLOCKWISE, Side::Left) => 0,
            (Direction::COUNTERCLOCKWISE, Side::TopLeftCorner) => 0,
            (Direction::COUNTERCLOCKWISE, Side::Left) => 0,
            (Direction::COUNTERCLOCKWISE, Side::BottomLeftCorner) => 1,
            (Direction::COUNTERCLOCKWISE, Side::Bottom) => 1,
            (Direction::COUNTERCLOCKWISE, Side::BottomRightCorner) => -1,
            (Direction::COUNTERCLOCKWISE, Side::Diagonal) => -1,
        }
}

pub fn y_velocity_for_side(side: &Side, escalator: &Escalator) -> i32 {
    escalator.speed
        * match (escalator.direction, side) {
            (Direction::CLOCKWISE, Side::TopLeftCorner) => -1,
            (Direction::CLOCKWISE, Side::Diagonal) => -1,
            (Direction::CLOCKWISE, Side::BottomRightCorner) => 0,
            (Direction::CLOCKWISE, Side::Bottom) => 0,
            (Direction::CLOCKWISE, Side::BottomLeftCorner) => 1,
            (Direction::CLOCKWISE, Side::Left) => 1,
            (Direction::COUNTERCLOCKWISE, Side::TopLeftCorner) => -1,
            (Direction::COUNTERCLOCKWISE, Side::Left) => -1,
            (Direction::COUNTERCLOCKWISE, Side::BottomLeftCorner) => 0,
            (Direction::COUNTERCLOCKWISE, Side::Bottom) => 0,
            (Direction::COUNTERCLOCKWISE, Side::BottomRightCorner) => 1,
            (Direction::COUNTERCLOCKWISE, Side::Diagonal) => 1,
        }
}
