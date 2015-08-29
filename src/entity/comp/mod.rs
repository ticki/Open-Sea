use std::rc::Weak;

use entity::Entity;


pub mod draw;
pub mod matter;


/// The base trait for entity components.
pub trait Comp {
    /// Get the type of the component. See `CompType`.
    fn get_type(&self) -> CompType;

    /// Associate this component with an owner
    ///
    /// Results in a `Err(())` if this component already has an owner.
    fn set_owner(&mut self, owner: Weak<Entity>) -> Result<(), ()>;

    /// Remove the owner of this component if it has one.
    fn remove_owner(&mut self) -> Option<Weak<Entity>>;
}


/// The type of the component (drawing, battler, etc.)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompType {
    /// The type of components which provide rendering functionality.
    Draw,
    /// The type of components which provide matter data.
    Matter,
}
