use piston::event::RenderArgs;

use opengl_graphics::GlGraphics;

use core::state::State;

use renderer::Renderer;


/// The in-game view
pub struct Game;

impl Game {
  pub fn new() -> Game {
    Game
  }
}

impl State for Game {
  fn render(&self, args: &RenderArgs,
            gl: &mut GlGraphics,
            renderer: &mut Renderer) {
    renderer.draw_text(args, gl, "Test");
  }
}
