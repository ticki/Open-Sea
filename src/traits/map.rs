// Todo: Add tile metadata

use super::models::*;

/// The tiles
pub enum Tile {
  Water,
  Grass,
}

/// A map
struct Map<'a> {
  objects: Vec<Object>,
  player: &'a Player,
  tile_map: &TileMap,
}

/// A tiled map
trait TileMap {
  fn get_tile(&self, x: i64, i64) -> Tile;
}