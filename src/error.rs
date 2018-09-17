use reader::input_reader::InputChars;
use std::error::Error as StdError;
use std::fmt;

/// Different types of errors that can occur.
#[derive(Debug)]
pub enum ErrorType {
    /// RDF writer produces invalid RDF (e.g. if invalid node types are provided).
    InvalidWriterOutput,

    /// RDF lexer reads invalid RDF (e.g. non-closing string).
    InvalidReaderInput,

    /// RDF reader reads an invalid token (e.g. invalid node type).
    InvalidToken,

    /// RDF reader reaches the end of the input and stores the characters that were read last.
    EndOfInput(InputChars),

    /// Input reader encounters invalid byte encoding.
    InvalidByteEncoding,

  /// Incorrect namespace.
  InvalidNamespace,

  /// RDF SPARQL reader reads invalid SPARQL input.
  InvalidSparqlInput
}

/// An error related to the rdf-rs module.
#[derive(Debug)]
pub struct Error {
    error_type: ErrorType,
    error: Box<StdError>,
}

impl Error {
    /// Constructor of `Error`.
    pub fn new<E>(error_type: ErrorType, error: E) -> Error
    where
        E: Into<Box<StdError>>,
    {
        Error {
            error_type,
            error: error.into(),
        }
    }

    /// Returns the type of the error.
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
