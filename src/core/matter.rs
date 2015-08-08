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
}
