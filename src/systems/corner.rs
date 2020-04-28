use crate::components::{Direction, Escalator, Step};
use crate::systems::utils::escalator_bounds_read;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};
#[derive(SystemDesc)]
pub struct CornerSystem;

impl<'s> System<'s> for CornerSystem {
    type SystemData = (
        WriteStorage<'s, Step>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Escalator>,
    );

    fn run(&mut self, (mut steps, locals, escalators): Self::SystemData) {
        let escalator_map = escalator_bounds_read(&locals, &escalators);

        for (step, step_local) in (&mut steps, &locals).join() {
            let escalator = escalator_map.get(&step.escalator_id).unwrap();

            if step_local.translation().y + step.height * 0.5 >= escalator.top {
                info!("Hit top");
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        step.x_velocity = escalator.speed;
                        step.y_velocity = -escalator.speed;
                    }
                    Direction::COUNTERCLOCKWISE => {
                        step.x_velocity = 0.;
                        step.y_velocity = -escalator.speed;
                        step.is_pushing = true;
                    }
                }
                continue;
            }
            if step_local.translation().x + step.width * 0.5 >= escalator.right {
                info!("Hit right corner");
                match escalator.direction {
                    Direction::CLOCKWISE => {
                        step.x_velocity = -escalator.speed;
                        step.y_velocity = 0.;
                    }
                    Direction::COUNTERCLOCKWISE => {
                        step.x_velocity = -escalator.speed;
                        step.y_velocity = escalator.speed;
                        step.is_pushing = false;
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
                        step.is_pushing = false;
                    }
                }
            }
        }
    }
}
