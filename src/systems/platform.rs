use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct PlatformSystem;

impl<'s> System<'s> for PlatformSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {}
}
