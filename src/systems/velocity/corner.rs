use crate::components::{Escalator, Rectangle, Step, Velocity};
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

            step_velocity.x = match step.side {
                crate::components::Side::VERTICAL => 0.,
                crate::components::Side::HORIZONTAL => -escalator.speed,
                crate::components::Side::DIAGONAL => escalator.speed,
            };
            step_velocity.y = match step.side {
                crate::components::Side::VERTICAL => escalator.speed,
                crate::components::Side::HORIZONTAL => 0.,
                crate::components::Side::DIAGONAL => -escalator.speed,
            };
        }
    }

    //     for (step, step_velocity, step_transform, step_rectangle) in
    //         (&mut steps, &mut velocities, &transforms, &rectangles).join()
    //     {
    //         let escalator = escalators.get(step.escalator).unwrap();
    //         let escalator_transform = transforms.get(step.escalator).unwrap();
    //         let escalator_rectangle = rectangles.get(step.escalator).unwrap();
    //         let escalator_box = BoundingBox::new(escalator_rectangle, escalator_transform);
    //         let step_box = BoundingBox::new(step_rectangle, step_transform);

    // // left edge
    // if step_box.left <= escalator_box.left
    //     && step_box.top < escalator_box.top
    //     && step_box.bottom > escalator_box.bottom
    // {
    //     match escalator.direction {
    //         Direction::CLOCKWISE => {
    //             up_left(step, step_velocity, escalator.speed);
    //         }
    //         Direction::COUNTERCLOCKWISE => {
    //             down_left(step, step_velocity, escalator.speed);
    //         }
    //     }
    // }
    //         // diagonal edge
    //         else if step_box.left > escalator_box.left
    //             && step_box.right < escalator_box.right
    //             && step_box.top < escalator_box.top
    //             && step_box.bottom > escalator_box.bottom
    //         {
    //             match escalator.direction {
    //                 Direction::CLOCKWISE => {
    //                     down_right_diag(step, step_velocity, escalator.speed);
    //                 }
    //                 Direction::COUNTERCLOCKWISE => {
    //                     up_left_diag(step, step_velocity, escalator.speed);
    //                 }
    //             }
    //         }
    //         // bottom edge
    //         else if step_box.bottom <= escalator_box.bottom
    //             && step_box.left > escalator_box.left
    //             && step_box.right < escalator_box.right
    //         {
    //             match escalator.direction {
    //                 Direction::CLOCKWISE => {
    //                     left_bottom(step, step_velocity, escalator.speed);
    //                 }
    //                 Direction::COUNTERCLOCKWISE => {
    //                     right_bottom(step, step_velocity, escalator.speed);
    //                 }
    //             }
    //         }
    //         // top left corner
    //         else if step_box.top >= escalator_box.top {
    //             match escalator.direction {
    //                 Direction::CLOCKWISE => {
    //                     down_right_diag(step, step_velocity, escalator.speed);
    //                 }
    //                 Direction::COUNTERCLOCKWISE => down_left(step, step_velocity, escalator.speed),
    //             }
    //         }
    //         // bottom right corner
    //         else if step_box.right >= escalator_box.right {
    //             match escalator.direction {
    //                 Direction::CLOCKWISE => {
    //                     left_bottom(step, step_velocity, escalator.speed);
    //                 }
    //                 Direction::COUNTERCLOCKWISE => {
    //                     up_left_diag(step, step_velocity, escalator.speed);
    //                 }
    //             }
    //         }
    //         // middle corner
    //         else if (step_box.left <= escalator_box.left)
    //             && (step_box.bottom <= escalator_box.bottom)
    //         {
    //             match escalator.direction {
    //                 Direction::CLOCKWISE => {
    //                     up_left(step, step_velocity, escalator.speed);
    //                 }
    //                 Direction::COUNTERCLOCKWISE => {
    //                     right_bottom(step, step_velocity, escalator.speed);
    //                 }
    //             }
    //         }
    //     }
    // }
}

// fn up_left(step: &mut Step, velocity: &mut Velocity, speed: f32) {
//     velocity.x = 0.;
//     velocity.y = speed;
//     step.push_velocity = 0.;
// }

// fn down_left(step: &mut Step, velocity: &mut Velocity, speed: f32) {
//     velocity.x = 0.;
//     velocity.y = -speed;
//     step.push_velocity = -speed;
// }

// fn left_bottom(step: &mut Step, velocity: &mut Velocity, speed: f32) {
//     velocity.x = -speed;
//     velocity.y = 0.;
//     step.push_velocity = speed;
// }

// fn right_bottom(step: &mut Step, velocity: &mut Velocity, speed: f32) {
//     velocity.x = speed;
//     velocity.y = 0.;
//     step.push_velocity = 0.;
// }

// fn up_left_diag(step: &mut Step, velocity: &mut Velocity, speed: f32) {
//     velocity.x = -speed;
//     velocity.y = speed;
//     step.push_velocity = 0.;
// }

// fn down_right_diag(step: &mut Step, velocity: &mut Velocity, speed: f32) {
//     velocity.x = speed;
//     velocity.y = -speed;
//     step.push_velocity = 0.;
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut t = Transform::default();
        t.face_towards(Vector3::new(0., 50., 0.), Vector3::new(0., 0., 1.));

        let x = Vector3::new(0., 0., 0.);
        let y = Vector3::new(0., 10., 0.);

        println!("{}", x.metric_distance(&y));
        t.move_forward(60.);
        print!("{:?}", t.translation());
    }
}
