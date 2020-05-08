use crate::components::{Rectangle, Thing, ThingTape, Velocity};
use amethyst::{core::transform::Transform, prelude::*, renderer::SpriteRender};

pub fn initialize_thing(
    world: &mut World,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    x_velocity: f32,
    y_velocity: f32,
    sprite_render: SpriteRender,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);
    world
        .create_entity()
        .with(Thing::new(x_velocity, y_velocity))
        .with(Rectangle::default(width, height))
        .with(ThingTape::new())
        .with(Velocity::default())
        .with(sprite_render)
        .with(transform)
        .build();
}
