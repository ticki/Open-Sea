//! The module for the automatic map generation

use noise::{Brownian2, Seed, open_simplex2};

use core::{Tile, TileMap, Vec2};

mod island;
pub use self::island::{Island};
pub mod seed;
mod chunk;
use self::chunk::{Chunk};
use self::seed::{Seed2};

// TODO: consider just using `usize`. It /is/ an unsigned scalar.
pub const CHUNK_SIZE: i64 = 128;
#[allow(non_upper_case_globals)]
pub const CHUNK_SIZE_usize: usize = 128;

/// Chunk coordinate
pub type ChunkCoord = Vec2<i64>;
/// Chunk relative coordinate
pub type ChunkRelCoord = Vec<i16>;


/// A map
///
/// Note: It's chunked into big chunks and small chunks.
///       Small chunks keeps the noise data. Big chunks
///       determines the overlay layer.
pub struct MapGenerator {
  seed: Seed2, // TODO: Use pointer to seed instead?
}

/// Types of big chunks
pub enum ChunkType {
  /// Reserved for history, empty per default, but gets opened as the game is
  /// played. Manually designed.
  History,
  /// Automatic map generated chunk
  Auto,
  /// Manually random generated chunk
  Manually,
  /// Empty chunk
  Empty,
}

// TODO: Find out how to prevent double islands (manually generated islands)

// TODO: Use Vec2 in this methods:
impl MapGenerator { 
  /// Creates a new map
  pub fn new(seed: Seed2) -> MapGenerator {
    MapGenerator {
      seed: seed,
    }
  }

  /// Get the type of the chunk
  fn get_chunk_type(&self, pos: ChunkCoord) -> ChunkType {
    let val = self.seed.feed_vec(pos).get_f64();

    if val > 0.3 {
      ChunkType::Empty
    } else if val > 0.1 {
      ChunkType::Auto
    } else if val > 0.05 {
      ChunkType::History
    } else {
      ChunkType::Manually
    }
  }
  
  /// Get the overlay value
  fn get_overlay_value(&self, pos: ChunkRelCoord) {
    match self.get_chunk_type(pos) {
      ChunkType::Empty => 0.0,
      ChunkType::Auto => {
        let chunk = Chunk::generate(pos);

        chunk.islands.min_by(|&x| x.get_overlay(pos))
                     .get_overlay(pos)
       }
        _ => 0.0,
    }
  }
}

impl<'a> TileMap<'a> for MapGenerator {

  /// Get the tile at a given point
  fn get_tile(&self, coord: Vec2<i64>) -> Tile<'a> {
    let val = (self.get_noise_value(coord)
               + self.get_overlay_value(coord)) / 2.0;

    unimplemented!();
  }
}
