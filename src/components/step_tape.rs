use super::Step;
use crate::utils::Snapshot;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct StepTape {
    pub snapshots: Vec<Snapshot<Step>>,
}

impl Component for StepTape {
    type Storage = DenseVecStorage<Self>;
}

impl StepTape {
    pub fn new() -> StepTape {
        StepTape { snapshots: vec![] }
    }
}
