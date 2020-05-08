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

pub fn is_atop(atop_candidate: &BoundingBox, base_candidate: &BoundingBox) -> bool {
    if atop_candidate.top < base_candidate.top {
        return false;
    }
    if !overlaps(
        base_candidate.left,
        base_candidate.right,
        atop_candidate.left,
        atop_candidate.right,
    ) {
        return false;
    }

    if !overlaps(
        base_candidate.bottom,
        base_candidate.top,
        atop_candidate.bottom,
        atop_candidate.top,
    ) {
        return false;
    }

    return true;
}

fn overlaps(a: f32, b: f32, x: f32, y: f32) -> bool {
    (a <= x && b >= x) || (x <= a && y >= a)
}
