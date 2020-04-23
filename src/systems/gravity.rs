
use crate::components::Thing;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};
#[derive(SystemDesc)]
pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        ReadStorage<'s, Thing>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (things, mut locals): Self::SystemData) {
        for (thing, local) in (&things, &mut locals).join() {
            local.prepend_translation_y(-1.);
        }
    }
}