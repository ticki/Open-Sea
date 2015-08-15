use piston::event::RenderArgs;

use opengl_graphics::GlGraphics;

use core::{Vec2};
use core::cache::*;
use core::cam::*;
use renderer::Renderer;

use state::State;


/// The in-game view
pub struct Game<'a> {
  cache: Cache<'a>,
  cam: Cam<'a>, 
}


impl<'a> State for Game<'a> {
  fn render(&self, dt: f64, args: &RenderArgs,
            gl: &mut GlGraphics,
            renderer: &mut Renderer) {

    // TODO: Draw objects here.

    renderer.draw_text(args,
                       gl,
                       "Here's a long string of text.",
                       Vec2(10.0, 30.0) );
  }
}
