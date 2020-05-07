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
        .with(systems::CornerSystem, "corner_system", &[])
        .with(systems::EscalatorSystem, "escalator_system", &[])
        .with(systems::AtopSystem, "atop_system", &[])
        .with(systems::MoveSystem, "move_system", &[])
        .with(systems::FPSSystem, "fps_system", &[])
        .with(systems::ThingTapeSystem, "thing_tape_system", &[])
        .with(systems::StepTapeSystem, "step_tape_system", &[])
        .with(systems::DownKeysSystem, "down_key_system", &[])
        .with(systems::ToggleSystem, "toggle_system", &[])
        .with(systems::PlatformSystem, "platform_system", &[])
        .with(
            systems::RewindableClockSystem,
            "rewindable_clock_system",
            &[],
        );

    let mut game = Application::new(assets_dir, Game::default(), game_data)?;
    game.run();

    Ok(())
}
