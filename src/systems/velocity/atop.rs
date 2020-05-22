use crate::{
    components::{Atop, BaseEntity, GridLocation, Platform, Rectangle, Step, Thing, Velocity},
    resources::RewindableClock,
    utils::{is_atop, BoundingBox},
};
use amethyst::{
    derive::SystemDesc,
    ecs::{
        prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
        Entity,
    },
};

#[derive(SystemDesc)]
pub struct AtopSystem;

impl<'s> System<'s> for AtopSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
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
        // for each atop
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
    }
}
