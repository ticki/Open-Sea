mod traits;

use traits::*;

struct Player {
  x: i64,
  y: i64,
  dir: Direction,
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

impl Moving for Player {
  fn get_dir(&self) -> Dir {
    self.dir
  }
  fn set_direction(&self, new_dir: Dir) -> Dir {
    self.dir = new_dir;
  }
  fn take_step(&mut self) -> i64 {
    self.x += match self.dir {
      Dir::Left => 1,
      Dir::Right => -1,
      Dir::Up => 0,
      Dir::Down => 0,
    };

    self.y += match self.dir {
      Dir::Left => 0,
      Dir::Right => 0,
      Dir::Up => 1,
      Dir::Down => -1,
    };
  }
}
