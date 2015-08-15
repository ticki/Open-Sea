use opengl_graphics::Texture;
use core::Matter;

/// Props, i.e. non dynamic objects
pub trait Prop: Matter + Clone + Copy {
  fn get_sprite(&self) -> &Texture;
}
