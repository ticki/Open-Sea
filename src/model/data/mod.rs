#![allow(dead_code)] // TODO remove

use std::collections::BTreeMap;
// TODO remove:
// use std::path::Path;

use core::Vec2;

mod error;
mod load;

pub use self::error::{LoadModelError, ModelError};


struct Frame {
  texture: u16,
  cut_from: (u16, u16),
  cut_offset: (i8, i8),
  size: (u16, u16),
  pin_to: (i32, i32),
  pin_offset: (i8, i8),
}


struct Sprite {
  frames: Vec<Frame>
}


/// This struct simplifies the implementation of `ModelData::parse_sprite_data`
struct SpriteDataDefaults {
  pub sprite: String,
  pub frame: u64,
  pub cut_from: (u16, u16),
  pub cut_offset: (i8, i8),
  pub size: (u16, u16),
  pub pin_to: (i32, i32),
  pub pin_offset: (i8, i8),
}


impl SpriteDataDefaults {
  pub fn new() -> SpriteDataDefaults {
    SpriteDataDefaults {
      sprite: "default".to_string(),
      frame: 0,
      cut_from: (0, 0),
      cut_offset: (0, 0),
      size: (1, 1),
      pin_to: (0, 0),
      pin_offset: (0, 0),
    }
  }
}


struct ModelData {
  sprite_data: BTreeMap<String, Sprite>,
  occupied_tiles: Vec<Vec2<u8>>
}
