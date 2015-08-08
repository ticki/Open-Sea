use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum ModelError {
  TopLevelNotObject,
  MissingKey { key: &'static str, context: &'static str },
  TypeError { obj: &'static str, expected: &'static str },
  InvalidKey { key: String, context: &'static str },
  MultiKeyWith,
  InvalidFrames { sprite_name: String, length: usize, max_index: usize },
  FrameRedef { sprite_name: String, frame_index: usize },
}


impl fmt::Display for ModelError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      &ModelError::TopLevelNotObject =>
        f.write_str("Model file must be JSON object"),

      &ModelError::MissingKey { ref key, ref context } =>
        f.write_fmt(format_args!("{} is missing key {:?}", context, key)),

      &ModelError::TypeError { ref obj, ref expected } =>
        f.write_fmt(
          format_args!("Type error: expected value of type `JSON {}` for {}",
                       expected,
                       obj )),

      &ModelError::InvalidKey { ref key, ref context } =>
        f.write_fmt(
          format_args!("found unexpected key {:?} in {}", key, context) ),

      &ModelError::MultiKeyWith =>
        f.write_str("Object containing \"with\" also contains other keys"),

      &ModelError::InvalidFrames { ref sprite_name, length, max_index } =>
        f.write_fmt(format_args!(
          "Missing frame definitions in sprite {:?} ({} frames found, but max index is {})",
          sprite_name,
          length,
          max_index )),

      &ModelError::FrameRedef { ref sprite_name, frame_index } =>
        f.write_fmt(format_args!(
                      "Multiple definitions found for frame {} of sprite {:?}",
                      frame_index,
                      sprite_name )),
    }
  }
}


impl Error for ModelError {
  fn description(&self) -> &str {
    "the model data is invalid"
  }
}
