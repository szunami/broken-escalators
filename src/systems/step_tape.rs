use crate::{
    components::{RewindableClock, Step, StepTape}, utils::{move_tape_forwards, move_tape_backwards},
};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct StepTapeSystem;

impl<'s> System<'s> for StepTapeSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, RewindableClock>,
        WriteStorage<'s, Step>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, StepTape>,
    );

    fn run(&mut self, (input, clocks, mut steps, mut locals, mut step_tapes): Self::SystemData) {
        for clock in (&clocks).join() {
            for (step, local, step_tape) in (&mut steps, &mut locals, &mut step_tapes).join() {
                let snapshots = &mut step_tape.snapshots;
                if input.key_is_down(VirtualKeyCode::Z) {
                    move_tape_backwards(snapshots, local, step);
                } else {
                    move_tape_forwards(snapshots, local, step, clock);
                }
            }
        }
    }
}