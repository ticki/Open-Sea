// Todo: Add tile metadata

// use models::*;
use core::{Vec2, Entity, Prop};
use model::{Player};

/// A block: a field consisting of three layers, containing tiles.
pub struct Tile<'a> {
  layers: Vec<&'a Prop>,
}

/// A map
pub struct Map<'a> {
  entities: Vec<&'a Entity>,
  player: &'a Player,
  tile_map: &'a TileMap, // TODO: Instead, implement this trait for Map
}

/// A tiled map
pub trait TileMap {
  /// Get the tile on this given coordinate
  fn get_tile(&self, coord: Vec2<i64>) -> Tile;
}
