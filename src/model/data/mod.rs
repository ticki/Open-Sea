use std::collections::BTreeMap;

use math::Vec2;

mod error;
mod load;
mod occupied_tile_data;
mod sprite_data;

pub use self::error::{LoadModelError, ModelError};

/// A frame, i.e. a sprite cutted out of a sprite sheet.
#[derive(Clone, Debug)]
pub struct Frame {
    cut_from: Vec2<u16>,
    cut_offset: Vec2<i8>,
    size: Vec2<u16>,
    pin_to: Vec2<i16>,
    pin_offset: Vec2<i8>,
}

/// A sprite
#[derive(Clone, Debug)]
pub struct Sprite {
    resource: usize,
    frames: Vec<Frame>,
}


impl Sprite {
    /// Create new sprite
    pub fn new(resource: usize, frames: Vec<Frame>) -> Sprite {
        Sprite { resource: resource, frames: frames }
    }
}

/// Model data: data about a given model
#[derive(Debug)]
pub struct ModelData {
    sprite_data: BTreeMap<String, Sprite>,
    occupied_tiles: Vec<Vec2<i8>>,
}
