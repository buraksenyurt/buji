use crate::actor::Actor;
use crate::ActorContext;
use std::cell::RefCell;
use std::rc::Rc;

type ActorWithContext = (Box<dyn Actor>, Rc<RefCell<ActorContext>>);

#[derive(Default)]
pub struct World {
    pub actors: Vec<ActorWithContext>,
}

impl World {
    pub fn new() -> Self {
        Self { actors: Vec::new() }
    }

    pub fn add_actor<T: Actor + 'static>(&mut self, actor: T, actor_context: ActorContext) {
        let ac = Rc::new(RefCell::new(actor_context));
        self.actors.push((Box::new(actor), ac));
    }
}
