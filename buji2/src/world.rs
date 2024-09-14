use crate::actor::Actor;
use crate::EngineState;

#[derive(Default)]
pub struct World {
    actors: Vec<Box<dyn Actor>>,
}

impl World {
    pub fn new() -> Self {
        Self { actors: Vec::new() }
    }

    pub fn add_actor<T: Actor + 'static>(&mut self, actor: T) {
        self.actors.push(Box::new(actor));
    }

    pub fn draw_all(&self) {
        for actor in &self.actors {
            actor.draw();
        }
    }

    pub fn update_all(&mut self) -> Option<EngineState> {
        for actor in &mut self.actors {
            if let Some(state) = actor.update() {
                return Some(state);
            }
        }
        None
    }
}
