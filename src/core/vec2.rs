use std::fmt::Debug;
use std::ops::{Add, Sub, Mul, Div};

use num::{Float, Num};

use core::angle::{Angle, AngleDatum};


#[derive(Clone, Copy, Debug)]
/// Vector struct, implements vector operations
pub struct Vec2<T>(pub T, pub T)
  where T: Copy + Debug + Num;


impl<T> Vec2<T>
  where T: Copy + Debug + Num {

  /// Get x component
  pub fn x(&self) -> T { self.0 }
  /// Get y component
  pub fn y(&self) -> T { self.1 }
  /// Get norm (i.e. complex absolute value) 
  pub fn norm(&self) -> T {
    self.x() * self.x() + self.y() * self.y()
  }
}


impl<F: AngleDatum> Vec2<F> {
  /// Convert polar form to Vec2
  pub fn from_magnitude(magnitude: F, direction: Angle<F>) -> Vec2<F> {
    Vec2(
      F::cos(direction.as_radians()) * magnitude,
      F::sin(direction.as_radians()) * magnitude
    )
  }
}


impl<T> Add for Vec2<T>
  where T: Copy + Debug + Num {

  type Output = Vec2<T>;

  fn add(self, other: Vec2<T>) -> Self::Output {
    Vec2(self.0 + other.0, self.1 + other.1)
  }
}


impl<T> Sub for Vec2<T>
  where T: Copy + Debug + Num {

  type Output = Vec2<T>;

  fn sub(self, other: Vec2<T>) -> Self::Output {
    Vec2(self.0 - other.0, self.1 - other.1)
  }
}


impl<T> Mul<T> for Vec2<T>
  where T: Copy + Debug + Num {

  type Output = Vec2<T>;

  fn mul(self, scalar: T) -> Self::Output {
    Vec2(self.0 * scalar, self.1 * scalar)
  }
}


impl<T> Div<T> for Vec2<T>
  where T: Copy + Debug + Num {

  type Output = Vec2<T>;

  fn div(self, scalar: T) -> Self::Output {
    Vec2(self.0 / scalar, self.1 / scalar)
  }
}

impl From<Vec2<i64>> for Vec2<f64> {
  #[inline] 
  fn from(old: Vec2<i64>) -> Vec2<f64> {
    Vec2(old.x() as f64, old.y() as f64)
  }
}
