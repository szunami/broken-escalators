use crate::{components::Direction, utils::ColorFlag};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelConfig {
    pub escalators: Vec<EscalatorConfig>,
    pub things: Vec<ThingConfig>,
    pub platforms: Vec<PlatformConfig>,
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
    pub color_flag: ColorFlag,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThingConfig {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformConfig {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
