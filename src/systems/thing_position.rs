use crate::{
    components::{GridLocation, Thing, Velocity},
    resources::{RewindableClock, DownKeys},
};
use amethyst::input::VirtualKeyCode;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct ThingPositionSystem;

impl<'s> System<'s> for ThingPositionSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, GridLocation>,
    );

    fn run(&mut self, (clock, things, velocities, mut grid_locations): Self::SystemData) {
        if !clock.going_forwards() {
            return;
        }
        for (thing, thing_velocity, thing_location) in
            (&things, &velocities, &mut grid_locations).join()
        {
            thing_location.x = thing_location.x + thing_velocity.x;
            thing_location.y = thing_location.y + thing_velocity.y;
            info!("thing_position: {:?}", thing_location);
        }
    }
}
