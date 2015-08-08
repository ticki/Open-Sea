use mapgen::*;
use core::{Vec2, Tile};

/// A chunk grid, i.e. a grid of chunk size cotaining Block
type ChunkGrid = [[&Tile; CHUNK_SIZE]; CHUNK_SIZE];
/// A cache of the relevant chunks
///
/// The ordering of the chunks are as follows.
/// 1---2
/// |   |
/// 3---4
struct cache {
  /// The offset of the cache, i.e. the place where you find chunk1
  offset: Vec2<i64>,
  chunk1: ChunkGrid,
  chunk2: ChunkGrid,
  chunk3: ChunkGrid,
  chunk4: ChunkGrid,
}
// NOTE: When moving chunks around remember to use mem::replace

impl cache {
  fn update(&mut self, new_offset: Vec2<i64>, mapgen: T)
            where T: TileMap {
    // TODO: Optimize this by using parts of the old cache which just should be
    //       moved.
    for (y, &mut row) in chunk1.into_iter().enumerate() {
      for (x, &mut block_ptr) in chunk1.into_iter().enumerate() {
        *block_ptr = &mapgen.get_tile(new_offset);
      }
    }
    for (y, &mut row) in chunk2.into_iter().enumerate() {
      for (x, &mut block_ptr) in chunk2.into_iter().enumerate() {
        *block_ptr = &mapgen.get_tile(new_offset
                                      + Vec2(CHUNK_SIZE, 0));
      }
    }
    for (y, &mut row) in chunk3.into_iter().enumerate() {
      for (x, &mut block_ptr) in chunk3.into_iter().enumerate() {
        *block_ptr = &mapgen.get_tile(new_offset
                                      + Vec2(0, CHUNK_SIZE));
      }
    }
    for (y, &mut row) in chunk4.into_iter().enumerate() {
      for (x, &mut block_ptr) in chunk4.into_iter().enumerate() {
        *block_ptr = &mapgen.get_tile(new_offset
                                      + Vec2(CHUNK_SIZE, CHUNK_SIZE));
      }
    }
    self.offset = new_offset;
  }

  fn get_block(&self) -> Block {
    unimplemented!();
  }
}
