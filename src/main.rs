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
mod resources;
mod systems;
mod utils;

use systems::{core::{StepTapeSystem, DownKeysSystem, ThingTapeSystem, FPSSystem, ToggleSystem}, constants::*, core::{RewindableClockSystem, PlatformSystem}, velocity};
use velocity::{AtopSystem, CornerSystem};


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

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
        .with(FPSSystem, FPS_SYSTEM, &[])
        .with(ThingTapeSystem, THING_TAPE_SYSTEM, &[])
        .with(StepTapeSystem, STEP_TAPE_SYSTEM, &[])
        .with(DownKeysSystem, DOWN_KEY_SYSTEM, &[])
        .with(ToggleSystem, TOGGLE_SYSTEM, &[])
        .with(PlatformSystem, PLATFORM_SYSTEM, &[])
        .with(
            RewindableClockSystem,
            REWINDABLE_CLOCK_SYSTEM,
            &[],
        )
        // velocity systems go second
        .with(CornerSystem, CORNER_SYSTEM, &core_systems())
        .with(AtopSystem, ATOP_SYSTEM, &atop_dependencies())
        // position systems go last
        .with(systems::EscalatorSystem, ESCALATOR_SYSTEM, &velocity_systems())
        .with(systems::position::MoveSystem, MOVE_SYSTEM, &velocity_systems());


    let mut game = Application::new(assets_dir, Game::default(), game_data)?;
    game.run();

    Ok(())
}
