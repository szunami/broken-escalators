use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    input::{InputHandler, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::{
    components::Direction,
    entities::{
        initialize_camera, initialize_clock, initialize_down_keys, initialize_escalator,
        initialize_thing,
    },
};

#[derive(Default)]
pub struct Game {}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        reset_level(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if data
            .world
            .fetch::<InputHandler<StringBindings>>()
            .key_is_down(VirtualKeyCode::R)
        {
            reset_level(data.world);
        }
        Trans::None
    }
}

fn reset_level(world: &mut World) {
    world.delete_all();
    initialize_camera(world);

    let sprite_sheet = load_sprite_sheet(world);
    let step_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    let thing_render = SpriteRender {
        sprite_sheet,
        sprite_number: 1,
    };

    let sprite_width = 32.;

    initialize_escalator(
        world,
        0,
        2. * sprite_width,
        2. * sprite_width,
        4. * sprite_width,
        4. * sprite_width,
        4,
        32.,
        Direction::CLOCKWISE,
        step_render.clone(),
    );
    initialize_thing(
        world,
        sprite_width * 0.5,
        4.5 * sprite_width,
        sprite_width,
        sprite_width,
        0.,
        0.,
        thing_render.clone(),
    );

    initialize_escalator(
        world,
        1,
        8. * sprite_width,
        2. * sprite_width,
        4. * sprite_width,
        4. * sprite_width,
        4,
        32.,
        Direction::COUNTERCLOCKWISE,
        step_render,
    );
    initialize_thing(
        world,
        sprite_width * 10.,
        4.5 * sprite_width,
        sprite_width,
        sprite_width,
        0.,
        0.,
        thing_render,
    );

    initialize_clock(world);
    initialize_down_keys(world);
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
