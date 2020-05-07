use crate::components::{Direction, Escalator, Step};
use crate::{utils::BoundingBox, resources::RewindableClock};
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
    );

    fn run(&mut self, (clock, mut steps, locals, escalators): Self::SystemData) {
        if !clock.going_forwards() {
            return;
        }

        for (step, step_local) in (&mut steps, &locals).join() {
            let escalator = escalators.get(step.escalator).unwrap();
            let escalator_local = locals.get(step.escalator).unwrap();
            let escalator_box = BoundingBox::new(escalator.width, escalator.height, escalator_local);
            let step_box = BoundingBox::new(step.width, step.height, step_local);

            // top left corner
            // bottom right corner
            // middle corner
            // left edge
            if step_box.left <= escalator_box.left && step_box.top < escalator_box.top && step_box.bottom > escalator_box.bottom {
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
            else if step_box.left > escalator_box.left && step_box.right < escalator_box.right && step_box.top < escalator_box.top && step_box.bottom > escalator_box.bottom {
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        down_right_diag(step, escalator.speed);
                    },
                    Direction::COUNTERCLOCKWISE => {
                        up_left_diag(step, escalator.speed);
                    }
                }
            }
            
            // bottom edge
            else if step_box.bottom <= escalator_box.bottom && step_box.left > escalator_box.left && step_box.right < escalator_box.right {
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        left_bottom(step, escalator.speed);
                    },
                    Direction::COUNTERCLOCKWISE => {
                        right_bottom(step, escalator.speed);
                    }
                }
            }
            

            if step_local.translation().y + step.height * 0.5 >= escalator_box.top {
                info!("Hit top");
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        step.x_velocity = escalator.speed;
                        step.y_velocity = -escalator.speed;
                        step.push_velocity = 0.;
                    }
                    Direction::COUNTERCLOCKWISE => {
                        down_left(step, escalator.speed)
                    }
                }
                continue;
            }

            if step_local.translation().x + step.width * 0.5 >= escalator_box.right {
                info!("escalator.right: {}", escalator_box.right);
                info!("step x: {}", step_local.translation().x);
                info!("Hit right corner");
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        step.x_velocity = -escalator.speed;
                        step.y_velocity = 0.;
                        step.push_velocity = escalator.speed;
                    }
                    Direction::COUNTERCLOCKWISE => {
                        step.x_velocity = -escalator.speed;
                        step.y_velocity = escalator.speed;
                        step.push_velocity = 0.;
                    }
                }

                continue;
            }
            if (step_local.translation().x - step.width * 0.5 <= escalator_box.left)
                && (step_local.translation().y - step.height * 0.5 <= escalator_box.bottom)
            {
                info!("Hit middle corner");
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        up_left(step, escalator.speed);
                    }
                    Direction::COUNTERCLOCKWISE => {
                        step.x_velocity = escalator.speed;
                        step.y_velocity = 0.;
                        step.push_velocity = 0.;
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

fn left_bottom(step: &mut Step, speed: f32) {}

fn right_bottom(step: &mut Step, speed: f32) {}

fn up_left_diag(step: &mut Step, speed: f32) {}

fn down_right_diag(step: &mut Step, speed: f32) {}

