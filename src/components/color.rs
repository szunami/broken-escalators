use crate::utils::ColorFlag;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Color {
    pub color_flag: ColorFlag,
}

impl<'s> Component for Color {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> Color {
    pub fn new(color_flag: ColorFlag) -> Color {
        Color { color_flag }
    }
}
