// use crate::{
//     components::{Color, Platform, Rectangle},
//     levels::PlatformConfig,
// };
// use amethyst::{
//     core::transform::Transform, prelude::*, renderer::resources::Tint, renderer::SpriteRender,
// };

// pub fn initialize_platform(
//     world: &mut World,
//     platform_config: PlatformConfig,
//     step_sprite: SpriteRender,
// ) {
//     let mut transform = Transform::default();
//     transform.set_translation_xyz(platform_config.x, platform_config.y, 0.);

//     world
//         .create_entity()
//         .with(Platform::default())
//         .with(Rectangle::new(
//             platform_config.width,
//             platform_config.height,
//         ))
//         .with(transform)
//         .with(step_sprite)
//         .with(Color::new(platform_config.color_flag))
//         .with(Tint(platform_config.color_flag.to_srgba()))
//         .build();
// }
