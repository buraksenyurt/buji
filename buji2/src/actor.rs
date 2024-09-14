use crate::EngineState;

pub trait Actor {
    fn draw(&self);
    fn update(&self) -> Option<EngineState>;
}
