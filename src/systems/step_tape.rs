use crate::{
    components::{RewindableClock, Step, StepTape},
    utils::Snapshot,
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
                step_tape.snapshots.pop();
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

fn move_tape_backwards(step_tape: &mut Vec<Snapshot<Step>>, local: &mut Transform, step: &mut Step) {
    match step_tape.pop() {
        Some(snapshot) => {
            info!("Found a previous state");
            local.set_translation(*snapshot.local.translation());
            step.x_velocity = snapshot.component.x_velocity;
            step.y_velocity = snapshot.component.y_velocity;
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