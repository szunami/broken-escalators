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
    core::{DownKeysSystem, RewindableClockSystem, StepTapeSystem, ThingTapeSystem, ToggleSystem},
    velocity::AtopSystem,
    AbsoluteEscalatorVelocitySystem, AbsoluteStepVelocitySystem, AbsoluteThingVelocity,
    EscalatorCorrectionSystem, EscalatorPositionSystem, EscalatorTapeSystem,
    GridLocationTransformSystem, IntrinsicStepVelocitySystem, StepPositionSystem,
    ThingCorrectionSystem, ThingPositionSystem,
};

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
        .with(DownKeysSystem, any::type_name::<DownKeysSystem>(), &[])
        .with(
            ToggleSystem,
            any::type_name::<ToggleSystem>(),
            &[any::type_name::<DownKeysSystem>()],
        )
        .with(
            RewindableClockSystem,
            any::type_name::<RewindableClockSystem>(),
            &[any::type_name::<DownKeysSystem>()],
        )
        .with(
            StepTapeSystem,
            any::type_name::<StepTapeSystem>(),
            &[any::type_name::<ToggleSystem>()],
        )
        .with(
            ThingTapeSystem,
            any::type_name::<ThingTapeSystem>(),
            &[any::type_name::<RewindableClockSystem>()],
        )
        .with(
            EscalatorTapeSystem,
            any::type_name::<EscalatorTapeSystem>(),
            &[any::type_name::<RewindableClockSystem>()],
        )
        // escalator tape system
        .with(
            IntrinsicStepVelocitySystem,
            any::type_name::<IntrinsicStepVelocitySystem>(),
            &[any::type_name::<StepTapeSystem>()],
        )
        .with(
            AtopSystem,
            any::type_name::<AtopSystem>(),
            &[
                any::type_name::<IntrinsicStepVelocitySystem>(),
                any::type_name::<ThingTapeSystem>(),
            ],
        )
        .with(
            AbsoluteEscalatorVelocitySystem,
            any::type_name::<AbsoluteEscalatorVelocitySystem>(),
            &[
                any::type_name::<IntrinsicStepVelocitySystem>(),
                any::type_name::<AtopSystem>(),
            ],
        )
        .with(
            AbsoluteStepVelocitySystem,
            any::type_name::<AbsoluteStepVelocitySystem>(),
            &[any::type_name::<AbsoluteEscalatorVelocitySystem>()],
        )
        .with(
            AbsoluteThingVelocity,
            any::type_name::<AbsoluteThingVelocity>(),
            // this is probably overly restrictive for now
            &[
                any::type_name::<IntrinsicStepVelocitySystem>(),
                any::type_name::<AtopSystem>(),
            ],
        )
        .with(
            EscalatorPositionSystem,
            any::type_name::<EscalatorPositionSystem>(),
            &[any::type_name::<AbsoluteEscalatorVelocitySystem>()],
        )
        .with(
            StepPositionSystem,
            any::type_name::<StepPositionSystem>(),
            &[
                any::type_name::<AbsoluteStepVelocitySystem>(),
                any::type_name::<StepTapeSystem>(),
            ],
        )
        .with(
            ThingPositionSystem,
            any::type_name::<ThingPositionSystem>(),
            &[any::type_name::<AbsoluteThingVelocity>()],
        )
        .with(
            EscalatorCorrectionSystem,
            any::type_name::<EscalatorCorrectionSystem>(),
            &[any::type_name::<StepPositionSystem>()],
        )
        .with(
            ThingCorrectionSystem,
            any::type_name::<ThingCorrectionSystem>(),
            &[any::type_name::<ThingPositionSystem>()],
        )
        .with(
            GridLocationTransformSystem,
            any::type_name::<GridLocationTransformSystem>(),
            &[
                any::type_name::<EscalatorCorrectionSystem>(),
                any::type_name::<ThingCorrectionSystem>(),
            ],
        );

    let mut game = Application::new(assets_dir, game, game_data)?;
    game.run();

    Ok(())
}

fn parse_config(args: Vec<String>) -> Game {
    let level_file_name = args
        .get(1)
        .unwrap_or(&String::from("assets/levels/level1.ron"))
        .clone();
    Game::new(level_file_name)
}
