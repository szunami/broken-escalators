use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    core::timing::Time,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
#[derive(Default)]
pub struct BrokenEscalators {}

pub struct Step {
    pub width: f32,
    pub height: f32,
}

impl Component for Step {
    type Storage = DenseVecStorage<Self>;
}

impl Step {
    fn new(width: f32, height: f32) -> Step {
        Step {
            width,
            height
        }
    }
}

impl SimpleState for BrokenEscalators {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);


        let sprite_sheet = load_sprite_sheet(world);
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet,
            sprite_number: 0,
        };

        let mut transform = Transform::default();

        transform.set_translation_xyz(0., 0., 0.);

        world
        .create_entity()
        .with(Step::new(100., 100.))
        .with(sprite_render.clone())
        .with(transform)
        .build();
    }
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
            "texture/yellow_box.png",
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