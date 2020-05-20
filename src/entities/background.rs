use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    input::{is_close_requested, is_key_down, InputHandler, StringBindings, VirtualKeyCode},
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
core::transform::Transform, prelude::*};

pub fn initialize_background(
    world: &mut World,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(512., 512., -1.);

    let sprite_sheet = load_grid_sprite_sheet(world);
    let grid_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(grid_render)
        .build();
}

fn load_grid_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/grid.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/grid_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}