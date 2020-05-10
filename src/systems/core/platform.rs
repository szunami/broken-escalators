use crate::{
    components::{Platform, Rectangle, Thing},
    utils::{is_atop, BoundingBox},
};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData},
};

#[derive(SystemDesc)]
pub struct PlatformSystem;

impl<'s> System<'s> for PlatformSystem {
    type SystemData = (
        ReadStorage<'s, Thing>,
        ReadStorage<'s, Platform>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Rectangle>,
    );

    fn run(&mut self, (things, platforms, transforms, rectangles): Self::SystemData) {
        let mut all_atop = true;
        for (_thing, thing_transform, thing_rectangle) in (&things, &transforms, &rectangles).join()
        {
            let thing_box = BoundingBox::new(
                thing_rectangle,
                thing_transform,
            );
            if thing_box.top < 0. {
                warn!("You lose!");
            }

            let mut atop_some = false;
            for (_platform, platform_transform, platform_rectangle) in
                (&platforms, &transforms, &rectangles).join()
            {
                let platform_bounds = BoundingBox::new(
                    platform_rectangle,
                    platform_transform,
                );
                atop_some = atop_some || is_atop(&thing_box, &platform_bounds);
            }
            all_atop = all_atop && atop_some;
        }

        if all_atop {
            warn!("You win!");
        }
    }
}
