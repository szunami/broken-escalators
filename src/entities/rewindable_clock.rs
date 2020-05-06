use crate::resources::RewindableClock as tmp;
use amethyst::prelude::*;

pub fn initialize_clock(world: &mut World) {
    world.insert(tmp::default());
}
