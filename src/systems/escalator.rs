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
        // Write<'s, ScoreBoard>,
        // ReadExpect<'s, ScoreText>,
    );

    fn run(
        &mut self,
        (
            steps,
            mut locals,
            escalators,
        ): Self::SystemData,
    ) {

        for (step, step_local) in (&steps, &mut locals).join() {
            for (escalator, escalator_local) in (&escalators, &locals).join() {
                let distance = escalator.speed;

                if step_local.translation().x - step.width / 2. <= escalator_local.translation().x - escalator.width / 2. {
                    step_local.set_translation_x(escalator_local.translation().x - escalator.width / 2. + step.width);
                    step_local.prepend_translation_y(distance);
                    continue;
                }
                if step_local.translation().y - step.height / 2. <= escalator_local.translation().y - escalator.height / 2. {
                    step_local.set_translation_y(escalator_local.translation().y - escalator.height / 2. + step.height);
                    step_local.prepend_translation_x(-distance);
                    continue
                }
                step_local.prepend_translation_x(distance);
                step_local.prepend_translation_y(-distance);
           }
        }
    }
}
