use crate::{
    components::{GridLocation, Thing, ThingTape},
    resources::RewindableClock,
    utils::{move_tape_backwards, move_tape_forwards},
};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct ThingTapeSystem;

impl<'s> System<'s> for ThingTapeSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        WriteStorage<'s, Thing>,
        WriteStorage<'s, GridLocation>,
        WriteStorage<'s, ThingTape>,
    );

    fn run(&mut self, (clock, mut things, mut grid_locations, mut thing_tapes): Self::SystemData) {
        if clock.velocity == 0 {
            return;
        }
        for (thing, thing_grid_location, thing_tape) in
            (&mut things, &mut grid_locations, &mut thing_tapes).join()
        {
            let snapshots = &mut thing_tape.snapshots;
            if clock.going_forwards() {
                info!("Going forwards");
                move_tape_forwards(snapshots, thing_grid_location, thing);
            } else {
                info!("Going backward");
                move_tape_backwards(snapshots, thing_grid_location, thing);
            }
        }
    }
}
