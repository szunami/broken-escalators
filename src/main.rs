#[macro_use]
extern crate log;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};
use game::Game;

mod components;
mod entities;
mod game;
mod levels;
mod resources;
mod systems;
mod utils;

use std::any;
use std::env;
use systems::{
    constants::*,
    core::{DownKeysSystem, FPSSystem, StepTapeSystem, ThingTapeSystem, ToggleSystem},
    core::{PlatformSystem, RewindableClockSystem},
    position::MoveSystem,
    velocity,
};
use velocity::{AtopSystem, CornerSystem};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let args: Vec<String> = env::args().collect();

    let game: Game = parse_config(args);

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let input_bundle = InputBundle::<StringBindings>::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0., 0., 0., 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(FpsCounterBundle {})?
        // core systems go first
        .with(FPSSystem, any::type_name::<FPSSystem>(), &[])
        .with(ThingTapeSystem, any::type_name::<ThingTapeSystem>(), &[])
        .with(StepTapeSystem, any::type_name::<StepTapeSystem>(), &[])
        .with(DownKeysSystem, any::type_name::<DownKeysSystem>(), &[])
        .with(ToggleSystem, any::type_name::<ToggleSystem>(), &[])
        .with(PlatformSystem, any::type_name::<PlatformSystem>(), &[])
        .with(
            RewindableClockSystem,
            any::type_name::<RewindableClockSystem>(),
            &[],
        )
        // velocity systems go second
        .with(
            CornerSystem,
            any::type_name::<CornerSystem>(),
            &core_systems(),
        )
        .with(
            AtopSystem,
            any::type_name::<AtopSystem>(),
            &atop_dependencies(),
        )
        // position systems go last
        .with(
            systems::position::MoveSystem,
            any::type_name::<MoveSystem>(),
            &velocity_systems(),
        );

    let mut game = Application::new(assets_dir, game, game_data)?;
    game.run();

    Ok(())
}

fn parse_config(args: Vec<String>) -> Game {
    // info!("Args: {}", args);
    let level_file_name = args
        .get(1)
        .or(Some(&String::from("assets/levels/level.ron")))
        .unwrap()
        .clone();
    Game::new(level_file_name)
}
