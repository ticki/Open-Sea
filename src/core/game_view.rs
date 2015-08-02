use piston::event::RenderArgs;

use opengl_graphics::GlGraphics;

use core::View;

use renderer::Renderer;


/// The in-game view
pub struct GameView;

impl GameView {
  pub fn new() -> GameView {
    GameView
  }
}

impl View for GameView {
  fn render(&self, args: &RenderArgs, gl: &mut GlGraphics, renderer: &Renderer) {
    renderer.draw_text(args, gl, "Test");
  }
}
