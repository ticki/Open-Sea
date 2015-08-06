use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum ModelError {
  MissingKey(&'static str),
  TypeError { key: &'static str, expected: &'static str },
  Other(&'static str),
}


impl fmt::Display for ModelError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      &ModelError::MissingKey(ref key) =>
        f.write_fmt(format_args!("model data is missing key {:?}", key)),

      &ModelError::TypeError { ref key, ref expected } =>
        f.write_fmt(
          format_args!("type error: expected JSON {} for model key {:?}",
                       expected,
                       key )),

      &ModelError::Other(message) => f.write_str(message)
    }
  }
}


impl Error for ModelError {
  fn description(&self) -> &str {
    "the model data is invalid"
  }
}
