use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};
use crate::components::GridLocation;

#[derive(SystemDesc)]
pub struct StepPosition;

impl<'s> System<'s> for StepPosition {
    type SystemData = (
        ReadStorage<'s, Step>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, GridLocation>
    );

    fn run(&mut self, (steps, velocities, grid_locations): Self::SystemData) {
        for (step, step_velocity, step_location) in (&steps, &velocities, &mut velocities).join() {
            step_location.x = step_location.x + step_velocity.x;
            step_location.y = step_location.y + step_velocity.y;
            info!("step_position: {:?}", step_position);
        }
    }
}