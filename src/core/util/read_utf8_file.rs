use std::error::Error;
use std::io;
use std::io::Read;
use std::fmt;
use std::fs::File;
use std::string::FromUtf8Error;


/// Wraps a couple types of errors for `read_file`.
#[derive(Debug)]
pub enum ReadUtf8FileError {
    IoError(io::Error),
    Utf8Error(FromUtf8Error)
}


impl fmt::Display for ReadUtf8FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &ReadUtf8FileError::IoError(ref e) =>
                f.write_fmt(format_args!("{}", e)),
                &ReadUtf8FileError::Utf8Error(ref e) =>
                    f.write_fmt(format_args!("{}", e))
        }
    }
}


impl Error for ReadUtf8FileError {
    fn description(&self) -> &str {
        "a problem occurred trying to read a file"
    }
}


impl From<io::Error> for ReadUtf8FileError {
    fn from(e: io::Error) -> ReadUtf8FileError { ReadUtf8FileError::IoError(e) }
}


impl From<FromUtf8Error> for ReadUtf8FileError {
    fn from(e: FromUtf8Error) -> ReadUtf8FileError {
        ReadUtf8FileError::Utf8Error(e)
    }
}


/// An easy function to quickly read a whole file.
///
/// The price you pay is that errors are wrapped a bit.
pub fn read_utf8_file(path: &str) -> Result<String, ReadUtf8FileError> {
    let mut f = try!(File::open(path));
    let mut buffer = Vec::new();
    let _ = try!(f.read_to_end(&mut buffer));
    Ok( try!(String::from_utf8(buffer)) )
}
