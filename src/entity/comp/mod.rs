use std::rc::Weak;

use entity::Entity;


pub mod draw;


pub trait Comp {
  fn get_type(&self) -> CompType;
  fn set_owner(&mut self, owner: Weak<Entity>) -> Result<(), ()>;
  fn remove_owner(&mut self) -> Option<Weak<Entity>>;
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompType {
  Draw,
}
