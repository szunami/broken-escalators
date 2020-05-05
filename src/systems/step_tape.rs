use crate::components::{RewindableClock, Step, StepSnapshot, StepTape};
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
                if input.key_is_down(VirtualKeyCode::Z) {
                    match step_tape.snapshots.pop() {
                        Some(snapshot) => {
                            info!("Found a previous state");
                            local.set_translation(*snapshot.local.translation());
                            step.x_velocity = snapshot.step.x_velocity;
                            step.y_velocity = snapshot.step.y_velocity;
                        }
                        None => {}
                    }
                } else {
                    step_tape.snapshots.push(StepSnapshot {
                        step: step.clone(),
                        timestamp: clock.current_time,
                        local: local.clone(),
                    })
                }
            }
        }
    }
}
