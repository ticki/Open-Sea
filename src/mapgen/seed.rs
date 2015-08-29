// A wrapper around noise.rs

// Consider writing our own noise lib

// TODO: Take more pointers as arguments

use noise::*;
use math::Vec2;
use std::hash::*;

/// A seed
#[derive(Clone, Hash)]
struct Seed2 {
    seed: i64,
}

impl Seed2 {
    fn new(seed: i64) -> Seed2 {
        Seed2 {
            seed: seed,
        }
    }

    /// Add information to the seed
    fn feed(&self, with: i64) -> Seed2 {
        Seed2::new(((self.seed) >> 16) ^ with)
    }

    /// Add a vector to the seed
    fn feed_vec(&self, with: Vec2<i64>) -> Seed2 {
        self.feed(with.x()).feed(with.y())
    }

    /// Get the value of the seed (note: non-continious)
    fn get(&self) -> u64 {
        let hasher = SipHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    /// Get a value in the interval [0, 1]
    fn get_f64(&self) -> f64 {
        (self.get() as f64) / (u64::max_value() as f64)
    }

    /// Get the continious value of the seed
    fn get_noise(&self, chunk_size: f64, pos: Vec2<i64>) -> f64 {
        let noise = Brownian2::new(open_simplex2, 4)
            .wavelength(chunk_size);
        noise.apply(&self.to_seed(),
        &[pos.x() as f64, pos.y() as f64])
    }

    fn to_seed(&self) -> Seed {
        Seed::new(self.seed as u32)
    }

}


