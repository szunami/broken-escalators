use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

#[derive(Clone, PartialEq, Debug)]
pub enum Side {
    TopLeftCorner,
    BottomRightCorner,
    BottomLeftCorner,
    Left,
    Bottom,
    Diagonal,
}

impl Side {
    pub fn is_corner(&self) -> bool {
        match self {
            crate::components::Side::TopLeftCorner => true,
            crate::components::Side::BottomRightCorner => true,
            crate::components::Side::BottomLeftCorner => true,
            crate::components::Side::Left => false,
            crate::components::Side::Bottom => false,
            crate::components::Side::Diagonal => false,
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
