use reader::lexer::rdf_lexer::RdfLexer;
use reader::lexer::token::Token;
use std::io::Read;
use helper;
use error::Error;
use Result;


pub struct NTriplesLexer<R: Read> {
  input: R,
  peeked_token: Option<Token>
}


impl<R: Read> RdfLexer<R> for NTriplesLexer<R> {
  /// Constructor for `NTriplesLexer`;
  ///
  /// # Example
  ///
  /// ```
  /// use rdf_rs::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf_rs::reader::lexer::n_triples_lexer::NTriplesLexer;
  ///
  /// let input = "<example.org/a>".as_bytes();
  ///
  /// let mut lexer = NTriplesLexer::new(input);
  /// ```
  fn new(input: R) -> NTriplesLexer<R> {
    NTriplesLexer {
      input: input,
      peeked_token: None
    }
  }

  /// Determines the next token from the input.
  ///
  /// # Example
  ///
  /// ```
  /// use rdf_rs::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf_rs::reader::lexer::n_triples_lexer::NTriplesLexer;
  /// use rdf_rs::reader::lexer::token::Token;
  ///
  /// let input = "_:auto <example.org/b> \"test\" .".as_bytes();
  ///
  /// let mut lexer = NTriplesLexer::new(input);
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
      Ok(Some('#')) => self.get_comment(),
      Ok(Some('@')) => self.get_language_specification(),
      Ok(Some('"')) => self.get_literal(),
      Ok(Some('<')) => self.get_uri(),
      Ok(Some('_')) => self.get_blank_node(),
      Ok(Some('^')) => self.get_data_type(),
      Ok(Some('.')) => Ok(Token::TripleDelimiter),
      Ok(None) => Ok(Token::EndOfInput),
      e => Err(Error::InvalidReaderInput)
    }
  }

  /// Determines the next token without consuming it.
  ///
  /// # Examples
  ///
  /// ```
  /// use rdf_rs::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf_rs::reader::lexer::n_triples_lexer::NTriplesLexer;
  /// use rdf_rs::reader::lexer::token::Token;
  ///
  /// let input = "_:auto <example.org/b> \"test\" .".as_bytes();
  ///
  /// let mut lexer = NTriplesLexer::new(input);
  ///
  /// assert_eq!(lexer.peek_next_token().unwrap(), Token::BlankNode("auto".to_string()));
  /// assert_eq!(lexer.peek_next_token().unwrap(), Token::BlankNode("auto".to_string()));
  /// assert_eq!(lexer.get_next_token().unwrap(), Token::BlankNode("auto".to_string()));
  /// assert_eq!(lexer.peek_next_token().unwrap(), Token::Uri("example.org/b".to_string()));
  /// ```
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


impl<R: Read> NTriplesLexer<R> {
  /// Parses the comment from the input and returns it as token.
  fn get_comment(&mut self) -> Result<Token> {
    match helper::get_until_discard_leading_spaces(&mut self.input, |c| c == '\n' || c == '\r') {
      Ok(str) => Ok(Token::Comment(str)),
      Err(Error::EndOfInput(str)) => Ok(Token::Comment(str)),
      _ => Err(Error::InvalidReaderInput)
    }
  }

  /// Parses the language specification from the input and returns it as token.
  fn get_language_specification(&mut self) -> Result<Token> {
    match helper::get_until(&mut self.input, |c| c == '\n' || c == '\r' || c == ' ' || c == '.') {
      Ok(str) => Ok(Token::LanguageSpecification(str)),
      Err(Error::EndOfInput(str)) => Ok(Token::LanguageSpecification(str)),
      _ => Err(Error::InvalidReaderInput)
    }
  }

  /// Parses a literal from the input and returns it as token.
  fn get_literal(&mut self) -> Result<Token> {
    match helper::get_until(&mut self.input, |c| c == '"') {
      Ok(str) => Ok(Token::Literal(str)),
      Err(err) => Err(err)
    }
  }

  /// Parses a URI from the input and returns it as token.
  fn get_uri(&mut self) -> Result<Token> {
    match helper::get_until(&mut self.input, |c| c == '>') {
      Ok(str) => Ok(Token::Uri(str)),
      Err(err) => Err(err)
    }
  }

  /// Parses a blank node ID from the input and returns it as token.
  fn get_blank_node(&mut self) -> Result<Token> {
    // get colon after under score
    match helper::get_next_char(&mut self.input) {
      Ok(Some(':')) => { }
      _ => return Err(Error::InvalidReaderInput)
    }

    match helper::get_until(&mut self.input, |c| c == '\n' || c == '\r' || c == ' ' || c == '.') {
      Ok(str) => Ok(Token::BlankNode(str)),
      Err(Error::EndOfInput(str)) => Ok(Token::BlankNode(str)),
      _ => Err(Error::InvalidReaderInput)
    }
  }

  /// Parses the data type from the input and returns it as token.
  fn get_data_type(&mut self) -> Result<Token> {
    match helper::get_next_char(&mut self.input) {
      Ok(Some('^')) => Ok(Token::DataTypeStart),
      _ => return Err(Error::InvalidReaderInput)
    }
  }
}


#[cfg(test)]
mod tests {
  use reader::lexer::rdf_lexer::RdfLexer;
  use reader::lexer::n_triples_lexer::NTriplesLexer;
  use reader::lexer::token::Token;

  #[test]
  fn parse_comment() {
    let mut input = "# Hello World!\n# Foo".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::Comment("Hello World!".to_string()));
    assert_eq!(lexer.get_next_token().unwrap(), Token::Comment("Foo".to_string()));
  }

  #[test]
  fn parse_literal() {
    let mut input = "\"a\"@abc".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::Literal("a".to_string()));
  }

  #[test]
  fn parse_uri() {
    let mut input = "<example.org/a>".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::Uri("example.org/a".to_string()));
  }

  #[test]
  fn parse_language_specification() {
    let mut input = "\"a\"@abc".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    // get literal
    lexer.get_next_token();

    assert_eq!(lexer.get_next_token().unwrap(), Token::LanguageSpecification("abc".to_string()));
  }

  #[test]
  fn parse_blank_node() {
    let mut input = "_:auto".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::BlankNode("auto".to_string()));
  }

  #[test]
  fn parse_data_type() {
    let mut input = "\"a\"^^<example.org/abc>".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::Literal("a".to_string()));
    assert_eq!(lexer.get_next_token().unwrap(), Token::DataTypeStart);
    assert_eq!(lexer.get_next_token().unwrap(), Token::Uri("example.org/abc".to_string()));
  }

  #[test]
  fn parse_triple_delimiter() {
    let mut input = ".   \"a\"   .".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
    assert_eq!(lexer.get_next_token().unwrap(), Token::Literal("a".to_string()));
    assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
  }
}