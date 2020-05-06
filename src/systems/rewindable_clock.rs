use crate::resources::RewindableClock;
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, Write},
};

#[derive(SystemDesc)]
pub struct RewindableClockSystem;

impl<'s> System<'s> for RewindableClockSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Write<'s, RewindableClock>,
    );

    fn run(&mut self, (input, time, mut clock): Self::SystemData) {
        let clock_velocity = if input.key_is_down(VirtualKeyCode::Z) {
            -1.
        } else {
            1.
        };
        clock.update_clock(clock_velocity, clock_velocity * time.delta_seconds());
    }
}
