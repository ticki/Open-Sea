trait Positioned {
  fn get_x(&self) -> i64;
  fn get_y(&self) -> i64;
  fn set_x(&mut self, new_x: i64);
  fn set_y(&mut self, new_y: i64);
}

enum Dir {
  Left,
  Right,
  Up,
  Down,
}

// TODO: Implement two directions at once.
// TODO: Make drawable trait

trait Moving: Positioned {
  fn get_dir(&self) -> Dir;
  fn set_dir(&mut self, new_dir: Dir);
  fn move(&mut self, mov_x: i64, mov_y: i64) {
    self.set_x(self.get_x() + mov_x);
    self.set_y(self.get_y() + mov_y);
  }
  fn take_step(&mut self) {
    self.move(

      match self.get_dir() {
        Dir::Left => 1,
        Dir::Right => -1,
        Dir::Up => 0,
        Dir::Down => 0,
      },

      match self.get_dir() {
        Dir::Left => 0,
        Dir::Right => 0,
        Dir::Up => 1,
        Dir::Down => -1,
      }

    );
  }
}
