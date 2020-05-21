use crate::components::{GridLocation, Rectangle};
use amethyst::core::transform::Transform;

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

mod tests {
    use super::*;
    use crate::components::{GridLocation, Rectangle};
    #[test]
    fn touching_multiple_edges_works() {
        let inner = BoundingBox::new(&Rectangle::new(1, 1), &GridLocation::new(6, 9));
        let outer = BoundingBox::new(&Rectangle::new(4, 4), &GridLocation::new(8, 7));

        assert!(touching_multiple_edges(&inner, &outer));
    }

    #[test]
    fn is_atop_works() {
        let above = BoundingBox::new(
            &Rectangle::new(2, 2),
            // l = 5, r = 7
            // t = 8, 10
            &GridLocation::new(6, 9),
        );
        let base = BoundingBox::new(
            &Rectangle::new(4, 2),
            // l = 6, r = 10,
            // b = 5, t = 7
            &GridLocation::new(8, 6),
        );

        assert!(is_atop(&above, &base));
    }

    #[test]
    fn overlap_exists_works() {
        let above = BoundingBox::new(&Rectangle::new(4, 4), &GridLocation::new(6, 14));
        // left = 4, r = 8
        // top = 16, bottom = 12
        let base = BoundingBox::new(&Rectangle::new(2, 2), &GridLocation::new(4, 13));

        // left = 3, r = 5
        // top = 15, bottom = 11

        assert!(overlap_exists(&above, &base));
    }
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

// // how much do we have to move a such that it does not collide with b
// pub fn y_overlap(a: &BoundingBox, b: &BoundingBox) -> f32 {
//     if !overlap_exists(a, b) {
//         return 0.;
//     }
//     b.top - a.bottom
// }

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

// pub fn extrusion(container: &BoundingBox, containee: &BoundingBox) -> f32 {
//     f32::max(
//         f32::max(
//             f32::max(
//                 f32::max(0., container.left - containee.left),
//                 containee.right - container.right,
//             ),
//             container.bottom - containee.bottom,
//         ),
//         containee.top - container.top,
//     )
// }
