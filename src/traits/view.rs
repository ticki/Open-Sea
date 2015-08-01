use opengl_graphics::GlGraphics;


/// A view
pub trait View {
  /// Render the view.
  fn render(&self, gl: &mut GlGraphics);

  /// Call this on start of view
  fn start(&mut self) {}
  /// Call this on end of view
  fn end(&mut self) {}
}

