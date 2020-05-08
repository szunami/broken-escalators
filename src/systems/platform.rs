use crate::{utils::BoundingBox, components::{Thing, Platform}};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{ReadStorage, System, SystemData, Join},
    core::transform::Transform,
};

#[derive(SystemDesc)]
pub struct PlatformSystem;

impl<'s> System<'s> for PlatformSystem {
    type SystemData = (
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Platform>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (things, platforms, transforms): Self::SystemData) {
        for (thing, thing_transform) in (&things, &transforms).join() {
            let thing_box = BoundingBox::new(thing.width, thing.height, thing_transform);
            if thing_box.top < 0. {
                warn!("You lose!");
            }
        }
    }
}
