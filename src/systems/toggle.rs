use crate::{components::Escalator, resources::{DownKeys}};

use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, Join, WriteStorage},
};
use std::collections::HashSet;



#[derive(SystemDesc)]
pub struct ToggleSystem;

impl<'s> System<'s> for ToggleSystem {
    type SystemData = (
        Read<'s, DownKeys>,
        WriteStorage<'s, Escalator>,
    );

    fn run(&mut self, (down_keys, mut escalators): Self::SystemData) {
        let key_ups: HashSet<_> = down_keys.key_ups();
        let key_downs: HashSet<_> = down_keys.key_downs();
        let key_union = key_ups.union(&key_downs);
        let key_changes: HashSet<_> = key_union.collect();

        for escalator in (&mut escalators).join() {
            if key_changes.contains(&escalator.toggle_key) {
                escalator.toggle_direction();
            }
        }
    }
}
