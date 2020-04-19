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
    fn update(&self, x: f64, y: f64) -> Step {
        return Step { x, y, ..*self };
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
        let down_bit: Vec<Step> = (1..(num_units - 1)).collect::<Vec<i32>>().iter().map(|i| {
            Step {
                x: x + step_width * (*i as f64),
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
            speed: 50.,
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
        self.steps = self.compute_new_steps(dt);
    }

    fn compute_new_steps(&self, dt: f64) -> Vec<Step> {
        return self.steps.iter().map(|step| {
            let distance = dt * self.speed;
            if step.left() <= self.left() && step.top() <= self.top() {
                return step.update(self.left(), step.y + distance);
            }
            if step.bottom() <= self.bottom() && step.left() >= self.left() {
                return step.update(step.x - distance, step.y);
            }
            return step.update(step.x + distance, step.y - distance);
        }).collect();
    }
}

