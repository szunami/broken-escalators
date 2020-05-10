use crate::components::{Step, Thing, Velocity};
use crate::resources::RewindableClock;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (clock, things, steps, velocities, mut transforms, time): Self::SystemData) {
        if !clock.going_forwards() {
            return;
        }

        for (_step, step_transform, step_velocity) in (&steps, &mut transforms, &velocities).join()
        {
            step_transform.prepend_translation_x(step_velocity.x * time.delta_seconds());
            step_transform.prepend_translation_y(step_velocity.y * time.delta_seconds());
        }

        for (_thing, thing_transform, thing_velocity) in
            (&things, &mut transforms, &velocities).join()
        {
            thing_transform.prepend_translation_x(thing_velocity.x * time.delta_seconds());
            thing_transform.prepend_translation_y(thing_velocity.y * time.delta_seconds());
        }
    }
}
