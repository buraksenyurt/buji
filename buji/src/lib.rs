use std::thread::sleep;
use std::time::{Duration, Instant};

pub enum MainState {
    Init,
    Running,
    Exit,
}

pub trait GameObject {
    fn draw(&self);
    fn update(&mut self) -> MainState;
}

pub struct GameEngine {
    pub game_object: Box<dyn GameObject>,
    pub fps: u32,
}

impl GameEngine {
    pub fn run(&mut self) -> Result<(), String> {
        let mut state = MainState::Init;
        let mut last_update = Instant::now();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / self.fps);

        loop {
            match state {
                MainState::Init => {
                    state = MainState::Running;
                    continue;
                }
                MainState::Running => {
                    let now = Instant::now();
                    let delta = now.duration_since(last_update);

                    self.game_object.draw();
                    state = self.game_object.update();

                    if frame_duration > delta {
                        sleep(frame_duration - delta);
                    }

                    last_update = now;
                }
                MainState::Exit => break,
            }
        }

        Ok(())
    }
}

pub struct GameEngineBuilder {
    game_object: Option<Box<dyn GameObject>>,
    fps: Option<u32>,
}

impl GameEngineBuilder {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            game_object: None,
            fps: None,
        })
    }

    pub fn setup_window(mut self) -> Result<Self, String> {
        Ok(self)
    }

    pub fn change_fps(mut self, fps: u32) -> Self {
        self.fps = Some(fps);
        self
    }

    pub fn add_game(mut self, game: Box<dyn GameObject>) -> Self {
        self.game_object = Some(game);
        self
    }
    pub fn build(self) -> Result<GameEngine, String> {
        Ok(GameEngine {
            game_object: self.game_object.unwrap(),
            fps: self.fps.unwrap(),
        })
    }
}
