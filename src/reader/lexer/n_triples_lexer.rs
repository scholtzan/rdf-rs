use reader::lexer::rdf_lexer::RdfLexer;
use reader::lexer::token::Token;
use reader::input_reader::InputReader;
use std::io::Read;
use error::{Error, ErrorType};
use Result;

/// Produces tokens from NTriples input.
pub struct NTriplesLexer<R: Read> {
  input_reader: InputReader<R>,
  peeked_token: Option<Token>
}


impl<R: Read> RdfLexer<R> for NTriplesLexer<R> {
  /// Constructor for `NTriplesLexer`;
  ///
  /// # Examples
  ///
  /// ```
  /// use rdf::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf::reader::lexer::n_triples_lexer::NTriplesLexer;
  ///
  /// let input = "<example.org/a>".as_bytes();
  ///
  /// NTriplesLexer::new(input);
  /// ```
  fn new(input: R) -> NTriplesLexer<R> {
    NTriplesLexer {
      input_reader: InputReader::new(input),
      peeked_token: None
    }
  }

  /// Determines the next token from the input.
  ///
  /// # Examples
  ///
  /// ```
  /// use rdf::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf::reader::lexer::n_triples_lexer::NTriplesLexer;
  /// use rdf::reader::lexer::token::Token;
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
  ///
  /// # Failures
  ///
  /// - Input that does not conform to the NTriples standard.
  ///
  fn get_next_token(&mut self) -> Result<Token> {
    match self.peeked_token.clone() {
      Some(token) => {
        self.peeked_token = None;
        return Ok(token)
      },
      None => { }
    }

    match self.input_reader.peek_next_char_discard_leading_spaces()? {
      Some('#') => self.get_comment(),
      Some('"') => self.get_literal(),
      Some('<') => self.get_uri(),
      Some('_') => self.get_blank_node(),
      Some('.') => {
        self.consume_next_char();  // consume '.'
        Ok(Token::TripleDelimiter)
      },
      None => Ok(Token::EndOfInput),
      Some(c) => Err(Error::new(ErrorType::InvalidReaderInput,
                                    "Invalid NTriples input: ".to_string() + &c.to_string()))
    }
  }

  /// Determines the next token without consuming it.
  ///
  /// # Examples
  ///
  /// ```
  /// use rdf::reader::lexer::rdf_lexer::RdfLexer;
  /// use rdf::reader::lexer::n_triples_lexer::NTriplesLexer;
  /// use rdf::reader::lexer::token::Token;
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
  ///
  /// # Failures
  ///
  /// - End of input reached.
  /// - Invalid input that does not conform with NTriples standard.
  ///
  fn peek_next_token(&mut self) -> Result<Token> {
    match self.peeked_token.clone() {
      Some(token) => Ok(token),
      None => {
        let next = self.get_next_token()?;
        self.peeked_token = Some(next.clone());
        return Ok(next)
      }
    }
  }
}


impl<R: Read> NTriplesLexer<R> {
  /// Consumes the next character of the input reader.
  fn consume_next_char(&mut self) {
    let _ = self.input_reader.get_next_char();
  }

  /// Parses the comment from the input and returns it as token.
  fn get_comment(&mut self) -> Result<Token> {
    self.consume_next_char();    // consume '#'

    match self.input_reader.get_until_discard_leading_spaces(|c| c == '\n' || c == '\r') {
      Ok(chars) => {
        self.consume_next_char();  // consume comment delimiter
        Ok(Token::Comment(chars.to_string()))
      },
      Err(err) => {
        match err.error_type() {
          &ErrorType::EndOfInput(ref chars) => Ok(Token::Comment(chars.to_string())),
          _ => Err(Error::new(ErrorType::InvalidReaderInput,
                              "Invalid input for Turtle lexer while parsing comment."))
        }
      }
    }
  }

  /// Parses the language specification from the input and returns it as token.
  fn get_language_specification(&mut self) -> Result<String> {
    match self.input_reader.get_until(|c| c == '\n' || c == '\r' || c == ' ' || c == '.') {
      Ok(chars) => Ok(chars.to_string()),
      Err(err) => {
        match err.error_type() {
          &ErrorType::EndOfInput(ref chars) => Ok(chars.to_string()),
          _ => Err(Error::new(ErrorType::InvalidReaderInput,
                              "Invalid input for NTriples lexer while parsing language specification."))
        }
      }
    }
  }

