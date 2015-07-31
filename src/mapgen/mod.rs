//! A module for the automatic map generation

use super::noise::{Brownian2, Seed, open_simplex2};

// Probably really buggy code...

// TODO: Make the map gen output a `Map`

/// A map
pub struct MapGenerator<'a> {
  seed: &'a Seed,
}

pub impl<'a> MapGenerator<'a> {
  /// Creates a new map
  fn new(seed: &'a Seed) -> Map<'a> {
    Map {
      seed: seed,
    }
  }

  // TODO: Add some sort of cache

  /// Get the noise value at a given point
  fn get_noise_value(&self, x: i64, y: i64) -> f64 {
    let noise = Brownian2::new(open_simplex2, 4).wavelength(32.0);
    noise.apply(&self.seed, &[x as f64, y as f64])
  }

  /// Get overlay value (used for customizing the noise)
  fn get_overlay_value(&self, x: i64, y: i64) -> f64 {
    // Stuff here
    1.0
  }

  // Add foreground/background
  /// Get the tile at a given point
  fn get_tile(&self, x: i64, y: i64) -> Tile {
    let val = (self.get_noise_value(x, y) + self.get_overlay_value(x, y)) / 2.0;

    if(val > 0.9) {
      Tile::Grass
    } else {
      Tile::Water
    }
  }
}
