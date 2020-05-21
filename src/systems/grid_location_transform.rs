use crate::components::GridLocation;
use amethyst::core::transform::Transform;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join,  ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct GridLocationTransformSystem;

impl<'s> System<'s> for GridLocationTransformSystem {
    type SystemData = (ReadStorage<'s, GridLocation>, WriteStorage<'s, Transform>);

    fn run(&mut self, (grid_locations, mut transforms): Self::SystemData) {
        for (grid_location, mut transform) in (&grid_locations, &mut transforms).join() {
            transform.set_translation_x(32. * grid_location.x as f32);
            transform.set_translation_y(32. * grid_location.y as f32);
        }
    }
}
