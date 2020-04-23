use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{Step, Escalator};
#[derive(SystemDesc)]
pub struct EscalatorSystem;

impl EscalatorSystem {
    fn escalator_bounds(
        locals: &WriteStorage<Transform>,
        escalators: &ReadStorage<Escalator>,
    ) -> (f32, f32, f32, f32) {
        for (escalator, escalator_local) in (escalators, locals).join() {
            return (
                escalator_local.translation().y + escalator.height * 0.5,
                escalator_local.translation().y - escalator.height * 0.5,
                escalator_local.translation().x - escalator.width * 0.5,
                escalator_local.translation().x + escalator.width * 0.5,
            );
        }
        panic!("Unable to find escalator");
    }
}

impl<'s> System<'s> for EscalatorSystem {
    type SystemData = (
        ReadStorage<'s, Step>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Escalator>,
    );

    fn run(
        &mut self,
        (
            steps,
            mut locals,
            escalators
        ): Self::SystemData,
    ) {
        let (top, bottom, left, right) = EscalatorSystem::escalator_bounds(&locals, &escalators);
        for (step, step_local) in (&steps, &mut locals).join() {
            let x = (step_local.translation().x + step.x_velocity)
                .max(left + step.width * 0.5)
                .min(right - step.width * 0.5);
            step_local.set_translation_x(x);

            let y = (step_local.translation().y + step.y_velocity)
                .max(bottom + step.height * 0.5)
                .min(top - step.height * 0.5);

            step_local.set_translation_y(y);
            println!("{}, {}", step_local.translation().x, step_local.translation().y);
        }
    }
}


