use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{Escalator, Rectangle, Step, Velocity};
use crate::{resources::RewindableClock, utils::BoundingBox};
use std::collections::HashMap;
#[derive(SystemDesc)]
pub struct EscalatorSystem;

impl<'s> System<'s> for EscalatorSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Step>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Rectangle>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, clock, steps, mut transforms, velocities, escalators, rectangles, time): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }

        let mut map = HashMap::new();

        for (step_entity, step, step_rectangle, step_velocity) in
            (&entities, &steps, &rectangles, &velocities).join()
        {
            let step_transform = transforms.get(step_entity).unwrap();
            let escalator = escalators.get(step.escalator).unwrap();
            let escalator_transform = transforms.get(step.escalator).unwrap().clone();
            let escalator_box =
                BoundingBox::new(escalator.width, escalator.height, &escalator_transform);
            let x = (step_transform.translation().x + step_velocity.x * time.delta_seconds())
                .max(escalator_box.left + step_rectangle.width * 0.5)
                .min(escalator_box.right - step_rectangle.width * 0.5);

            let y = (step_transform.translation().y + step_velocity.y * time.delta_seconds())
                .max(escalator_box.bottom + step_rectangle.height * 0.5)
                .min(escalator_box.top - step_rectangle.height * 0.5);

            let mut new_transform = Transform::default();
            new_transform.append_translation_xyz(x, y, 0.);
            map.insert(step_entity, new_transform);
        }

        for (step_entity, _step, step_transform) in (&entities, &steps, &mut transforms).join() {
            let new_transform = map.get(&step_entity).unwrap();
            step_transform.set_translation(*new_transform.translation());
        }
    }
}
