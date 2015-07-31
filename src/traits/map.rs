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
}


/// A map
pub struct Map<'a> {
  objects: Vec<Object>,
  player: &'a Player,
  tile_map: &'a TileMap,
}


/// A tiled map
pub trait TileMap {
  fn get_tile(&self, x: i64, i64) -> Tile;
}
