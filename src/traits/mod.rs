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

trait Moving {
  fn get_dir(&self) -> Dir;
  fn set_dir(&mut self, new_dir: Dir);
  fn take_step(&mut self);
}
