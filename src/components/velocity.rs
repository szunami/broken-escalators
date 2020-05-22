use amethyst::ecs::prelude::{Component, DenseVecStorage};

use amethyst::core::math::Vector3;

#[derive(Clone, Debug)]
pub struct Velocity {
    pub intrinsic: Vector3<i32>,
    pub absolute: Vector3<i32>,
}

impl<'s> Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl<'s> Velocity {
    pub fn default() -> Velocity {
        Velocity {
            intrinsic: Vector3::new(0, 0, 0),
            absolute: Vector3::new(0, 0, 0),
        }
    }

    // pub fn new(x: i32, y: i32) -> Velocity {
    //     Velocity { x, y }
    // }
}
