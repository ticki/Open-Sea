// TODO what is this?: // use super::graphics::*;

/// A view
pub trait View {
  /// Render the view.
  fn render<G: Graphics>(&self, gl: &G);

  fn start(&mut self);
  fn end(&mut self);
}

