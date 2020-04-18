pub enum Direction {
    CLOCKWISE,
    COUNTERCLOCKWISE,
}

#[derive(Debug)]
pub struct Step {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Step {
    fn top(&self) -> f64 {
        self.y + self.height
    }

    fn bottom(&self) -> f64 {
        self.y
    }

    fn left(&self) -> f64 {
        self.x
    }

    fn right(&self) -> f64 {
        self.x + self.width
    }
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
        let a: Vec<_> = (0..10).collect();


        let left_edge: Vec<Step> = a.iter().map(|i| {
            Step {
                x: 0.0,
                y: *i as f64 * 10.,
                width: 10.,
                height: 10.,
            }
        }).collect();

        Escalator {
            direction: Direction::CLOCKWISE,
            active: false,
            steps: left_edge,
            speed: 100.0,
            height: 100.,
            width: 100.,
            x: 0.,
            y: 0.,
        }
    }

    fn top(&self) -> f64 {
        self.y + self.height
    }

    fn bottom(&self) -> f64 {
        self.y
    }

    fn left(&self) -> f64 {
        self.x
    }

    fn right(&self) -> f64 {
        self.x + self.width
    }


    pub fn update(&mut self, dt: f64) {
        for step in self.steps.iter_mut() {
//            println!("{:?}", step);

            // left edge
            if step.left() <= self.x && step.top() <= self.y + self.height {
//                println!("left edge");
                step.y += self.speed * dt;
                step.x = self.x;
            } else if step.top() >= self.y + self.height && step.x <= step.width {
//                println!("top edge");
                step.x += self.speed * dt;
                step.y = self.height - step.height;
            } else if step.right() >= self.x + self.width && step.bottom() >= self.y {
//                println!("right edge");
                step.x = self.x + self.width - step.width;
                step.y -= self.speed * dt;
            } else if step.bottom() <= self.y {
//                println!("bottom edge");
                step.x -= self.speed * dt;
                step.y = self.y;
            } else {
                // down bit
//                println!("going down");
                step.x += self.speed * dt;
                step.y -= self.speed * dt;
            }


        }
    }
}