use crate::resources::DownKeys;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, Write},
};

#[derive(SystemDesc)]
pub struct DownKeysSystem;

impl<'s> System<'s> for DownKeysSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>, Write<'s, DownKeys>);

    fn run(&mut self, (input, mut down_keys): Self::SystemData) {
        down_keys.update(input.keys_that_are_down());
    }
}
