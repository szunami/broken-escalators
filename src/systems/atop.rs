use crate::{
    components::Atop,
    components::Platform,
    components::Step,
    components::Thing,
    components::Rectangle,
    utils::{is_atop, BoundingBox},
};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

pub const GRAVITY_VELOCITY: f32 = -50.;

#[derive(SystemDesc)]
pub struct AtopSystem;

impl<'s> System<'s> for AtopSystem {
    type SystemData = (
        WriteStorage<'s, Atop>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Platform>,
        ReadStorage<'s, Rectangle>
    );

    fn run(&mut self, (mut atops, things, transforms, steps, platforms, rectangles): Self::SystemData) {
        for (thing_atop, _thing, thing_transform, thing_rectangle) in (&mut atops, &things, &transforms, &rectangles).join() {
            let thing_bounds = BoundingBox::new(thing_rectangle.width, thing_rectangle.height, thing_transform);

            let mut atop_step: Option<Step> = None;
            let mut atop_platform: Option<Platform> = None;
            let mut max_atopness = 0.;
            for (step, step_transform, step_rectangle) in (&steps, &transforms, &rectangles).join() {
                let step_bounds = BoundingBox::new(step_rectangle.width, step_rectangle.height, step_transform);
                let atopness = is_atop(&thing_bounds, &step_bounds);
                if atopness && step_bounds.top > max_atopness {
                    atop_step = Some(step.clone());
                    max_atopness = step_bounds.top;
                }
            }

            for (platform, platform_transform) in (&platforms, &transforms).join() {
                let platform_bounds =
                    BoundingBox::new(platform.width, platform.height, platform_transform);
                let atopness = is_atop(&thing_bounds, &platform_bounds);
                if atopness && platform_bounds.top > max_atopness {
                    atop_step = None;
                    atop_platform = Some(platform.clone());
                    max_atopness = platform_bounds.top;
                }
            }

            if let Some(step) = atop_step {
                match step.push_velocity != 0. {
                    true => thing_atop.x_velocity = step.push_velocity,
                    false => thing_atop.x_velocity = step.x_velocity,
                }
                thing_atop.y_velocity = step.y_velocity;
            } else if let Some(_platform) = atop_platform {
                thing_atop.x_velocity = 0.;
                thing_atop.y_velocity = 0.;
            } else {
                thing_atop.x_velocity = 0.;
                thing_atop.y_velocity = GRAVITY_VELOCITY;
            }
        }
    }
}
