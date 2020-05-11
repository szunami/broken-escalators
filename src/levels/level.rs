use serde::{Serialize, Deserialize};
use crate::components::Direction;

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelConfig {
    escalators: Vec<EscalatorConfig>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EscalatorConfig {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    num_steps: i32,
    speed: f32,
    direction: Direction,
}
