//! Traits for the game

pub mod view;
mod map;

pub use self::view::View;
// TODO uncomment:
pub use self::map::*;


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
#[derive(Clone, Copy)]
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
    let x = self.get_x();
    let y = self.get_y();
    self.set_x(x + mov_x);
    self.set_y(y + mov_y);
  }
  /// Take a step in the current direction
  fn take_step(&mut self) {
    let dir = self.get_dir();
    self.move_obj(

      match dir {
        Dir::Left => 1,
        Dir::Right => -1,
        Dir::Up => 0,
        Dir::Down => 0,
      },

      match dir {
        Dir::Left => 0,
        Dir::Right => 0,
        Dir::Up => 1,
        Dir::Down => -1,
      }

    );
  }
}
