use crate::{
    components::{Color, Rectangle, Side, Step, StepTape, Velocity, GridLocation},
    levels::ColorFlag,
};
use amethyst::{
    core::transform::Transform, ecs::Entity, prelude::*, renderer::resources::Tint,
    renderer::SpriteRender,
};

pub fn initialize_step(
    world: &mut World,
    escalator_entity: Entity,
    x: i32,
    y: i32,
    x_velocity: i32,
    y_velocity: i32,
    side: Side,
    step_width: i32,
    step_height: i32,
    step_render: SpriteRender,
    color: ColorFlag,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x as f32 * 32., y as f32 * 32., 0.);
    
    info!("Registering step at {:?}", (x, y));
    info!("With velocity: {:?}", (x_velocity, y_velocity));

    world
        .create_entity()
        .with(Step::new(escalator_entity, side))
        .with(GridLocation::new(x, y))
        .with(Velocity::new(x_velocity, y_velocity))
        .with(Rectangle::new(step_width, step_height))
        .with(step_render)
        // .with(StepTape::new())
        .with(transform)
        // .with(Color::new(color))
        // .with(Tint(color.to_srgba()))
        .build();
}
