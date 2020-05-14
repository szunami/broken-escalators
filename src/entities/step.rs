use crate::{
    components::{Color, Rectangle, Step, StepTape, Velocity},
    utils::ColorFlag,
};
use amethyst::{
    core::transform::Transform, ecs::Entity, prelude::*, renderer::resources::Tint,
    renderer::SpriteRender,
};

pub fn initialize_step(
    world: &mut World,
    escalator_entity: Entity,
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    push_velocity: f32,
    step_width: f32,
    step_height: f32,
    step_render: SpriteRender,
    color: ColorFlag,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);
    info!("Color: {:?}", color);
    let tint = Tint(color.to_srgba());
    info!("Tint: {:?}", tint);

    world
        .create_entity()
        .with(Step::new(escalator_entity, push_velocity))
        .with(Velocity::new(x_velocity, y_velocity))
        .with(Rectangle::default(step_width, step_height))
        .with(step_render)
        .with(StepTape::new())
        .with(transform)
        .with(Color::new(color))
        .with(tint)
        .build();
}
