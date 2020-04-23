use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};
use std::collections::HashMap;


use crate::components::{Escalator, Step};
#[derive(SystemDesc)]
pub struct EscalatorSystem;

struct Box {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32
}

impl EscalatorSystem {
    fn escalator_bounds(
        locals: &WriteStorage<Transform>,
        escalators: &ReadStorage<Escalator>,
    ) -> HashMap<i32, Box> {
        let mut escalator_map = HashMap::new();

        for (escalator, escalator_local) in (escalators, locals).join() {
            escalator_map.insert(escalator.id,
            Box {
                left: escalator_local.translation().x - escalator.width * 0.5,
                right: escalator_local.translation().x + escalator.width * 0.5,
                bottom: escalator_local.translation().y - escalator.height * 0.5,
                top: escalator_local.translation().y + escalator.height * 0.5,
            });
        }
        return escalator_map;
    }
}

impl<'s> System<'s> for EscalatorSystem {
    type SystemData = (
        ReadStorage<'s, Step>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Escalator>,
    );

    fn run(&mut self, (steps, mut locals, escalators): Self::SystemData) {
        let escalator_map = EscalatorSystem::escalator_bounds(&locals, &escalators);
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
            println!(
                "{}, {}",
                step_local.translation().x,
                step_local.translation().y
            );
        }
    }
}
