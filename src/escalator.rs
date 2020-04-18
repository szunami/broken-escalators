enum Direction {
    CLOCKWISE,
    COUNTERCLOCKWISE
}

struct Step {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

struct Escalator {
    direction: Direction,
    active: bool,
    steps: Vec<Step>,
    speed: f64,
}