use reader::lexer::rdf_lexer::RdfLexer;
use reader::lexer::token::Token;
use std::io::Read;
use helper;
use error::Error;
use Result;

pub struct TurtleLexer<R: Read> {
  input: R,
  peeked_token: Option<Token>
}

impl<R: Read> RdfLexer<R> for TurtleLexer<R> {
  /// Constructor for `TurtleLexer`;
  ///
  /// # Example
  ///
  /// ```
  /// use rdf_rs::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf_rs::reader::lexer::turtle_lexer::TurtleLexer;
  ///
  /// let input = "<example.org/a>".as_bytes();
  ///
  /// TurtleLexer::new(input);
  /// ```
  fn new(input: R) -> TurtleLexer<R> {
    TurtleLexer {
      input: input,
      peeked_token: None
    }
  }

  /// Determines the next token from the input.
  /// todo
  ///
  /// # Example
  ///
  /// ```
  /// use rdf_rs::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf_rs::reader::lexer::turtle_lexer::TurtleLexer;
  /// use rdf_rs::reader::lexer::token::Token;
  ///
  /// let input = "_:auto <example.org/b> \"test\" .".as_bytes();
  ///
  /// let mut lexer = TurtleLexer::new(input);
  ///
  /// assert_eq!(lexer.get_next_token().unwrap(), Token::BlankNode("auto".to_string()));
  /// assert_eq!(lexer.get_next_token().unwrap(), Token::Uri("example.org/b".to_string()));
  /// assert_eq!(lexer.get_next_token().unwrap(), Token::Literal("test".to_string()));
  /// assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
  /// ```
  fn get_next_token(&mut self) -> Result<Token> {
    match self.peeked_token.clone() {
      Some(token) => {
        self.peeked_token = None;
        return Ok(token)
      },
      None => { }
    }

    match helper::get_next_char_discard_leading_spaces(&mut self.input) {
//      Ok(Some('#')) => self.get_comment(),
//      Ok(Some('@')) => self.get_language_specification(), // todo: base, prefix
//      Ok(Some('"')) => self.get_literal(),
//      Ok(Some('<')) => self.get_uri(),
//      Ok(Some('_')) => self.get_blank_node(),
//      Ok(Some('^')) => self.get_data_type(),
//      Ok(Some('.')) => Ok(Token::TripleDelimiter),
//      Ok(None) => Ok(Token::EndOfInput),
      _ => Err(Error::InvalidReaderInput)
    }
  }


  /// Determines the next token without consuming it.
  ///
  /// # Examples
  ///
  /// todo
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