use crate::resources::DownKeys;
use amethyst::prelude::*;

pub fn initialize_down_keys(world: &mut World) {
    world.insert(DownKeys::default());
}
