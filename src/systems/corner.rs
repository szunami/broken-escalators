use crate::components::{Direction, Escalator, RewindableClock, Step};
use crate::{systems::utils::escalator_bounds_read, utils::backwards_clock_check};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct CornerSystem;

impl<'s> System<'s> for CornerSystem {
    type SystemData = (
        ReadStorage<'s, RewindableClock>,
        WriteStorage<'s, Step>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Escalator>,
    );

    fn run(&mut self, (clocks, mut steps, locals, escalators): Self::SystemData) {
        if backwards_clock_check(clocks) {
            return;
        }

        let escalator_map = escalator_bounds_read(&locals, &escalators);

        for (step, step_local) in (&mut steps, &locals).join() {
            let escalator = escalator_map.get(&step.escalator_id).unwrap();

            if step_local.translation().y + step.height * 0.5 >= escalator.top {
                info!("Hit top");
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        step.x_velocity = escalator.speed;
                        step.y_velocity = -escalator.speed;
                        step.push_velocity = 0.;
                    }
                    Direction::COUNTERCLOCKWISE => {
                        step.x_velocity = 0.;
                        step.y_velocity = -escalator.speed;
                        step.push_velocity = -escalator.speed;
                    }
                }
                continue;
            }
            if step_local.translation().x + step.width * 0.5 >= escalator.right {
                info!("escalator.right: {}", escalator.right);
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
            if (step_local.translation().x - step.width * 0.5 <= escalator.left)
                && (step_local.translation().y - step.height * 0.5 <= escalator.bottom)
            {
                info!("Hit middle corner");
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        step.x_velocity = 0.;
                        step.y_velocity = escalator.speed;
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
