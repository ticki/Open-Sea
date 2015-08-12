use core::*;
use opengl_graphics::*;
use graphics::*;

/// A player
pub struct Player {
  pos: Vec2<i64>,
  dir: Dir,
  trans: f64,
  frame: i16,
  last_move: f64,
}


impl Position for Player {
  fn get_pos(&self) -> Vec2<i64> {
    self.pos
  }
  fn set_pos(&mut self, new_pos: Vec2<i64>) {
    self.pos = new_pos;
  }
}

// TODO: Implement new trait methods for Player
impl Move for Player {
  fn get_dir(&self) -> Dir {
    self.dir
  }
  fn set_dir(&mut self, new_dir: Dir) {
    self.dir = new_dir;
  }
  fn is_moving(&self) -> bool {
    // TODO: Code here.
    unimplemented!()
  }
  fn can_move(&self) -> bool {
    // TODO: Check if the way is blocked, using the cache.
    unimplemented!()
  }
  fn get_last_move(&self) -> f64 {
    self.last_move
  }
  fn set_last_move(&mut self, new: &f64) {
    self.last_move = *new;
  }
  fn get_speed(&self) -> f64 {
    1.0
  }
  fn get_trans_state(&self) -> f64 {
    self.trans
  }
  fn get_animation_frame(&self) -> i16 {
    self.frame
  }
}

impl Sprite for Player {
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
  fn draw(&self, c: &Context, gl: &mut GlGraphics) {
     unimplemented!()
  }
}

impl Matter for Player {}

impl Entity for Player {
  fn id(&self) -> Id {
    Id(0) // NOTE: Player's ID is always 0
  }
  fn is_solid(&self, x: i16, y: i16) -> bool {
    false // TODO: Should it be solid?
  }
  fn update(&mut self, dt: f64) {
    self.move_reg();
  }
}
