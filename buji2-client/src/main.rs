use buji2::{EngineState, Game, GameEngine};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), String> {
    let my_game = MyGame::new();

    let mut game_engine = GameEngine::builder()
        .set_window("Tower Defense Game", 800, 400)
        .fps(30)
        .background_color(100, 100, 150)
        .build(Box::new(my_game));

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

pub struct MyGame {
    pub players: Vec<Player>,
    pub level: u8,
}

impl MyGame {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            level: 0,
        }
    }
}

impl Default for MyGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Game for MyGame {
    fn draw(&self) {
        sleep(Duration::from_millis(500));
        println!("Draw operations...");
    }

    fn update(&mut self) -> EngineState {
        println!("Update operations...");
        EngineState::Running
    }
}
