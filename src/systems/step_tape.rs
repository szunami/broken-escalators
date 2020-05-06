use crate::{
    components::{RewindableClock, Step, StepTape},
    utils::{UpdateFrom, Snapshot},
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
                    move_tape_forwards(step_tape, local, step, clock);
                }
            }
        }
    }
}

fn move_tape_backwards<T: UpdateFrom<T>>(snapshots: &mut Vec<Snapshot<T>>, local: &mut Transform, component: &mut T) {
    match snapshots.pop() {
        Some(snapshot) => {
            local.set_translation(*snapshot.local.translation());
            component.update_from(snapshot.component);
        }
        None => {}
    }
}
fn move_tape_forwards(step_tape: &mut StepTape, local: &mut Transform, step: &mut Step, clock: &RewindableClock) {
    step_tape.snapshots.push(Snapshot {
        component: step.clone(),
        timestamp: clock.current_time,
        local: local.clone(),
    })
}