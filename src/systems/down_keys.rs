use crate::resources::DownKeys;
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, Write},
};

#[derive(SystemDesc)]
pub struct DownKeysSystem;

impl<'s> System<'s> for DownKeysSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>, Write<'s, DownKeys>);

    fn run(&mut self, (input, down_keys): Self::SystemData) {
        let x = input.keys_that_are_down();
        down_keys.update(input.keys_that_are_down());
    }
}
