use crate::components::RewindableClock;
use crate::resources::RewindableClock as tmp;
use amethyst::prelude::*;

pub fn initialize_clock(world: &mut World) {
    world.create_entity().with(RewindableClock::new()).build();
    world.insert(tmp::default());
}
