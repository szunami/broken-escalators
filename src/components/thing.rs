use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::utils::UpdateFrom;

#[derive(Copy, Clone)]
pub struct Thing {
    pub width: f32,
    pub height: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
}


impl UpdateFrom<Thing> for Thing {
    fn update_from(&mut self, other: Thing) {
        self.x_velocity = other.x_velocity;
        self.y_velocity = other.y_velocity;
    }
}

impl Component for Thing {
    type Storage = DenseVecStorage<Self>;
}

impl Thing {
    pub fn new(width: f32, height: f32, x_velocity: f32, y_velocity: f32) -> Thing {
        Thing {
            width,
            height,
            x_velocity,
            y_velocity,
        }
    }
}
