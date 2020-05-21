use crate::components::{Escalator, GridLocation, Rectangle, Side, Step, Velocity};
use crate::{
    levels::Direction,
    resources::DownKeys,
    resources::RewindableClock,
    utils::{touching_multiple_edges, BoundingBox},
};
use amethyst::input::VirtualKeyCode;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct StepVelocitySystem;

impl<'s> System<'s> for StepVelocitySystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, GridLocation>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, Step>,
        WriteStorage<'s, Velocity>,
    );

    fn run(
        &mut self,
        (clock, escalators, grid_locations, rectangles, mut steps, mut velocities): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }
        for (step, step_velocity, step_grid_location, step_rectangle) in
            (&mut steps, &mut velocities, &grid_locations, &rectangles).join()
        {
            let step_box = BoundingBox::new(step_rectangle, step_grid_location);

            let escalator = escalators.get(step.escalator).unwrap();
            let escalator_rectangle = rectangles.get(step.escalator).unwrap();
            let escalator_grid_location = grid_locations.get(step.escalator).unwrap();
            let escalator_box = BoundingBox::new(escalator_rectangle, escalator_grid_location);
            // if touching_multiple_edges(&step_box, &escalator_box) && !step.side.is_corner() {
            //     step.side = escalator.next_side(&step.side);
            //     info!("Newly at corner, setting to {:?}", step.side);
            // }
            // if !touching_multiple_edges(&step_box, &escalator_box) && step.side.is_corner() {
            //     step.side = escalator.next_side(&step.side);
            //     info!("No longer at corner, setting to {:?}", step.side);
            // }
            info!("Side is: {:?}", step.side);
            step_velocity.x = x_velocity_for_side_and_direction(&step.side, &escalator);
            step_velocity.y = y_velocity_for_side(&step.side, &escalator);
            info!("Step velocity: {:?}", step_velocity);
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
