use crate::components::RewindableClock;
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct RewindableClockSystem;

impl<'s> System<'s> for RewindableClockSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        WriteStorage<'s, RewindableClock>,
    );

    fn run(&mut self, (input, time, mut clocks): Self::SystemData) {
        let clock_direction = if input.key_is_down(VirtualKeyCode::Z) {
            -1.
        } else {
            1.
        };
        for clock in (&mut clocks).join() {
            clock.update_clock(clock_direction * time.delta_seconds());
            info!("Clock: {}", clock.current_time);
        }
    }
}
