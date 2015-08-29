//use core::{Sprite, Matter}; TODO uncomment. squelching errors

/// Events.
pub enum Event {
    /// When player 'contacts' (press space in front of) the object
    Contact,
    /// When the entity move
    Movement,
    /// When the entity collides with player
    PlayerCollision,
    /// When the entity is destroyed
    Destroy,
    // TODO: Add more
}

/// Entity ID type
pub struct Id(pub i64);

/// An entity, a dynamic in-game object with non-trivial functionality
pub trait Entity { //: Sprite + Matter { // TODO uncomment; squelching errors
    /// Get the ID of the given entity
    fn id(&self) -> Id;
    /// Is the entity solid at point (x, y) relative to the position?
    fn is_solid(&self, x: i16, y: i16) -> bool;
    /// The default update method
    fn def_update(&mut self, dt: f64) {
        self.move_reg();
    }
    /// Update the entity
    fn update(&mut self, dt: f64) {
        self.def_update(dt);
    }

    /// Should be called when event happens
    fn on_event(&mut self, event: Event) {
        match event {
            _ => {}
        }
    }
    // Probably need more methods.
}
