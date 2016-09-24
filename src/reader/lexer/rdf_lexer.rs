use reader::lexer::token::Token;
use std::io::Read;
use Result;

/// Trait implemented by RDF lexer.
pub trait RdfLexer<R: Read> {
  fn new(input: R) -> Self;

  fn get_next_token(&mut self) -> Result<Token>;
}