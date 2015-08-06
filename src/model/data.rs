use std::collections::HashMap;
// TODO remove:
// use std::path::Path;

use rustc_serialize::json;

use core::Vec2;


struct Frame {
  texture: u16,
  cut_from: (u16, u16),
  size: (u16, u16),
  pin_to: (i32, i32)
}


struct Sprite {
  frames: Vec<Frame>
}


struct ModelData {
  sprite_data: HashMap<String, Sprite>,
  occupied_tiles: Vec<Vec2<u8>>
}


impl ModelData {
  pub fn load(name: &str) -> ModelData {
    //json::encode();
    ModelData {
      sprite_data: HashMap::new(),
      occupied_tiles: Vec::new()
    }
  }

  fn load_occupied_tiles() -> Vec<Vec2<u8>> {
    Vec::new()
  }
}
