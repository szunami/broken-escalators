use amethyst::ecs::prelude::{Component, DenseVecStorage};
use super::Escalator;
use crate::utils::Snapshot;

pub struct EscalatorTape {
    pub snapshots: Vec<Snapshot<Escalator>>,
}

impl<'s> Component for EscalatorTape {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> EscalatorTape {
    pub fn default() -> EscalatorTape {
        EscalatorTape {
            snapshots: vec![]
        }
    }
}