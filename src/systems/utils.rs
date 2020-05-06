use crate::components::{Escalator};
use amethyst::{
    core::transform::Transform,
};


pub struct BoundingBox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl BoundingBox {
    pub fn from_escalator(escalator: &Escalator, local: &Transform) -> BoundingBox {
        BoundingBox {
            left: local.translation().x - escalator.width * 0.5,
            right: local.translation().x + escalator.width * 0.5,
            top: local.translation().y + escalator.height * 0.5,
            bottom: local.translation().y - escalator.height * 0.5,
        }
    }
}
