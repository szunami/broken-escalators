use super::Thing;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ThingTape {
    things: Vec<Thing>,
    locals: Vec<Transform>,
}

impl Component for ThingTape {
    type Storage = DenseVecStorage<Self>;
}

impl ThingTape {
    pub fn new() -> ThingTape {
        ThingTape {
            things: vec![],
            locals: vec![],
        }
    }

    pub fn push(&mut self, thing: Thing, local: Transform) {
        self.things.push(thing);
        self.locals.push(local);
    }
}
