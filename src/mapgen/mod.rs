extern crate noise;

use noise::{Brownian2, Seed, open_simplex2};

// Probably really buggy code...

// Should be in some json file in the future
enum Tile {
  Water,
  Grass,
}

struct Map {
  seed: &'static Seed,
}

impl Map {
  fn new(seed: &Seed) -> Map {
    Map {
      seed: seed,
    }
  }

  // TODO: Add some sort of cache
  fn get_noise_value(&self, x: i64, y: i64) -> f64 {
    let noise = Brownian2::new(open_simplex2, 4).wavelength(32.0);
    noise.apply(&seed, &[x, y])
  }

  fn get_overlay_value(&self, x: i64, y: i64) -> f64 {
    // Stuff here
  }

  // Add foreground/background
  fn get_tile(&self, x: i64, y: i64) -> Tile {
    let val = (self.get_noise_value(x, y) + self.get_gradient_value(x, y)) / 2;

    if(val > 0.9) {
      Grass
    } else {
        Water
    }
  }
}
