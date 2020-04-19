use getset::{CopyGetters, Getters, MutGetters, Setters};

use graphics::types::Color;


#[derive(Getters)]
pub struct Item {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    pub color: Color,
}

pub trait Rectangular {
    fn x(&self) -> f64;

    fn y(&self) -> f64;

    fn width(&self) -> f64;

    fn height(&self) -> f64;

    fn left(&self) -> f64 { self.x() }

    fn right(&self) -> f64 { self.x() + self.width() }

    fn top(&self) -> f64 { self.y() + self.height() }

    fn bottom(&self) -> f64 { self.y() }
}

impl Rectangular for Item {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }
}

impl Item {
    pub fn new() -> Item {
        Item {
            x: 150.0,
            y: 0.0,
            width: 20.0,
            height: 20.0,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}