use std::fmt::Debug;
use std::f32;
use std::f64;
use std::ops::{Add, Sub, Mul, Div, Rem};


pub trait AngleDatum
  : Add<Output=Self>
  + Sub<Output=Self>
  + Mul<Self, Output=Self>
  + Div<Self, Output=Self>
  + Rem<Output=Self>
  + PartialOrd
  + Clone + Copy + Debug {

  fn zero() -> Self;
  fn pi() -> Self;
  fn pi_2() -> Self;
  fn _180_degrees() -> Self;
}


impl AngleDatum for f32 {
  fn zero() -> Self { 0.0 }
  fn pi() -> Self { f32::consts::PI }
  fn pi_2() -> Self { f32::consts::PI * 2.0 }
  fn _180_degrees() -> Self { 180.0 }
}


impl AngleDatum for f64 {
  fn zero() -> Self { 0.0 }
  fn pi() -> Self { f64::consts::PI }
  fn pi_2() -> Self { f64::consts::PI * 2.0 }
  fn _180_degrees() -> Self { 180.0 }
}


#[derive(Clone, Copy, Debug)]
pub struct Angle<T: AngleDatum> {
  radians: T
}


impl<T: AngleDatum> Angle<T> {
  pub fn radians(theta: T) -> Angle<T> {
    Angle { radians: Self::normalize(theta) }
  }

  pub fn degrees(theta: T) -> Angle<T> {
    Angle { radians: Self::normalize( Self::deg_to_rad(theta) ) }
  }

  pub fn as_radians(&self) -> T { self.radians }
  pub fn as_degrees(&self) -> T { Self::rad_to_deg(self.radians) }

  fn rad_to_deg(rad: T) -> T { (rad * T::_180_degrees()) / T::pi() }
  fn deg_to_rad(deg: T) -> T { (deg * T::pi()) / T::_180_degrees() }

  /// Return an equivalent radian value on the interval [-PI, PI)
  fn normalize(radians: T) -> T {
    let mut normalized = radians;
    let zero = T::zero();
    let pi_2 = T::pi_2();
    while normalized < zero {
      normalized = normalized + pi_2;
    }
    normalized % pi_2
  }
}


impl<T: AngleDatum> Add for Angle<T> {
  type Output = Angle<T>;

  fn add(self, other: Angle<T>) -> Angle<T> {
    Angle { radians: Self::normalize(self.radians + other.radians) }
  }
}


impl<T: AngleDatum> Sub for Angle<T> {
  type Output = Angle<T>;

  fn sub(self, other: Angle<T>) -> Angle<T> {
    Angle { radians: Self::normalize(self.radians - other.radians) }
  }
}
