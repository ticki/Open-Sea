use std::rc::Weak;

use entity::Entity;
use entity::comp::{Comp, CompType};


mod multi_texture;
mod one_texture;

pub use self::multi_texture::MultiTexture;
pub use self::one_texture::OneTexture;


pub trait DrawComp {
  fn draw(&self);

  // Comp
  fn set_owner(&mut self, owner: Weak<Entity>) -> Result<(), ()>;
  fn remove_owner(&mut self) -> Option<Weak<Entity>>;
}


impl Comp for DrawComp {
  fn get_type(&self) -> CompType {
    CompType::Draw
  }

  fn set_owner(&mut self, owner: Weak<Entity>) -> Result<(), ()> {
    let _self = self as &mut DrawComp;
    _self.set_owner(owner)
  }

  fn remove_owner(&mut self) -> Option<Weak<Entity>> {
    let _self = self as &mut DrawComp;
    _self.remove_owner()
  }
}
