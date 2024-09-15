use crate::{EngineState, Position};

pub trait Actor {
    fn draw(&self, context: &ActorContext);
    fn update(&self, context: &mut ActorContext) -> Option<EngineState>;
}
#[derive(Debug)]
pub struct ActorContext {
    pub position: Position,
    pub scale: f32,
}

impl ActorContext {
    pub fn new(position: Position, scale: f32) -> Self {
        Self { position, scale }
    }
}
