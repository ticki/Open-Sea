use core::*;

/// A player
pub struct Player {
  pos: Vec2<i64>,
  dir: Dir,
  trans: f64,
  frame: i16,
}


impl Positioned for Player {
  fn get_pos(&self) -> Vec2<i64> {
    self.pos
  }
  fn set_pos(&mut self, new_pos: Vec2<i64>) {
    self.pos = new_pos;
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
  fn get_animation_frame(&self) -> i16 {
    self.frame
  }
}
// TODO: Implement other traits
