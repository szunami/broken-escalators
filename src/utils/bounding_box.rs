use crate::components::{GridLocation, Rectangle};
use amethyst::core::transform::Transform;

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
    use crate::components::{Rectangle, GridLocation};
    use super::*;
    #[test]
    fn touching_multiple_edges_works() {
        let inner = BoundingBox::new(
            &Rectangle::new(1,1),
            &GridLocation::new(6, 9),
        );
        let outer = BoundingBox::new(
            &Rectangle::new(4, 4),
            &GridLocation::new(8, 7),
        );

        assert!(touching_multiple_edges(&inner, &outer));
    }
}

// pub fn is_atop(atop_candidate: &BoundingBox, base_candidate: &BoundingBox) -> bool {
//     if atop_candidate.top < base_candidate.top {
//         return false;
//     }
//     if !overlap_exists(atop_candidate, base_candidate) {
//         return false;
//     }

//     true
// }

// // how much do we have to move a such that it does not collide with b
// pub fn y_overlap(a: &BoundingBox, b: &BoundingBox) -> f32 {
//     if !overlap_exists(a, b) {
//         return 0.;
//     }
//     b.top - a.bottom
// }

// // how much do we have to move a such that it does not collide with b
// pub fn x_overlap(a: &BoundingBox, b: &BoundingBox) -> f32 {
//     if !overlap_exists(a, b) {
//         return 0.;
//     }

//     if a.left < b.left {
//         return b.left - a.right;
//     }

//     b.right - a.left
// }

// fn overlap_exists(a: &BoundingBox, b: &BoundingBox) -> bool {
//     if !overlaps(a.left, a.right, b.left, b.right) {
//         return false;
//     }

//     if !overlaps(a.bottom, a.top, b.bottom, b.top) {
//         return false;
//     }
//     true
// }

// fn overlaps(a: f32, b: f32, x: f32, y: f32) -> bool {
//     (a <= x && b >= x) || (x <= a && y >= a)
// }

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
