// use crate::components::{Escalator, Side, Step, Velocity};
// use crate::resources::RewindableClock;
// use amethyst::{
//     derive::SystemDesc,
//     ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
// };

// #[derive(SystemDesc)]
// pub struct CornerSystem;

// impl<'s> System<'s> for CornerSystem {
//     type SystemData = (
//         Read<'s, RewindableClock>,
//         WriteStorage<'s, Step>,
//         WriteStorage<'s, Velocity>,
//         ReadStorage<'s, Escalator>,
//     );

//     fn run(&mut self, (clock, mut steps, mut velocities, escalators): Self::SystemData) {
//         if !clock.going_forwards() {
//             return;
//         }
//         for (step, step_velocity) in (&mut steps, &mut velocities).join() {
//             let escalator = escalators.get(step.escalator).unwrap();
//             step_velocity.x = x_velocity_for_side(&step.side, &escalator);
//             step_velocity.y = y_velocity_for_side(&step.side, &escalator);
//         }
//     }
// }

// pub fn x_velocity_for_side(side: &Side, escalator: &Escalator) -> f32 {
//     escalator.speed * escalator.direction.direction_factor() * side.base_x_component()
// }

// pub fn y_velocity_for_side(side: &Side, escalator: &Escalator) -> f32 {
//     escalator.speed * escalator.direction.direction_factor() * side.base_y_component()
// }
