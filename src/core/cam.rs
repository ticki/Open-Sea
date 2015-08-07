use core::{Entity, Vec2};

/// The camera
struct Cam<'a> {
  /// The former camera pos
  former_pos: Vec2<i64>,
  /// The entity in focus
  in_focus: &'a Entity,
  /// The transition state
  trans_state: f64,
}

// TODO: Make the camera locked to the player? (Or is this a bad idea?)

const CAM_VELOCITY: f64 = 0.1;

impl<'a> Cam<'a> {
  /// Creates a new Cam
  fn new(focus: &'a Entity) -> Cam {
    Cam {
      former_pos: focus.get_pos(),
      in_focus: focus,
      trans_state: 0.0,
    }
  }
  /// Updates the Cam
  fn update(&mut self, dt: f64) {
    self.trans_state += dt * CAM_VELOCITY;
    if self.trans_state > 1.0 {
      self.trans_state = 0.0;
      self.former_pos = self.in_focus.get_pos();
    }
  }
  /// Get position
  fn get_pos(&self) -> Vec2<f64> {
    let pos = self.in_focus.get_pos() as Vec2<f64>;
    pos * self.trans_state
    + pos * (1.0 - self.trans_state)
  }
}
