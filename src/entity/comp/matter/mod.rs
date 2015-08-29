use std::rc::Weak;

use entity::Entity;
use entity::comp::{Comp, CompType};
use math::Vec2;


pub trait MatterComp {
    fn is_solid(&self) -> bool { false }

    fn is_destroyable(&self) -> bool { false }

    fn get_hardness(&self) -> u32 { 3 }

    fn get_collision_map(&self) -> Vec<Vec2<i64>>;

    // Comp
    fn set_owner(&mut self, owner: Weak<Entity>) -> Result<(), ()>;
    fn remove_owner(&mut self) -> Option<Weak<Entity>>;
}


impl Comp for MatterComp {
    fn get_type(&self) -> CompType {
        CompType::Matter
    }

    fn set_owner(&mut self, owner: Weak<Entity>) -> Result<(), ()> {
        let _self = self as &mut MatterComp;
        _self.set_owner(owner)
    }

    fn remove_owner(&mut self) -> Option<Weak<Entity>> {
        let _self = self as &mut MatterComp;
        _self.remove_owner()
    }
}
