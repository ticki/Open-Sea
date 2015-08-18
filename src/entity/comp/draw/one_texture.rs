use std::rc::Weak;

use opengl_graphics::Texture;

use entity::Entity;
use entity::comp::draw::DrawComp;
use math::Rect;


/// A draw component for an entity with a single texture
pub struct OneTexture {
  owner: Option<Weak<Entity>>,
  texture: Weak<Texture>,
}


impl DrawComp for OneTexture {
  fn draw(&self) {
    // TODO
    unimplemented!();
  }

  fn draw_bounds(&self) -> Rect<i64> {
    // TODO
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
