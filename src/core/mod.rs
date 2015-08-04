//! Traits, structs, and other objects for the game

mod angle;
mod game_view;
pub mod view;
mod map;
mod vec2;

pub use self::angle::Angle;
pub use self::game_view::GameView;
pub use self::view::View;
pub use self::map::{Block, Map, Tile, TileMap};
pub use self::vec2::Vec2;

// TODO: Rename coord to pos

/// A trait for positioned objects
pub trait Positioned {
  /// Get the x coordinate
  fn get_coord(&self) -> Vec2<i64>;
  /// Set the Vec coordinate
  fn set_coord(&mut self, coord: Vec2<i64>);
}

/// The direction of a given object
#[derive(Clone, Copy)]
pub enum Dir {
  Left,
  Right,
  Up,
  Down,
}
impl Dir {
  fn to_vec(&self) -> Vec2<i64> {
    Vec2(
      match *self {
        Dir::Left => 1,
        Dir::Right => -1,
        _ => 0,
      },

      match *self {
        Dir::Up => 1,
        Dir::Down => -1,
      _ => 0,
      }
    )
  }
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
  fn move_obj(&mut self, mov: Vec2<i64>) {
    let coord = self.get_coord();
    self.set_coord(coord + mov);
  }
  /// Get new coordinate
  fn get_new_coord(&self) -> Vec2<i64> {
    let dir = self.get_dir();
    self.get_coord() + dir.to_vec()
  }
  /// Move object in direction.
  fn move_obj_dir(&mut self) {
    let new_coord = self.get_new_coord();
    self.set_coord(new_coord)
  }
}

pub trait Animated: Movable {
  /// Get transitition point, which is in the interval [0,1]
  fn get_trans_state(&self) -> f64;
}
