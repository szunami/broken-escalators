use crate::components::{Thing, ThingTape};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct ThingTapeSystem;

impl<'s> System<'s> for ThingTapeSystem {
    type SystemData = (
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, ThingTape>,
    );

    fn run(&mut self, (things, locals, mut thing_tapes): Self::SystemData) {
        for (thing, thing_local, thing_tape) in (&things, &locals, &mut thing_tapes).join() {
            thing_tape.push((*thing).clone(), (*thing_local).clone());
        }
    }
}
