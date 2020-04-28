use crate::components::{Atop, Push, Thing};
use crate::{components::Step, utils::BoundsProvider};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        ReadStorage<'s, Thing>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Atop>,
        ReadStorage<'s, Push>,
    );

    fn run(&mut self, (things, mut locals, atops, pushes): Self::SystemData) {
        for (thing, mut thing_local, atop, push) in (&things, &mut locals, &atops, &pushes).join() {
            if atop.atop_name != push.pusher_name {
                thing_local.prepend_translation_x(push.x_velocity);
            }
            thing_local.prepend_translation_x(atop.x_velocity);
            thing_local.prepend_translation_y(atop.y_velocity);
        }
    }
}
