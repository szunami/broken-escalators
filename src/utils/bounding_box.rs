use crate::components::Rectangle;
use amethyst::core::transform::Transform;

pub struct BoundingBox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl BoundingBox {
    pub fn new(rectangle: &Rectangle, transform: &Transform) -> BoundingBox {
        BoundingBox {
            left: transform.translation().x - rectangle.width * 0.5,
            right: transform.translation().x + rectangle.width * 0.5,
            top: transform.translation().y + rectangle.height * 0.5,
            bottom: transform.translation().y - rectangle.height * 0.5,
        }
    }
}

pub fn is_atop(atop_candidate: &BoundingBox, base_candidate: &BoundingBox) -> bool {
    if atop_candidate.top < base_candidate.top {
        return false;
    }
    if !overlap_exists(atop_candidate, base_candidate) {
        return false;
    }

    true
}

// how much do we have to move a such that it does not collide with b
pub fn y_overlap(a: &BoundingBox, b: &BoundingBox) -> f32 {
    if !overlap_exists(a, b) {
        return 0.;
    }
    b.top - a.bottom
}

// how much do we have to move a such that it does not collide with b
pub fn x_overlap(a: &BoundingBox, b: &BoundingBox) -> f32 {
    if !overlap_exists(a, b) {
        return 0.;
    }

    if a.left < b.left {
        return b.left - a.right;
    }

    b.right - a.left
}

fn overlap_exists(a: &BoundingBox, b: &BoundingBox) -> bool {
    if !overlaps(a.left, a.right, b.left, b.right) {
        return false;
    }

    if !overlaps(a.bottom, a.top, b.bottom, b.top) {
        return false;
    }
    true
}

fn overlaps(a: f32, b: f32, x: f32, y: f32) -> bool {
    (a <= x && b >= x) || (x <= a && y >= a)
}

pub fn contains(container: &BoundingBox, containee: &BoundingBox) -> bool {
    container.left <= containee.left
        && container.right >= containee.right
        && container.top >= containee.top
        && container.bottom <= containee.bottom
}

pub fn extrusion(container: &BoundingBox, containee: &BoundingBox) -> f32 {
    f32::max(
        f32::max(
            f32::max(
                f32::max(0., container.left - containee.left),
                containee.right - container.right,
            ),
            container.bottom - containee.bottom,
        ),
        containee.top - container.top,
    )
}
