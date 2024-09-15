use buji2::*;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), String> {
    let mut game_engine = GameEngine::builder()
        .set_window("Tower Defense Game", 800, 400)
        .fps(30)
        .background_color(100, 100, 150)
        .build();

    game_engine.world.add_actor(
        Player::new(1, "Legolas".to_string()),
        ActorContext::new(Position::new(10, 10), 1.5),
    );

    game_engine.run()?;

    Ok(())
}

pub struct Player {
    pub id: usize,
    pub nick_name: String,
    pub health: Option<u8>,
}

impl Player {
    pub fn new(id: usize, nick_name: String) -> Player {
        Self {
            id,
            nick_name,
            health: Some(100),
        }
    }
}

impl Actor for Player {
    fn draw(&self, context: &ActorContext) {
        sleep(Duration::from_millis(500));
        println!("Player {}-{} drawn. {:?}", self.id, self.nick_name, context);
    }

    fn update(&self, context: &mut ActorContext) -> Option<EngineState> {
        context.position.x += 1;
        context.position.y += 1;
        sleep(Duration::from_millis(500));
        println!("Player {}-{} update", self.id, self.nick_name);
        None
    }
}
