use super::noise::{Brownian2, Seed, open_simplex2};

// Probably really buggy code...

// Should be in some json file in the future

/// A tile
enum Tile {
  Water,
  Grass,
}

/// A map
struct Map {
  seed: &'static Seed,
}

impl Map {
  /// Creates a new map
  fn new(seed: &Seed) -> Map {
    Map {
      seed: seed,
    }
  }

  // TODO: Add some sort of cache

  /// Get the noise value at a given point
  fn get_noise_value(&self, x: i64, y: i64) -> f64 {
    let noise = Brownian2::new(open_simplex2, 4).wavelength(32.0);
    noise.apply(&self.seed, &[x, y])
  }

  /// Get overlay value (used for customizing the noise)
  fn get_overlay_value(&self, x: i64, y: i64) -> f64 {
    // Stuff here
  }

  // Add foreground/background
  /// Get the tile at a given point
  fn get_tile(&self, x: i64, y: i64) -> Tile {
    let val = (self.get_noise_value(x, y) + self.get_gradient_value(x, y)) / 2;

    if(val > 0.9) {
      Tile::Grass
    } else {
      Tile::Water
    }
  }
}
