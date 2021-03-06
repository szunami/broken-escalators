use crate::resources::{DownKeys, RewindableClock};
use amethyst::input::VirtualKeyCode;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, Write},
};

#[derive(SystemDesc)]
pub struct RewindableClockSystem;

impl<'s> System<'s> for RewindableClockSystem {
    type SystemData = (Read<'s, DownKeys>, Write<'s, RewindableClock>);

    fn run(&mut self, (down_keys, mut clock): Self::SystemData) {
        let velocity = if down_keys.key_downs().contains(&VirtualKeyCode::Right) {
            1
        } else if down_keys.key_downs().contains(&VirtualKeyCode::Left) {
            -1
        } else {
            0
        };

        clock.update_clock(velocity);
    }
}
