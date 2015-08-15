//! The module for rendering

use std::path::Path;

use piston::event::RenderArgs;

use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use graphics;

use core::cache::*;
use core::Vec2;

mod texture_manager;
pub use self::texture_manager::{LoadTextureError, TextureManager};


/// The renderer
pub struct Renderer {
  font: GlyphCache<'static>,
  text: graphics::text::Text,
}


impl Renderer {
  /// Create a new renderer
  pub fn new() -> Renderer {
    Renderer {
      font: GlyphCache::new(Path::new("./data/font.ttf")).unwrap(),
      text: graphics::text::Text::new(20)
    }
  }

  /// Draw text
  pub fn draw_text(&mut self,
                   args: &RenderArgs,
                   gl: &mut GlGraphics,
                   s: &str,
                   pos: Vec2<f64> ) {
    use graphics::*;
    gl.draw(args.viewport(), |c, gl| {
      let Vec2(x, y) = pos;
      self.text.draw(s,
                     &mut self.font,
                     default_draw_state(),
                     c.trans(x, y).transform,
                     gl );
    });
  }
}
