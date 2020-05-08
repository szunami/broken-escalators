use crate::components::{Thing, Velocity};
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
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (clock, things, velocities, mut transforms, time): Self::SystemData) {
        if !clock.going_forwards() {
            return;
        }
        for (_thing, thing_transform, thing_velocity) in
            (&things, &mut transforms, &velocities).join()
        {
            thing_transform.prepend_translation_x(thing_velocity.x * time.delta_seconds());
            thing_transform.prepend_translation_y(thing_velocity.y * time.delta_seconds());
        }
    }
}
