use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    input::{is_close_requested, is_key_down, InputHandler, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::{
    components::Color,
    entities::{
        initialize_camera, initialize_clock, initialize_down_keys, initialize_escalator,
        initialize_platform, initialize_thing,
    },
    levels::LevelConfig,
};

#[derive(Default)]
pub struct Game {
    pub level: String,
}

impl Game {
    pub fn new(level: String) -> Game {
        Game { level }
    }
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Color>();
        reset_level(world, &self.level);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if data
            .world
            .fetch::<InputHandler<StringBindings>>()
            .key_is_down(VirtualKeyCode::R)
        {
            reset_level(data.world, &self.level);
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn reset_level(world: &mut World, level_path: &str) {
    world.delete_all();
    initialize_camera(world);
    initialize_clock(world);
    initialize_down_keys(world);

    let sprite_sheet = load_sprite_sheet(world);
    let white_box_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };

    match LevelConfig::load(level_path) {
        Ok(level_config) => {
            let level: LevelConfig = level_config;

            for escalator in level.escalators {
                initialize_escalator(world, escalator, white_box_render.clone());
            }

            for thing in level.things {
                initialize_thing(world, thing, white_box_render.clone());
            }

            for platform in level.platforms {
                initialize_platform(world, platform, white_box_render.clone());
            }
        }
        Err(e) => {
            warn!("Failed to load level at path {}.", level_path);
            warn!("Error was:\n{}", e)
        }
    };
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
