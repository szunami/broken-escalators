// use crate::{
//     components::{Color, Rectangle, Thing, ThingTape, Velocity},
//     levels::ThingConfig,
// };
// use amethyst::{
//     core::transform::Transform, prelude::*, renderer::resources::Tint, renderer::SpriteRender,
// };

// pub fn initialize_thing(world: &mut World, thing_config: ThingConfig, sprite_render: SpriteRender) {
//     let mut transform = Transform::default();
//     transform.set_translation_xyz(thing_config.x, thing_config.y, 0.);
//     world
//         .create_entity()
//         .with(Thing::new())
//         .with(Velocity::default())
//         .with(Rectangle::new(thing_config.width, thing_config.height))
//         .with(ThingTape::new())
//         .with(Velocity::default())
//         .with(sprite_render)
//         .with(transform)
//         .with(Color::new(thing_config.color_flag))
//         .with(Tint(thing_config.color_flag.to_srgba()))
//         .build();
// }
