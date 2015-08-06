use std::error::Error;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;

use rustc_serialize::json;

use core::util::ReadUtf8FileError;

use super::ModelError;


#[derive(Debug)]
pub enum LoadModelError {
  IoError(io::Error),
  Utf8Error(FromUtf8Error),
  JsonError(json::ParserError),
  ModelError(ModelError),
}


impl fmt::Display for LoadModelError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      &LoadModelError::IoError(ref e) => f.write_fmt(format_args!("{}", e)),
      &LoadModelError::Utf8Error(ref e) => f.write_fmt(format_args!("{}", e)),
      &LoadModelError::JsonError(ref e) => f.write_fmt(format_args!("{}", e)),
      &LoadModelError::ModelError(ref e) => f.write_fmt(format_args!("{}", e)),
    }
  }
}


impl Error for LoadModelError {
  fn description(&self) -> &str {
    "a problem occurred trying to load a model"
  }
}


impl From<io::Error> for LoadModelError {
  fn from(e: io::Error) -> LoadModelError { LoadModelError::IoError(e) }
}


impl From<FromUtf8Error> for LoadModelError {
  fn from(e: FromUtf8Error) -> LoadModelError { LoadModelError::Utf8Error(e) }
}


impl From<ReadUtf8FileError> for LoadModelError {
  fn from(e: ReadUtf8FileError) -> LoadModelError {
    match e {
      ReadUtf8FileError::IoError(e) => LoadModelError::IoError(e),
      ReadUtf8FileError::Utf8Error(e) => LoadModelError::Utf8Error(e)
    }
  }
}


impl From<json::ParserError> for LoadModelError {
  fn from(e: json::ParserError) -> LoadModelError { LoadModelError::JsonError(e) }
}
