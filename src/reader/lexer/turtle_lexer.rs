use reader::lexer::rdf_lexer::RdfLexer;
use reader::lexer::token::Token;
use reader::input_reader::InputReader;
use std::io::Read;
use error::{Error, ErrorType};
use Result;

pub struct TurtleLexer<R: Read> {
  input_reader: InputReader<R>,
  peeked_token: Option<Token>
}

impl<R: Read> RdfLexer<R> for TurtleLexer<R> {
  /// Constructor for `TurtleLexer`.
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
      input_reader: InputReader::new(input),
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

    match try!(self.input_reader.get_next_char_discard_leading_spaces()) {
//      Some('#') => self.get_comment(),
//      Some('@') => self.get_language_specification(), // todo: base, prefix
//      Some('"') => self.get_literal(),
//      Some('<') => self.get_uri(),
//      Some('_') => self.get_blank_node(),
//      Some('^') => self.get_data_type(),
//      Some('.') => Ok(Token::TripleDelimiter),
//      None => Ok(Token::EndOfInput),
      Some(c) => Err(Error::new(ErrorType::InvalidReaderInput,
                                    "Invalid character while parsing Turtle syntax: ".to_string() + &c.to_string())),
      None => Err(Error::new(ErrorType::InvalidReaderInput, "Invalid Turtle input."))
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