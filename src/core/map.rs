// Todo: Add tile metadata

// use models::*;
use core::{Vec2, Entity};
use model::{Player};
use opengl_graphics::{Texture};

/// Props, i.e. non dynamic objects
pub trait Prop {
  fn get_sprite(&self) -> &Texture;
  // TODO: Make a trait with the following methods, called Matter. Implement this for entity too.
  fn is_solid(&self) -> bool;
  fn is_destroyable(&self) -> bool;
  fn get_hardness(&self) -> i32;
}

/// A block: a field consisting of three layers, containing tiles.
pub struct Tile<'a> {
  layers: Vec<&'a Prop>,
}

/// A map
pub struct Map<'a> {
  entities: Vec<&'a Entity>,
  player: &'a Player,
  tile_map: &'a TileMap,
}

/// A tiled map
pub trait TileMap {
  /// Get the tile on this given coordinate
  fn get_tile(&self, coord: Vec2<i64>) -> Tile;
}
