use crate::states::EngineState;

pub trait Game {
    fn draw(&self);
    fn update(&mut self) -> EngineState;
}
