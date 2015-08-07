#![allow(dead_code)] // TODO remove

use std::collections::BTreeMap;
// TODO remove:
// use std::path::Path;

use core::Vec2;

mod error;
mod load;
mod sprite_data;

pub use self::error::{LoadModelError, ModelError};


#[derive(Debug)]
pub struct Frame {
  texture: u16,
  cut_from: (u16, u16),
  cut_offset: (i8, i8),
  size: (u16, u16),
  pin_to: (i32, i32),
  pin_offset: (i8, i8),
}


#[derive(Debug)]
pub struct Sprite {
  frames: Vec<Frame>
}


#[derive(Debug)]
pub struct ModelData {
  sprite_data: BTreeMap<String, Sprite>,
  occupied_tiles: Vec<Vec2<u8>>
}
