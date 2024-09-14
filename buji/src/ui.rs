use logy::*;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

/// Fundamental Game Color data structure
pub struct GameColor {
    /// Red color value between 0..255
    pub red: u8,
    /// Green color value between 0..255
    pub green: u8,
    /// Blue color value between 0..255
    pub blue: u8,
    /// Alpha value between 0 to 100
    pub alpha: u8,
}

impl GameColor {
    /// Initialize a Color from RGBA values
    ///
    /// # Parameters
    ///
    /// - `red`: Red value (0..255)
    /// - `blue` : Blue value (0..255)
    /// - `green` : Green value (0..255)
    /// - `alpha` : Alpha value (0..100) %
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Creates a new `GameColor` from an RGB array with default Alpha value (100%)
    ///
    /// # Parameters
    ///
    /// - `codes`: An array containing the red, green, and blue values.
    ///
    /// # Returns
    ///
    /// A new `Color` instance.
    pub fn from_rgb(codes: [u8; 3]) -> Self {
        Self {
            red: codes[0],
            blue: codes[1],
            green: codes[2],
            alpha: 0,
        }
    }
}

impl Default for GameColor {
    /// Initialize a default Color
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }
}

impl From<&GameColor> for Color {
    /// Convert GameColor instance to SDL2 Color instance
    ///
    /// # Parameters
    ///
    /// - `color`: Instance of GameColor
    fn from(color: &GameColor) -> Self {
        Self::RGBA(color.red, color.blue, color.green, color.alpha)
    }
}

/// Represents a 2D scale with width and height.
pub struct Scale2D {
    /// The width of the scale.
    pub width: u32,
    /// The height of the scale.
    pub height: u32,
}

impl Scale2D {
    /// Creates a new `Scale2D` with the given width and height.
    ///
    /// # Parameters
    ///
    /// - `width`: The width of the scale.
    /// - `height`: The height of the scale.
    ///
    /// # Returns
    ///
    /// A new `Scale2D` instance.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Default for Scale2D {
    /// Returns the default `Scale2D` with a width of 1280 and a height of 640.
    ///
    /// # Returns
    ///
    /// A default `Scale2D` instance.
    fn default() -> Self {
        Self {
            width: 1280,
            height: 640,
        }
    }
}

/// Represents a window with a title, scale(2D) and background color.
pub struct GameWindow {
    /// The 2D scale of the window.
    pub scale2d: Scale2D,
    /// The title of the window.
    pub title: &'static str,
    /// The background color of the window.
    pub background_color: GameColor,
    /// SDL2 Context
    pub sdl_context: Option<Sdl>,
    /// Canvas zone
    pub canvas: Option<Canvas<Window>>,
}

impl GameWindow {
    /// Creates a new `Window` with the given scale, title, and background color.
    ///
    /// # Parameters
    ///
    /// - `scale2d`: The 2D scale of the window.
    /// - `title`: The title of the window.
    /// - `background_color`: The background color of the window.
    ///
    /// # Returns
    ///
    /// A new `Window` instance.
    pub fn new(scale2d: Scale2D, title: &'static str, background_color: GameColor) -> Self {
        Self {
            scale2d,
            title,
            background_color,
            ..Default::default()
        }
    }

    /// Initializes the SDL2 context and creates the game window and canvas.
    ///
    /// # Returns
    ///
    /// `Result<(), String>` - Returns `Ok(())`
    /// if the initialization is successful or an error message if something goes wrong.
    ///
    /// # Errors
    ///
    /// This function will return an error if SDL2 fails to initialize or if the window or canvas creation fails.
    pub fn init(&mut self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(self.title, self.scale2d.width, self.scale2d.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::from(&self.background_color));
        canvas.clear();
        canvas.present();

        self.sdl_context = Some(sdl_context);
        self.canvas = Some(canvas);

        linfo!(LogLevel::Warn, "Video sub system is ready");

        Ok(())
    }

    /// Clean up the canvas by setting it to a black color and clearing it.
    pub fn cleanup(&mut self) {
        if let Some(ref mut canvas) = self.canvas {
            canvas.set_draw_color(Color::from(&self.background_color));
            canvas.clear();
        }
    }

    /// Presents the current canvas content on the screen.
    pub fn present(&mut self) {
        if let Some(ref mut canvas) = self.canvas {
            canvas.present();
        }
    }

    // pub fn shut_down(&mut self) {
    //     if let Some(ref mut canvas) = self.canvas.take() {
    //         drop(canvas.flush());
    //     }
    //
    //     if let Some(ref mut sdl_context) = self.sdl_context.take() {
    //         drop(sdl_context.flush());
    //     }
    // }
}

impl Default for GameWindow {
    /// Returns the default `GameWindow` with the title "Anonymous Game"
    /// a default scale and a black background color.
    ///
    /// # Returns
    ///
    /// A default `GameWindow` instance.
    fn default() -> Self {
        Self {
            title: "Anonymous Game",
            scale2d: Scale2D::default(),
            sdl_context: None,
            canvas: None,
            background_color: GameColor::default(),
        }
    }
}

/// 2D Positions of something
#[derive(Default)]
pub struct Position {
    /// x origin value
    pub x: i32,
    /// y origin value
    pub y: i32,
}

impl Position {
    /// Create a new 2D position instance
    ///
    /// # Arguments
    ///
    /// `x` - value of x origin
    /// `y` - value of y origin
    ///
    /// # Returns
    ///
    /// `Position` - a new instance of x,y origin
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
