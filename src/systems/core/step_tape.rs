use crate::{
    components::{Step, StepTape, GridLocation},
    resources::RewindableClock,   resources::DownKeys,

    utils::{move_tape_backwards, move_tape_forwards},
};
use amethyst::input::VirtualKeyCode;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct StepTapeSystem;

impl<'s> System<'s> for StepTapeSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        WriteStorage<'s, Step>,
        WriteStorage<'s, GridLocation>,
        WriteStorage<'s, StepTape>,
    );

    fn run(&mut self, (clock, mut steps, mut grid_locations, mut step_tapes): Self::SystemData) {
        if clock.velocity == 0 {
            return
        }
        for (step, step_grid_location, step_tape) in (&mut steps, &mut grid_locations, &mut step_tapes).join() {
            let snapshots = &mut step_tape.snapshots;
            if clock.going_forwards() {
                info!("Going forwards");
                move_tape_forwards(snapshots, step_grid_location, step);
            } else {
                info!("Going backward");
                move_tape_backwards(snapshots, step_grid_location, step);
            }
        }
    }
}
