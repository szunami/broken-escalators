use crate::components::Thing;
use crate::{components::Atop, components::Step, utils::BoundsProvider};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

pub const GRAVITY_VELOCITY: f32 = -5.;

#[derive(SystemDesc)]
pub struct AtopSystem;

impl<'s> System<'s> for AtopSystem {
    type SystemData = (
        WriteStorage<'s, Atop>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Step>,
    );

    fn run(&mut self, (mut atops, things, transforms, steps): Self::SystemData) {
        for (thing_atop, thing, thing_transform) in (&mut atops, &things, &transforms).join() {
            warn!("Calculating atopness");
            let mut atop = None;
            let mut max_atopness = 0.;
            for (step, step_transform) in (&steps, &transforms).join() {
                let atopness = calculate_atopness(&thing, &thing_transform, &step, &step_transform);
                info!("Atopness: {}", atopness);
                warn!("Atopness: {}", atopness);
                if atopness > max_atopness {
                    atop = Some(&(*step).clone());
                    max_atopness = atopness;
                }
            }

            match atop {
                Some(step) => {
                    match step.push_velocity != 0. {
                        true => thing_atop.x_velocity = step.push_velocity,
                        false => thing_atop.x_velocity = step.x_velocity,
                    }
                    thing_atop.y_velocity = step.y_velocity;
                }
                None => {
                    thing_atop.x_velocity = 0.;
                    thing_atop.y_velocity = GRAVITY_VELOCITY;
                }
            }
        }
    }
}

fn calculate_atopness(
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
        return 0.;
    }

    if !overlaps(
        step_bounds.bottom(),
        step_bounds.top(),
        thing_bounds.bottom(),
        thing_bounds.top(),
    ) {
        return 0.;
    }

    f32::min(
        step_bounds.right() - thing_bounds.left(),
        thing_bounds.right() - step_bounds.left(),
    )
}

fn overlaps(a: f32, b: f32, x: f32, y: f32) -> bool {
    (a <= x && b >= x) || (x <= a && y >= a)
}
