// Todo: Add tile metadata

// use models::*;
use core::{Vec2, Entity, Prop};
use model::{Player};

/// A block: a field consisting of three layers, containing tiles.
pub struct Tile<'a> {
  pub layers: Vec<&'a Prop>,
  solid: bool, // NOTE: When a tile load it's occupied_tiles should be set
               //       to have solid = true (This is sorta a todo)
}
// TODO: Make load_also, for the sprites which occupies more than one field.
//       This is to tell the renderer that it should also render this sprite.

/// A map
pub struct Map<'a> {
  entities: Vec<&'a Entity>,
  player: &'a Player,
  tile_map: &'a TileMap<'a>, // TODO: Instead, implement this trait for Map
}

/// A tiled map
pub trait TileMap<'a> {
  /// Get the tile on this given coordinate
  fn get_tile(&self, coord: Vec2<i64>) -> Tile<'a>;
  fn add_prop(&mut self, prop: &Prop) {
    unimplemented!()
    // TODO: Load the prob, and it's collision map, set all tiles in the collision maps's solid
    //       variable to true. And push the prop to the tile.
  }
  fn set_tile(&mut self, new_tile: Tile) {}
  fn get_solid(&self) -> bool {
    false
  }
  fn set_solid(&mut self, new: bool) {
    unimplemented!()
  }
}
