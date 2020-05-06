use super::{Snapshot };
use crate::components::RewindableClock;
use amethyst::core::transform::Transform;

pub fn move_tape_backwards<T>(
    snapshots: &mut Vec<Snapshot<T>>,
    local: &mut Transform,
    component: &mut T,
) {
    match snapshots.pop() {
        Some(snapshot) => {
            local.set_translation(*snapshot.local.translation());
            *component = snapshot.component;
        }
        None => {}
    }
}

pub fn move_tape_forwards<T: Clone>(
    snapshots: &mut Vec<Snapshot<T>>,
    local: &mut Transform,
    component: &mut T,
    clock: &RewindableClock,
) {
    snapshots.push(Snapshot {
        component: component.clone(),
        timestamp: clock.current_time,
        local: local.clone(),
    })
}
