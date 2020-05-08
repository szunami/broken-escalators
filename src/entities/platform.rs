use crate::components::Platform;
use amethyst::{core::transform::Transform, prelude::*, renderer::SpriteRender, core::math::Vector3};

pub fn initialize_platform(
    world: &mut World,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    step_sprite: SpriteRender,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    world
        .create_entity()
        .with(Platform::default(width, height))
        .with(transform)
        .with(step_sprite)
        .build();
}
