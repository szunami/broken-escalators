use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData,ReadStorage, WriteStorage},
};
use crate::{resources::DownKeys, components::{Step, GridLocation, Velocity}};
use amethyst::input::VirtualKeyCode;

#[derive(SystemDesc)]
pub struct StepPositionSystem;

impl<'s> System<'s> for StepPositionSystem {
    type SystemData = (
        Read<'s, DownKeys>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, GridLocation>
    );

    fn run(&mut self, (down_keys, steps, velocities, mut grid_locations): Self::SystemData) {
        if !down_keys.key_downs().contains(&VirtualKeyCode::Space) {
            return;
        }
        for (step, step_velocity, step_location) in (&steps, &velocities, &mut grid_locations).join() {
            step_location.x = step_location.x + step_velocity.x;
            step_location.y = step_location.y + step_velocity.y;
            info!("step_position: {:?}", step_location);
        }
    }
}