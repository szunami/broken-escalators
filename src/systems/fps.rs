use crate::systems::utils::escalator_bounds_write;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    utils::fps_counter::FpsCounter,
};

#[derive(SystemDesc)]
pub struct FPSSystem;

impl<'s> System<'s> for FPSSystem {
    type SystemData = (Read<'s, Time>, Read<'s, FpsCounter>);

    fn run(&mut self, (time, fps_counter): Self::SystemData) {
        if time.frame_number() % 20 == 0 {
            info!("FPS: {}", fps_counter.sampled_fps());
        }
    }
}
