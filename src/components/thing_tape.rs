use super::Thing;
use crate::utils::Snapshot;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ThingTape {
    pub snapshots: Vec<Snapshot<Thing>>,
}

impl Component for ThingTape {
    type Storage = DenseVecStorage<Self>;
}

impl ThingTape {
    pub fn new() -> ThingTape {
        ThingTape { snapshots: vec![] }
    }
}
