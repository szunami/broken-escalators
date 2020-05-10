use crate::components::{Rectangle, Step, Thing, Velocity};
use crate::{
    resources::RewindableClock,
    utils::{x_overlap, BoundingBox},
};
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, clock, things, steps, velocities, rectangles, mut transforms, time): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }

        for (_step, step_transform, step_velocity) in (&steps, &mut transforms, &velocities).join()
        {
            step_transform.prepend_translation_x(step_velocity.x * time.delta_seconds());
            step_transform.prepend_translation_y(step_velocity.y * time.delta_seconds());
        }

        for (_thing, thing_entity, thing_velocity) in (&things, &entities, &velocities).join() {
            let mut thing_transform;
            {
                thing_transform = transforms.get_mut(thing_entity).unwrap();
            }
            // info!("Thing velocity: x: {}, y: {}", thing_velocity.x, thing_velocity.y);
            thing_transform.prepend_translation_x(thing_velocity.x * time.delta_seconds());
            thing_transform.prepend_translation_y(thing_velocity.y * time.delta_seconds());
        }

        // for (_thing, thing_entity, thing_velocity, thing_rectangle) in
        //     (&things, &entities, &velocities, &rectangles).join()
        // {
        //     let mut thing_transform;
        //     {
        //         thing_transform = transforms.get_mut(thing_entity).unwrap().clone();
        //     }

        //     let thing_box = BoundingBox::new(
        //         thing_rectangle.width,
        //         thing_rectangle.height,
        //         &thing_transform,
        //     );

        //     for (_step, step_entity, step_rectangle) in (&steps, &entities, &rectangles).join() {
        //         let step_transform;
        //         {
        //             step_transform = transforms.get_mut(step_entity).unwrap().clone();
        //         }
        //         let step_box = BoundingBox::from_rectangle(step_rectangle, &step_transform);
        //         thing_transform.prepend_translation_x(x_overlap(&thing_box, &step_box));
        //     }
        // }
    }
}
