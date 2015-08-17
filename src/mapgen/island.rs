use core::{Vec2};
use mapgen::CHUNK_SIZE;
use noise::Seed;

const ISLAND_HEIGHT: f64 = 1.4;

pub struct Island {
  hills: Vec<Vec2<i64>>,
}

impl Island {
  /// Get overlay value. NOTE: Pos is relative to chunk coordinate.
  fn get_overlay(&self, pos: Vec2<i64>) -> f64 {
    let value = CHUNK_SIZE / self.hills.iter.min_by(|&x| (x - pos).norm())
                * ISLAND_HEIGHT;

    if value < 1.0 {
      value
    } else {
      1
    }
  }

  /// Generate a the `num` island of a chunk consiting of `nhills` number of
  /// hills within a radius of `radius` given a seed, `seed`.
  fn generate_island(seed: Seed, radius: f64, nhills: i64,
                     chunk: Vec2<i64>, num: i64) -> Island {
    let hills: Vec<Vec2<i64>> = vec![];
    for i in 1..nhills {
      hills.push(Vec2(
        seed.conc(0).conc(i).conc(num)
            .conc(chunk.x()).conc(chunk.y()).get(),
        seed.conc(1).conc(i).conc(num)
            .conc(chunk.x()).conc(chunk.y()))).get();
    }

    Island {
      hills: hills,
    }
  }
}
