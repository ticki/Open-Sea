use mapgen::{CHUNK_SIZE_usize, CHUNK_SIZE};
use core::{Vec2, Tile, TileMap};

/// A chunk grid, i.e. a grid of chunk size cotaining Block
pub type ChunkGrid<'a> = [[Tile<'a>; CHUNK_SIZE_usize]; CHUNK_SIZE_usize];
/// A cache of the relevant chunks
///
/// The ordering of the chunks are as follows.
/// 1---2
/// |   |
/// 3---4
pub struct Cache<'a> {
  /// The offset of the cache, i.e. the place where you find chunk1
  offset: Vec2<i64>,
  chunk1: ChunkGrid<'a>,
  chunk2: ChunkGrid<'a>,
  chunk3: ChunkGrid<'a>,
  chunk4: ChunkGrid<'a>,
}
// NOTE: When moving chunks around remember to use mem::replace

impl<'a> Cache<'a> {
  fn update<'b, T>(&mut self, new_offset: Vec2<i64>, mapgen: T)
            where T: TileMap<'b>, 'b: 'a {
    let mut collision_map: Vec<Vec2<i64>> = Vec::new();
    // TODO: Optimize this by using parts of the old cache which just should be
    //       moved.
    // TODO: Consume iters?
    for (y, row) in self.chunk1.iter_mut().enumerate() {
      for (x, block_ptr) in row.iter_mut().enumerate() {
       
        let coord = Vec2(x as i64, y as i64);
        *block_ptr = mapgen.get_tile(new_offset + coord); // Lifetime 'a
        
        for &i in block_ptr.layers.iter() {
          for j in i.get_collision_map() {
            collision_map.push(j);
          }
        }
     }
    }
    for (y, row) in self.chunk2.iter_mut().enumerate() {
      for (x, block_ptr) in row.iter_mut().enumerate() {
        let coord = Vec2(x as i64, y as i64);

        *block_ptr = mapgen.get_tile(new_offset + coord
                                      + Vec2(CHUNK_SIZE, 0));

        for &i in block_ptr.layers.iter() {
          for j in i.get_collision_map() {
            collision_map.push(j);
          }
        }
      }
    }
    for (y, row) in self.chunk3.iter_mut().enumerate() {
      for (x, block_ptr) in row.iter_mut().enumerate() {
        let coord = Vec2(x as i64, y as i64);

        *block_ptr = mapgen.get_tile(new_offset + coord
                                      + Vec2(0, CHUNK_SIZE));
        
       for &i in block_ptr.layers.iter() {
          for j in i.get_collision_map() {
            collision_map.push(j);
          }
        }
      }
    }
    for (y, row) in self.chunk4.iter_mut().enumerate() {
      for (x, block_ptr) in row.iter_mut().enumerate() {
        let coord = Vec2(x as i64, y as i64);

        *block_ptr = mapgen.get_tile(new_offset + coord
                                      + Vec2(CHUNK_SIZE, CHUNK_SIZE));
        for &i in block_ptr.layers.iter() {
          for j in i.get_collision_map() {
            collision_map.push(j);
          }
        }
      }
    }
    self.offset = new_offset;
    for i in collision_map {
      if (i - self.offset).x() < CHUNK_SIZE
         && (i - self.offset).y() < CHUNK_SIZE {
        let coord = i - self.offset;
        self.chunk1[coord.y() as usize][coord.x() as usize].solid = true;
      } else if (i - self.offset).x() >= CHUNK_SIZE
         && (i - self.offset).y() < CHUNK_SIZE {
        let coord = i - self.offset - Vec2(0, CHUNK_SIZE);
        self.chunk2[coord.y() as usize][coord.x() as usize].solid = true;
      } else if (i - self.offset).x() < CHUNK_SIZE
         && (i - self.offset).y() >= CHUNK_SIZE {
        let coord = i - self.offset - Vec2(CHUNK_SIZE, 0);
        self.chunk3[coord.y() as usize][coord.x() as usize].solid = true;
      } else if (i - self.offset).x() >= CHUNK_SIZE
         && (i - self.offset).y() >= CHUNK_SIZE {
        let coord = i - self.offset - Vec2(CHUNK_SIZE, CHUNK_SIZE);
        self.chunk3[coord.y() as usize][coord.x() as usize].solid = true;
      }
    }
  }
}

impl<'a> TileMap<'a> for Cache<'a> {
  fn get_tile(&self, pos: Vec2<i64>) -> Tile<'a> {
    let rel_coord = pos - self.offset;
    // TODO uncomment all this; had to squelch the errors
    unimplemented!()
    /*
    // TODO: Handle error cases where requested tile is off cache
    if rel_coord.x() < CHUNK_SIZE && rel_coord.y() < CHUNK_SIZE {
      self.chunk1.clone()[rel_coord.y() as usize][rel_coord.x() as usize]
    } else if rel_coord.x() >= CHUNK_SIZE && rel_coord.y() < CHUNK_SIZE {
      self.chunk2.clone()[rel_coord.y() as usize][rel_coord.x() as usize]
    } else if rel_coord.x() < CHUNK_SIZE && rel_coord.y() >= CHUNK_SIZE {
      self.chunk3.clone()[rel_coord.y() as usize][rel_coord.x() as usize]
    } else if rel_coord.x() >= CHUNK_SIZE && rel_coord.y() >= CHUNK_SIZE {
      self.chunk4.clone()[rel_coord.y() as usize][rel_coord.x() as usize]
    } else {
      Tile {
        layers: vec![],
        solid: false,
      }
    }
    */
  }
  // TODO: Add method which_chunk
}

// TODO: Implement tilemap for cache.
