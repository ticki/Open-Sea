use core::*;

/// A player
pub struct Player {
  x: i64,
  y: i64,
  dir: Dir,
}


impl Positioned for Player {
  fn get_x(&self) -> i64 {
    self.x
  }
  fn get_y(&self) -> i64 {
    self.y
  }
  fn set_x(&mut self, new_x: i64) {
    self.x = new_x;
  }
  fn set_y(&mut self, new_y: i64) {
    self.y = new_y;
  }
}


impl Movable for Player {
  fn get_dir(&self) -> Dir {
    self.dir
  }
  fn set_dir(&mut self, new_dir: Dir) {
    self.dir = new_dir;
  }
}
