use crate::{
    components::{Escalator, EscalatorTape, GridLocation},
    resources::RewindableClock,
    utils::{move_tape_backwards, move_tape_forwards},
};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
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

    fn run(
        &mut self,
        (clock, mut escalators, mut grid_locations, mut escalator_tapes): Self::SystemData,
    ) {
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
                // probably need to handle direction specially!
                move_tape_backwards(snapshots, escalator_grid_location, escalator);
            }
        }
    }
}
