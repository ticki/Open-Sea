//! Traits for the game

/// A trait for positioned objects
pub trait Positioned {
  /// Get the x coordinate
  fn get_x(&self) -> i64;
  /// Get the y coordinate
  fn get_y(&self) -> i64;
  /// Set the x coordinate
  fn set_x(&mut self, new_x: i64);
  /// Set the y coordinate
  fn set_y(&mut self, new_y: i64);
}

/// The direction of a given object
pub enum Dir {
  Left,
  Right,
  Up,
  Down,
}

// TODO: Implement two directions at once.
// TODO: Make drawable trait

// A movable object
pub trait Movable: Positioned {
  /// Get the direction
  fn get_dir(&self) -> Dir;
  /// Set the direction
  fn set_dir(&mut self, new_dir: Dir);
  /// Move the object
  fn move_obj(&mut self, mov_x: i64, mov_y: i64) {
    self.set_x(self.get_x() + mov_x);
    self.set_y(self.get_y() + mov_y);
  }
  /// Take a step in the current direction
  fn take_step(&mut self) {
    self.move_obj(

      match self.get_dir() {
        Dir::Left => 1,
        Dir::Right => -1,
        Dir::Up => 0,
        Dir::Down => 0,
      },

      match self.get_dir() {
        Dir::Left => 0,
        Dir::Right => 0,
        Dir::Up => 1,
        Dir::Down => -1,
      }

    );
  }
}