  /// Parses a literal from the input and returns it as token.
  fn get_literal(&mut self) -> Result<Token> {
    self.consume_next_char();  // consume '"'
    let literal = self.input_reader.get_until(|c| c == '"')?.to_string();
    self.consume_next_char(); // consume '"'

    match self.input_reader.peek_next_char()? {
      Some('@') => {
        self.consume_next_char(); // consume '@'
        let language = self.get_language_specification()?;
        Ok(Token::LiteralWithLanguageSpecification(literal, language))
      },
      Some('^') => {
        self.consume_next_char(); // consume '^'
        self.consume_next_char(); // consume '^'

        match self.input_reader.peek_next_char()? {
          Some('<') => {    // data type is an URI (NTriples allows only URI data types)
            match self.get_uri()? {
              Token::Uri(datatype_uri) => {
                Ok(Token::LiteralWithUrlDatatype(literal, datatype_uri))
              },
              _ => Err(Error::new(ErrorType::InvalidReaderInput,
                                  "Invalid data type URI for NTriples literal."))
            }
          },
          Some(c) => Err(Error::new(ErrorType::InvalidReaderInput,
                                        "Invalid data type token for NTriples: ". to_string() + &c.to_string())),
          None => Err(Error::new(ErrorType::InvalidReaderInput, "Invalid NTriples input."))
        }
      },
      _ => {
        self.consume_next_char(); // consume '"'
        Ok(Token::Literal(literal))
      }
    }
  }

  /// Parses a URI from the input and returns it as token.
  fn get_uri(&mut self) -> Result<Token> {
    self.consume_next_char();    // consume '<'
    let chars = self.input_reader.get_until(|c| c == '>')?;
    self.consume_next_char();    // consume '>'
    Ok(Token::Uri(chars.to_string()))
  }

  /// Parses a blank node ID from the input and returns it as token.
  fn get_blank_node(&mut self) -> Result<Token> {
    self.consume_next_char();    // consume '_'

    // get colon after under score
    match self.input_reader.get_next_char()? {
      Some(':') => { }
      Some(c) => return Err(Error::new(ErrorType::InvalidReaderInput,
                                           "Invalid character while parsing NTriples blank node: ". to_string() + &c.to_string())),
      None => return Err(Error::new(ErrorType::InvalidReaderInput,
                         "Error while parsing NTriples blank node."))
    }

    match self.input_reader.get_until(|c| c == '\n' || c == '\r' || c == ' ' || c == '.') {
      Ok(chars) => Ok(Token::BlankNode(chars.to_string())),
      Err(err) => {
        match err.error_type() {
          &ErrorType::EndOfInput(ref chars) => Ok(Token::BlankNode(chars.to_string())),
          _ => Err(Error::new(ErrorType::InvalidReaderInput,
                              "Invalid input for NTriples lexer while parsing blank node."))
        }
      }
    }
  }
}


#[cfg(test)]
mod tests {
  use reader::lexer::rdf_lexer::RdfLexer;
  use reader::lexer::n_triples_lexer::NTriplesLexer;
  use reader::lexer::token::Token;

  #[test]
  fn test_n_triples_parse_comment() {
    let input = "# Hello World!\n# Foo".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::Comment("Hello World!".to_string()));
    assert_eq!(lexer.get_next_token().unwrap(), Token::Comment("Foo".to_string()));
  }

  #[test]
  fn test_n_triples_parse_literal() {
    let input = "\"a\"".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::Literal("a".to_string()));
  }

  #[test]
  fn test_n_triples_parse_uri() {
    let input = "<example.org/a>".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::Uri("example.org/a".to_string()));
  }

  #[test]
  fn test_n_triples_parse_literal_with_language_specification() {
    let input = "\"a\"@abc".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::LiteralWithLanguageSpecification("a".to_string(),
                                                                                        "abc".to_string()));
  }

  #[test]
  fn test_n_triples_parse_blank_node() {
    let input = "_:auto".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::BlankNode("auto".to_string()));
  }

  #[test]
  fn test_n_triples_parse_literal_with_data_type() {
    let input = "\"a\"^^<example.org/abc>".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::LiteralWithUrlDatatype("a".to_string(),
                                                                              "example.org/abc".to_string()));
  }

  #[test]
  fn test_n_triples_parse_triple_delimiter() {
    let input = ".   \"a\"   .".as_bytes();

    let mut lexer = NTriplesLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
    assert_eq!(lexer.get_next_token().unwrap(), Token::Literal("a".to_string()));
    assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
  }
}