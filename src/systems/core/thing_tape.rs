use crate::{
    components::{Thing, ThingTape, GridLocation},
    resources::{DownKeys, RewindableClock},
    utils::{move_tape_backwards, move_tape_forwards},
};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct ThingTapeSystem;

impl<'s> System<'s> for ThingTapeSystem {
    type SystemData = (
        Read<'s, DownKeys>,
        WriteStorage<'s, Thing>,
        WriteStorage<'s, GridLocation>,
        WriteStorage<'s, ThingTape>,
    );

    fn run(&mut self, (down_keys, mut things, mut grid_locations, mut thing_tapes): Self::SystemData) {
        let going_forward = down_keys.key_downs().contains(&VirtualKeyCode::Space);
        let going_backward = down_keys.key_downs().contains(&VirtualKeyCode::Z);
        if !going_forward && !going_backward {
            return;
        }
        for (thing, thing_grid_location, thing_tape) in
            (&mut things, &mut grid_locations, &mut thing_tapes).join()
        {
            let snapshots = &mut thing_tape.snapshots;
            if going_forward {
                info!("Going forwards");
                move_tape_forwards(snapshots, thing_grid_location, thing);
            } 
            if going_backward {
                info!("Going backward");
                move_tape_backwards(snapshots, thing_grid_location, thing);
            }
        }
    }
}
