use piston::event::RenderArgs;

use opengl_graphics::GlGraphics;

use renderer::Renderer;


/// A view
pub trait View {
  /// Render the view.
  fn render(&self, args: &RenderArgs, gl: &mut GlGraphics, renderer: &Renderer);

  /// This gets called when the view starts
  fn start(&mut self) {}
  /// This gets called when the view ends
  fn end(&mut self) {}
}

