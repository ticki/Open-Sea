use opengl_graphics::GlGraphics;

use traits::View;


pub fn render<V: View>(gl: &mut GlGraphics, view: &V) {
  view.render(gl);
}

pub struct GameView;

impl GameView {
  pub fn new() -> GameView {
    GameView
  }
}

impl View for GameView {
  fn render(&self, gl: &mut GlGraphics) {}
}
