use crate::{
    components::{Platform, Rectangle, Thing, Color},
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
        ReadStorage<'s, Color>,
    );

    fn run(&mut self, (things, platforms, transforms, rectangles, colors): Self::SystemData) {
        let mut all_things_atop_some_platform = true;
        for (_thing, thing_transform, thing_rectangle, thing_color) in (&things, &transforms, &rectangles, &colors).join()
        {
            let thing_box = BoundingBox::new(thing_rectangle, thing_transform);
            if thing_box.top < 0. {
                warn!("You lose!");
            }

            let mut atop_some_platform = false;
            for (_platform, platform_transform, platform_rectangle, platform_color) in
                (&platforms, &transforms, &rectangles, &colors).join()
            {
                let platform_bounds = BoundingBox::new(platform_rectangle, platform_transform);
                atop_some_platform = atop_some_platform || (is_atop(&thing_box, &platform_bounds) && thing_color.color_flag == platform_color.color_flag);
            }
            all_things_atop_some_platform = all_things_atop_some_platform && atop_some_platform;
        }

        if all_things_atop_some_platform {
            warn!("You win!");
        }
    }
}
