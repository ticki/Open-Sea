use opengl_graphics::Texture;
use core::Matter;

/// Props, i.e. non dynamic objects
pub trait Prop: Matter {
  fn get_sprite(&self) -> &Texture;
}
