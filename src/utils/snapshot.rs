use amethyst::core::transform::Transform;

pub struct Snapshot<T> {
    pub timestamp: f32,
    pub component: T,
    pub local: Transform,
}
