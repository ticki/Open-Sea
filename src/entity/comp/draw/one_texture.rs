use std::rc::Weak;

use opengl_graphics::Texture;

use entity::Entity;
use entity::comp::draw::DrawComp;


pub struct OneTexture {
  owner: Option<Weak<Entity>>,
  texture: Weak<Texture>,
}


impl DrawComp for OneTexture {
  fn draw(&self) {
    //
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
