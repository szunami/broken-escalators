use super::Snapshot;
use amethyst::core::transform::Transform;

use crate::resources::RewindableClock;
use amethyst::ecs::prelude::Read;

pub fn move_tape_backwards<T>(
    snapshots: &mut Vec<Snapshot<T>>,
    transform: &mut Transform,
    component: &mut T,
) {
    if let Some(snapshot) = snapshots.pop() {
        transform.set_translation(*snapshot.transform.translation());
        *component = snapshot.component;
    }
}

pub fn move_tape_forwards<T: Clone>(
    snapshots: &mut Vec<Snapshot<T>>,
    transform: &mut Transform,
    component: &mut T,
    clock: &Read<RewindableClock>,
) {
    snapshots.push(Snapshot {
        component: component.clone(),
        timestamp: clock.current_time,
        transform: transform.clone(),
    })
}
