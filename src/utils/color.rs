use amethyst::{input::VirtualKeyCode, renderer::palette::Srgba};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ColorFlag {
    WHITE,
    YELLOW,
    BLUE,
}

impl ColorFlag {
    pub fn to_srgba(self) -> Srgba {
        match self {
            ColorFlag::WHITE => Srgba::new(1.0, 1.0, 1.0, 1.0),
            ColorFlag::YELLOW => Srgba::new(0.9921875, 1.0, 0.539, 1.0),
            ColorFlag::BLUE => Srgba::new(0., 0., 1.0, 1.0),
        }
    }

    pub fn to_virtual_key(self) -> VirtualKeyCode {
        match self {
            ColorFlag::WHITE => VirtualKeyCode::W,
            ColorFlag::YELLOW => VirtualKeyCode::Y,
            ColorFlag::BLUE => VirtualKeyCode::B,
        }
    }
}
