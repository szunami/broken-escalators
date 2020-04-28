use crate::components::{Atop, Thing};
use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage, Read},
};

#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        ReadStorage<'s, Thing>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Atop>,
        Read<'s, Time>,
    );

    fn run(&mut self, (things, mut locals, atops, time): Self::SystemData) {
        for (_thing, thing_local, atop) in (&things, &mut locals, &atops).join() {
            thing_local.prepend_translation_x(atop.x_velocity * time.delta_seconds());
            thing_local.prepend_translation_y(atop.y_velocity * time.delta_seconds());
        }
    }
}
