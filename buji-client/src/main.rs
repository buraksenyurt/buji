use buji::*;
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), String> {
    let logger = Log::new(stdout());
    let my_game = MyGame::new();
    let mut buji = GameEngineBuilder::new()?
        .setup_window()?
        .change_fps(60)
        .add_game(Box::new(my_game))
        .add_logger(logger)
        .build()?;
    buji.run()
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

impl GameObject for MyGame {
    fn draw(&self) {
        sleep(Duration::from_secs(5));
        println!("Draw operations...");
    }

    fn update(&mut self) -> MainState {
        println!("Update operations...");
        MainState::Running
    }
}
