use reader::lexer::token::Token;
use std::io::Read;

/// Trait implemented by RDF lexer.
pub trait RdfLexer<R: Read> {
  fn new(input: R) -> Self;

  fn get_next_token(&mut self) -> Token;
}