use crate::components::{Escalator, GridLocation, Velocity};
use crate::resources::RewindableClock;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct EscalatorPositionSystem;

impl<'s> System<'s> for EscalatorPositionSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, GridLocation>,
    );

    fn run(&mut self, (clock, escalators, velocities, mut grid_locations): Self::SystemData) {
        if !clock.going_forwards() {
            return;
        }
        for (_escalator, escalator_velocity, escalator_location) in
            (&escalators, &velocities, &mut grid_locations).join()
        {
            escalator_location.x += escalator_velocity.absolute[0];
            escalator_location.y += escalator_velocity.absolute[1];
            debug!("escalator_position: {:?}", escalator_location);
        }
    }
}
