use std::fmt;
use std::error::Error as StdError;

/// todo
#[derive(Debug)]
pub enum ErrorType {
  InvalidWriterOutput,
  InvalidReaderInput,
  InvalidToken,
  EndOfInput(String),
  InvalidByteEncoding,
  InvalidNamespace
}


/// An error related to the rdf-rs module.
///
/// # Example
///
/// todo
///
#[derive(Debug)]
pub struct Error {
  error_type: ErrorType,
  error: Box<StdError>
}


impl Error {
  pub fn new<E>(error_type: ErrorType, error: E) -> Error
    where E: Into<Box<StdError>> {
    Error {
      error_type: error_type,
      error: error.into()
    }
  }

  pub fn error_type(&self) -> &ErrorType {
    &self.error_type
  }
}


impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.error.fmt(f)
  }
}


impl StdError for Error {
  fn description(&self) -> &str {
    self.error.description()
  }
}