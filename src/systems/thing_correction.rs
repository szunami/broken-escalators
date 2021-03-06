use crate::{
    resources::RewindableClock,
    utils::{overlap_exists, y_overlap, BoundingBox, x_overlap},
    entities::ARENA_HEIGHT,
components::{GridLocation, Rectangle, Step, Thing}};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct ThingCorrectionSystem;

impl<'s> System<'s> for ThingCorrectionSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, RewindableClock>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, GridLocation>,
    );

    fn run(
        &mut self,
        (entities, clock, things, steps, rectangles, mut grid_locations): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }
        for (_thing, thing_entity, thing_rectangle) in (&things, &entities, &rectangles).join() {
            for (_step, step_entity, step_rectangle) in (&steps, &entities, &rectangles).join() {
                let step_grid_location = grid_locations.get(step_entity).unwrap().clone();
                let step_box = BoundingBox::new(&step_rectangle, &step_grid_location);

                let mut thing_grid_location = grid_locations.get_mut(thing_entity).unwrap();
                let thing_box = BoundingBox::new(thing_rectangle, thing_grid_location);

                if overlap_exists(&thing_box, &step_box) {
                    // choose between y and x overlap based on abs?
                    debug!("Found overlap between {:?} and {:?}", thing_box, step_box);
                    let x = x_overlap(&thing_box, &step_box);
                    let y = y_overlap(&thing_box, &step_box);
                    if x.abs() < y.abs() {
                        thing_grid_location.x += x;
                    } else {
                        thing_grid_location.y += y;
                    }
                }
            }

            let mut thing_grid_location = grid_locations.get_mut(thing_entity).unwrap();
            if thing_grid_location.y < 0 {
                thing_grid_location.y += (ARENA_HEIGHT / 32.) as i32;
            }
        }
    }
}
