use piston::event::RenderArgs;

use opengl_graphics::GlGraphics;

// use core::cache::*; // TODO uncomment. squelching errors
use core::cam::*;
use math::Vec2;
use renderer::Renderer;

use state::State;


/// The in-game view
pub struct Game<'a> {
    //cache: Cache<'a>, // TODO uncomment. squelching errors
    cam: Cam<'a>, 
}


impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        unimplemented!()
    }
}


impl<'a> State for Game<'a> {
    fn render(&self,
              //dt: f64, // render should not care about dt
              args: &RenderArgs,
              gl: &mut GlGraphics,
              renderer: &mut Renderer) {

        // TODO: Draw objects here.

        renderer.draw_text(args,
                           gl,
                           "Here's a long string of text.",
                           Vec2(10.0, 30.0) );
    }
}
