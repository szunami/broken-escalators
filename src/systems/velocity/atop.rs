use crate::{
    components::Platform,
    components::Rectangle,
    components::Step,
    components::Thing,
    components::Velocity,
    utils::{is_atop, BoundingBox},
};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
    ecs::Entity,
};

pub const GRAVITY_VELOCITY: f32 = -32.;

#[derive(SystemDesc)]
pub struct AtopSystem;

impl<'s> System<'s> for AtopSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Step>,
        ReadStorage<'s, Platform>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, Velocity>,
    );

    fn run(
        &mut self,
        (entities, things, transforms, mut steps, platforms, rectangles, mut velocities): Self::SystemData,
    ) {
        for step in (&mut steps).join() {
            step.thing_atop = None;
        }

        for (_thing, thing_entity, thing_transform, thing_rectangle) in
            (&things, &entities, &transforms, &rectangles).join()
        {
            let thing_bounds = BoundingBox::new(thing_rectangle, thing_transform);

            let mut atop_step: Option<Entity> = None;
            let mut atop_platform = false;
            let mut max_atopness = 0.;
            for (_step, step_entity, step_transform, step_rectangle) in
                (&steps, &entities, &transforms, &rectangles).join()
            {
                let step_bounds = BoundingBox::new(step_rectangle, step_transform);
                let atopness = is_atop(&thing_bounds, &step_bounds);
                if atopness && step_bounds.top > max_atopness {
                    atop_step = Some(step_entity);
                    max_atopness = step_bounds.top;
                }
            }

            for (_platform, platform_transform, platform_rectangle) in
                (&platforms, &transforms, &rectangles).join()
            {
                let platform_bounds = BoundingBox::new(platform_rectangle, platform_transform);
                let atopness = is_atop(&thing_bounds, &platform_bounds);
                if atopness && platform_bounds.top > max_atopness {
                    atop_step = None;
                    atop_platform = true;
                    max_atopness = platform_bounds.top;
                }
            }

            if let Some(step_entity) = atop_step {
                let step_velocity = velocities.get(step_entity).unwrap().clone();
                let thing_velocity = velocities.get_mut(thing_entity).unwrap();
                *thing_velocity = step_velocity.clone();
                let step = steps.get_mut(step_entity).unwrap();
                step.thing_atop = Some(thing_entity);
            } else if atop_platform {
                let thing_velocity = velocities.get_mut(thing_entity).unwrap();
                thing_velocity.x = 0.;
                thing_velocity.y = 0.;
            } else {
                let thing_velocity = velocities.get_mut(thing_entity).unwrap();
                thing_velocity.x = 0.;
                thing_velocity.y = GRAVITY_VELOCITY;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::initialize_thing, game::register_components, levels::ColorFlag,
        levels::ThingConfig,
    };
    use amethyst::{prelude::*, Error};
    use amethyst_test::prelude::*;

    #[test]
    fn test_non_atop_thing_falls() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(AtopSystem, "test", &[])
            .with_effect(|world| {
                register_components(world);
                let thing_config = ThingConfig {
                    x: 10.,
                    y: 10.,
                    width: 10.,
                    height: 10.,
                    color_flag: ColorFlag::BLUE,
                };
                // let mut transform = Transform::default();
                // transform.set_translation_xyz(thing_config.x, thing_config.y, 0.);
                let entity = initialize_thing(world, thing_config, None);
                world.insert(EffectReturn(entity));
            })
            .with_assertion(|world| {
                let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();

                let my_component_storage = world.read_storage::<Velocity>();

                let my_component = my_component_storage
                    .get(entity)
                    .expect("Entity should have a `MyComponent` component.");
                assert_eq!(my_component.x, 0.);
                assert_eq!(my_component.y, -32.);
            })
            .run()
    }
}
