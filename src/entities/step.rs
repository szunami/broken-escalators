use crate::{
    components::{Color, GridLocation, Rectangle, Side, Step, StepTape, Velocity},
    levels::ColorFlag,
    utils::grid_coordinate_to_transform_coordinate,
};
use amethyst::core::math::Vector3;
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
    transform.set_translation_xyz(
        grid_coordinate_to_transform_coordinate(x),
        grid_coordinate_to_transform_coordinate(y),
        0.,
    );
    transform.set_scale(Vector3::new(step_width as f32, step_height as f32, 1.));

    debug!("Registering step at {:?}", (x, y));
    debug!("With velocity: {:?}", (x_velocity, y_velocity));

    world
        .create_entity()
        .with(Step::new(escalator_entity, side))
        .with(GridLocation::new(x, y))
        .with(Velocity::new(x_velocity, y_velocity))
        .with(Rectangle::new(step_width, step_height))
        .with(step_render)
        .with(StepTape::new())
        .with(transform)
        .with(Color::new(color))
        .with(Tint(color.to_srgba()))
        .build();
}
