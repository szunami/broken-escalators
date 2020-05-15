use crate::components::{Escalator, Rectangle, Step, Thing, Velocity};
use crate::{
    resources::RewindableClock,
    utils::{contains, extrusion, x_overlap, y_overlap, BoundingBox},
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
        WriteStorage<'s, Step>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Rectangle>,
        ReadStorage<'s, Escalator>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            clock,
            things,
            mut steps,
            velocities,
            rectangles,
            escalators,
            mut transforms,
            time,
        ): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }

        for (step, step_entity, step_velocity, step_rectangle) in
            (&mut steps, &entities, &velocities, &rectangles).join()
        {
            info!("Step velocity: {:?}", step_velocity);
            let escalator_transform = transforms.get(step.escalator).unwrap().clone();
            let mut step_transform = transforms.get_mut(step_entity).unwrap();
            step_transform.prepend_translation_x(step_velocity.x * time.delta_seconds());
            step_transform.prepend_translation_y(step_velocity.y * time.delta_seconds());

            let step_box = BoundingBox::new(step_rectangle, step_transform);
            let escalator = escalators.get(step.escalator).unwrap();
            let escalator_rectangle = rectangles.get(step.escalator).unwrap();
            let escalator_box = BoundingBox::new(escalator_rectangle, &escalator_transform);
            if !contains(&escalator_box, step_box) {
                step.side = escalator.next_side(&step.side);
                info!("New side: {:?}", step.side);
                info!("Extrusion: {}", extrusion(&escalator_box, step_box));
            }
        }

        for (_thing, thing_entity, thing_velocity) in (&things, &entities, &velocities).join() {
            info!("velocity: {:?}", thing_velocity);
            let thing_transform = transforms.get_mut(thing_entity).unwrap();
            thing_transform.prepend_translation_x(thing_velocity.x * time.delta_seconds());
            thing_transform.prepend_translation_y(thing_velocity.y * time.delta_seconds());
        }
        // account for overshooting escalator corners?

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
                    info!("Applying x correction: {}", x_overlap);
                    thing_transform.prepend_translation_x(x_overlap);
                } else {
                    info!("Applying y correction: {}", y_overlap);
                    thing_transform.prepend_translation_y(y_overlap);
                }
            }
        }
    }
}
