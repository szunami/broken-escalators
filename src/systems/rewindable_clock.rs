use crate::components::RewindableClock;
use crate::resources::RewindableClock as tmp;
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, Write, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct RewindableClockSystem;

impl<'s> System<'s> for RewindableClockSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Write<'s, tmp>,
        WriteStorage<'s, RewindableClock>,
    );

    fn run(&mut self, (input, time, mut clock, mut clocks): Self::SystemData) {
        let clock_velocity = if input.key_is_down(VirtualKeyCode::Z) {
            -1.
        } else {
            1.
        };
        clock.update_clock(clock_velocity, clock_velocity * time.delta_seconds());
        for asdf in (&mut clocks).join() {
            asdf.update_clock(clock_velocity, clock_velocity * time.delta_seconds());
            info!("Clock: {}", asdf.current_time);
        }
    }
}
