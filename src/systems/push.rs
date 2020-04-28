use amethyst::{
    core::{transform::Transform, Named},
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{Escalator, Push, Step, Thing};
use crate::utils::BoundsProvider;

#[derive(SystemDesc)]
pub struct PushSystem;

impl<'s> System<'s> for PushSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Push>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, (entities, mut atops, things, transforms, steps, names): Self::SystemData) {
        for (thing_entity, thing_push, thing, thing_transform) in
            (&entities, &mut atops, &things, &transforms).join()
        {
            let mut max_pushness = 0.;
            let mut maybe_push_velocity = None;
            let mut maybe_push_name = None;

            for (step_entity, step, step_transform, step_name) in
                (&entities, &steps, &transforms, &names).join()
            {
                let pushness = calculate_pushness(&thing, &thing_transform, &step, &step_transform);
                warn!(
                    "pushness with {} is {}",
                    step_name.name.to_string(),
                    pushness
                );
                if pushness > max_pushness {
                    maybe_push_velocity = Some(step.x_velocity);
                    max_pushness = pushness;
                    maybe_push_name = Some(step_name.name.to_string());
                }
            }

            thing_push.pusher_name = maybe_push_name;
            thing_push.x_velocity = maybe_push_velocity.unwrap_or(0.);
        }
    }
}

fn calculate_pushness(
    thing: &Thing,
    thing_transform: &Transform,
    step: &Step,
    step_transform: &Transform,
) -> f32 {
    let step_bounds = BoundsProvider::new(step.width, step.height, step_transform);
    let thing_bounds = BoundsProvider::new(thing.width, thing.height, thing_transform);

    if !overlaps(
        step_bounds.left(),
        step_bounds.right(),
        thing_bounds.left(),
        thing_bounds.right(),
    ) {
        warn!("no left right overlap");
        return 0.;
    }

    if !overlaps(
        step_bounds.bottom(),
        step_bounds.top(),
        thing_bounds.bottom(),
        thing_bounds.top(),
    ) {
        warn!("no top bottom overlap");
        return 0.;
    }

    let overlap = f32::min(
        step_bounds.top() - thing_bounds.bottom(),
        thing_bounds.top() - step_bounds.bottom(),
    );
    warn!("overlap: {}", overlap);
    return overlap;
}

fn overlaps(a: f32, b: f32, x: f32, y: f32) -> bool {
    return (a <= x && b >= x) || (x <= a && y >= a);
}
