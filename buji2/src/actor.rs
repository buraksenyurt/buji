use crate::{EngineState, Position, Rotation, ScaleFactor};

pub trait Actor {
    fn draw(&self, context: &ActorContext);
    fn update(&self, context: &mut ActorContext) -> Option<EngineState>;
}
#[derive(Debug, Default)]
pub struct ActorContext {
    pub id: u32,
    pub position: Position,
    pub scale: ScaleFactor,
    pub rotation: Rotation,
    pub image_path: String,
}

impl ActorContext {
    pub fn new(
        id: u32,
        position: Position,
        scale: ScaleFactor,
        rotation: Rotation,
        image_path: String,
    ) -> Self {
        Self {
            id,
            position,
            scale,
            rotation,
            image_path,
        }
    }
}
