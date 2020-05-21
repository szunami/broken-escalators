use super::Snapshot;
use crate::components::GridLocation;
use crate::resources::RewindableClock;
use amethyst::ecs::prelude::Read;

pub fn move_tape_backwards<T>(
    snapshots: &mut Vec<Snapshot<T>>,
    grid_location: &mut GridLocation,
    component: &mut T,
) {
    if let Some(snapshot) = snapshots.pop() {
        *grid_location = snapshot.grid_location;
        *component = snapshot.component;
    }
}

pub fn move_tape_forwards<T: Clone>(
    snapshots: &mut Vec<Snapshot<T>>,
    grid_location: &mut GridLocation,
    component: &mut T,
) {
    snapshots.push(Snapshot {
        component: component.clone(),
        grid_location: grid_location.clone(),
    })
}
