use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{Escalator, Rectangle, Step};
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
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Rectangle>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, clock, steps, mut locals, escalators, rectangles, time): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }

        let mut map = HashMap::new();

        for (step_entity, step, step_rectangle) in (&entities, &steps, &rectangles).join() {
            let step_local = locals.get(step_entity).unwrap();
            let escalator = escalators.get(step.escalator).unwrap();
            let escalator_local = locals.get(step.escalator).unwrap().clone();
            let escalator_box =
                BoundingBox::new(escalator.width, escalator.height, &escalator_local);
            let x = (step_local.translation().x + step.x_velocity * time.delta_seconds())
                .max(escalator_box.left + step_rectangle.width * 0.5)
                .min(escalator_box.right - step_rectangle.width * 0.5);

            let y = (step_local.translation().y + step.y_velocity * time.delta_seconds())
                .max(escalator_box.bottom + step_rectangle.height * 0.5)
                .min(escalator_box.top - step_rectangle.height * 0.5);

            let mut new_local = Transform::default();
            new_local.append_translation_xyz(x, y, 0.);
            map.insert(step_entity, new_local);
        }

        for (step_entity, _step, step_local) in (&entities, &steps, &mut locals).join() {
            let new_local = map.get(&step_entity).unwrap();
            step_local.set_translation(*new_local.translation());
        }
    }
}
