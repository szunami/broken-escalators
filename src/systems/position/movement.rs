use crate::components::{Step, Thing, Velocity};
use crate::resources::RewindableClock;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        WriteStorage<'s, Step>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            clock,
            things,
            mut steps,
            velocities,
            mut transforms,
            time,
        ): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }

        for (_step, step_entity, step_velocity) in (&mut steps, &entities, &velocities).join() {
            let step_transform = transforms.get_mut(step_entity).unwrap();
            step_transform.prepend_translation_x(step_velocity.x * time.delta_seconds());
            step_transform.prepend_translation_y(step_velocity.y * time.delta_seconds());
        }

        for (_thing, thing_entity, thing_velocity) in (&things, &entities, &velocities).join() {
            let thing_transform = transforms.get_mut(thing_entity).unwrap();
            thing_transform.prepend_translation_x(thing_velocity.x * time.delta_seconds());
            thing_transform.prepend_translation_y(thing_velocity.y * time.delta_seconds());
        }
    }
}
