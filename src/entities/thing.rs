use crate::{
    components::{Atop, Color, GridLocation, Rectangle, Thing, ThingTape, Velocity},
    levels::ThingConfig,
    utils::grid_coordinate_to_transform_coordinate,
};
use amethyst::core::math::Vector3;
use amethyst::{
    core::transform::Transform, prelude::*, renderer::resources::Tint, renderer::SpriteRender,
};

pub fn initialize_thing(world: &mut World, thing_config: ThingConfig, sprite_render: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        grid_coordinate_to_transform_coordinate(thing_config.x),
        grid_coordinate_to_transform_coordinate(thing_config.y),
        0.,
    );
    transform.set_scale(Vector3::new(
        thing_config.width as f32,
        thing_config.height as f32,
        1.,
    ));

    world
        .create_entity()
        .with(Thing::new())
        .with(GridLocation::new(thing_config.x, thing_config.y))
        .with(Velocity::default())
        .with(Rectangle::new(thing_config.width, thing_config.height))
        .with(ThingTape::new())
        .with(Velocity::default())
        .with(Atop::default())
        .with(sprite_render)
        .with(transform)
        .with(Color::new(thing_config.color_flag))
        .with(Tint(thing_config.color_flag.to_srgba()))
        .build();
}
