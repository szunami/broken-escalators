use crate::{
    components::{Step, Velocity},
    resources::RewindableClock,
};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Entities, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct AbsoluteStepVelocitySystem;

impl<'s> System<'s> for AbsoluteStepVelocitySystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Step>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (entities, clock, steps, mut velocities): Self::SystemData) {
        // for each step, it's absolute velocity = it's intrinsic velocity + escalator's velocity + whatever is down the chain
        // for now, just intrinsic
        if !clock.going_forwards() {
            return;
        }
        for (step, step_entity) in (&steps, &entities).join() {
            let escalator_velocity = velocities.get(step.escalator).unwrap().clone();
            let mut step_velocity = velocities.get_mut(step_entity).unwrap();
            step_velocity.absolute[0] = escalator_velocity.absolute[0] + step_velocity.intrinsic[0];
            step_velocity.absolute[1] = escalator_velocity.absolute[1] + step_velocity.intrinsic[1];
            info!("Step velocity: {:?}", step_velocity);
        }
    }
}
