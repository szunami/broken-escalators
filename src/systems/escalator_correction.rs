use crate::components::{Atop, BaseEntity, Escalator, GridLocation, Rectangle, Step};
use crate::{
    resources::RewindableClock, utils::overlap_exists, utils::x_overlap, utils::BoundingBox,
};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use std::collections::HashMap;

#[derive(SystemDesc)]
pub struct EscalatorCorrectionSystem;

impl<'s> System<'s> for EscalatorCorrectionSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Rectangle>,
        ReadStorage<'s, Atop>,
        WriteStorage<'s, GridLocation>,
    );

    fn run(
        &mut self,
        (entities, clock, steps, escalators, rectangles, atops, mut grid_locations): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }
        let mut escalator_corrections: HashMap<Entity, i32> = HashMap::new();

        // do we need to move step?
        for (step, step_entity, step_rectangle) in (&steps, &entities, &rectangles).join() {
            for (other_step, other_step_entity, other_step_rectangle) in
                (&steps, &entities, &rectangles).join()
            {
                // only worried about cross escalator corrections for now
                if step.escalator == other_step.escalator {
                    continue;
                }

                let other_step_grid_location =
                    grid_locations.get(other_step_entity).unwrap().clone();
                let other_step_box =
                    BoundingBox::new(&other_step_rectangle, &other_step_grid_location);

                let step_grid_location = grid_locations.get_mut(step_entity).unwrap();
                let step_box = BoundingBox::new(&step_rectangle, &step_grid_location);
                if overlap_exists(&step_box, &other_step_box) {
                    info!(
                        "Found overlap between {:?} and {:?}",
                        step_entity, other_step_entity
                    );
                    info!("x overlap is {:?}", x_overlap(&step_box, &other_step_box));
                    let step_atop = atops.get(step.escalator).unwrap();

                    // is step atop other_step's escalator?
                    for base_entity in step_atop.bases.iter() {
                        match base_entity {
                            BaseEntity::Step(entity) => {
                                // we're atop a step; is that step in other_step's escalator?
                                let step_step_is_atop = steps.get(*entity).unwrap();
                                if step_step_is_atop.escalator == other_step.escalator {
                                    // we're atop other step's escalator! we get corrected
                                    info!("Need to apply correction to this step");
                                    escalator_corrections.insert(
                                        step.escalator,
                                        x_overlap(&step_box, &other_step_box),
                                    );
                                }
                            }
                            // not worried about other cases now
                            _ => {}
                        }
                    }
                }
            }
        }

        info!("Corrections: {:?}", escalator_corrections);

        for (_escalator, escalator_entity, escalator_grid_location) in
            (&escalators, &entities, &mut grid_locations).join()
        {
            escalator_corrections
                .get(&escalator_entity)
                .map(|correction| {
                    escalator_grid_location.x += correction;
                });
        }

        for (step, step_grid_location) in (&steps, &mut grid_locations).join() {
            escalator_corrections
                .get(&step.escalator)
                .map(|correction| {
                    step_grid_location.x += correction;
                });
        }
    }
}
