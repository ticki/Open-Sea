use std::collections::HashMap;
use std::rc::{Rc, Weak};

use entity::comp::{Comp, CompType};
use math::Vec2;


pub struct Entity {
  position: Vec2<i64>,
  comps: HashMap<CompType, Rc<Comp>>,
}


impl Entity {
  /// Add a `Comp` to this `Entity`. Return Err(()) if this `Entity` already
  /// owns a component of this type.
  fn add_comp(&mut self, comp: Rc<Comp>) -> Result<(), ()> {
    if let None = self.comps.get(&comp.get_type()) {
      self.comps.insert(comp.get_type(), comp);
      return Ok(());
    }
    Err(())
  }

  ///
  fn get_comp(&mut self, comp_type: CompType) -> Option<Rc<Comp>> {
    self.comps.get(&comp_type).cloned()
  }
}
