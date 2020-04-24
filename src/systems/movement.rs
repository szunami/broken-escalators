use crate::components::Thing;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};
use crate::{utils::BoundsProvider, components::Step};


#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        ReadStorage<'s, Thing>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (things, mut locals): Self::SystemData) {
        for (thing, mut thing_local) in (&things, &mut locals).join() {
            thing_local.prepend_translation_x(thing.x_velocity);
            thing_local.prepend_translation_y(thing.y_velocity);
            warn!("location: {} {}", thing_local.translation().x, thing_local.translation().y);
        }
    }
}
