use mapgen::{ChunkCoord, Island};

/// A chunk
struct Chunk {
  islands: Vec<Island>,
}

impl Chunk {
  fn generate(pos: ChunkCoord) -> Chunk {
    unimplemented!()
  }
}
