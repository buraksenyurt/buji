use crate::BLACK;

/// Represents a 2D scale with width and height.
pub struct Scale2D {
    /// The width of the scale.
    pub width: f32,
    /// The height of the scale.
    pub height: f32,
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
    pub fn new(width: f32, height: f32) -> Self {
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
            width: 1280f32,
            height: 640f32,
        }
    }
}

/// Represents a window with a title, scale(2D) and background color.
pub struct Window {
    /// The 2D scale of the window.
    pub scale2d: Scale2D,
    /// The title of the window.
    pub title: &'static str,
    /// The background color of the window.
    pub background_color: Color,
}

/// Represents a color with red, green, and blue components.
pub struct Color {
    /// The red component of the color.
    pub red: u8,
    /// The green component of the color.
    pub green: u8,
    /// The blue component of the color.
    pub blue: u8,
}

impl Color {
    /// Creates a new `Color` from an RGB array.
    ///
    /// # Parameters
    ///
    /// - `codes`: An array containing the red, green, and blue values.
    ///
    /// # Returns
    ///
    /// A new `Color` instance.
    pub fn from_rgb(codes: [u8; 3]) -> Color {
        Self {
            red: codes[0],
            blue: codes[1],
            green: codes[2],
        }
    }
}

impl Default for Color {
    /// Returns the default `Color`, which is black.
    ///
    /// # Returns
    ///
    /// A default `Color` instance.
    fn default() -> Self {
        Self::from_rgb(BLACK)
    }
}

impl Window {
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
    pub fn new(scale2d: Scale2D, title: &'static str, background_color: Color) -> Self {
        Self {
            scale2d,
            title,
            background_color,
        }
    }
}

impl Default for Window {
    /// Returns the default `Window` with the title "Anonymous Game"
    /// a default scale and a black background color.
    ///
    /// # Returns
    ///
    /// A default `Window` instance.
    fn default() -> Self {
        Self {
            title: "Anonymous Game",
            scale2d: Scale2D::default(),
            background_color: Color::default(),
        }
    }
}
