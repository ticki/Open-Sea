// NOTE: Object is a term for an entity or a prop.

use time;

use opengl_graphics::*;
use graphics::*;

use core::Dir;
use math::Vec2;


/// A movable object
pub trait Move {
  /// Get the direction
  fn get_dir(&self) -> Dir;
  /// Set the direction
  fn set_dir(&mut self, new_dir: Dir);
  /// Is the object moving?
  fn is_moving(&self) -> bool;
  /// Can the object move? Or is it blocked?
  fn can_move(&self) -> bool;
  /// Move the object
  fn move_obj(&mut self, mov: Vec2<i64>) {
    let coord = self.get_pos();
    self.set_pos(coord + mov);
  }
  fn get_cur_pos(&self) -> Vec2<f64> {
    Vec2::from(self.get_new_pos()) * self.get_trans_state()
    + Vec2::from(self.get_pos())
  }
  /// Get new coordinate
  fn get_new_pos(&self) -> Vec2<i64> {
    // TODO: This is only temporary. Should diagonal moves be allowed or not?
    let mov = self.get_dir();
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

// TODO: Add event trait, for objects you can click on etc.
