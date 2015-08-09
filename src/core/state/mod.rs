use piston::event::RenderArgs;

use opengl_graphics::GlGraphics;

use renderer::Renderer;


/// A state, i.e. an rendering state
pub trait State {
  /// Render the view.
  fn render(&self, args: &RenderArgs,
            gl: &mut GlGraphics,
            renderer: &mut Renderer);

  /// This gets called when the view starts
  fn start(&mut self) {}
  /// This gets called when the view ends
  fn end(&mut self) {}
}

mod game;
pub use self::game::Game;
