use std::collections::BTreeMap;

use core::Vec2;

mod error;
mod load;
mod sprite_data;

pub use self::error::{LoadModelError, ModelError};


#[derive(Clone, Debug)]
pub struct Frame {
  cut_from: (u16, u16),
  cut_offset: (i8, i8),
  size: (u16, u16),
  pin_to: (i16, i16),
  pin_offset: (i8, i8),
}


#[derive(Clone, Debug)]
pub struct Sprite {
  resource: usize,
  frames: Vec<Frame>,
}


impl Sprite {
  pub fn new(resource: usize, frames: Vec<Frame>) -> Sprite {
    Sprite { resource: resource, frames: frames }
  }
}


#[derive(Debug)]
pub struct ModelData {
  sprite_data: BTreeMap<String, Sprite>,
  occupied_tiles: Vec<Vec2<u8>>,
}
