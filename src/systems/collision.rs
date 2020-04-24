use crate::components::Thing;
use crate::{components::Step, utils::BoundsProvider};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Thing>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Step>,
    );

    fn run(&mut self, (mut things, locals, steps): Self::SystemData) {
        for (mut thing, thing_local) in (&mut things, &locals).join() {
            let thing_bounds = BoundsProvider::new(thing.width, thing.height, thing_local);
            let mut any_collisions = false;

            for (step, step_local) in (&steps, &locals).join() {
                let step_bounds = BoundsProvider::new(step.width, step.height, step_local);

                if thing_bounds.bottom() < step_bounds.top()
                    && overlaps(
                        thing_bounds.left(),
                        thing_bounds.right(),
                        step_bounds.left(),
                        step_bounds.right(),
                    )
                {
                    warn!("Top collision");
                    thing.x_velocity = step.x_velocity;
                    thing.y_velocity = step.y_velocity;
                    warn!("velocity: {} {}", thing.x_velocity, thing.y_velocity);
                    any_collisions = true;
                }

                if (thing_bounds.right() == step_bounds.left()
                    || thing_bounds.left() == thing_bounds.right())
                    && overlaps(
                        thing_bounds.bottom(),
                        thing_bounds.top(),
                        step_bounds.bottom(),
                        step_bounds.top(),
                    )
                {
                    warn!("Side collision");
                    thing.x_velocity = step.x_velocity;
                    // any_collisions = false;
                }
            }
            if !any_collisions {
                thing.y_velocity = -5.
            };
        }
    }
}

fn overlaps(a: f32, b: f32, x: f32, y: f32) -> bool {
    return (a < x && b > x) || (x < a && y > a);
}
