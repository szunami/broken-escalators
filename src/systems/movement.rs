use crate::components::{Atop, RewindableClock, Thing};
use crate::utils::backwards_clock_check;
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
        ReadStorage<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Atop>,
        Read<'s, Time>,
    );

    fn run(&mut self, (clocks, things, mut locals, atops, time): Self::SystemData) {
        if backwards_clock_check(clocks) {
            return;
        }
        for (_thing, thing_local, atop) in (&things, &mut locals, &atops).join() {
            thing_local.prepend_translation_x(atop.x_velocity * time.delta_seconds());
            thing_local.prepend_translation_y(atop.y_velocity * time.delta_seconds());
        }
    }
}
