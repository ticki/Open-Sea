use opengl_graphics::GlGraphics;

use core::View;


/// The in-game view
pub struct GameView;

impl GameView {
  pub fn new() -> GameView {
    GameView
  }
}

impl View for GameView {
  fn render(&self, gl: &mut GlGraphics) {}
}
