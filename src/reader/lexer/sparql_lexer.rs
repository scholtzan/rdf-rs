use reader::lexer::rdf_lexer::{RdfLexer, TokensFromRdf};
use reader::input_reader::{InputReader};
use reader::lexer::n_triples_lexer::TokensFromNTriples;
use reader::lexer::turtle_lexer::TokensFromTurtle;
use std::io::Read;
use error::{Error, ErrorType};
use Result;
use reader::lexer::token::Token;

/// Produces tokens from SPARQL input.
pub struct SparqlLexer<R: Read> {
  input_reader: InputReader<R>,
  peeked_token: Option<Token>
}


impl<R: Read> RdfLexer<R> for SparqlLexer<R> {
  /// Constructor for `SparqlLexer`.
  ///
  /// # Examples
  ///
  /// ```
  /// use rdf_rs::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf_rs::reader::lexer::sparql_lexer::SparqlLexer;
  ///
  /// let input = "SELECT ?name".as_bytes();
  ///
  /// SparqlLexer::new(input);
  /// ```
  fn new(input: R) -> SparqlLexer<R> {
    SparqlLexer {
      input_reader: InputReader::new(input),
      peeked_token: None
    }
  }

  // todo
  /// Determines the next token from the input and consumes the read input characters.
  ///
  /// # Examples
  ///
  /// ```
  /// use rdf_rs::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf_rs::reader::lexer::sparql_lexer::SparqlLexer;
  /// use rdf_rs::reader::lexer::token::Token;
  ///
  /// let input = "SELECT".as_bytes();
  ///
  /// let mut lexer = SparqlLexer::new(input);
  /// ```
  ///
  /// # Failures
  ///
  /// - Input that does not conform to the SPARQL syntax standard.
  ///
  fn get_next_token(&mut self) -> Result<Token> {
    // first read peeked characters
    match self.peeked_token.clone() {
      Some(token) => {
        self.peeked_token = None;
        return Ok(token)
      },
      None => { }
    }

    match try!(self.input_reader.peek_next_char_discard_leading_spaces()) {
//      Some(c) if InputReaderHelper::letter(c) => {},
      Some('#') => return Self::get_comment(&mut self.input_reader),
      _ => Err(Error::new(ErrorType::InvalidSparqlInput, "Invalid SPARQL input."))
    }
  }

  // todo
  /// Determines the next token without consuming the input.
  ///
  /// # Examples
  ///
  /// ```
  /// use rdf_rs::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf_rs::reader::lexer::sparql_lexer::SparqlLexer;
  /// use rdf_rs::reader::lexer::token::Token;
  ///
  /// let input = "SELECT".as_bytes();
  ///
  /// let mut lexer = SparqlLexer::new(input);
  /// ```
  ///
  ///  # Failures
  ///
  /// - End of input reached.
  /// - Invalid input that does not conform with NTriples standard.
  ///
  fn peek_next_token(&mut self) -> Result<Token> {
    match self.peeked_token.clone() {
      Some(token) => Ok(token),
      None => {
        match self.get_next_token() {
          Ok(next) => {
            self.peeked_token = Some(next.clone());
            return Ok(next)
          },
          Err(err) => return Err(err)
        }
      }
    }
  }
}

/// Contains all implemented rules for creating tokens from SPARQL syntax.
pub trait TokensFromSparql<R: Read>: TokensFromTurtle<R> { }


impl<R: Read> TokensFromRdf<R> for SparqlLexer<R> { }
impl<R: Read> TokensFromNTriples<R> for SparqlLexer<R> { }
impl<R: Read> TokensFromTurtle<R> for SparqlLexer<R> { }
