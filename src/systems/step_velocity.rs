use crate::components::{Escalator, Side, Step, Velocity, Rectangle, GridLocation};
use crate::{utils::{touching_multiple_edges, BoundingBox}, resources::RewindableClock, resources::DownKeys};
use amethyst::input::VirtualKeyCode;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct StepVelocitySystem;

impl<'s> System<'s> for StepVelocitySystem {
    type SystemData = (
        Read<'s, DownKeys>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, GridLocation>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, Step>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (down_keys, escalators, grid_locations, rectangles, mut steps, mut velocities): Self::SystemData) {
        if !down_keys.key_downs().contains(&VirtualKeyCode::Space) {
            return;
        }
        for (step, step_velocity, step_grid_location, step_rectangle)
            in (&mut steps, &mut velocities, &grid_locations, &rectangles).join() {

            let step_box = BoundingBox::new(step_rectangle, step_grid_location);
            
            let escalator = escalators.get(step.escalator).unwrap();
            let escalator_rectangle = rectangles.get(step.escalator).unwrap();
            let escalator_grid_location = grid_locations.get(step.escalator).unwrap();
            let escalator_box = BoundingBox::new(escalator_rectangle, escalator_grid_location);

            if touching_multiple_edges(&step_box, &escalator_box) {
                info!("Hitting edge, changing direction");
                step.side = escalator.next_side(&step.side);
            }
            info!("Side: {:?}", step.side);
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
