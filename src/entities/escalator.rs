use super::initialize_step;
use crate::components::{Escalator, GridLocation, Rectangle, Side};
use crate::levels::{Direction, EscalatorConfig};
use amethyst::{core::transform::Transform, prelude::*, renderer::SpriteRender};

pub fn initialize_escalator(
    world: &mut World,
    escalator: EscalatorConfig,
    step_sprite: SpriteRender,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(escalator.x as f32 * 32., escalator.y as f32 * 32., 0.);
    info!("Registering escalator at: {:?}", transform.translation());

    let escalator_entity = world
        .create_entity()
        .with(Escalator::new(
            escalator.speed,
            escalator.direction,
            escalator.color_flag.to_virtual_key(),
        ))
        .with(GridLocation::new(escalator.x, escalator.y))
        .with(Rectangle::new(escalator.width, escalator.height))
        .with(transform)
        .build();

    let step_height = escalator.height / escalator.num_steps;
    let step_width = escalator.width / escalator.num_steps;
    let left_arm_x = escalator.x - escalator.width / 2 + step_width / 2;
    let bottom_arm_y = escalator.y - escalator.height / 2 + step_height / 2;

    // create left arm
    {
        let x_velocity = 0;
        let y_velocity = match escalator.direction {
            Direction::CLOCKWISE => escalator.speed,
            Direction::COUNTERCLOCKWISE => -escalator.speed,
        };
        for step_index in 1..(escalator.num_steps - 1) {
            let step_y = bottom_arm_y + (step_index) * step_height;
            initialize_step(
                world,
                escalator_entity,
                left_arm_x,
                step_y,
                x_velocity,
                y_velocity,
                Side::VERTICAL,
                step_width,
                step_height,
                step_sprite.clone(),
                escalator.color_flag,
            );
        }
    }
    // // create top left corner
    {
        let x_velocity = match escalator.direction {
            Direction::CLOCKWISE => escalator.speed,
            Direction::COUNTERCLOCKWISE => 0,
        };
        let y_velocity = -escalator.speed;
        let side = match escalator.direction {
            Direction::CLOCKWISE => Side::VERTICAL,
            Direction::COUNTERCLOCKWISE => Side::DIAGONAL,
        };
        let step_y = bottom_arm_y + (escalator.num_steps - 1) * step_height;
        initialize_step(
            world,
            escalator_entity,
            left_arm_x,
            step_y,
            x_velocity,
            y_velocity,
            side,
            step_width,
            step_height,
            step_sprite.clone(),
            escalator.color_flag,
        );
    }
    // create diag
    // {
    //     for step_index in 1..(escalator.num_steps - 1) {
    //         let x_velocity = match escalator.direction {
    //             Direction::CLOCKWISE => escalator.speed,
    //             Direction::COUNTERCLOCKWISE => -escalator.speed,
    //         };
    //         let y_velocity = match escalator.direction {
    //             Direction::CLOCKWISE => -escalator.speed,
    //             Direction::COUNTERCLOCKWISE => escalator.speed,
    //         };
    //         let step_x = left_arm_x + (step_index as f32) * step_width;
    //         let step_y =
    //             bottom_arm_y + ((escalator.num_steps - step_index - 1) as f32) * step_height;
    //         initialize_step(
    //             world,
    //             escalator_entity,
    //             step_x,
    //             step_y,
    //             x_velocity,
    //             y_velocity,
    //             Side::DIAGONAL,
    //             step_width,
    //             step_height,
    //             step_sprite.clone(),
    //             escalator.color_flag,
    //         );
    //     }
    // }
    // // create bottom right corner
    // {
    //     let x_velocity = -escalator.speed;
    //     let y_velocity = match escalator.direction {
    //         Direction::CLOCKWISE => 0.,
    //         Direction::COUNTERCLOCKWISE => escalator.speed,
    //     };
    //     let side = match escalator.direction {
    //         Direction::CLOCKWISE => Side::HORIZONTAL,
    //         Direction::COUNTERCLOCKWISE => Side::DIAGONAL,
    //     };
    //     let step_x = left_arm_x + ((escalator.num_steps - 1) as f32) * step_width;
    //     initialize_step(
    //         world,
    //         escalator_entity,
    //         step_x,
    //         bottom_arm_y,
    //         x_velocity,
    //         y_velocity,
    //         side,
    //         step_width,
    //         step_height,
    //         step_sprite.clone(),
    //         escalator.color_flag,
    //     );
    // }
    // create bottom
    // {
    //     let x_velocity = match escalator.direction {
    //         Direction::CLOCKWISE => -escalator.speed,
    //         Direction::COUNTERCLOCKWISE => escalator.speed,
    //     };
    //     let y_velocity = 0.;
    //     for step_index in 1..(escalator.num_steps - 1) {
    //         let step_x = left_arm_x + (step_index as f32) * step_width;
    //         initialize_step(
    //             world,
    //             escalator_entity,
    //             step_x,
    //             bottom_arm_y,
    //             x_velocity,
    //             y_velocity,
    //             Side::HORIZONTAL,
    //             step_width,
    //             step_height,
    //             step_sprite.clone(),
    //             escalator.color_flag,
    //         );
    //     }
    // }
    // create lower left corner
    // {
    //     {
    //         let x_velocity = match escalator.direction {
    //             Direction::CLOCKWISE => 0.,
    //             Direction::COUNTERCLOCKWISE => escalator.speed,
    //         };
    //         let y_velocity = match escalator.direction {
    //             Direction::CLOCKWISE => escalator.speed,
    //             Direction::COUNTERCLOCKWISE => 0.,
    //         };
    //         let side = match escalator.direction {
    //             Direction::CLOCKWISE => Side::VERTICAL,
    //             Direction::COUNTERCLOCKWISE => Side::HORIZONTAL,
    //         };
    //         initialize_step(
    //             world,
    //             escalator_entity,
    //             left_arm_x,
    //             bottom_arm_y,
    //             x_velocity,
    //             y_velocity,
    //             side,
    //             step_width,
    //             step_height,
    //             step_sprite,
    //             escalator.color_flag,
    //         );
    //     }
    // }
}
