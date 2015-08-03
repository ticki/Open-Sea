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
  + Clone + Copy + Debug {

  fn pi() -> Self;
  fn pi_2() -> Self;
  fn _180_degrees() -> Self;
}


impl AngleDatum for f32 {
  fn pi() -> Self { f32::consts::PI }
  fn pi_2() -> Self { f32::consts::PI * 2.0 }
  fn _180_degrees() -> Self { 180.0 }
}


impl AngleDatum for f64 {
  fn pi() -> Self { f64::consts::PI }
  fn pi_2() -> Self { f64::consts::PI * 2.0 }
  fn _180_degrees() -> Self { 180.0 }
}


pub struct Angle<T: AngleDatum> {
  radians: T
}


impl<T: AngleDatum> Angle<T> {
  pub fn radians(theta: T) -> Angle<T> {
    Angle { radians: theta }
  }

  pub fn degrees(theta: T) -> Angle<T> {
    Angle { radians: (theta * T::pi()) / T::_180_degrees() }
  }

  pub fn as_radians(&self) -> T {
    self.radians
  }

  pub fn as_degrees(&self) -> T {
    (self.radians * T::_180_degrees()) / T::pi()
  }
}


impl<T: AngleDatum> Add for Angle<T> {
  type Output = Angle<T>;

  fn add(self, other: Angle<T>) -> Angle<T> {
    let result = (self.radians + other.radians) % T::pi();
    Angle { radians: result }
  }
}


impl<T: AngleDatum> Sub for Angle<T> {
  type Output = Angle<T>;

  fn sub(self, other: Angle<T>) -> Angle<T> {
    let result = ((self.radians - other.radians) + T::pi_2()) % T::pi_2();
    Angle { radians: result }
  }
}
