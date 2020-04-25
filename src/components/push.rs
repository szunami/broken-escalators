use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Push {
    pub x_velocity: f32,
}

impl Component for Push {
    type Storage = DenseVecStorage<Self>;
}

impl Push {
    pub fn default() -> Push {
        Push { x_velocity: 0.}
    }
}