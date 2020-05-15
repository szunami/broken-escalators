use crate::components::{Rectangle, Step, Thing};
use crate::{
    resources::RewindableClock,
    utils::{x_overlap, y_overlap, BoundingBox},
};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
#[derive(SystemDesc)]
pub struct ThingCorrectionSystem;

impl<'s> System<'s> for ThingCorrectionSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (entities, clock, things, steps, rectangles, mut transforms): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }
        // account for collisions
        for (_thing, thing_entity, thing_rectangle) in (&things, &entities, &rectangles).join() {
            for (_step, step_entity, step_rectangle) in (&steps, &entities, &rectangles).join() {
                let step_transform = transforms.get(step_entity).unwrap().clone();
                let thing_transform = transforms.get_mut(thing_entity).unwrap();

                let thing_box = BoundingBox::new(thing_rectangle, &thing_transform);

                let step_box = BoundingBox::new(step_rectangle, &step_transform);
                let x_overlap = x_overlap(&thing_box, &step_box);
                let y_overlap = y_overlap(&thing_box, &step_box);

                if x_overlap.abs() < y_overlap.abs() {
                    thing_transform.prepend_translation_x(x_overlap);
                } else {
                    thing_transform.prepend_translation_y(y_overlap);
                }
            }
        }
    }
}
