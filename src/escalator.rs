pub enum Direction {
    CLOCKWISE,
    COUNTERCLOCKWISE,
}

pub struct Step {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub(crate) struct Escalator {
    direction: Direction,
    active: bool,
    pub steps: Vec<Step>,
    speed: f64,
    height: f64,
    width: f64,
    x: f64,
    y: f64,
}

impl Escalator {
    pub fn new() -> Escalator {
        let step = Step {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };
        Escalator {
            direction: Direction::CLOCKWISE,
            active: false,
            steps: vec![step],
            speed: 1.0,
            height: 100.,
            width: 100.,
            x: 0.,
            y: 0.,
        }
    }

    pub fn update(&mut self, dt: f64) {
        for step in self.steps.iter_mut() {

            // left edge
            if step.y == self.y && step.x != self.x {
                step.x -= self.speed * dt;
            }

            // top

            // down bit

            // right

            // bottom
        }
    }
}