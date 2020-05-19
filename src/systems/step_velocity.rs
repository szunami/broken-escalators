use crate::components::{Escalator, Side, Step, Velocity};
use crate::resources::RewindableClock;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct StepVelocitySystem;

impl<'s> System<'s> for StepVelocitySystem {
    type SystemData = (
        ReadStorage<'s, Step>,
        ReadStorage<'s, Escalator>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (steps, escalators, mut velocities): Self::SystemData) {
        for (step, step_velocity) in (&steps, &mut velocities).join() {
            let escalator = escalators.get(step.escalator).unwrap();
            step_velocity.x = x_velocity_for_side(&step.side, &escalator);
            step_velocity.y = y_velocity_for_side(&step.side, &escalator);
            info!("step_velocity: {:?}", step_velocity);
        }
    }
}

pub fn x_velocity_for_side(side: &Side, escalator: &Escalator) -> i32 {
    escalator.speed * escalator.direction.direction_factor() * side.base_x_component()
}

pub fn y_velocity_for_side(side: &Side, escalator: &Escalator) -> i32 {
    escalator.speed * escalator.direction.direction_factor() * side.base_y_component()
}
