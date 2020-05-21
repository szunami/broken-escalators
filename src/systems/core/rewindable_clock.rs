use crate::resources::{DownKeys, RewindableClock};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, Write},
};

#[derive(SystemDesc)]
pub struct RewindableClockSystem;

impl<'s> System<'s> for RewindableClockSystem {
    type SystemData = (Read<'s, DownKeys>, Write<'s, RewindableClock>);

    fn run(&mut self, (down_keys, mut clock): Self::SystemData) {
        let velocity = if down_keys.key_downs().contains(&VirtualKeyCode::Space) {
            1
        } else if down_keys.key_downs().contains(&VirtualKeyCode::Z) {
            -1
        } else {
            0
        };

        clock.update_clock(velocity);
    }
}
