use crate::reader::input_reader::InputReader;
use crate::reader::lexer::token::Token;
use std::io::Read;
use crate::Result;

/// Trait implemented by RDF lexer.
pub trait RdfLexer<R: Read> {
    /// Constructor.
    fn new(input: R) -> Self;

    /// Determines the next token from the input.
    fn get_next_token(&mut self) -> Result<Token>;

    // Determines the next token without consuming it.
    fn peek_next_token(&mut self) -> Result<Token>;
}

/// Contains implemented rules for parsing RDF input.
pub trait TokensFromRdf<R: Read> {
    /// Consumes the next character of the input reader.
    fn consume_next_char(input_reader: &mut InputReader<R>) {
        let _ = input_reader.get_next_char();
    }
}
