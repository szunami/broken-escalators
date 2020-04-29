use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use crate::components::{Atop, Thing};

pub fn initialize_thing(world: &mut World, x: f32, y: f32, width: f32, height: f32, x_velocity: f32, y_velocity: f32, sprite_render: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);
    world
        .create_entity()
        .with(Thing::new(width, height, x_velocity, y_velocity))
        .with(Atop::default())
        .with(sprite_render.clone())
        .with(transform.clone())
        .build();
}