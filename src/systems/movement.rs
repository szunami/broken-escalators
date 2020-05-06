use crate::components::{Atop, Thing};
use crate::resources::RewindableClock;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Atop>,
        Read<'s, Time>,
    );

    fn run(&mut self, (clock, things, mut locals, atops, time): Self::SystemData) {
        if !clock.going_forwards() {
            return;
        }
        for (_thing, thing_local, atop) in (&things, &mut locals, &atops).join() {
            thing_local.prepend_translation_x(atop.x_velocity * time.delta_seconds());
            thing_local.prepend_translation_y(atop.y_velocity * time.delta_seconds());
        }
    }
}
