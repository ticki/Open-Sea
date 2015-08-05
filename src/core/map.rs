// Todo: Add tile metadata

// use models::*;
use core::{Vec2};

struct Entity;
struct Player;

/// The tiles
pub enum Tile {
  Water,
  Grass,
  Tree,
  None,
}

/// A block: a field consisting of three layers, containing tiles.
pub struct Block {
  background: Tile,
  ground: Tile,
  foreground: Tile,
}

impl Block {
  pub fn new(background: Tile, ground: Tile, foreground: Tile) -> Block {
    Block {
      background: background,
      ground: ground,
      foreground: foreground,
    }
  }
}

/// A map
pub struct Map<'a> {
  objects: Vec<Entity>,
  player: &'a Player,
  tile_map: &'a TileMap,
}

// Todo add layers
/// A tiled map
pub trait TileMap {
  /// Get the tile on this given coordinate
  fn get_tile(&self, coord: Vec2<i64>) -> Block;
}
