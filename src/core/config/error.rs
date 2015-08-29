use std::error::Error;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;

use rustc_serialize::{Decoder, json};

use core::util::ReadUtf8FileError;


#[derive(Debug)]
pub enum LoadConfigError {
    IoError(io::Error),
    Utf8Error(FromUtf8Error),
    JsonError(<json::Decoder as Decoder>::Error),
}


impl fmt::Display for LoadConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &LoadConfigError::IoError(ref e) =>
                f.write_fmt(format_args!("{}", e)),
                &LoadConfigError::Utf8Error(ref e) =>
                    f.write_fmt(format_args!("{}", e)),
                    &LoadConfigError::JsonError(ref e) =>
                        f.write_fmt(format_args!("{}", e)),
        }
    }
}


impl Error for LoadConfigError {
    fn description(&self) -> &str {
        "a problem occurred trying to load the configuration file"
    }
}


impl From<ReadUtf8FileError> for LoadConfigError {
    fn from(e: ReadUtf8FileError) -> LoadConfigError {
        match e {
            ReadUtf8FileError::IoError(e) => LoadConfigError::IoError(e),
            ReadUtf8FileError::Utf8Error(e) => LoadConfigError::Utf8Error(e),
        }
    }
}


/* This code causes an error (the line numbers are probably correct/close):

/*
src/core/config/error.rs:81:1: 85:2 error: conflicting implementations for trait `core::convert::From` [E0119]
src/core/config/error.rs:81 impl From<<json::Decoder as Decoder>::Error> for LoadConfigError {
src/core/config/error.rs:82   fn from(e: <json::Decoder as Decoder>::Error) -> LoadConfigError {
src/core/config/error.rs:83     LoadConfigError::JsonError(e)
src/core/config/error.rs:84   }
src/core/config/error.rs:85 }
src/core/config/error.rs:81:1: 85:2 note: conflicting implementation in crate `core`
src/core/config/error.rs:81 impl From<<json::Decoder as Decoder>::Error> for LoadConfigError {
src/core/config/error.rs:82   fn from(e: <json::Decoder as Decoder>::Error) -> LoadConfigError {
src/core/config/error.rs:83     LoadConfigError::JsonError(e)
src/core/config/error.rs:84   }
src/core/config/error.rs:85 }
src/core/config/error.rs:40:1: 47:2 error: conflicting implementations for trait `core::convert::From` [E0119]
src/core/config/error.rs:40 impl From<ReadUtf8FileError> for LoadConfigError {
src/core/config/error.rs:41   fn from(e: ReadUtf8FileError) -> LoadConfigError {
src/core/config/error.rs:42     match e {
src/core/config/error.rs:43       ReadUtf8FileError::IoError(e) => LoadConfigError::IoError(e),
src/core/config/error.rs:44       ReadUtf8FileError::Utf8Error(e) => LoadConfigError::Utf8Error(e),
src/core/config/error.rs:45     }
...
src/core/config/error.rs:81:1: 85:2 note: note conflicting implementation here
src/core/config/error.rs:81 impl From<<json::Decoder as Decoder>::Error> for LoadConfigError {
src/core/config/error.rs:82   fn from(e: <json::Decoder as Decoder>::Error) -> LoadConfigError {
src/core/config/error.rs:83     LoadConfigError::JsonError(e)
src/core/config/error.rs:84   }
src/core/config/error.rs:85 }
*/

impl From<<json::Decoder as Decoder>::Error> for LoadConfigError {
    fn from(e: <json::Decoder as Decoder>::Error) -> LoadConfigError {
        LoadConfigError::JsonError(e)
    }
}
// */
