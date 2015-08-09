use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::mem;

use opengl_graphics::Texture;


pub struct TextureManager {
  cache: HashMap<String, Texture>
}


impl TextureManager {
  pub fn new() -> TextureManager {
    TextureManager { cache: HashMap::new() }
  }

  pub fn get(&mut self, path: &str) -> Result<&Texture, LoadTextureError> {
    // Thank you, qrlpx in #rust on Moznet.
    // TODO: Fix when borrow checker gets smarter.
    // related issue: https://github.com/rust-lang/rust/issues/6393
    // related rfc:   https://github.com/rust-lang/rfcs/issues/811
    // article (#2):  http://blog.ezyang.com/2013/12/two-bugs-in-the-borrow-checker-every-rust-developer-should-know-about/
    let self_dup: &Self = unsafe { mem::transmute(&*self) };
    if let Some(t) = self_dup.cache.get(path) {
      return Ok(t);
    }
    self.cache.insert(path.to_string(), try!(Texture::from_path(path)));
    Ok(&self.cache[path])
  }
}


// TODO fix `Texture::from_path` to do more robust error handling
#[derive(Debug)]
pub struct LoadTextureError {
  message: String
}


impl LoadTextureError {
  pub fn new(message: String) -> LoadTextureError {
    LoadTextureError { message: message }
  }
}


// TODO remove this as soon as `Texture::from_path` is fixed
impl From<String> for LoadTextureError {
  fn from(s: String) -> LoadTextureError {
    LoadTextureError::new(s)
  }
}


impl fmt::Display for LoadTextureError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    f.write_fmt(format_args!("LoadTextureError: {}", self.message))
  }
}


impl Error for LoadTextureError {
  fn description(&self) -> &str {
    "problem loading the texture (it probably doesn't exist)"
  }
}
