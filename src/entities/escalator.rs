use super::initialize_step;
use crate::components::{Direction, Escalator};
use amethyst::{core::transform::Transform, prelude::*, renderer::SpriteRender};

pub fn initialize_escalator(
    world: &mut World,
    escalator_id: i32,
    x: f32,
    y: f32,
    escalator_width: f32,
    escalator_height: f32,
    num_steps: i32,
    speed: f32,
    direction: Direction,
    step_sprite: SpriteRender,
) {
    let step_height = escalator_height / (num_steps as f32);
    let step_width = escalator_width / (num_steps as f32);
    let left_arm_x = x - 0.5 * escalator_width + 0.5 * step_width;
    let bottom_arm_y = y - 0.5 * escalator_height + 0.5 * step_height;

    info!("step height: {}", step_height);
    info!("left_arm_x: {}", left_arm_x);
    // create left arm
    {
        let x_velocity = 0.;
        let y_velocity = match direction {
            Direction::CLOCKWISE => speed,
            Direction::COUNTERCLOCKWISE => -speed,
        };
        let push_velocity = match direction {
            Direction::CLOCKWISE => 0.,
            Direction::COUNTERCLOCKWISE => -speed,
        };
        for step_index in 1..(num_steps - 1) {
            let step_y = bottom_arm_y + (step_index as f32) * step_height;
            initialize_step(
                world,
                escalator_id,
                left_arm_x,
                step_y,
                x_velocity,
                y_velocity,
                push_velocity,
                step_width,
                step_height,
                step_sprite.clone(),
            );
        }
    }
    // // create top left corner
    {
        let x_velocity = match direction {
            Direction::CLOCKWISE => speed,
            Direction::COUNTERCLOCKWISE => 0.,
        };
        let y_velocity = -speed;
        let push_velocity = match direction {
            Direction::CLOCKWISE => 0.,
            Direction::COUNTERCLOCKWISE => -speed,
        };
        let step_y = bottom_arm_y + ((num_steps - 1) as f32) * step_height;
        initialize_step(
            world,
            escalator_id,
            left_arm_x,
            step_y,
            x_velocity,
            y_velocity,
            push_velocity,
            step_width,
            step_height,
            step_sprite.clone(),
        );
    }
    // create diag
    {
        for step_index in 1..(num_steps - 1) {
            let x_velocity = match direction {
                Direction::CLOCKWISE => speed,
                Direction::COUNTERCLOCKWISE => -speed,
            };
            let y_velocity = match direction {
                Direction::CLOCKWISE => -speed,
                Direction::COUNTERCLOCKWISE => speed,
            };
            let push_velocity = 0.;
            let step_x = left_arm_x + (step_index as f32) * step_width;
            let step_y = bottom_arm_y + ((num_steps - step_index - 1) as f32) * step_height;
            initialize_step(
                world,
                escalator_id,
                step_x,
                step_y,
                x_velocity,
                y_velocity,
                push_velocity,
                step_width,
                step_height,
                step_sprite.clone(),
            );
        }
    }
    // // create bottom right corner
    {
        let x_velocity = -speed;
        let y_velocity = match direction {
            Direction::CLOCKWISE => 0.,
            Direction::COUNTERCLOCKWISE => speed,
        };
        let push_velocity = match direction {
            Direction::CLOCKWISE => speed,
            Direction::COUNTERCLOCKWISE => 0.,
        };
        let step_x = left_arm_x + ((num_steps - 1) as f32) * step_width;
        initialize_step(
            world,
            escalator_id,
            step_x,
            bottom_arm_y,
            x_velocity,
            y_velocity,
            push_velocity,
            step_width,
            step_height,
            step_sprite.clone(),
        );
    }
    // create bottom
    {
        let x_velocity = match direction {
            Direction::CLOCKWISE => -speed,
            Direction::COUNTERCLOCKWISE => speed,
        };
        let y_velocity = 0.;
        let push_velocity = 0.;
        for step_index in 1..(num_steps - 1) {
            let step_x = left_arm_x + (step_index as f32) * step_width;
            initialize_step(
                world,
                escalator_id,
                step_x,
                bottom_arm_y,
                x_velocity,
                y_velocity,
                push_velocity,
                step_width,
                step_height,
                step_sprite.clone(),
            );
        }
    } // create lower left corner
    {
        {
            let x_velocity = match direction {
                Direction::CLOCKWISE => 0.,
                Direction::COUNTERCLOCKWISE => speed,
            };
            let y_velocity = match direction {
                Direction::CLOCKWISE => speed,
                Direction::COUNTERCLOCKWISE => 0.,
            };
            let push_velocity = 0.;
            initialize_step(
                world,
                escalator_id,
                left_arm_x,
                bottom_arm_y,
                x_velocity,
                y_velocity,
                push_velocity,
                step_width,
                step_height,
                step_sprite,
            );
        }
    }
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    world
        .create_entity()
        .with(Escalator::new(
            escalator_id,
            escalator_width,
            escalator_height,
            speed,
            direction,
        ))
        .with(transform)
        .build();
}
