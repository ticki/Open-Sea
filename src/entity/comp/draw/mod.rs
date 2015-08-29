use std::rc::Weak;

use entity::Entity;
use entity::comp::{Comp, CompType};
use math::Rect;


mod model;

pub use self::model::Model;


// TODO add draw component for characters (players/mobs/living things)


pub trait DrawComp {
    fn draw(&self);
    fn draw_bounds(&self) -> Rect<i64>;

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
