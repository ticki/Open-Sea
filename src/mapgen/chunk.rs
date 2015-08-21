use mapgen::{ChunkCoord, Island};
use mapgen::seed::Seed2;

/// A chunk
struct Chunk {
  islands: Vec<Island>,
}

const MAX_ISLANDS: u16 = 6;
const MAX_RADIUS: f64 = 10.0;

impl Chunk {
  fn new() {
    Chunk {
      islands: Vec::new()
    }
  }
  fn generate(seed: Seed2, pos: ChunkCoord) -> Chunk {
    let mut chunk = Chunk::new();
    let num_islands = (seed.feed_vec(pos).feed(42).get_f64()
                       * (MAX_ISLANDS as f64)).round() as u16;
    for i in 1..num_islands {
      chunk.islands.push(seed,
                         // Radius
                         seed.feed(666).feed(i).get_f64()
                          * MAX_RADIUS,
                         // Number of hills
                         4,
                         // Chunk Coordinate
                         pos,
                         // The island number
                         i);
    }
    chunk
  }
}
