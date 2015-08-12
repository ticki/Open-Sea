// NOTE: Object is a term for an entity or a prop.

use time;
use opengl_graphics::*;
use graphics::*;
use core::{Vec2};

/// A trait for positioned objects
pub trait Position {
  /// Get the x coordinate
  fn get_pos(&self) -> Vec2<i64>;
  /// Set the Vec coordinate
  fn set_pos(&mut self, new_pos: Vec2<i64>);
}

// TODO: This should probably just be a Vec2, especially since you're doing
//       things like Dir::to_vec, and you want to allow two different
//       directions at once.
/// The direction of a given object
#[derive(Clone, Copy)]
pub enum Dir {
  Left,
  Right,
  Up,
  Down,
}

pub struct Movement {
  dir1: Option<Dir>,
  dir2: Option<Dir>,
  state: i64, // This should change every half second
}

impl Movement {
  fn to_vec(&self) -> Vec2<i64> {
    unimplemented!()
  }
}

impl Dir {
  fn to_vec(self) -> Vec2<i64> {
    Vec2(
      match self {
        Dir::Left => 1,
        Dir::Right => -1,
        _ => 0,
      },

      match self {
        Dir::Up => 1,
        Dir::Down => -1,
      _ => 0,
      }
    )
  }
  /// Get the dir in the opposite direction
  fn invert(self) -> Dir {
    match self {
      Dir::Left => Dir::Right,
      Dir::Right => Dir::Left,
      Dir::Up => Dir::Down,
      Dir::Down => Dir::Up,
    }
  }
}

// TODO: Implement two directions at once. (See comment above.)

/// A movable object
pub trait Move: Position {
  /// Get the direction
  fn get_movement(&self) -> Movement;
  /// Set the direction
  fn set_movement(&mut self, new_dir: Movement);
  /// Is the object moving?
  fn is_moving(&self) -> bool;
  /// Can the object move? Or is it blocked?
  fn can_move(&self) -> bool;
  /// Move the object
  fn move_obj(&mut self, mov: Vec2<i64>) {
    let coord = self.get_pos();
    self.set_pos(coord + mov);
  }
  /// Get new coordinate
  fn get_new_pos(&self) -> Vec2<i64> {
    let mov = self.get_movement();
    self.get_pos() + mov.to_vec()
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
    if self.is_moving() && self.can_move()
       && self.get_last_move() - now > 1.0 / self.get_speed() {
      self.set_last_move(now);
      self.move_obj_dir();
    }
  }
  /// Get transitition point, which is in the interval [0,1]
  fn get_trans_state(&self) -> f64;
  /// Get animation frame
  fn get_animation_frame(&self) -> i16;
}

/// Trait for sprited objects
pub trait Sprite: Move {
  /// Get current sprite
  fn get_sprite(&self) -> &Texture;
  /// Get width, not neccesarily the width of the sprite, but rather the space
  /// the given object occupies. Given in fields.
  fn get_width(&self) -> i16;
  /// Get height, see note above
  fn get_height(&self) -> i16;
  /// Get the opacity of the object
  fn get_opacity(&self) -> f64;
  //
  fn draw(&self, c: &Context, gl: &mut GlGraphics);
}

// TODO: Add event trait, for objects you can click on etc.
