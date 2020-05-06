use crate::{
    components::{Step, StepTape},
    resources::RewindableClock,
    utils::{move_tape_backwards, move_tape_forwards},
};

use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct StepTapeSystem;

impl<'s> System<'s> for StepTapeSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        WriteStorage<'s, Step>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, StepTape>,
    );

    fn run(&mut self, (clock, mut steps, mut locals, mut step_tapes): Self::SystemData) {
        for (step, local, step_tape) in (&mut steps, &mut locals, &mut step_tapes).join() {
            let snapshots = &mut step_tape.snapshots;
            if clock.velocity < 0. {
                move_tape_backwards(snapshots, local, step);
            } else {
                move_tape_forwards(snapshots, local, step, &clock);
            }
        }
    }
}
