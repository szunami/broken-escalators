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

pub const GRAVITY_VELOCITY: i32 = -1;

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
        for (_thing, thing_entity, thing_atop) in (&things, &entities, &atops).join() {
            let absolute_velocity = velocity(&thing_atop, &atops, &steps, &velocities);
            let mut thing_velocity = velocities.get_mut(thing_entity).unwrap();
            thing_velocity.absolute = absolute_velocity;
            debug!("Thing velocity: {:?}", thing_velocity);
        }
    }
}

// iterate down chain of atops
pub fn velocity<'s>(
    atop: &Atop,
    atops: &ReadStorage<'s, Atop>,
    steps: &ReadStorage<'s, Step>,
    velocities: &WriteStorage<'s, Velocity>,
) -> Vector3<i32> {
    let atop_velocities: Vec<Vector3<i32>> = atop
        .bases
        .iter()
        .map(|base_entity| {
            match base_entity {
                BaseEntity::Step(entity) => {
                    // recursion here
                    let step = steps.get(*entity).unwrap();
                    let escalator_atop = atops.get(step.escalator).unwrap();
                    let step_velocity = velocities.get(*entity).unwrap().intrinsic;
                    let inherited_velocity = velocity(&escalator_atop, atops, steps, velocities);
                    step_velocity + inherited_velocity
                }
                BaseEntity::Platform(_) => Vector3::new(0, 0, 0),
                BaseEntity::Thing(entity) => {
                    // recursion here
                    let thing_atop = atops.get(*entity).unwrap();
                    velocity(&thing_atop, atops, steps, velocities)
                }
            }
        })
        .collect();

    *atop_velocities
        .iter()
        .max_by_key(|velocity| velocity[1])
        .unwrap_or(&Vector3::new(0, GRAVITY_VELOCITY, 0))
}
