use crate::{
    components::{Atop, BaseEntity, Step, Thing, Escalator, Velocity},
    resources::RewindableClock,
};
use amethyst::core::math::Vector3;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use crate::systems::absolute_thing_velocity::velocity;
#[derive(SystemDesc)]
pub struct AbsoluteEscalatorVelocitySystem;

impl<'s> System<'s> for AbsoluteEscalatorVelocitySystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Atop>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (entities, clock, escalators, atops, mut velocities): Self::SystemData) {
        if !clock.going_forwards() {
            return;
        }
        for (escalator, escalator_entity, escalator_atop) in (&escalators, &entities, &atops).join() {
            let absolute_velocity = velocity(&escalator_atop, &atops, &velocities);
            let mut escalator_velocity = velocities.get_mut(escalator_entity).unwrap();
            escalator_velocity.absolute = absolute_velocity;
            info!("Escalator velocity: {:?}", escalator_velocity);
        }
    }
}
