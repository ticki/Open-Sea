use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum ModelError {
  TopLevelNotObject,
  MissingKey(&'static str),
  TypeError { key: &'static str, expected: &'static str },
  MultiKeyWith,
  WithMissingKey(&'static str),
  WithTypeError { key: &'static str, expected: &'static str },
}


impl fmt::Display for ModelError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      &ModelError::TopLevelNotObject =>
        f.write_str("Model file must be JSON object"),

      &ModelError::MissingKey(ref key) =>
        f.write_fmt(format_args!("Model data is missing key {:?}", key)),

      &ModelError::TypeError { ref key, ref expected } =>
        f.write_fmt(
          format_args!("Type error: expected JSON {} for model key {:?}",
                       expected,
                       key )),

      &ModelError::MultiKeyWith =>
        f.write_str("Object containing \"with\" also contains other keys."),

      &ModelError::WithMissingKey(ref key) =>
        f.write_fmt(format_args!("\"with\" value is missing key {:?}", key)),

      &ModelError::WithTypeError { ref key, ref expected } =>
        f.write_fmt(
          format_args!("Type error: expected JSON {} for \"with\" key {:?}",
                       expected,
                       key )),
    }
  }
}


impl Error for ModelError {
  fn description(&self) -> &str {
    "the model data is invalid"
  }
}
