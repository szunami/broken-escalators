use amethyst::core::transform::Transform;

pub struct BoundingBox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl BoundingBox {
    pub fn new(width: f32, height: f32, local: &Transform) -> BoundingBox {
        BoundingBox {
            left: local.translation().x - width * 0.5,
            right: local.translation().x + width * 0.5,
            top: local.translation().y + height * 0.5,
            bottom: local.translation().y - height * 0.5,
        }
    }
}