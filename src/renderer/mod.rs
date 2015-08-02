//! The module for rendering

use std::collections::HashMap;
use std::path::Path;

use opengl_graphics;
use opengl_graphics::GlGraphics;

use graphics;
use graphics::ImageSize;

use graphics::character::{Character, CharacterCache};


struct Renderer {
  font: Font,
  text: graphics::text::Text,
}


const FONT_CHARS_IN_ROW: u32 = 16;
const FONT_CHARS_IN_COL: u32 = 16;

const FONT_UNKNOWN_GLYPH_CHAR: char = '?';


struct Font {
  char_size: (u32, u32),
  texture: opengl_graphics::Texture,
  cache: HashMap<u32, Character<opengl_graphics::Texture>>,
}


impl Font {
  pub fn new(path: &Path) -> Font {
    let texture = opengl_graphics::Texture::from_path(path).unwrap();
    let (tw, th) = texture.get_size();
    Font {
      char_size: (tw / FONT_CHARS_IN_ROW, th / FONT_CHARS_IN_COL),
      texture: texture,
      cache: HashMap::new(),
    }
  }

  fn create_character(&self, code: u32) -> Character<opengl_graphics::Texture> {
    let (cw, ch) = self.char_size;
    let left_offs = (code % FONT_CHARS_IN_ROW) * cw;
    let top_offs = (code / FONT_CHARS_IN_ROW) * ch;
    Character {
      offset: [left_offs as f64, top_offs as f64],
      size: [cw as f64, ch as f64],
      texture: self.texture,
    }
  }
}


impl CharacterCache for Font {

  type Texture = opengl_graphics::Texture;

  fn character(&mut self,
               font_size: graphics::types::FontSize,
               ch: char ) -> &Character<Self::Texture> {
    // Get the code of the char
    let code = ch as u32;
    // Return the glyph if it's cached
    if let Some(c) = self.cache.get(&code) {
      return c;
    }
    // If our font doesn't contain a glyph for it, return the glyph we use for
    // unsupported characters (FONT_UNKNOWN_GLYPH_CHAR's glyph)
    if code >= FONT_CHARS_IN_COL * FONT_CHARS_IN_ROW {
      let unknown_glyph_code = FONT_UNKNOWN_GLYPH_CHAR as u32;
      if let None = self.cache.get(&unknown_glyph_code) {
        self.cache.insert(unknown_glyph_code, self.create_character(unknown_glyph_code));
      }
      return &self.cache[&unknown_glyph_code];
    }
    // Otherwise, cache the actual glyph
    self.cache.insert(code, self.create_character(code));
    // And return it
    &self.cache[&code]
  }
}
