use crate::components::{Escalator, Rectangle, Step, Thing, Velocity};
use crate::{
    resources::RewindableClock,
    utils::{extrusion, x_overlap, y_overlap, BoundingBox},
};
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
#[derive(SystemDesc)]
pub struct StepCorrection;

impl<'s> System<'s> for StepCorrection {
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
            let step_box = BoundingBox::new(step_rectangle, step_transform);
            let escalator = escalators.get(step.escalator).unwrap();
            let escalator_rectangle = rectangles.get(step.escalator).unwrap();
            let escalator_box = BoundingBox::new(escalator_rectangle, &escalator_transform);

            let step_extrusion = extrusion(&escalator_box, &step_box);
            if step_extrusion > 0. {
                // move back to corner
                step_transform.prepend_translation_x(
                    -step.side.base_x_component()
                        * escalator.direction.direction_factor()
                        * step_extrusion,
                );
                step_transform.prepend_translation_y(
                    -step.side.base_y_component()
                        * escalator.direction.direction_factor()
                        * step_extrusion,
                );
                // move to next side
                step.side = escalator.next_side(&step.side);
                // move in new direction
                step_transform.prepend_translation_x(
                    step.side.base_x_component()
                        * escalator.direction.direction_factor()
                        * step_extrusion,
                );
                step_transform.prepend_translation_y(
                    step.side.base_y_component()
                        * escalator.direction.direction_factor()
                        * step_extrusion,
                );
            }
    }
    }
}