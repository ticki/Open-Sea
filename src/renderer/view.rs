use super::graphics::*;

/// A view
trait View {
  /// Render the view.
  pub fn render<G: Graphics>(&self, gl: &G)

  pub fn start(&mut self);
  pub fn end(&mut self);
}

