use crate::{
    components::Platform,
    components::Rectangle,
    components::Step,
    components::Thing,
    components::Velocity,
    utils::{is_atop, BoundingBox},
};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
    ecs::Entity,
};
use std::collections::HashMap;

pub const GRAVITY_VELOCITY: f32 = -50.;

#[derive(SystemDesc)]
pub struct AtopSystem;

impl<'s> System<'s> for AtopSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Platform>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, Velocity>,
    );

    fn run(
        &mut self,
        (entities, things, transforms, steps, platforms, rectangles, mut velocities): Self::SystemData,
    ) {
        let mut step_velocity_map: HashMap<Entity, Velocity> = HashMap::new();
        for (step_entity, step, step_velocity) in (&entities, &steps, &velocities).join() {
            if step.push_velocity == 0. {
                step_velocity_map.insert(step_entity, step_velocity.clone());
            } else {
                step_velocity_map.insert(
                    step_entity,
                    Velocity::new(step.push_velocity, step_velocity.y),
                );
            }
        }

        for (_thing, thing_transform, thing_rectangle, thing_velocity) in
            (&things, &transforms, &rectangles, &mut velocities).join()
        {
            let thing_bounds = BoundingBox::new(thing_rectangle, thing_transform);

            let mut atop_step: Option<Entity> = None;
            let mut atop_platform = false;
            let mut max_atopness = 0.;
            for (_step, step_entity, step_transform, step_rectangle) in
                (&steps, &entities, &transforms, &rectangles).join()
            {
                let step_bounds = BoundingBox::new(step_rectangle, step_transform);
                let atopness = is_atop(&thing_bounds, &step_bounds);
                if atopness && step_bounds.top > max_atopness {
                    atop_step = Some(step_entity);
                    max_atopness = step_bounds.top;
                }
            }

            for (_platform, platform_transform, platform_rectangle) in
                (&platforms, &transforms, &rectangles).join()
            {
                let platform_bounds = BoundingBox::new(platform_rectangle, platform_transform);
                let atopness = is_atop(&thing_bounds, &platform_bounds);
                if atopness && platform_bounds.top > max_atopness {
                    atop_step = None;
                    atop_platform = true;
                    max_atopness = platform_bounds.top;
                }
            }

            if let Some(step_entity) = atop_step {
                let step_velocity = step_velocity_map.get(&step_entity).unwrap();
                *thing_velocity = step_velocity.clone();
            } else if atop_platform {
                thing_velocity.x = 0.;
                thing_velocity.y = 0.;
            } else {
                thing_velocity.x = 0.;
                thing_velocity.y = GRAVITY_VELOCITY;
            }
        }
    }
}
