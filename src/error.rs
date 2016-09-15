use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
  InvalidWriterOutput
}


impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::InvalidWriterOutput => write!(f, "Invalid writer output"),
    }
  }
}


impl StdError for Error {
  fn description(&self) -> &str {
    match *self {
      Error::InvalidWriterOutput => "Invalid writer output",
    }
  }
}