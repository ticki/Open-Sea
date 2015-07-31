use super::traits::*;

/// A player
pub struct Player {
  x: i64,
  y: i64,
  dir: Direction,
}

impl Positioned for Player {
  pub fn get_x(&self) -> i64 {
    self.x
  }
  pub fn get_y(&self) -> i64 {
    self.y
  }
  pub fn set_x(&mut self, new_x: i64) {
    self.x = new_x;
  }
  pub fn set_y(&mut self, new_y: i64) {
    self.y = new_y;
  }
}

impl Movable for Player {
  pub fn get_dir(&self) -> Dir {
    self.dir
  }
  pub fn set_direction(&self, new_dir: Dir) -> Dir {
    self.dir = new_dir;
  }
}
