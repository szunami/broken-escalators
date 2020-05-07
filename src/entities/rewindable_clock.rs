use crate::resources::RewindableClock;
use amethyst::prelude::*;

pub fn initialize_clock(world: &mut World) {
    world.insert(RewindableClock::default());
}
