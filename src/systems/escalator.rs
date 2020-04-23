use crate::systems::utils::escalator_bounds_write;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{Escalator, Step};
#[derive(SystemDesc)]
pub struct EscalatorSystem;

impl<'s> System<'s> for EscalatorSystem {
    type SystemData = (
        ReadStorage<'s, Step>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Escalator>,
    );

    fn run(&mut self, (steps, mut locals, escalators): Self::SystemData) {
        let escalator_map = escalator_bounds_write(&locals, &escalators);
        for (step, step_local) in (&steps, &mut locals).join() {
            let escalator_box = escalator_map.get(&step.escalator_id).unwrap();
            let x = (step_local.translation().x + step.x_velocity)
                .max(escalator_box.left + step.width * 0.5)
                .min(escalator_box.right - step.width * 0.5);
            step_local.set_translation_x(x);

            let y = (step_local.translation().y + step.y_velocity)
                .max(escalator_box.bottom + step.height * 0.5)
                .min(escalator_box.top - step.height * 0.5);

            step_local.set_translation_y(y);
            info!(
                "{}, {}",
                step_local.translation().x,
                step_local.translation().y
            );
        }
    }
}
