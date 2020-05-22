use crate::{
    components::{Atop, BaseEntity, Step, Thing, Velocity},
    resources::RewindableClock,
};
use amethyst::core::math::Vector3;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
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
            let absolute_velocity = velocity(&thing_atop, &atops, &velocities);
            let mut thing_velocity = velocities.get_mut(thing_entity).unwrap();
            thing_velocity.absolute = absolute_velocity;
            info!("Thing velocity: {:?}", thing_velocity);
        }
    }
}

// iterate down chain of atops
fn velocity<'s>(
    atop: &Atop,
    atops: &ReadStorage<'s, Atop>,
    velocities: &WriteStorage<'s, Velocity>,
) -> Vector3<i32> {
    let mut atop_velocities: Vec<Vector3<i32>> = atop
        .bases
        .iter()
        .map(|base_entity| {
            match base_entity {
                BaseEntity::Step(entity) => {
                    // recursion here
                    // let step_atop = atops.get(*entity).unwrap();
                    // velocity(&step_atop, atops, velocities)
                    velocities.get(*entity).unwrap().absolute
                }
            }
        })
        .collect();
    atop_velocities.push(Vector3::new(0, -1, 0));
    let z = *atop_velocities
        .iter()
        .max_by_key(|velocity| velocity[1])
        .unwrap();
    return z;
}
