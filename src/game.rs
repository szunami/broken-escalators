use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components::{Atop, Direction, Escalator, Step, Thing};

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;
#[derive(Default)]
pub struct Game {}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Escalator>();
        world.register::<Thing>();
        initialise_camera(world);

        let sprite_sheet = load_sprite_sheet(world);
        let step_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };
        let thing_render = SpriteRender {
            sprite_sheet: sprite_sheet,
            sprite_number: 1,
        };

        initialize_escalators(world, step_render.clone());
        initialise_thing(world, thing_render.clone());
    }
}

fn initialize_escalators(world: &mut World, sprite_render: SpriteRender) {
    {
        let escalator_id = 0;

        let mut transform = Transform::default();
        // transform.set_translation_xyz(16., 16., 0.);
        // world
        //     .create_entity()
        //     .with(Step::new(escalator_id, 32., 32., 0., 5.))
        //     .with(sprite_render.clone())
        //     .with(transform.clone())
        //     .build();

        // transform.set_translation_xyz(48., 16., 0.);
        // world
        //     .create_entity()
        //     .with(Step::new(escalator_id, 32., 32., 5., -5.))
        //     .with(sprite_render.clone())
        //     .with(transform.clone())
        //     .build();

        transform.set_translation_xyz(16., 48., 0.);
        world
            .create_entity()
            .named("doofus")
            .with(Step::new(escalator_id, 32., 32., 0., 0.))
            .with(sprite_render.clone())
            .with(transform.clone())
            .build();

        transform.set_translation_xyz(32., 32., 0.);
        world
            .create_entity()
            .with(Escalator::new(
                escalator_id,
                64.,
                64.,
                1.,
                Direction::CLOCKWISE,
            ))
            .with(transform.clone())
            .build();
    }
    // {
    //     let escalator_id = 1;

    //     let mut transform = Transform::default();
    //     transform.set_translation_xyz(168., 16., 0.);
    //     world
    //         .create_entity()
    //         .with(Step::new(escalator_id, 32., 32., 0., 0.))
    //         .with(sprite_render.clone())
    //         .with(transform.clone())
    //         .build();

    //     transform.set_translation_xyz(200., 16., 0.);
    //     world
    //         .create_entity()
    //         .named("other")
    //         .with(Step::new(escalator_id, 32., 32., 0., 0.))
    //         .with(sprite_render.clone())
    //         .with(transform.clone())
    //         .build();

    //     transform.set_translation_xyz(232., 16., 0.);
    //     world
    //         .create_entity()
    //         .named("carrier")
    //         .with(Step::new(escalator_id, 32., 32., 0., 0.))
    //         .with(sprite_render.clone())
    //         .with(transform.clone())
    //         .build();

    //     transform.set_translation_xyz(168., 48., 0.);
    //     world
    //         .create_entity()
    //         .with(Step::new(escalator_id, 32., 32., 0., 0.))
    //         .with(sprite_render.clone())
    //         .with(transform.clone())
    //         .build();

    //     transform.set_translation_xyz(168., 80., 0.);
    //     world
    //         .create_entity()
    //         .with(Step::new(escalator_id, 32., 32., 0., 0.))
    //         .with(sprite_render.clone())
    //         .with(transform.clone())
    //         .build();

    //     transform.set_translation_xyz(200., 48., 0.);
    //     world
    //         .create_entity()
    //         .with(Step::new(escalator_id, 32., 32., 0., 0.))
    //         .with(sprite_render.clone())
    //         .with(transform.clone())
    //         .build();
    //     transform.set_translation_xyz(200., 48., 0.);
    //     world
    //         .create_entity()
    //         .with(Escalator::new(
    //             escalator_id,
    //             96.,
    //             96.,
    //             1.,
    //             Direction::COUNTERCLOCKWISE,
    //         ))
    //         .with(transform.clone())
    //         .build();
    // }
}

fn initialise_thing(world: &mut World, sprite_render: SpriteRender) {
    let mut transform = Transform::default();
    // transform.set_translation_xyz(232., 48., 0.);
    // world
    //     .create_entity()
    //     .with(Thing::new(32., 32., 0., 0.))
    //     .with(Atop::default())
    //     .with(sprite_render.clone())
    //     .with(transform.clone())
    //     .build();

    transform.set_translation_xyz(16., 80., 0.);
    world
        .create_entity()
        .with(Thing::new(32., 32., 0., 0.))
        .with(Atop::default())
        .with(sprite_render.clone())
        .with(transform.clone())
        .build();
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
