use std::rc::Weak;

use entity::Entity;
use entity::comp::draw::DrawComp;
use math::Rect;
use model;


/// A draw component for an entity with a model
pub struct Model {
  owner: Option<Weak<Entity>>,
  model: Weak<model::ModelData>,
}


impl DrawComp for Model {
  fn draw(&self) {
    // TODO implement
    unimplemented!();
  }

  fn draw_bounds(&self) -> Rect<i64> {
    // TODO implement
    unimplemented!()
  }

  fn set_owner(&mut self, owner: Weak<Entity>) -> Result<(), ()> {
    if self.owner.is_some() {
      return Err(());
    }
    self.owner = Some(owner);
    Ok(())
  }

  fn remove_owner(&mut self) -> Option<Weak<Entity>> {
    self.owner.take()
  }
}
