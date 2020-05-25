use crate::systems::absolute_thing_velocity::velocity;
use crate::{
    components::{Atop, Escalator, Step, Velocity},
    resources::RewindableClock,
};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
#[derive(SystemDesc)]
pub struct AbsoluteEscalatorVelocitySystem;

impl<'s> System<'s> for AbsoluteEscalatorVelocitySystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Atop>,
        WriteStorage<'s, Velocity>,
    );

    fn run(
        &mut self,
        (entities, clock, escalators, steps, atops, mut velocities): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }
        for (_escalator, escalator_entity, escalator_atop) in (&escalators, &entities, &atops).join()
        {
            let absolute_velocity = velocity(&escalator_atop, &atops, &steps, &velocities);
            let mut escalator_velocity = velocities.get_mut(escalator_entity).unwrap();
            escalator_velocity.absolute = absolute_velocity;
            info!("Escalator velocity: {:?}", escalator_velocity);
        }
    }
}
