use crate::{
    components::{
        Atop, BaseEntity, Escalator, GridLocation, Platform, Rectangle, Step, Thing, Velocity,
    },
    resources::RewindableClock,
    utils::{is_atop, BoundingBox},
};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct AtopSystem;

impl<'s> System<'s> for AtopSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, GridLocation>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Platform>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Atop>,
    );

    fn run(
        &mut self,
        (
            entities,
            clock,
            things,
            escalators,
            grid_locations,
            steps,
            platforms,
            rectangles,
            mut velocities,
            mut atops,
        ): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }
        for (_thing, thing_entity, thing_grid_location, thing_rectangle, thing_atop) in
            (&things, &entities, &grid_locations, &rectangles, &mut atops).join()
        {
            thing_atop.bases.clear();
            let thing_bounds = BoundingBox::new(thing_rectangle, thing_grid_location);
            for (_step, step_entity, step_grid_location, step_rectangle, step_velocity) in
                (&steps, &entities, &grid_locations, &rectangles, &velocities).join()
            {
                let step_bounds = BoundingBox::new(step_rectangle, step_grid_location);
                if is_atop(&thing_bounds, &step_bounds) {
                    thing_atop.bases.insert(BaseEntity::Step(step_entity));
                }
            }
            for (_platform, platform_entity, platform_grid_location, platform_rectangle) in
                (&platforms, &entities, &grid_locations, &rectangles).join()
            {
                let platform_bounds = BoundingBox::new(platform_rectangle, platform_grid_location);
                if is_atop(&thing_bounds, &platform_bounds) {
                    thing_atop
                        .bases
                        .insert(BaseEntity::Platform(platform_entity));
                }
            }
            for (
                other_thing,
                other_thing_entity,
                other_thing_grid_location,
                other_thing_rectangle,
            ) in (&things, &entities, &grid_locations, &rectangles).join()
            {
                let other_thing_bounds =
                    BoundingBox::new(other_thing_rectangle, other_thing_grid_location);
                if is_atop(&thing_bounds, &other_thing_bounds) {
                    thing_atop
                        .bases
                        .insert(BaseEntity::Thing(other_thing_entity));
                }
            }
            info!("Thing {:?} is atop {:?}", thing_entity, thing_atop.bases);
        }

        for (escalator, escalator_atop) in (&escalators, &mut atops).join() {
            escalator_atop.bases.clear();
        }

        for (step, step_entity, step_grid_location, step_rectangle, step_velocity) in
            (&steps, &entities, &grid_locations, &rectangles, &velocities).join()
        {
            let step_bounds = BoundingBox::new(step_rectangle, step_grid_location);
            let mut escalator_atop = atops.get_mut(step.escalator).unwrap();

            for (_platform, platform_entity, platform_grid_location, platform_rectangle) in
                (&platforms, &entities, &grid_locations, &rectangles).join()
            {
                let platform_bounds = BoundingBox::new(platform_rectangle, platform_grid_location);
                if is_atop(&step_bounds, &platform_bounds) {
                    escalator_atop
                        .bases
                        .insert(BaseEntity::Platform(platform_entity));
                }
            }

            info!(
                "Escalator {:?} is atop {:?}",
                step.escalator, escalator_atop.bases
            );
        }

        // for (
        //     escalator,
        //     escalator_entity,
        //     escalator_grid_location,
        //     escalator_rectangle,
        //     escalator_atop,
        // ) in (
        //     &escalators,
        //     &entities,
        //     &grid_locations,
        //     &rectangles,
        //     &mut atops,
        // )
        //     .join()
        // {
        //     escalator_atop.bases.clear();

        //     let escalator_bounds = BoundingBox::new(escalator_rectangle, escalator_grid_location);

        //     for (_platform, platform_entity, platform_grid_location, platform_rectangle) in
        //         (&platforms, &entities, &grid_locations, &rectangles).join()
        //     {
        //         let platform_bounds = BoundingBox::new(platform_rectangle, platform_grid_location);
        //         if is_atop(&escalator_bounds, &platform_bounds) {
        //             escalator_atop
        //                 .bases
        //                 .insert(BaseEntity::Platform(platform_entity));
        //         }
        //     }

        //     for (_step, step_entity, step_grid_location, step_rectangle, step_velocity) in
        //         (&steps, &entities, &grid_locations, &rectangles, &velocities).join()
        //     {
        //         let step_bounds = BoundingBox::new(step_rectangle, step_grid_location);

        //         if is_atop(&escalator_bounds, &step_bounds) {
        //             escalator_atop.bases.insert(BaseEntity::Step(step_entity));
        //         }
        //     }
        //     info!(
        //         "Escalator {:?} is atop {:?}",
        //         escalator_entity, escalator_atop.bases
        //     );
        // }
    }
}
