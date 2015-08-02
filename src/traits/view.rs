use opengl_graphics::GlGraphics;


/// A view
pub trait View {
  /// Render the view.
  fn render(&self, gl: &mut GlGraphics);

  /// This gets called when the view starts
  fn start(&mut self) {}
  /// This gets called when the view ends
  fn end(&mut self) {}
}

