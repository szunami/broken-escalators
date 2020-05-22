use crate::{
    components::{Escalator, GridLocation, Rectangle, Step, Velocity},
    resources::RewindableClock,
    utils::{touching_multiple_edges, BoundingBox},
};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct StepPositionSystem;

impl<'s> System<'s> for StepPositionSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        WriteStorage<'s, Step>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Rectangle>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, GridLocation>,
    );

    fn run(
        &mut self,
        (clock, mut steps, escalators, rectangles, velocities, mut grid_locations): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }
        for (_step, step_velocity, step_location) in
            (&steps, &velocities, &mut grid_locations).join()
        {
            step_location.x += step_velocity.absolute[0];
            step_location.y += step_velocity.absolute[1];
            debug!("step_position: {:?}", step_location);
        }

        for (step, step_grid_location, step_rectangle) in
            (&mut steps, &grid_locations, &rectangles).join()
        {
            let step_box = BoundingBox::new(step_rectangle, step_grid_location);

            let escalator = escalators.get(step.escalator).unwrap();
            let escalator_rectangle = rectangles.get(step.escalator).unwrap();
            let escalator_grid_location = grid_locations.get(step.escalator).unwrap();
            let escalator_box = BoundingBox::new(escalator_rectangle, escalator_grid_location);
            if touching_multiple_edges(&step_box, &escalator_box) && !step.side.is_corner() {
                step.side = escalator.next_side(&step.side);
                debug!("Newly at corner, setting to {:?}", step.side);
            }
            if !touching_multiple_edges(&step_box, &escalator_box) && step.side.is_corner() {
                step.side = escalator.next_side(&step.side);
                debug!("No longer at corner, setting to {:?}", step.side);
            }
        }
    }
}
