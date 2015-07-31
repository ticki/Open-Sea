/// A view
pub trait View {
  /// Render the view.
  fn render<G: Graphics>(&self, gl: &G);

  /// Call this on start of view
  fn start(&mut self) {}
  /// Call this on end of view
  fn end(&mut self) {}
}

