use time;

use math::Vec2;


/// The direction of a given object
#[derive(Clone, Copy)]
pub enum Dir {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

impl Dir {
    fn to_vec(self) -> Vec2<i64> {
        match self {
            Dir::N => Vec2(0, 1),
            Dir::S => Vec2(0, -1),
            Dir::E => Vec2(1, 0),
            Dir::W => Vec2(-1, 0),
            Dir::NE => Vec2(1, 1),
            Dir::NW => Vec2(1, -1),
            Dir::SE => Vec2(-1, 1),
            Dir::SW => Vec2(-1, -1),
        }
    }
    /// Calculate the "locked" direction (i.e. the direction locked to the grid)
    fn to_locked(self) -> Dir {
        let mov = time::precise_time_s().round() as i64 % 2;
        match (self, mov) {
            (Dir::NE, 0) | (Dir::NW, 0) => Dir::N,
            (Dir::SE, 0) | (Dir::SW, 0) => Dir::S,
            (Dir::NE, 1) | (Dir::SE, 1) => Dir::E,
            (Dir::NW, 1) | (Dir::SW, 1) => Dir::W,
            _ => self,
        }
    }
}
