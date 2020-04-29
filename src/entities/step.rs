use crate::components::Step;
use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::SpriteRender,
};

pub fn initialize_step(
    world: &mut World,
    escalator_id: i32,
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    push_velocity: f32,
    step_width: f32,
    step_height: f32,
    step_render: SpriteRender,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);
    world
        .create_entity()
        .with(Step::new(
            escalator_id,
            step_width,
            step_height,
            x_velocity,
            y_velocity,
            push_velocity,
        ))
        .with(step_render.clone())
        .with(transform.clone())
        .build();
}
