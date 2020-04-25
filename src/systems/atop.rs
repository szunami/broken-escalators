use crate::components::Thing;
use crate::{components::Step, components::Atop, utils::BoundsProvider};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
};

pub const GRAVITY_VELOCITY: f32 = -5.;

#[derive(SystemDesc)]
pub struct AtopSystem;

impl<'s> System<'s> for AtopSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Atop>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Step>,
    );

    fn run(&mut self, (
        entities, mut atops, things, transforms, steps
    ): Self::SystemData) {
        for (thing_entity, thing_atop, thing, thing_transform) in (&entities, &mut atops, &things, &transforms).join() {
            let mut max_atopness = 0.;
            let mut atop = None;

            for (step_entity, step, step_transform) in (&entities, &steps, &transforms).join() {
                let atopness = calculate_atopness(&thing, &thing_transform, &step, &step_transform);
                if atopness > max_atopness {
                    atop = Some([step.x_velocity, step.y_velocity]);
                    max_atopness = atopness;
                }
            }

            // should need to match / account for gravity
            match atop {
                Some(vec) => {
                    warn!("Atop something.");
                    thing_atop.x_velocity = vec[0];
                    thing_atop.y_velocity = vec[1];
                },
                None => {
                    warn!("Atop nothing.");
                    thing_atop.x_velocity = 0.;
                    thing_atop.y_velocity = GRAVITY_VELOCITY;
                }
            }
        }
    }
}

fn calculate_atopness(thing: &Thing, thing_transform: &Transform, step: &Step, step_transform: &Transform) -> f32 {
    let step_bounds = BoundsProvider::new(step.width, step.height, step_transform);
    let thing_bounds = BoundsProvider::new(thing.width, thing.height, thing_transform);

    if !overlaps(step_bounds.left(), step_bounds.right(), thing_bounds.left(), thing_bounds.right()) {
        warn!("no left right overlap");
        return 0.;
    }

    if !overlaps(step_bounds.bottom(), step_bounds.top(), thing_bounds.bottom(), thing_bounds.top()) {
        warn!("no top bottom overlap");
        return 0.;
    }

    let overlap =  f32::min(step_bounds.right() - thing_bounds.left(), thing_bounds.right() - step_bounds.left());
    warn!("overlap: {}", overlap);
    return overlap;
}

fn overlaps(a: f32, b: f32, x: f32, y: f32) -> bool {
    return (a <= x && b >= x) || (x <= a && y >= a);
}
