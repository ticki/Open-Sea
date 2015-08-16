use std::rc::Weak;

use entity::Entity;
use entity::comp::draw::DrawComp;


pub struct MultiTexture {
  owner: Option<Weak<Entity>>,
}


impl DrawComp for MultiTexture {
  fn draw(&self) {
    // get owner's textures (possible downcasting his type) and draw them
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
