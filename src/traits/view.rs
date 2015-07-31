use opengl_graphics::GlGraphics;


/// A view
pub trait View {
  /// Render the view.
  fn render(&self, gl: &GlGraphics);

  fn start(&mut self);
  fn end(&mut self);
}

