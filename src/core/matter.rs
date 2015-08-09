use core::Vec2;

/// Matter, a trait carrying information about destroyability, solidness, hardness, and other
/// properties of matter.
pub trait Matter {
  fn is_solid(&self) -> bool {
    false
  }
  fn is_destroyable(&self) -> bool {
    false
  }
  fn get_hardness(&self) -> i32 {
    3
  }
  /// Get their collision map
  // TODO: Relative coordinates?
  fn get_collision_map(&self) -> Vec<Vec2<i64>> {
    Vec::new()
  }
}
