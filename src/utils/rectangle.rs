use amethyst::core::transform::Transform;


pub trait Rectangle {
    fn width(&self) -> f32;

    fn height(&self) -> f32;
}

pub struct BoundsProvider<'s, T: Rectangle> {
    pub rectangle: &'s T,
    pub transform: &'s Transform,
}

impl<'s, T: Rectangle> BoundsProvider<'s, T> {
    pub fn new(rectangle: &'s T,
        transform: &'s Transform) -> BoundsProvider<'s, T> {
            return BoundsProvider{ rectangle, transform};
        }
}

impl<'s, T: Rectangle> BoundsProvider<'s, T> {

    pub fn top(&self) -> f32 {
        return self.transform.translation().y + 0.5 * self.rectangle.height();
    }

    pub fn bottom(&self) -> f32 {
        return self.transform.translation().y - 0.5 * self.rectangle.height();
    }

    pub fn left(&self) -> f32 {
        return self.transform.translation().x - 0.5 * self.rectangle.width();
    }

    pub fn right(&self) -> f32 {
        return self.transform.translation().x + 0.5 * self.rectangle.width();
    }
}
