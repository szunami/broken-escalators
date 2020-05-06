use super::Snapshot;
use crate::components::RewindableClock;
use amethyst::core::transform::Transform;

use amethyst::ecs::prelude::{Join, ReadStorage};

pub fn backwards_clock_check(clocks: ReadStorage<RewindableClock>) -> bool {
    for clock in (&clocks).join() {
        return clock.velocity < 0.;
    }
    return false;
}

pub fn move_tape_backwards<T>(
    snapshots: &mut Vec<Snapshot<T>>,
    local: &mut Transform,
    component: &mut T,
) {
    if let Some(snapshot) = snapshots.pop() {
        local.set_translation(*snapshot.local.translation());
        *component = snapshot.component;
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
