use crate::components::{Direction, Escalator, Rectangle, Step};
use crate::{resources::RewindableClock, utils::BoundingBox};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct CornerSystem;

impl<'s> System<'s> for CornerSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        WriteStorage<'s, Step>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Rectangle>,
    );

    fn run(&mut self, (clock, mut steps, transforms, escalators, rectangles): Self::SystemData) {
        if !clock.going_forwards() {
            return;
        }

        for (step, step_transform, step_rectangle) in (&mut steps, &transforms, &rectangles).join() {
            let escalator = escalators.get(step.escalator).unwrap();
            let escalator_transform = transforms.get(step.escalator).unwrap();
            let escalator_box =
                BoundingBox::new(escalator.width, escalator.height, escalator_transform);
            let step_box =
                BoundingBox::new(step_rectangle.width, step_rectangle.height, step_transform);

            // left edge
            if step_box.left <= escalator_box.left
                && step_box.top < escalator_box.top
                && step_box.bottom > escalator_box.bottom
            {
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        up_left(step, escalator.speed);
                    }
                    Direction::COUNTERCLOCKWISE => {
                        down_left(step, escalator.speed);
                    }
                }
            }
            // diagonal edge
            else if step_box.left > escalator_box.left
                && step_box.right < escalator_box.right
                && step_box.top < escalator_box.top
                && step_box.bottom > escalator_box.bottom
            {
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        down_right_diag(step, escalator.speed);
                    }
                    Direction::COUNTERCLOCKWISE => {
                        up_left_diag(step, escalator.speed);
                    }
                }
            }
            // bottom edge
            else if step_box.bottom <= escalator_box.bottom
                && step_box.left > escalator_box.left
                && step_box.right < escalator_box.right
            {
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        left_bottom(step, escalator.speed);
                    }
                    Direction::COUNTERCLOCKWISE => {
                        right_bottom(step, escalator.speed);
                    }
                }
            }
            // top left corner
            else if step_box.top >= escalator_box.top {
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        down_right_diag(step, escalator.speed);
                    }
                    Direction::COUNTERCLOCKWISE => down_left(step, escalator.speed),
                }
            }
            // bottom right corner
            else if step_box.right >= escalator_box.right {
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        left_bottom(step, escalator.speed);
                    }
                    Direction::COUNTERCLOCKWISE => {
                        up_left_diag(step, escalator.speed);
                    }
                }
            }
            // middle corner
            else if (step_box.left <= escalator_box.left)
                && (step_box.bottom <= escalator_box.bottom)
            {
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        up_left(step, escalator.speed);
                    }
                    Direction::COUNTERCLOCKWISE => {
                        right_bottom(step, escalator.speed);
                    }
                }
            }
        }
    }
}

fn up_left(step: &mut Step, speed: f32) {
    step.x_velocity = 0.;
    step.y_velocity = speed;
    step.push_velocity = 0.;
}

fn down_left(step: &mut Step, speed: f32) {
    step.x_velocity = 0.;
    step.y_velocity = -speed;
    step.push_velocity = -speed;
}

fn left_bottom(step: &mut Step, speed: f32) {
    step.x_velocity = -speed;
    step.y_velocity = 0.;
    step.push_velocity = speed;
}

fn right_bottom(step: &mut Step, speed: f32) {
    step.x_velocity = speed;
    step.y_velocity = 0.;
    step.push_velocity = 0.;
}

fn up_left_diag(step: &mut Step, speed: f32) {
    step.x_velocity = -speed;
    step.y_velocity = speed;
    step.push_velocity = 0.;
}

fn down_right_diag(step: &mut Step, speed: f32) {
    step.x_velocity = speed;
    step.y_velocity = -speed;
    step.push_velocity = 0.;
}
