use crate::components::{Escalator, Side, Step, Velocity, Rectangle, GridLocation};
use crate::resources::RewindableClock;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct StepVelocitySystem;

impl<'s> System<'s> for StepVelocitySystem {
    type SystemData = (
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, GridLocation>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, Step>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (escalators, grid_locations, rectangles, mut steps, mut velocities): Self::SystemData) {
        for (step, step_velocity, step_grid_location) in (&mut steps, &mut velocities, &grid_locations).join() {
            let escalator = escalators.get(step.escalator).unwrap();

            

            step_velocity.x = x_velocity_for_side(&step.side, &escalator);
            step_velocity.y = y_velocity_for_side(&step.side, &escalator);
            info!("step_velocity: {:?}", step_velocity);
        }
    }
}

pub fn x_velocity_for_side(side: &Side, escalator: &Escalator) -> i32 {
    escalator.speed * escalator.direction.direction_factor() * side.base_x_component()
}

pub fn y_velocity_for_side(side: &Side, escalator: &Escalator) -> i32 {
    escalator.speed * escalator.direction.direction_factor() * side.base_y_component()
}
