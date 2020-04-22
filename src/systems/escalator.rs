use crate::broken_escalators::{Step, Escalator};
use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, ReadStorage, System, SystemData, World, Write, WriteStorage},
    ui::UiText,
};

#[derive(SystemDesc)]
pub struct EscalatorSystem;

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
        for (step, step_local) in (&steps, &mut locals).join() {
            for (escalator, escalator_local) in (&escalators, &locals).join() {
                step_local.prepend_translation_x(step.x_velocity);
                step_local.translation().x = step_local.translation().x
                    .max(escalator_local.translation().x - escalator.width * 0.5)
                    .min(escalator_local.translation().x + escalator.width * 0.5);
                
                step_local.prepend_translation_y(step.y_velocity);
                step_local.translation().y = step_local.translation().y
                    .max(escalator_local.translation().y - escalator.height * 0.5)
                    .min(escalator_local.translation().y + escalator.height* 0.5);




                println!("{}, {}", step_local.translation().x, step_local.translation().y)
            }
        }
    }
}
