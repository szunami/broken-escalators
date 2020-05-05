use super::{Step};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct StepTape {
    pub snapshots: Vec<StepSnapshot>,
}

pub struct StepSnapshot {
    pub timestamp: f32,
    pub step: Step,
    pub local: Transform,
}

impl Component for StepTape {
    type Storage = DenseVecStorage<Self>;
}

impl StepTape {
    pub fn new() -> StepTape {
        StepTape {
            snapshots: vec![],
        }
    }
}
