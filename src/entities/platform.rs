use crate::{
    components::{Color, Platform, Rectangle, GridLocation},
    levels::PlatformConfig, utils::grid_coordinate_to_transform_coordinate,
};
use amethyst::{
    core::transform::Transform, prelude::*, renderer::resources::Tint, renderer::SpriteRender,
};
use amethyst::core::math::Vector3;


pub fn initialize_platform(
    world: &mut World,
    platform_config: PlatformConfig,
    step_sprite: SpriteRender,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        grid_coordinate_to_transform_coordinate(platform_config.x),
        grid_coordinate_to_transform_coordinate(platform_config.y),
        0.,
    );
    transform.set_scale(Vector3::new(
        platform_config.width as f32,
        platform_config.height as f32,
        1.,
    ));
    world
        .create_entity()
        .with(Platform::default())
        .with(GridLocation::new(platform_config.x, platform_config.y))
        .with(Rectangle::new(
            platform_config.width,
            platform_config.height,
        ))
        .with(transform)
        .with(step_sprite)
        .with(Color::new(platform_config.color_flag))
        .with(Tint(platform_config.color_flag.to_srgba()))
        .build();
}
