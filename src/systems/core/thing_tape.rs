use crate::{
    components::{Thing, ThingTape},
    resources::RewindableClock,
    utils::{move_tape_backwards, move_tape_forwards},
};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct ThingTapeSystem;

impl<'s> System<'s> for ThingTapeSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, RewindableClock>,
        WriteStorage<'s, Thing>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ThingTape>,
    );

    fn run(
        &mut self,
        (input, clock, mut things, mut transforms, mut thing_tapes): Self::SystemData,
    ) {
        for (thing, transform, thing_tape) in
            (&mut things, &mut transforms, &mut thing_tapes).join()
        {
            let snapshots = &mut thing_tape.snapshots;
            if input.key_is_down(VirtualKeyCode::Z) {
                move_tape_backwards(snapshots, transform, thing);
            } else {
                move_tape_forwards(snapshots, transform, thing, &clock);
            }
        }
    }
}
