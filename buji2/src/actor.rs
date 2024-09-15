use crate::{EngineState, Position, Rotation, ScaleFactor};

pub trait Actor {
    fn draw(&self, context: &ActorContext);
    fn update(&self, context: &mut ActorContext) -> Option<EngineState>;
}
#[derive(Debug, Default)]
pub struct ActorContext {
    pub position: Position,
    pub scale: ScaleFactor,
    pub rotation: Rotation,
    pub image_path: String,
}

impl ActorContext {
    pub fn new(
        position: Position,
        scale: ScaleFactor,
        rotation: Rotation,
        image_path: String,
    ) -> Self {
        Self {
            position,
            scale,
            rotation,
            image_path,
        }
    }
}
