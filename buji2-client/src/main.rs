use buji2::*;
// use std::thread::sleep;
// use std::time::Duration;

fn main() -> Result<(), String> {
    let mut game_engine = GameEngine::builder()
        .set_window("Tower Defense Game", 800, 400)
        .fps(30)
        .background_color(100, 100, 150)
        .build();

    game_engine.world.add_actor(
        Player::new(1, "Legolas".to_string()),
        ActorContext::new(
            Position::new(200, 0),
            ScaleFactor(2.),
            Rotation::ZERO,
            "../assets/hero.png".to_string(),
        ),
    );

    game_engine.world.add_actor(
        Tower {
            name: "Gate East North".to_string(),
            power: 100.,
        },
        ActorContext {
            position: Position::new(500, 0),
            scale: ScaleFactor(2.),
            image_path: "../assets/tile.png".to_string(),
            ..Default::default()
        },
    );

    game_engine.world.add_actor(
        Tower {
            name: "Gate West North".to_string(),
            power: 100.,
        },
        ActorContext {
            position: Position::new(100, 0),
            scale: ScaleFactor(2.),
            image_path: "../assets/tile.png".to_string(),
            ..Default::default()
        },
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
        println!("Player {}-{} drawn. {:?}", self.id, self.nick_name, context);
    }

    fn update(&self, context: &mut ActorContext) -> Option<EngineState> {
        if context.position.y < 200 {
            context.position.y += 1;
        }
        println!("Player {}-{} update", self.id, self.nick_name);
        None
    }
}

pub struct Tower {
    name: String,
    power: f32,
}

impl Actor for Tower {
    fn draw(&self, context: &ActorContext) {
        println!("Tower {}-{} drawn. {:?}", self.name, self.power, context);
    }

    fn update(&self, context: &mut ActorContext) -> Option<EngineState> {
        println!("Tower {}-{} update. {:?}", self.name, self.power, context);
        None
    }
}
