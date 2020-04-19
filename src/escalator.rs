
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

    pub fn top(&self) -> f64 {
        self.y + self.height
    }

    pub fn bottom(&self) -> f64 {
        self.y
    }

    pub fn left(&self) -> f64 {
        self.x
    }

    pub fn right(&self) -> f64 {
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
            speed: 10.,
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
        let distance = self.speed * dt;
        self.steps = self.steps.iter().map(|step| { self.compute_new_step(step, distance) })
            .collect();
    }

    fn compute_new_step(&self, step: &Step, distance: f64) -> Step {
        // left edge
        if step.left() <= self.left() && step.top() <= self.top() {
            let spare_movement: f64 = f64::max(step.top() + distance - self.top(), 0.);
            if spare_movement > 0. {
                return step.update(self.left() + spare_movement, self.top() - spare_movement - step.height);
            }
            return step.update(self.left(), step.y + distance);
        }
        // bottom edge
        if step.bottom() <= self.bottom() && step.left() >= self.left() {
            let spare_movement = f64::max(self.left() - (step.x - distance), 0.);
            if spare_movement > 0. {
                return step.update(self.left(), self.bottom() + spare_movement);
            }
            return step.update(step.x - distance, step.y);
        }
        let spare_movement = f64::max(self.bottom() - step.bottom(), 0.);

        if spare_movement > 0. {
            return step.update(self.right() - spare_movement - step.width, self.bottom());
        }

        return step.update(step.x + distance, step.y - distance);
    }
}

