//! The module for rendering

use std::path::Path;

use piston::event::RenderArgs;

use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use graphics;

pub mod loader;
pub use self::loader::*;


pub struct Renderer {
  font: GlyphCache<'static>,
  text: graphics::text::Text,
}


impl Renderer {
  pub fn new() -> Renderer {
    Renderer {
      font: GlyphCache::new(Path::new("./data/font.ttf")).unwrap(),
      text: graphics::text::Text::new(20)
    }
  }

  pub fn draw_text(&mut self,
                   args: &RenderArgs,
                   gl: &mut GlGraphics,
                   s: &str ) {
    use graphics::*;
    gl.draw(args.viewport(), |c, gl| {
      self.text.draw(s,
                     &mut self.font,
                     default_draw_state(),
                     c.trans(10.0, 20.0).transform,
                     gl );
    });
  }
}
