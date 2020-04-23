use crate::components::{Escalator, Step};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, Write, WriteStorage},
};
#[derive(SystemDesc)]
pub struct CornerSystem;

impl<'s> System<'s> for CornerSystem {
    type SystemData = (
        WriteStorage<'s, Step>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Escalator>,
    );

    fn run(&mut self, (mut steps, locals, escalators): Self::SystemData) {
        for (step, step_local) in (&mut steps, &locals).join() {
            for (escalator, escalator_local) in (&escalators, &locals).join() {
                if step_local.translation().y + step.height * 0.5
                    >= escalator_local.translation().y + escalator.height * 0.5
                {
                    println!("Hit top");
                    step.x_velocity = escalator.speed;
                    step.y_velocity = -escalator.speed;
                    continue;
                }
                if step_local.translation().x + step.width * 0.5
                    >= escalator_local.translation().x + escalator.width * 0.5
                {
                    println!("Hit right corner");
                    step.x_velocity = -escalator.speed;
                    step.y_velocity = 0.;
                    continue;
                }
                if step_local.translation().x - step.width * 0.5
                    <= escalator_local.translation().x - escalator.width * 0.5
                {
                    println!("Hit middle corner");
                    step.x_velocity = 0.;
                    step.y_velocity = escalator.speed;
                }
            }
        }
    }
}
