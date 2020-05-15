use crate::components::{Escalator, Rectangle, Step, Velocity, Side};
use crate::levels::Direction;
use crate::{
    resources::RewindableClock,
    utils::{contains, BoundingBox},
};
use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct CornerSystem;

impl<'s> System<'s> for CornerSystem {
    type SystemData = (
        Read<'s, RewindableClock>,
        WriteStorage<'s, Step>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Escalator>,
        ReadStorage<'s, Rectangle>,
    );

    fn run(
        &mut self,
        (clock, mut steps, mut velocities, transforms, escalators, rectangles): Self::SystemData,
    ) {
        if !clock.going_forwards() {
            return;
        }
        for (step, step_velocity, step_transform, step_rectangle) in
            (&mut steps, &mut velocities, &transforms, &rectangles).join()
        {
            let step_box = BoundingBox::new(step_rectangle, step_transform);
            let escalator = escalators.get(step.escalator).unwrap();
            let escalator_transform = transforms.get(step.escalator).unwrap();
            let escalator_rectangle = rectangles.get(step.escalator).unwrap();
            let escalator_box = BoundingBox::new(escalator_rectangle, escalator_transform);

            step_velocity.x = x_velocity_for_side(&step.side, &escalator);
            step_velocity.y = y_velocity_for_side(&step.side, &escalator);
        }
    }
}

pub fn x_velocity_for_side(side: &Side, escalator: &Escalator) -> f32 {
    escalator.speed * escalator.direction.direction_factor() * side.base_x_component()
}


pub fn y_velocity_for_side(side: &Side, escalator: &Escalator) -> f32 {
    escalator.speed * escalator.direction.direction_factor() * side.base_y_component()
}
