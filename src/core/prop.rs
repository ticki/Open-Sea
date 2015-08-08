use opengl_graphics::{Texture};

/// Props, i.e. non dynamic objects
pub trait Prop {
  fn get_sprite(&self) -> &Texture;
  // TODO: Make a trait with the following methods, called Matter. Implement this for entity too.
  fn is_solid(&self) -> bool;
  fn is_destroyable(&self) -> bool;
  fn get_hardness(&self) -> i32;
}
