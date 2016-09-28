use reader::lexer::token::Token;
use std::io::Read;
use Result;

/// Trait implemented by RDF lexer.
pub trait RdfLexer<R: Read> {
  /// Constructor.
  fn new(input: R) -> Self;

  /// Determines the next token from the input.
  fn get_next_token(&mut self) -> Result<Token>;

  /// Determines the next token without consuming it.
  fn peek_next_token(&mut self) -> Result<Token>;
}