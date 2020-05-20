use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, ReadStorage, WriteStorage},
};
use amethyst::input::VirtualKeyCode;
use crate::{
    components::{GridLocation, Thing, Velocity},
    resources::DownKeys,
};

#[derive(SystemDesc)]
pub struct ThingPositionSystem;

impl<'s> System<'s> for ThingPositionSystem {
    type SystemData = (
        Read<'s, DownKeys>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, GridLocation>,
    );

    fn run(&mut self, (down_keys, things, velocities, mut grid_locations): Self::SystemData) {
        if !down_keys.key_downs().contains(&VirtualKeyCode::Space) {
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