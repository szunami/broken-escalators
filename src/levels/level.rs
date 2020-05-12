use serde::{Serialize, Deserialize};
use crate::components::Direction;

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelConfig {
    pub escalators: Vec<EscalatorConfig>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EscalatorConfig {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub num_steps: i32,
    pub speed: f32,
    pub direction: Direction,
}
