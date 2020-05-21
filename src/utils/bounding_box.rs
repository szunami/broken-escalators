use crate::components::{GridLocation, Rectangle};

#[derive(Debug)]
pub struct BoundingBox {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl BoundingBox {
    pub fn new(rectangle: &Rectangle, grid_location: &GridLocation) -> BoundingBox {
        BoundingBox {
            left: grid_location.x - rectangle.width / 2,
            right: grid_location.x + rectangle.width / 2,
            top: grid_location.y + rectangle.height / 2,
            bottom: grid_location.y - rectangle.height / 2,
        }
    }
}

pub fn touching_multiple_edges(inner: &BoundingBox, outer: &BoundingBox) -> bool {
    let top = if inner.top == outer.top { 1 } else { 0 };
    let left = if inner.left == outer.left { 1 } else { 0 };
    let bottom = if inner.bottom == outer.bottom { 1 } else { 0 };
    let right = if inner.right == outer.right { 1 } else { 0 };
    top + left + bottom + right > 1
}

pub fn is_atop(atop_candidate: &BoundingBox, base_candidate: &BoundingBox) -> bool {
    if atop_candidate.bottom != base_candidate.top {
        return false;
    }
    overlaps(
        atop_candidate.left,
        atop_candidate.right,
        base_candidate.left,
        base_candidate.right,
    )
}

// how much do we have to move a such that it does not collide with b
pub fn x_overlap(a: &BoundingBox, b: &BoundingBox) -> i32 {
    if a.left < b.left {
        return b.left - a.right;
    }
    b.right - a.left
}

pub fn overlap_exists(a: &BoundingBox, b: &BoundingBox) -> bool {
    if !overlaps(a.left, a.right, b.left, b.right) {
        return false;
    }

    if !overlaps(a.bottom, a.top, b.bottom, b.top) {
        return false;
    }
    true
}

fn overlaps(a: i32, b: i32, x: i32, y: i32) -> bool {
    (a <= x && b > x) || (x <= a && y > a)
}
