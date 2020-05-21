use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

#[derive(Clone, PartialEq, Debug)]
pub enum Side {
    TOPPER_LEFT_CORNER,
    LOWER_RIGHT_CORNER,
    LOWER_LEFT_CORNER,
    VERTICAL,
    HORIZONTAL,
    DIAGONAL,
}

impl Side {
    // pub fn base_x_component(&self) -> i32 {
    //     match self {
    //         crate::components::Side::TOPPER_LEFT_CORNER => 1,
    //         crate::components::Side::LOWER_RIGHT_CORNER => -1,
    //         crate::components::Side::LOWER_LEFT_CORNER => 0,
    //         crate::components::Side::VERTICAL => 0,
    //         crate::components::Side::HORIZONTAL => -1,
    //         crate::components::Side::DIAGONAL => 1,
    //     }
    // }

    // pub fn base_y_component(&self) -> i32 {
    //     match self {
    //         crate::components::Side::TOPPER_LEFT_CORNER => -1,
    //         crate::components::Side::LOWER_RIGHT_CORNER => 0,
    //         crate::components::Side::LOWER_LEFT_CORNER => 1,
    //         crate::components::Side::VERTICAL => 1,
    //         crate::components::Side::HORIZONTAL => 0,
    //         crate::components::Side::DIAGONAL => -1,
    //     }
    // }

    pub fn is_corner(&self) -> bool {
        match self {
            crate::components::Side::TOPPER_LEFT_CORNER => true,
            crate::components::Side::LOWER_RIGHT_CORNER => true,
            crate::components::Side::LOWER_LEFT_CORNER => true,
            crate::components::Side::VERTICAL => false,
            crate::components::Side::HORIZONTAL => false,
            crate::components::Side::DIAGONAL => false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Step {
    pub escalator: Entity,
    pub thing_atop: Option<Entity>,
    pub side: Side,
}

impl Component for Step {
    type Storage = DenseVecStorage<Self>;
}

impl Step {
    pub fn new(escalator: Entity, side: Side) -> Step {
        Step {
            escalator,
            thing_atop: None,
            side,
        }
    }
}
