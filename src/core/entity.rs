use core::{Sprite};

/// Entity ID type
pub struct Id(pub i64);

/// An entity, a dynamic in-game object with non-trivial functionality
pub trait Entity: Sprite {
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
  // Probably need more methods.
}
