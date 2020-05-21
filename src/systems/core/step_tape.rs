use crate::{
    components::{Step, StepTape, GridLocation},
    resources::RewindableClock,   resources::DownKeys,

    utils::{move_tape_backwards, move_tape_forwards},
};
use amethyst::input::VirtualKeyCode;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct StepTapeSystem;

impl<'s> System<'s> for StepTapeSystem {
    type SystemData = (
        Read<'s, DownKeys>,
        WriteStorage<'s, Step>,
        WriteStorage<'s, GridLocation>,
        WriteStorage<'s, StepTape>,
    );

    fn run(&mut self, (down_keys, mut steps, mut grid_locations, mut step_tapes): Self::SystemData) {
        let going_forward = down_keys.key_downs().contains(&VirtualKeyCode::Space);
        let going_backward = down_keys.key_downs().contains(&VirtualKeyCode::Z);
        if !going_forward && !going_backward {
            return;
        }
        for (step, step_grid_location, step_tape) in (&mut steps, &mut grid_locations, &mut step_tapes).join() {
            let snapshots = &mut step_tape.snapshots;
            if going_forward {
                info!("Going forwards");
                move_tape_forwards(snapshots, step_grid_location, step);
            } 
            if going_backward {
                info!("Going backward");
                move_tape_backwards(snapshots, step_grid_location, step);
            }
        }
    }
}
