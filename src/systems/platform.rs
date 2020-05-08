use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, ReadStorage},
};
use crate::components::Platform;

#[derive(SystemDesc)]
pub struct PlatformSystem;

impl<'s> System<'s> for PlatformSystem {
    type SystemData = (ReadStorage<'s, Platform>);

    fn run(&mut self, (platforms): Self::SystemData) {}
}
