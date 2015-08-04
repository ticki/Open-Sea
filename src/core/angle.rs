use std::fmt::Debug;
use std::f32;
use std::f64;
use std::ops::{Add, Sub};

use num::Float;


pub trait AngleDatum : Copy + Debug + Float {
  fn pi() -> Self;
  fn _2_pi() -> Self;
  fn _180() -> Self;
}


impl AngleDatum for f32 {
  fn pi() -> Self { f32::consts::PI }
  fn _2_pi() -> Self { f32::consts::PI * 2.0 }
  fn _180() -> Self { 180.0 }
}


impl AngleDatum for f64 {
  fn pi() -> Self { f64::consts::PI }
  fn _2_pi() -> Self { f64::consts::PI * 2.0 }
  fn _180() -> Self { 180.0 }
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

  fn rad_to_deg(rad: T) -> T { (rad * T::_180()) / T::pi() }
  fn deg_to_rad(deg: T) -> T { (deg * T::pi()) / T::_180() }

  /// Return an equivalent radian value on the interval [-PI, PI)
  fn normalize(radians: T) -> T {
    let mut normalized = radians;
    let zero = T::zero();
    let _2_pi = T::_2_pi();
    while normalized < zero {
      normalized = normalized + _2_pi;
    }
    normalized % _2_pi
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
