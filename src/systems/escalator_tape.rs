use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use crate::{
    components::{Escalator, GridLocation, EscalatorTape},
    resources::RewindableClock, utils::{move_tape_forwards, move_tape_backwards}
};

#[derive(SystemDesc)]
pub struct EscalatorTapeSystem;

impl<'s> System<'s> for EscalatorTapeSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        WriteStorage<'s, Escalator>,
        WriteStorage<'s, GridLocation>,
        WriteStorage<'s, EscalatorTape>,
    );

    fn run(&mut self, (clock, mut escalators, mut grid_locations, mut escalator_tapes): Self::SystemData) {
        if clock.velocity == 0 {
            return;
        }
        for (escalator, escalator_grid_location, escalator_tape) in
            (&mut escalators, &mut grid_locations, &mut escalator_tapes).join()
        {
            let snapshots = &mut escalator_tape.snapshots;
            if clock.going_forwards() {
                info!("Going forwards");
                move_tape_forwards(snapshots, escalator_grid_location, escalator);
            } else {
                info!("Going backward");
                move_tape_backwards(snapshots, escalator_grid_location, escalator);
            }
        }
    }
}