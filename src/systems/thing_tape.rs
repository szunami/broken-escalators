use crate::components::{RewindableClock, Thing, ThingSnapshot, ThingTape};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct ThingTapeSystem;

impl<'s> System<'s> for ThingTapeSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, RewindableClock>,
        WriteStorage<'s, Thing>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ThingTape>,
    );

    fn run(&mut self, (input, clocks, mut things, mut locals, mut thing_tapes): Self::SystemData) {
        for clock in (&clocks).join() {
            for (thing, local, thing_tape) in (&mut things, &mut locals, &mut thing_tapes).join() {
                if input.key_is_down(VirtualKeyCode::Z) {
                    info!("Going backwards");
                    match thing_tape.thing_snapshots.pop() {
                        Some(snapshot) => {
                            info!("Found a previous state");
                            local.set_translation(*snapshot.local.translation());
                            thing.x_velocity = snapshot.thing.x_velocity;
                            thing.y_velocity = snapshot.thing.y_velocity;
                        }
                        None => {
                            info!("At the end of the line");
                        }
                    }
                } else {
                    info!("Going forwards!");
                    thing_tape.thing_snapshots.push(ThingSnapshot {
                        thing: thing.clone(),
                        timestamp: clock.current_time,
                        local: local.clone(),
                    })
                }
            }
        }
    }
}
