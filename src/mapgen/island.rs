use core::{Vec2};
use mapgen::{CHUNK_SIZE, ChunkRelCoord};
use mapgen::seed::Seed2;
use noise::Seed;

const ISLAND_HEIGHT: f64 = 1.4;

pub struct Island {
  hills: Vec<Vec2<i64>>,
}

impl Island {
  /// Get overlay value. NOTE: Pos is relative to chunk coordinate.
  fn get_overlay(&self, pos: Vec2<i64>) -> f64 {
    let value = (CHUNK_SIZE as f64) / (
        self.hills.iter().min_by(|&x| (*x - pos).norm()).unwrap().norm()
    as Vec2<f64>) * ISLAND_HEIGHT;

    if value < 1.0 {
      value
    } else {
      1.0
    }
  }

  /// Generate a the `num` island of a chunk consiting of `nhills` number of
  /// hills within a radius of `radius` given a seed, `seed`.
  fn generate_island(seed: Seed2, radius: f64, nhills: i64,
                     chunk: Vec2<i64>, num: i64) -> Island {
    let hills: Vec<Vec2<i64>> = vec![];
    for i in 1..nhills {
      hills.push(Vec2(
        seed.feed(0).feed(i).feed(num)
            .feed_vec(chunk).get() as i64,
        seed.feed(1).feed(i).feed(num)
            .feed_vec(chunk).get() as i64
      ));
    }

    Island {
      hills: hills,
    }
  }
}
