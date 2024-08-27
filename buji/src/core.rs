use crate::{Log, LogLevel, Window, DEFAULT_FPS, NANOS_PER_SECOND};
use std::io::Write;
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
    fn draw(&self);
    /// Update method for game actors. This is called every frame and
    /// should return the next state of main engine.
    ///
    /// # Returns
    ///
    /// A `MainState` value indicating the next state of the engine.
    fn update(&mut self) -> MainState;
}

/// Game Engine, responsible for managing the game loop.
pub struct GameEngine<W: Write> {
    /// `GameObject` trait's implementation.
    pub game_object: Option<Box<dyn GameObject>>,
    /// Frames per second value for the game
    pub fps: u32,
    /// Logger object which implements the Write trait
    pub logger: Option<Log<W>>,
    /// Main screen object of the game
    pub window: Window,
}

impl<W: Write> Default for GameEngine<W> {
    fn default() -> Self {
        Self {
            window: Window::default(),
            fps: DEFAULT_FPS,
            logger: None,
            game_object: None,
        }
    }
}

impl<W: Write> GameEngine<W> {
    /// Managing the main loop. The game loop initializes the game, runs the states and updates
    /// the game object on each frame until the `Exit` state is reached.
    ///
    /// # Returns
    ///
    /// `Result<(), String>` - Returns `Ok(())` if the main loop exists successfully,
    /// or an error message if something goes wrong.
    pub fn run(&mut self) -> Result<(), String> {
        self.log(LogLevel::Info, "Initializing the game engine");
        let mut state = MainState::Init;
        let mut last_update = Instant::now();
        let frame_duration = Duration::new(0, NANOS_PER_SECOND / self.fps);

        loop {
            match state {
                MainState::Init => {
                    state = MainState::Running;
                    self.log(LogLevel::Info, "Going to Running state");
                    continue;
                }
                MainState::Running => {
                    self.log(LogLevel::Info, "On Running state");

                    let now = Instant::now();
                    let delta = now.duration_since(last_update);

                    if let Some(game_object) = &mut self.game_object {
                        game_object.draw();
                        state = game_object.update();
                    }

                    if frame_duration > delta {
                        sleep(frame_duration - delta);
                    }

                    last_update = now;
                }
                MainState::PreExit => {
                    self.log(LogLevel::Info, "Pre Exit...");
                    state = MainState::Exit;
                    continue;
                }
                MainState::Exit => {
                    self.log(LogLevel::Info, "Exiting from game engine");
                    break;
                }
            }
        }

        Ok(())
    }

    /// A function which to simplify internal logging
    ///
    /// # Arguments
    ///
    /// * `log_level` - The log level (`Error`, `Warn`, `Info`)
    /// * `message` - The message to be logged.
    fn log(&mut self, log_level: LogLevel, message: &str) {
        if let Some(ref mut l) = self.logger {
            l.write(log_level, message);
        }
    }
}

/// A builder for constructing a `GameEngine` instance. It allows for setting
/// up the game window, FPS, and the game object in a more flexible and readable way.
///
/// # Example
///
/// ```rust
/// use buji::{GameObject, MainState, GameEngineBuilder, Log, LogLevel, MockLogger, DEFAULT_FPS};
/// use std::io::stdout;
///
/// struct YourGameObject;
///
/// impl GameObject for YourGameObject {
///     fn draw(&self) {
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
///     let logger = Log::new(MockLogger);
///     let game = Box::new(YourGameObject);
///
///     let mut engine = GameEngineBuilder::new()?
///         .change_fps(DEFAULT_FPS)
///         .add_game(game)
///         .add_logger(logger)
///         .build()?;
///
///     Ok(())
///
/// }
/// ```
///
/// This example demonstrates how to create a simple game using `GameEngineBuilder`.
/// A custom game object `YourGameObject` is implemented and added to the engine, which is then run at 60 FPS.
pub struct GameEngineBuilder<W: Write> {
    game_engine: GameEngine<W>,
}

impl<W: Write> GameEngineBuilder<W> {
    /// Creates a new `GameEngineBuilder` instance.
    ///
    /// # Returns
    ///
    /// `Result<Self, String>` - Returns a new `GameEngineBuilder` instance
    /// if successful or an error message if something goes wrong.
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            game_engine: GameEngine::<W>::default(),
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
    pub fn setup_window(mut self, window: Window) -> Result<Self, String> {
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
    pub fn add_game(mut self, game: Box<dyn GameObject>) -> Self {
        self.game_engine.game_object = Some(game);
        self
    }

    /// Add a logger to the game engine
    ///
    /// # Arguments
    ///
    /// * `logger` - An optional logger object that implements the `Write` trait.
    ///
    /// # Returns
    ///
    /// `Self` - Returns the `GameEngineBuilder` instance for chaining.
    pub fn add_logger(mut self, logger: Log<W>) -> Self {
        self.game_engine.logger = Some(logger);
        self
    }

    /// Builds the `GameEngine` instance with the specified configurations.
    ///
    /// # Returns
    ///
    /// `Result<GameEngine<W>, String>` - Returns a `GameEngine` instance
    /// if successful or an error message if the game object or FPS is not set.
    pub fn build(self) -> Result<GameEngine<W>, String> {
        Ok(self.game_engine)
    }
}
