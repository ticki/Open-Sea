//! The module for rendering

use std::path::Path;

use piston::event::RenderArgs;

use opengl_graphics::GlGraphics;

use graphics;

mod font;


pub struct Renderer {
  font: font::Font,
  text: graphics::text::Text,
}


impl Renderer {
  pub fn new() -> Renderer {
    Renderer {
      font: font::Font::new(Path::new("./assets/font-8x8.png")),
      text: graphics::text::Text::new(10)
    }
  }

  pub fn draw_text(&self, args: &RenderArgs, gl: &GlGraphics, s: &str) {
    gl.draw(args.viewport(), |c, gl| {
      self.text.draw(s,
                     &mut self.font,
                     graphics::default_draw_state(),
                     c.transform,
                     gl );
    });
  }
}
