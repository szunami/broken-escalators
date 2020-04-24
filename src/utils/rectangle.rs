use amethyst::core::transform::Transform;


pub struct BoundsProvider<'s> {
    pub width: f32,
    pub height: f32,
    pub transform: &'s Transform,
}

impl<'s> BoundsProvider<'s> {
    pub fn new(width: f32, height: f32,
        transform: &'s Transform) -> BoundsProvider<'s> {
            return BoundsProvider{ width, height, transform};
        }
}

impl<'s> BoundsProvider<'s> {

    pub fn top(&self) -> f32 {
        return self.transform.translation().y + 0.5 * self.height;
    }

    pub fn bottom(&self) -> f32 {
        return self.transform.translation().y - 0.5 * self.height;
    }

    pub fn left(&self) -> f32 {
        return self.transform.translation().x - 0.5 * self.width;
    }

    pub fn right(&self) -> f32 {
        return self.transform.translation().x + 0.5 * self.width;
    }
}
