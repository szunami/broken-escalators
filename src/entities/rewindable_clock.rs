use crate::components::RewindableClock;
use amethyst::prelude::*;

pub fn initialize_clock(world: &mut World) {
    world.create_entity().with(RewindableClock::new()).build();
}
