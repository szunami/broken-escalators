use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, System, SystemData, ReadStorage, WriteStorage},
};
use crate::components::{Step, Thing, GridLocation, Rectangle};
use crate::{utils::BoundingBox, utils::x_overlap, utils::overlap_exists, resources::DownKeys};
use amethyst::input::VirtualKeyCode;

#[derive(SystemDesc)]
pub struct ThingCorrectionSystem;

impl<'s> System<'s> for ThingCorrectionSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, DownKeys>,
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Step>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, GridLocation>,
    );

    fn run(&mut self, (entities, down_keys, things, steps, rectangles, mut grid_locations): Self::SystemData) {
        if !down_keys.key_downs().contains(&VirtualKeyCode::Space) {
            return;
        }
        for (thing, thing_entity, thing_rectangle) in (&things, &entities, &rectangles).join() {
            // let thing_box = BoundingBox::new(thing_rectangle, thing_gr)
            for (step, step_entity, step_rectangle) in (&steps, &entities, &rectangles).join() {

                let step_grid_location = grid_locations.get(step_entity).unwrap().clone();
                let step_box = BoundingBox::new(step_rectangle, step_grid_location);

                let mut thing_grid_location = grid_locations.get_mut(thing_entity).unwrap();
                let thing_box = BoundingBox::new(thing_rectangle, thing_grid_location);

                if overlap_exists(&thing_box, &step_box) {
                    info!("Found overlap between {:?} and {:?}", thing_box, step_box);
                    thing_grid_location.x = thing_grid_location.x + x_overlap(&thing_box, &step_box);
                }
            }
        }
    }
}