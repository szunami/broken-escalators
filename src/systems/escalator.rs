use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, System, SystemData, World, Write, ReadStorage, WriteStorage},
    ui::UiText,
};
use crate::broken_escalators::Step;


#[derive(SystemDesc)]
pub struct EscalatorSystem;

impl<'s> System<'s> for EscalatorSystem {
    type SystemData = (
        ReadStorage<'s, Step>,
        WriteStorage<'s, Transform>,
        // WriteStorage<'s, UiText>,
        // Write<'s, ScoreBoard>,
        // ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (
        steps,
        mut locals,
        // mut ui_text,
        // mut scores,
        // score_text
    ): Self::SystemData) {

        for (step, transform) in (&steps, &mut locals).join() {
            transform.prepend_translation_x(1.);
        }
    }
}