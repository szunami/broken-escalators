use crate::components::{Escalator, Rectangle, Step};
use crate::{
    resources::RewindableClock,
    utils::{extrusion, BoundingBox},
};
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
#[derive(SystemDesc)]
pub struct StepCorrectionSystem;

impl<'s> System<'s> for StepCorrectionSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        WriteStorage<'s, Step>,
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
            mut steps,
            rectangles,
            escalators,
            mut transforms,
            time,
        ): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }
        for (step, step_entity, step_rectangle) in
            (&mut steps, &entities, &rectangles).join()
        {
            let escalator_transform = transforms.get(step.escalator).unwrap().clone();
            let step_transform = transforms.get_mut(step_entity).unwrap();
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
