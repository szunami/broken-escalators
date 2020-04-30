use crate::components::{Direction, Escalator};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, WriteStorage},
};
use std::collections::HashMap;

pub struct Box {
    pub speed: f32,
    pub direction: Direction,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub fn escalator_bounds_read(
    locals: &ReadStorage<Transform>,
    escalators: &ReadStorage<Escalator>,
) -> HashMap<i32, Box> {
    let mut escalator_map = HashMap::new();

    for (escalator, escalator_local) in (escalators, locals).join() {
        escalator_map.insert(
            escalator.id,
            Box {
                direction: escalator.direction,
                speed: escalator.speed,
                left: escalator_local.translation().x - escalator.width * 0.5,
                right: escalator_local.translation().x + escalator.width * 0.5,
                top: escalator_local.translation().y + escalator.height * 0.5,
                bottom: escalator_local.translation().y - escalator.height * 0.5,
            },
        );
    }
    escalator_map
}

pub fn escalator_bounds_write(
    locals: &WriteStorage<Transform>,
    escalators: &ReadStorage<Escalator>,
) -> HashMap<i32, Box> {
    let mut escalator_map = HashMap::new();

    for (escalator, escalator_local) in (escalators, locals).join() {
        escalator_map.insert(
            escalator.id,
            Box {
                direction: escalator.direction,
                speed: escalator.speed,
                left: escalator_local.translation().x - escalator.width * 0.5,
                right: escalator_local.translation().x + escalator.width * 0.5,
                top: escalator_local.translation().y + escalator.height * 0.5,
                bottom: escalator_local.translation().y - escalator.height * 0.5,
            },
        );
    }
    escalator_map
}
