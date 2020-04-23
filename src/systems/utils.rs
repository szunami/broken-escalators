use crate::components::Escalator;
use std::collections::HashMap;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

pub struct Box {
    pub speed: f32,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

pub fn escalator_bounds_read(
    locals: &ReadStorage<Transform>,
    escalators: &ReadStorage<Escalator>,
) -> HashMap<i32, Box> {
    let mut escalator_map = HashMap::new();

    for (escalator, escalator_local) in (escalators, locals).join() {
        escalator_map.insert(escalator.id,
        Box {
            speed: escalator.speed,
            left: escalator_local.translation().x - escalator.width * 0.5,
            right: escalator_local.translation().x + escalator.width * 0.5,
            top: escalator_local.translation().y + escalator.height * 0.5,
            bottom: escalator_local.translation().y - escalator.height * 0.5,
        });
    }
    return escalator_map;
}

pub fn escalator_bounds_write(
    locals: &WriteStorage<Transform>,
    escalators: &ReadStorage<Escalator>,
) -> HashMap<i32, Box> {
    let mut escalator_map = HashMap::new();

    let z = (escalators, locals).join();

    for (escalator, escalator_local) in (escalators, locals).join() {
        escalator_map.insert(escalator.id,
        Box {
            speed: escalator.speed,
            left: escalator_local.translation().x - escalator.width * 0.5,
            right: escalator_local.translation().x + escalator.width * 0.5,
            top: escalator_local.translation().y + escalator.height * 0.5,
            bottom: escalator_local.translation().y - escalator.height * 0.5,
        });
    }
    return escalator_map;
}