use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Atop {
    pub x_velocity: f32,
    pub y_velocity: f32,
}

impl Component for Atop {
    type Storage = DenseVecStorage<Self>;
}

impl Atop {
    pub fn default() -> Atop {
        Atop { x_velocity: 0., y_velocity: 0. }
    }
}