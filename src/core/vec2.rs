use std::fmt::Debug;
use std::f64;
use std::ops::{ Add, Sub, Mul, Div };

use core::Angle;


#[derive(Clone, Copy, Debug)]
/// Vector struct, implements vector operations
pub struct Vec2<T>(pub T, pub T)
  where T: Add<Output=T>
         + Sub<Output=T>
         + Mul<Output=T>
         + Div<Output=T>
         + Clone + Copy + Debug;


impl<T> Vec2<T>
  where T: Add<Output=T>
         + Sub<Output=T>
         + Mul<Output=T>
         + Div<Output=T>
         + Clone + Copy + Debug {
  /// Get x component
  pub fn x(&self) -> T { self.0 }
  /// Get y component
  pub fn y(&self) -> T { self.1 }
  /// Get norm (i.e. complex absolute value) 
  pub fn norm(&self) -> T {
    self.x() * self.x() + self.y() * self.y()
  }
}

// Question for the blue_deref: Why implement this for f32 and f64 and not just Float
impl Vec2<f32> {
  /// Convert polar form to Vec2
  pub fn from_magnitude(magnitude: f32, direction: Angle<f32>) -> Vec2<f32> {
    Vec2(
      f32::cos(direction.as_radians()) * magnitude,
      f32::sin(direction.as_radians()) * magnitude
    )
  }
}


impl Vec2<f64> {
  pub fn from_magnitude(magnitude: f64, direction: Angle<f64>) -> Vec2<f64> {
    Vec2(
      f64::cos(direction.as_radians()) * magnitude,
      f64::sin(direction.as_radians()) * magnitude
    )
  }
}


impl<T> Add for Vec2<T>
  where T: Add<Output=T>
         + Sub<Output=T>
         + Mul<Output=T>
         + Div<Output=T>
         + Clone + Copy + Debug {

  type Output = Vec2<T>;

  fn add(self, other: Vec2<T>) -> Self::Output {
    Vec2(self.0 + other.0, self.1 + other.1)
  }
}


impl<T> Sub for Vec2<T>
  where T: Add<Output=T>
         + Sub<Output=T>
         + Mul<Output=T>
         + Div<Output=T>
         + Clone + Copy + Debug {

  type Output = Vec2<T>;

  fn sub(self, other: Vec2<T>) -> Self::Output {
    Vec2(self.0 - other.0, self.1 - other.1)
  }
}


impl<T> Mul<T> for Vec2<T>
  where T: Add<Output=T>
         + Sub<Output=T>
         + Mul<Output=T>
         + Div<Output=T>
         + Clone + Copy + Debug {

  type Output = Vec2<T>;

  fn mul(self, scalar: T) -> Self::Output {
    Vec2(self.0 * scalar, self.1 * scalar)
  }
}


impl<T> Div<T> for Vec2<T>
  where T: Add<Output=T>
         + Sub<Output=T>
         + Mul<Output=T>
         + Div<Output=T>
         + Clone + Copy + Debug {

  type Output = Vec2<T>;

  fn div(self, scalar: T) -> Self::Output {
    Vec2(self.0 / scalar, self.1 / scalar)
  }
}
