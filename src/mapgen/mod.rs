//! A module for the automatic map generation

use noise::{Brownian2, Seed, open_simplex2};

// TODO pick:
// use traits::*;
// use traits::{Map, Tile, TileMap};
use traits::{Tile, TileMap};

// Probably really buggy code...

/// A map
///
/// Note: It's chunked into big chunks and small chunks.
///       Small chunks keeps the noise data. Big chunks
///       determines the overlay layer.
pub struct MapGenerator<'a> {
  seed: &'a Seed,
}

/// Types of big chunks
enum BChunkType {
  /// Reserved for history, empty per default, but gets opened as the game is played. Manually designed.
  History,
  // Automatic map generated chunk
  Auto,
  // Manually random generated chunk
  Manually,
}

// TODO: Find out how to prevent double islands (manually generated islands)

impl<'a> MapGenerator<'a> {
  /// Creates a new map
  pub fn new(seed: &'a Seed) -> MapGenerator<'a> {
    MapGenerator {
      seed: seed,
    }
  }

  // TODO: Add some sort of cache

  /// Get the noise value at a given point
  fn get_noise_value(&self, x: i64, y: i64) -> f64 {
    let noise = Brownian2::new(open_simplex2, 4).wavelength(32.0);
    noise.apply(&self.seed, &[x as f64, y as f64])
  }

  /// Get big chunk coordinates
  fn get_bchunk(x: i64, y: i64) -> (i64, i64) {
    // TODO: Chunk size 64?
    (((x as f64) / 64.0).floor() as i64, ((y as f64) / 64.0).floor() as i64)
  }

  /* TODO: Finish this:
  fn get_bchunk_type(x: i64, y: i64) -> BChunkType {
    let noise = Brownian2::new(open_simplex2, 4).wavelength(32.0);
    noise.apply(&self.seed, &[x as f64, y as f64])
  }
  */

  /// Get overlay value (used for customizing the noise)
  fn get_overlay_value(&self, x: i64, y: i64) -> f64 {
    // TODO
    // Stuff here
    1.0
  }
}

impl<'a> TileMap for MapGenerator<'a> {

  // Add foreground/background
  /// Get the tile at a given point
  fn get_tile(&self, x: i64, y: i64) -> Tile {
    let val = (self.get_noise_value(x, y) + self.get_overlay_value(x, y)) / 2.0;

    if val > 0.9 {
      Tile::Grass
    } else {
      Tile::Water
    }
  }
}
