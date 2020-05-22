use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entities, ReadStorage, Join, Read, System, SystemData, WriteStorage},
};
use crate::{
    components::{Thing, Step, Atop, Velocity, BaseEntity},
    resources::RewindableClock
};

#[derive(SystemDesc)]
pub struct AbsoluteThingVelocity;

impl<'s> System<'s> for AbsoluteThingVelocity {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Atop>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (entities, clock, things, steps, atops, mut velocities): Self::SystemData) {
        if !clock.going_forwards() {
            return;
        }
        for (thing, thing_entity, thing_atop) in (&things, &entities, &atops).join() {
            let mut x = 0;
            let mut y = 0;
            if (!thing_atop.bases.is_empty()) {
                // this should max over all entities
                let maybe_base_entity = thing_atop.bases.iter().next();
                // this should check atop of base_entity and so on
                // also need to make sure that absolute velocity of the base has been updated first
                let maybe_base_velocity = maybe_base_entity.map(|base_entity| {
                    match base_entity {
                        BaseEntity::Step(entity) => velocities.get(*entity).unwrap().clone()
                    }
                });
                if let Some(base_velocity) = maybe_base_velocity {
                    x += base_velocity.absolute[0];
                    y += base_velocity.absolute[1];
                }
            }
            let mut thing_velocity = velocities.get_mut(thing_entity).unwrap();
            thing_velocity.absolute[0] = x;
            thing_velocity.absolute[1] = y;
            info!("Thing velocity: {:?}", thing_velocity);
        }
    }
}