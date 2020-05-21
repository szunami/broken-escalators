use crate::{
    components::{Escalator, GridLocation, Rectangle, Step, Velocity},
    resources::DownKeys,
    utils::{touching_multiple_edges, BoundingBox},
};
use amethyst::input::VirtualKeyCode;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct StepPositionSystem;

impl<'s> System<'s> for StepPositionSystem {
    type SystemData = (
        Read<'s, DownKeys>,
        WriteStorage<'s, Step>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Rectangle>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, GridLocation>,
    );

    fn run(
        &mut self,
        (down_keys, mut steps, escalators, rectangles, velocities, mut grid_locations): Self::SystemData,
    ) {
        if !down_keys.key_downs().contains(&VirtualKeyCode::Space) {
            return;
        }
        for (step, step_velocity, step_location) in
            (&steps, &velocities, &mut grid_locations).join()
        {
            step_location.x = step_location.x + step_velocity.x;
            step_location.y = step_location.y + step_velocity.y;
            info!("step_position: {:?}", step_location);
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
                info!("Newly at corner, setting to {:?}", step.side);
            }
            if !touching_multiple_edges(&step_box, &escalator_box) && step.side.is_corner() {
                step.side = escalator.next_side(&step.side);
                info!("No longer at corner, setting to {:?}", step.side);
            }
            // info!("Side is: {:?}", step.side);
            // step_velocity.x = x_velocity_for_side_and_direction(&step.side, &escalator);
            // step_velocity.y = y_velocity_for_side(&step.side, &escalator);
            // info!("Step velocity: {:?}", step_velocity);
        }
    }
}
