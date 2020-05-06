pub use self::rectangle::BoundsProvider;
pub use self::snapshot::Snapshot;
pub use self::tape::{move_tape_backwards, move_tape_forwards};

mod rectangle;
mod snapshot;
mod tape;
