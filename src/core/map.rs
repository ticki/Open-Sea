// Todo: Add tile metadata

// TODO: uncomment
// use models::*;


// TODO remove
struct Player;
// TODO also remove; I stubbed this out because it doesn't exist yet
struct Object;


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
  objects: Vec<Object>,
  player: &'a Player,
  tile_map: &'a TileMap,
}

// Todo add layers
/// A tiled map
pub trait TileMap {
  fn get_tile(&self, x: i64, i64) -> Block;
}
