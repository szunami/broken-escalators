use crate::{
    components::{Color, Rectangle, Side, Step, StepTape, Velocity},
    levels::ColorFlag,
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
    side: Side,
    step_width: f32,
    step_height: f32,
    step_render: SpriteRender,
    color: ColorFlag,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    world
        .create_entity()
        .with(Step::new(escalator_entity, side))
        .with(Velocity::new(x_velocity, y_velocity))
        .with(Rectangle::default(step_width, step_height))
        .with(step_render)
        .with(StepTape::new())
        .with(transform)
        .with(Color::new(color))
        .with(Tint(color.to_srgba()))
        .build();
}
