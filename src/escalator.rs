pub enum Direction {
    CLOCKWISE,
    COUNTERCLOCKWISE,
}

#[derive(Debug, Clone)]
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
    pub fn new(height: f64, width: f64, num_units: i32, x: f64, y: f64) -> Escalator {
        let step_height = height / num_units as f64;
        let step_width = width / num_units as f64;
        let left_edge: Vec<Step> = (0..num_units).collect::<Vec<i32>>().iter().map(|i| {
            Step {
                x: x,
                y: y + *i as f64 * 10.,
                width: step_width,
                height: step_height,
            }
        }).collect();
        let down_bit: Vec<Step> = (0..(num_units - 1)).collect::<Vec<i32>>().iter().map(|i| {
            Step {
                x: x + step_width * (*i as f64 + 1.),
                y: y + step_height * ((num_units - 1 - *i) as f64),
                width: step_width,
                height: step_height,
            }
        }).collect();
        let bottom_row: Vec<Step> = (1..num_units).collect::<Vec<i32>>().iter().map(|i| {
            Step {
                x: x + step_width * (*i as f64),
                y: y,
                width: step_width,
                height: step_height,
            }
        }).collect();

        let all_steps = [&left_edge[..], &down_bit[..], &bottom_row[..]].concat();

        Escalator {
            direction: Direction::CLOCKWISE,
            active: false,
            steps: all_steps,
            speed: 10.0,
            height: height,
            width: width,
            x: x,
            y: y,
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