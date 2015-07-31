trait Positioned {
  fn get_x(&self) -> i64;
  fn get_y(&self) -> i64;
  fn set_x(&mut self);
  fn set_y(&mut self);
}

trait Moving {
  fn get_vel_x(&self) -> i16;
  fn get_vel_y(&self) -> i16;
  fn set_vel_x(&mut self);
  fn set_vel_x(&mut self);
}
