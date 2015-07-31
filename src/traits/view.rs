<<<<<<< HEAD
use opengl_graphics::GlGraphics;


=======
>>>>>>> 78a5df5b7a5356f4cc2b00e7092da8234d6457d2
/// A view
pub trait View {
  /// Render the view.
  fn render(&self, gl: &GlGraphics);

  /// Call this on start of view
  fn start(&mut self) {}
  /// Call this on end of view
  fn end(&mut self) {}
}

