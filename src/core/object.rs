use time;
use opengl_graphics::*;
use core::{Vec2};

/// A trait for positioned objects
pub trait Position {
  /// Get the x coordinate
  fn get_pos(&self) -> Vec2<i64>;
  /// Set the Vec coordinate
  fn set_pos(&mut self, new_pos: Vec2<i64>);
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

// TODO: Movable is a bad naming. Consider renaming it, moving.
/// A movable object
pub trait Move: Position {
  /// Get the direction
  fn get_dir(&self) -> Dir;
  /// Set the direction
  fn set_dir(&mut self, new_dir: Dir);
  /// Is the object moving?
  fn is_moving(&self) -> bool;
  /// Move the object
  fn move_obj(&mut self, mov: Vec2<i64>) {
    let coord = self.get_pos();
    self.set_pos(coord + mov);
  }
  /// Get new coordinate
  fn get_new_pos(&self) -> Vec2<i64> {
    let dir = self.get_dir();
    self.get_pos() + dir.to_vec()
  }
  /// Move object in direction.
  fn move_obj_dir(&mut self) {
    let new_coord = self.get_new_pos();
    self.set_pos(new_coord)
  }
  /// Get the timestamp of the last move
  fn get_last_move(&self) -> f64;
  /// Set the timestamp of the last move
  fn set_last_move(&mut self, new: &f64);
  /// Get the speed of the object
  fn get_speed(&self) -> f64;
  /// Moves regularly
  fn move_reg(&mut self) {
    let now = &time::precise_time_s();
    if self.is_moving() && self.get_last_move() - now > 1.0 / self.get_speed() {
      self.set_last_move(now);
      self.move_obj_dir();
    }
  }
}

/// Trait for animated objects
// TODO: Merge with move?
pub trait Animate: Move {
  /// Get transitition point, which is in the interval [0,1]
  fn get_trans_state(&self) -> f64;
  /// Get animation frame
  fn get_animation_frame(&self) -> i16;
}

/// Trait for sprited objects
pub trait Sprite: Animate {
  /// Get current sprite
  fn get_sprite(&self) -> &Texture;
  /// Get width, not neccesarily the width of the sprite, but rather the space
  /// the given object occupies. Given in fields.
  fn get_width(&self) -> i16;
  /// Get height, see note above
  fn get_height(&self) -> i16;
  /// Get the opacity of the object
  fn get_opacity(&self) -> f64;

  // TODO: Add draw method.
}

// TODO: Add event trait, for objects you can click on etc.

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
