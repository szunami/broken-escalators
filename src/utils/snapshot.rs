use crate::components::GridLocation;

pub struct Snapshot<T> {
    pub component: T,
    pub grid_location: GridLocation,
}
