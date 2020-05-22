use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, ReadStorage, WriteStorage},
};
use crate::{
    resources::RewindableClock,
    components::{Step, Velocity}
};


#[derive(SystemDesc)]
pub struct AbsoluteStepVelocitySystem;

impl<'s> System<'s> for AbsoluteStepVelocitySystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        ReadStorage<'s, Step>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (clock, steps, mut velocities): Self::SystemData) {
        // for each step, it's absolute velocity = it's intrinsic velocity + escalator's velocity + whatever is down the chain
        // for now, just intrinsic
        if !clock.going_forwards() {
            return;
        }
        for (step, step_velocity) in (&steps, &mut velocities).join() {
            step_velocity.absolute[0] = step_velocity.intrinsic[0];
            step_velocity.absolute[1] = step_velocity.intrinsic[1];
            info!("Step velocity: {:?}", step_velocity);
        }
    }
}