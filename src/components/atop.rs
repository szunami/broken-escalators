use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum BaseEntity {
    Step(Entity),
}

pub struct Atop {
    bases: HashSet<BaseEntity>,
}

impl<'s> Component for Atop {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> Atop {
    pub fn default() -> Atop {
        Atop {
            bases: HashSet::new(),
        }
    }
}
