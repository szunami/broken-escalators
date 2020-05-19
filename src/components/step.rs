use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

#[derive(Clone, PartialEq, Debug)]
pub enum Side {
    VERTICAL,
    HORIZONTAL,
    DIAGONAL,
}

impl Side {
    pub fn base_x_component(&self) -> i32 {
        match self {
            crate::components::Side::VERTICAL => 0,
            crate::components::Side::HORIZONTAL => -1,
            crate::components::Side::DIAGONAL => 1,
        }
    }

    pub fn base_y_component(&self) -> i32 {
        match self {
            crate::components::Side::VERTICAL => 1,
            crate::components::Side::HORIZONTAL => 0,
            crate::components::Side::DIAGONAL => -1,
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
