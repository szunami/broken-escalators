use super::Thing;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::utils::Snapshot;

pub struct ThingTape {
    pub snapshots: Vec<Snapshot<Thing>>,
}

impl Component for ThingTape {
    type Storage = DenseVecStorage<Self>;
}

impl ThingTape {
    pub fn new() -> ThingTape {
        ThingTape {
            snapshots: vec![],
        }
    }
}
