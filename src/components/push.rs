use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Push {
    pub x_velocity: f32,
    pub pusher_name: Option<String>,
}

impl<'s> Component for Push {
    type Storage = DenseVecStorage<Self>;
}

impl Push {
    pub fn default() -> Push {
        Push {
            x_velocity: 0.,
            pusher_name: None,
        }
    }
}
