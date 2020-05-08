use crate::{
    components::{Platform, Thing},
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
    );

    fn run(&mut self, (things, platforms, transforms): Self::SystemData) {
        let mut all_atop = true;
        for (thing, thing_transform) in (&things, &transforms).join() {
            let thing_box = BoundingBox::new(thing.width, thing.height, thing_transform);
            if thing_box.top < 0. {
                warn!("You lose!");
            }

            let mut atop_some = false;
            for (platform, platform_transform) in (&platforms, &transforms).join() {
                let platform_bounds =
                    BoundingBox::new(platform.width, platform.height, platform_transform);
                atop_some = atop_some || is_atop(&thing_box, &platform_bounds);
            }
            all_atop = all_atop && atop_some;

        }

        if all_atop {
            warn!("You win!");
        }
    }
}
