use crate::asset_server::AssetServer;
use crate::world::World;
use crate::{GameWindow, DEFAULT_FPS, NANOS_PER_SECOND};
use logy::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Enum representing the main states of the game engine.
pub enum MainState {
    /// Initial state, where the game has not yet started.
    Init,
    /// Running state, where the game is actively running.
    Running,
    /// Runs before the exit signal. Can be use for despawning some resources.
    PreExit,
    /// Exit state, where the game should stop running.
    Exit,
}

/// A trait representing a game object. This must be implemented by and game object.
pub trait GameObject {
    /// Draw operations. Called every frame.
    ///
    /// # Arguments
    ///
    /// * `asset_server` - Reference of asset server
    ///
    fn draw(&self, asset_server: &AssetServer);
    /// Update method for game actors. This is called every frame and
    /// should return the next state of main engine.
    ///
    /// # Returns
    ///
    /// A `MainState` value indicating the next state of the engine.
    fn update(&mut self) -> MainState;
}

/// Game Engine, responsible for managing the game loop.
pub struct GameEngine {
    /// `GameObject` trait's implementation.
    pub game_object: Option<Box<dyn GameObject>>,
    /// Frames per second value for the game
    pub fps: u32,
    /// Main screen object of the game
    pub window: GameWindow,
    /// Asset manager of the game
    pub asset_server: AssetServer,
    /// World object to manage all game figures
    pub world: World<'static>,
}

impl Default for GameEngine {
    fn default() -> Self {
        Self {
            window: GameWindow::default(),
            fps: DEFAULT_FPS,
            game_object: None,
            asset_server: AssetServer::default(),
            world: World::default(),
        }
    }
}

impl GameEngine {
    /// Managing the main loop. The game loop initializes the game, runs the states and updates
    /// the game object on each frame until the `Exit` state is reached.
    ///
    /// # Returns
    ///
    /// `Result<(), String>` - Returns `Ok(())` if the main loop exists successfully,
    /// or an error message if something goes wrong.
    pub fn run(&mut self) -> Result<(), String> {
        self.window.init()?;
        linfo!(LogLevel::Info, "Initializing the game engine");

        let mut state = MainState::Init;
        let mut last_update = Instant::now();
        let frame_duration = Duration::new(0, NANOS_PER_SECOND / self.fps);

        let mut event_pump = self.window.sdl_context.as_ref().unwrap().event_pump()?;

        loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        linfo!(LogLevel::Warn, "Quit event received. Exiting...");
                        state = MainState::PreExit;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        linfo!(LogLevel::Warn, "Escaped key pressed. Exiting...");
                        state = MainState::PreExit;
                    }
                    _ => {}
                }
            }

            match state {
                MainState::Init => {
                    state = MainState::Running;
                    linfo!(LogLevel::Info, "Going to Running state");
                    continue;
                }
                MainState::Running => {
                    linfo!(LogLevel::Info, "On Running state");

                    let now = Instant::now();
                    let delta = now.duration_since(last_update);

                    self.window.cleanup();

                    if let Some(game_object) = &mut self.game_object {
                        game_object.draw(&self.asset_server);
                        state = game_object.update();
                    }

                    self.window.present();

                    if frame_duration > delta {
                        sleep(frame_duration - delta);
                    }

                    last_update = now;
                }
                MainState::PreExit => {
                    linfo!(LogLevel::Warn, "Pre Exit...");
                    state = MainState::Exit;
                    continue;
                }
                MainState::Exit => {
                    linfo!(LogLevel::Warn, "Exiting from game engine");
                    break;
                }
            }
        }

        Ok(())
    }
}

/// A builder for constructing a `GameEngine` instance. It allows for setting
/// up the game window, FPS, and the game object in a more flexible and readable way.
///
/// # Example
///
/// ```rust
/// use buji::{GameObject, MainState, GameEngineBuilder, DEFAULT_FPS};
/// use std::io::{stdout, Write};
/// use buji::AssetServer;
///
/// struct YourGameObject;
///
/// impl GameObject for YourGameObject {
///     fn draw(&self,asset_server: &AssetServer) {
///         // Draw game objects here
///     }
///
///     fn update(&mut self) -> MainState {
///         // Update game objects and return the next state
///         MainState::Running
///     }
/// }
///
/// fn main() -> Result<(), String> {
///
///     let game = YourGameObject;
///     let mut engine = GameEngineBuilder::new()?
///         .change_fps(DEFAULT_FPS)
///         .add_game(game)
///         .build()?;
///
///     Ok(())
///
/// }
/// ```
///
/// This example demonstrates how to create a simple game using `GameEngineBuilder`.
/// A custom game object `YourGameObject` is implemented and added to the engine, which is then run at 60 FPS.
pub struct GameEngineBuilder {
    game_engine: GameEngine,
}

impl GameEngineBuilder {
    /// Creates a new `GameEngineBuilder` instance.
    ///
    /// # Returns
    ///
    /// `Result<Self, String>` - Returns a new `GameEngineBuilder` instance
    /// if successful or an error message if something goes wrong.
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            game_engine: GameEngine::default(),
        })
    }

    /// Sets up the game window.
    ///
    /// # Arguments
    ///
    /// * `window` Windows object
    ///
    /// # Returns
    ///
    /// `Result<Self, String>` - Returns the `GameEngineBuilder` instance for chaining.
    pub fn setup_window(mut self, window: GameWindow) -> Result<Self, String> {
        self.game_engine.window = window;
        Ok(self)
    }

    /// Sets the frames per second (FPS) for the game engine.
    ///
    /// # Arguments
    ///
    /// * `fps` - The desired frames per second for the game.
    ///
    /// # Returns
    ///
    /// `Self` - Returns the `GameEngineBuilder` instance for chaining.
    pub fn change_fps(mut self, fps: u32) -> Self {
        self.game_engine.fps = fps;
        self
    }

    /// Adds a game object to the game engine.
    /// The game object must implement the `GameObject`
    ///
    /// # Arguments
    ///
    /// * `game` - A boxed game object that implements the `GameObject` trait.
    ///
    /// # Returns
    ///
    /// `Self` - Returns the `GameEngineBuilder` instance for chaining.
    pub fn add_game<T: GameObject + 'static>(mut self, game: T) -> Self {
        self.game_engine.game_object = Some(Box::new(game));
        self
    }

    /// Add an asset server to the game engine and load sprite sheet
    ///
    /// # Arguments
    ///
    /// * `source_path` - The file path for the sprite sheet.
    ///   This will automatically be placed under the "assets/" directory.
    /// * `tile_width` - Width of each tile.
    /// * `tile_height` - Height of each tile.
    ///
    /// # Returns
    ///
    /// `Self` - Returns the `GameEngineBuilder` instance for chaining.
    pub fn add_asset_server(
        mut self,
        source_path: &str,
        tile_width: u32,
        tile_height: u32,
        columns: u32,
        rows: u32,
    ) -> Self {
        let base_path = "assets/";
        let full_path = Path::new(base_path).join(source_path);
        let full_path_str = full_path.to_str().unwrap();

        self.game_engine
            .asset_server
            .init(full_path_str, tile_width, tile_height, columns, rows);

        self
    }

    /// Builds the `GameEngine` instance with the specified configurations.
    ///
    /// # Returns
    ///
    /// `Result<GameEngine<W>, String>` - Returns a `GameEngine` instance
    /// if successful or an error message if the game object or FPS is not set.
    pub fn build(self) -> Result<GameEngine, String> {
        Ok(self.game_engine)
    }
}
