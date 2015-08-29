use std::collections::HashMap;
use std::rc::Rc;

use entity::comp::{Comp, CompType};
use math::Vec2;


pub struct Entity {
    pub position: Vec2<i64>,
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

    /// Return an `Rc` to a `Comp` of the given `CompType` of the `Entity`, if
    /// the entity has a component of that type.
    fn get_comp(&mut self, comp_type: CompType) -> Option<Rc<Comp>> {
        self.comps.get(&comp_type).cloned()
    }
}
