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

impl Sprited for Player {
  fn get_sprite(&self) -> &Texture {
    unimplemented!();
    // TODO: Sprite here
  }
  fn get_width(&self) -> i16 {
    1
  }
  fn get_height(&self) -> i16 {
    1
  }
  fn get_opacity(&self) -> f64 {
    1.0
  }
}

impl Entity for Player {
  fn id(&self) -> Id {
    Id(0) // NOTE: Player's ID is always 0
  }
  fn is_solid(&self, x: i16, y: i16) -> bool {
    false // TODO: Should it be solid?
  }
}
