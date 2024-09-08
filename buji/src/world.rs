use crate::{Position, Scale2D};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

/// A structure that represents the game world.
/// This manages all figures (entities) within it.
///
/// # Type Parameters
///
/// * `''a` - Lifetime for the SDL2 `Texture` reference.
#[derive(Default)]
pub struct World<'a> {
    /// A vector that stores all figures within the game world.
    figures: Vec<Figure<'a>>,
}

impl<'a> World<'a> {
    /// Creates a new figure in the world with the given position and size.
    ///
    /// # Arguments
    ///
    /// * `pos` - The initial position of the figure.
    /// * `size` - The scale (width and height) of the figure.
    ///
    /// # Returns
    ///
    /// Returns the ID of the newly created figure as `u32`.
    pub fn create_figure(&mut self, pos: Position, size: Scale2D) -> u32 {
        let id = self.figures.len() as u32 + 1;
        self.figures.push(Figure {
            id,
            pos,
            size,
            texture: None,
        });
        id
    }

    /// Loads a texture for the specified figure by its ID.
    ///
    /// # Arguments
    ///
    /// * `figure_id` - The ID of the figure to which the texture will be assigned.
    /// * `texture` - The SDL2 texture to be loaded for the figure.
    ///
    /// # Panics
    ///
    /// Panics if the figure with the given ID does not exist.
    pub fn load_texture(&mut self, figure_id: u32, texture: Texture<'a>) {
        if let Some(figure) = self.figures.get_mut(figure_id as usize) {
            figure.texture = Some(texture);
        } else {
            panic!("Figure with ID {} does not exist", figure_id);
        }
    }
}

/// A structure that represents a drawable figure (entity) in the game world.
///
/// # Type Parameters
///
/// * `''a` - Lifetime for the SDL2 `Texture` reference.
#[derive(Default)]
pub struct Figure<'a> {
    /// Unique identifier for the figure.
    id: u32,
    /// Position of the figure in the world.
    pub pos: Position,
    /// Size (width and height) of the figure.
    pub size: Scale2D,
    /// Optional SDL2 texture that can be assigned to the figure.
    pub texture: Option<Texture<'a>>,
}

impl<'a> Figure<'a> {
    /// Draws the figure onto the provided SDL2 canvas if a texture is available.
    ///
    /// # Arguments
    ///
    /// * `canvas` - A mutable reference to the SDL2 canvas on which the figure will be drawn.
    ///
    /// # Panics
    ///
    /// Panics if the texture cannot be rendered due to SDL2 errors.
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if let Some(ref texture) = self.texture {
            let target_rect = Rect::new(self.pos.x, self.pos.y, self.size.width, self.size.height);
            canvas.copy(texture, None, target_rect).unwrap();
        }
    }
}
