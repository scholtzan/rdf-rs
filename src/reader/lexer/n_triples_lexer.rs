use reader::lexer::rdf_lexer::RdfLexer;
use reader::lexer::token::Token;
use std::io::Read;

pub struct NTriplesLexer<R: Read> {
  input: R
}

impl<R: Read> RdfLexer<R> for NTriplesLexer<R> {
  /// Constructor for `NTriplesLexer`;
  ///
  /// # Example
  ///
  /// todo
  ///
  fn new(input: R) -> NTriplesLexer<R> {
    NTriplesLexer {
      input: input
    }
  }

  /// Determines the next token from the input.
  fn get_next_token(&mut self) -> Token {
    // todo: maybe as result?
    Token::TripleDelimiter
  }
}