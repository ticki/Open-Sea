use core::*;

/// A player
pub struct Player {
  coord: Vec2<i64>,
  dir: Dir,
  trans: f64,
}


impl Positioned for Player {
  fn get_coord(&self) -> Vec2<i64> {
    self.coord
  }
  fn set_coord(&mut self, coord: Vec2<i64>) {
    self.coord = coord;
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

impl Animated for Player {
  fn get_trans_state(&self) -> f64 {
    self.trans
  }
}
