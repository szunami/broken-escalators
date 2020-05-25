use crate::components::{Atop, GridLocation, Rectangle,  Escalator, Step, Thing, BaseEntity};
use crate::{
    resources::RewindableClock, utils::overlap_exists, utils::x_overlap, utils::BoundingBox,
};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entity, Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use std::collections::HashMap;

#[derive(SystemDesc)]
pub struct EscalatorCorrectionSystem;

impl<'s> System<'s> for EscalatorCorrectionSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Rectangle>,
        ReadStorage<'s, Atop>,
        WriteStorage<'s, GridLocation>,
    );

    fn run(
        &mut self,
        (entities, clock, things, steps, escalators, rectangles, atops, mut grid_locations): Self::SystemData,
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

                let mut step_grid_location = grid_locations.get_mut(step_entity).unwrap();
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
                                        x_overlap(&step_box, &other_step_box)
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

        for (_escalator, escalator_entity, mut escalator_grid_location) in (&escalators, &entities, &mut grid_locations).join() {
            escalator_corrections.get(&escalator_entity).map(|correction| {
                escalator_grid_location.x += correction;
            });
        }

        for (step, mut step_grid_location) in (&steps, &mut grid_locations).join() {
            escalator_corrections.get(&step.escalator).map(|correction| {
                step_grid_location.x += correction;
            });
        }


        //     let step_grid_location = grid_locations.get(step_entity).unwrap().clone();
        //     let step_box = BoundingBox::new(&step_rectangle, &step_grid_location);

        //     let mut thing_grid_location = grid_locations.get_mut(thing_entity).unwrap();
        //     let thing_box = BoundingBox::new(thing_rectangle, thing_grid_location);

        //     if overlap_exists(&thing_box, &step_box) {
        //         debug!("Found overlap between {:?} and {:?}", thing_box, step_box);
        //         thing_grid_location.x += x_overlap(&thing_box, &step_box);
        //     }
        // }
    }
}
