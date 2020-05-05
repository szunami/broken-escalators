use super::Thing;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ThingTape {
    pub thing_snapshots: Vec<ThingSnapshot>,
}

pub struct ThingSnapshot {
    pub timestamp: f32,
    pub thing: Thing,
    pub local: Transform,
}

impl Component for ThingTape {
    type Storage = DenseVecStorage<Self>;
}

impl ThingTape {
    pub fn new() -> ThingTape {
        ThingTape {
            thing_snapshots: vec![],
        }
    }
}
