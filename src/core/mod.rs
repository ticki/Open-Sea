//! Traits, structs, and other objects for the game

mod angle;
mod game_view;
pub mod view;
mod map;
mod vec2;

pub use self::angle::Angle;
pub use self::game_view::GameView;
pub use self::view::View;
pub use self::map::*;
pub use self::vec2::Vec2;


/// A trait for positioned objects
pub trait Positioned {
  /// Get the x coordinate
  fn get_coord(&self) -> Vec<i64>;
  /// Set the Vec coordinate
  fn set_coord(&mut self, coord: Vec<i64>);
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
  fn move_obj(&mut self, mov: Vec<i64>) {
    self.set_coord(self.get_coord() + mov) 
  }
  /// Take a step in the current direction
  fn take_step(&mut self) {
    let dir = self.get_dir();
    self.move_obj(Vec2(

      match dir {
        Dir::Left => 1,
        Dir::Right => -1,
        _ => 0,
      },

      match dir {
        Dir::Up => 1,
        Dir::Down => -1,
        _ => 0,
      }

    ));
  }
}
