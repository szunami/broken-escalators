use crate::components::Thing;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};
use crate::{utils::BoundsProvider, components::Step};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Thing>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Step>,
    );

    fn run(&mut self, (mut things, locals, steps): Self::SystemData) {
        for (mut thing, thing_local) in (&things, &locals).join() {
            let thing_bounds = BoundsProvider::new(thing, thing_local);

            for (step, step_local) in (&steps, &locals).join() {
                let step_bounds = BoundsProvider::new(step, step_local);

                if thing_bounds.bottom() == step_bounds.top() && 
                overlaps(thing_bounds.left(), thing_bounds.right(), step_bounds.left(), step_bounds.right()){
                    thing.x_velocity = step.x_velocity;
                    thing.y_velocity = step.y_velocity;
                }

                if (thing_bounds.right() == step_bounds.left() || thing_bounds.left() == thing_bounds.right()) &&
                overlaps(thing_bounds.bottom(), thing_bounds.top(), step_bounds.bottom(), step_bounds.top()) {
                    thing.x_velocity = step.x_velocity;
                }
            }
        }

    }

}

fn overlaps(a: f32, b: f32, x: f32, y: f32) -> bool {
    return (a <= x && b >= x) || (x <= a && y >= a);
}