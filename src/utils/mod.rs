pub use self::bounding_box::{BoundingBox, touching_edge};
pub use self::snapshot::Snapshot;
pub use self::tape::{move_tape_backwards, move_tape_forwards};

mod bounding_box;
mod snapshot;
mod tape;
