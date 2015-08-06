use std::error::Error;
use std::io;
use std::io::Read;
use std::fmt;
use std::fs::File;
use std::string::FromUtf8Error;


/// Wraps a couple types of errors for `read_file`.
#[derive(Debug)]
pub enum ReadFileError {
  IoError(io::Error),
  Utf8Error(FromUtf8Error)
}


impl fmt::Display for ReadFileError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    f.write_fmt(format_args!("{:?}", *self))
  }
}


impl Error for ReadFileError {
  fn description(&self) -> &str {
    "a problem occurred trying to read a file"
  }
}


impl From<io::Error> for ReadFileError {
  fn from(e: io::Error) -> ReadFileError { ReadFileError::IoError(e) }
}


impl From<FromUtf8Error> for ReadFileError {
  fn from(e: FromUtf8Error) -> ReadFileError { ReadFileError::Utf8Error(e) }
}


/// An easy function to quickly read a whole file.
///
/// The price you pay is that errors are wrapped a bit.
pub fn read_file(path: &str) -> Result<String, ReadFileError> {
  let mut f = try!(File::open(path));
  let mut buffer = Vec::new();
  let _ = try!(f.read_to_end(&mut buffer));
  Ok( try!(String::from_utf8(buffer)) )
}
