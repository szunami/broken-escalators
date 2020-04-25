use crate::components::{Thing, Atop};
use crate::{components::Step, utils::BoundsProvider};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (ReadStorage<'s, Thing>, WriteStorage<'s, Transform>, ReadStorage<'s, Atop>);

    fn run(&mut self, (things, mut locals, atops): Self::SystemData) {
        for (thing, mut thing_local, atop) in (&things, &mut locals, &atops).join() {
            thing_local.prepend_translation_x(atop.x_velocity);
            thing_local.prepend_translation_y(atop.y_velocity);
            warn!(
                "location: {} {}",
                thing_local.translation().x,
                thing_local.translation().y
            );
        }
    }
}
