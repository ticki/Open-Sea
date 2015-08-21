use std::fmt::Debug;

use num::Num;

use math::Vec2;


/// A rectangle for simple bounds checking
pub struct Rect<T> where T: Copy + Debug + Num + PartialOrd {
  /// Top-left
  tl: Vec2<T>,
  /// Bottom-right
  br: Vec2<T>,
}


impl<T> Rect<T> where T: Copy + Debug + Num + PartialOrd {
  /// Create a `Rect` from a top-left corner and a bottom-right corner.
  pub fn new_from_corners(mut tl: Vec2<T>, mut br: Vec2<T>) -> Rect<T> {
    if tl.x() > br.x() {
      let temp = tl.0;
      tl.0 = br.0;
      br.0 = temp;
    }
    if tl.y() > br.y() {
      let temp = tl.1;
      tl.1 = br.1;
      br.1 = temp;
    }
    Rect { tl: tl, br: br }
  }

  /// Create a `Rect` from a top-left corner and a size vector.
  pub fn new_from_size(tl: Vec2<T>, size: Vec2<T>) -> Result<Rect<T>, ()> {
    if size.x() > T::zero() || size.y() > T::zero() {
      return Ok(Rect { tl: tl, br: tl + size })
    }
    Err(())
  }

  #[inline]
  pub fn left(&self) -> T { self.tl.x() }
  #[inline]
  pub fn top(&self) -> T { self.tl.y() }
  #[inline]
  pub fn right(&self) -> T { self.br.x() }
  #[inline]
  pub fn bottom(&self) -> T { self.br.y() }

  pub fn size(&self) -> Vec2<T> { self.br - self.tl }

  /// Check if `self` intersects with `other`.
  pub fn intersects(&self, other: &Rect<T>) -> bool {
    // Each of these variables means `self` is sufficiently distanced (up or to
    // the right) away from `other` such that they cannot intersect (or
    // vice-versa, where `other` is up or to the right).
    let self_right = self.left() > other.right();
    let other_right = other.left() > self.right();
    let self_up = other.top() > self.bottom();
    let other_up = self.top() > other.bottom();
    // If none of these are true, they are intersecting.
    return !(self_right || other_right || self_up || other_up);
  }
}
