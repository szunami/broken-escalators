use crate::{
    components::GridLocation,
    components::Platform,
    components::Rectangle,
    components::Step,
    components::Thing,
    components::Velocity,
    resources::{DownKeys, RewindableClock},
    utils::{is_atop, BoundingBox},
};
use amethyst::input::VirtualKeyCode;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
    ecs::Entity,
};

pub const GRAVITY_VELOCITY: i32 = -1;

#[derive(SystemDesc)]
pub struct AtopSystem;

impl<'s> System<'s> for AtopSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, GridLocation>,
        WriteStorage<'s, Step>,
        ReadStorage<'s, Platform>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            clock,
            things,
            grid_locations,
            mut steps,
            platforms,
            rectangles,
            mut velocities,
        ): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }
        // for step in (&mut steps).join() {
        //     step.thing_atop = None;
        // }
        for (_thing, thing_entity, thing_grid_location, thing_rectangle) in
            (&things, &entities, &grid_locations, &rectangles).join()
        {
            let thing_bounds = BoundingBox::new(thing_rectangle, thing_grid_location);

            let mut atop_step: Option<Entity> = None;
            let mut atop_platform = false;
            let mut max_y_velocity = GRAVITY_VELOCITY;

            for (_step, step_entity, step_grid_location, step_rectangle, step_velocity) in
                (&steps, &entities, &grid_locations, &rectangles, &velocities).join()
            {
                let step_bounds = BoundingBox::new(step_rectangle, step_grid_location);
                let atopness = is_atop(&thing_bounds, &step_bounds);
                if atopness && step_velocity.y >= max_y_velocity {
                    atop_step = Some(step_entity);
                    max_y_velocity = step_velocity.y;
                }
            }

            for (_platform, platform_grid_location, platform_rectangle) in
                (&platforms, &grid_locations, &rectangles).join()
            {
                let platform_bounds = BoundingBox::new(platform_rectangle, platform_grid_location);
                let atopness = is_atop(&thing_bounds, &platform_bounds);
                if atopness && max_y_velocity <= 0 {
                    atop_step = None;
                    atop_platform = true;
                }
            }

            if let Some(step_entity) = atop_step {
                let step_velocity = velocities.get(step_entity).unwrap().clone();
                let thing_velocity = velocities.get_mut(thing_entity).unwrap();
                *thing_velocity = step_velocity.clone();
            // let step = steps.get_mut(step_entity).unwrap();
            // step.thing_atop = Some(thing_entity);
            } else if atop_platform {
                let thing_velocity = velocities.get_mut(thing_entity).unwrap();
                thing_velocity.x = 0;
                thing_velocity.y = 0;
            } else {
                info!("Not atop");
                let thing_velocity = velocities.get_mut(thing_entity).unwrap();
                thing_velocity.x = 0;
                thing_velocity.y = GRAVITY_VELOCITY;
            }
        }
    }
}
