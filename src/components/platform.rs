use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Copy, Clone)]
pub struct Platform {
}

impl<'s> Component for Platform {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> Platform {
    pub fn default() -> Platform {
        Platform {}
    }
}
