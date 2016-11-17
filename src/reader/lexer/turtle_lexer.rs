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

    match try!(self.input_reader.peek_next_char_discard_leading_spaces()) {
      Some('#') => self.get_comment(),
      Some('@') => self.get_base_or_prefix(),
      Some('"') => self.get_literal(),
      Some('<') => self.get_uri(),
      Some('_') => self.get_blank_node(),
      Some('.') => {
        self.consume_next_char();  // consume '.'
        Ok(Token::TripleDelimiter)
      },
      Some(c) => self.get_qname(),
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

impl<R: Read> TurtleLexer<R> {
  /// Consumes the next character of the input reader.
  fn consume_next_char(&mut self) {
    let _ = self.input_reader.get_next_char();
  }
  
  /// Parses the base or prefix definition.
  fn get_base_or_prefix(&mut self) -> Result<Token> {
    self.consume_next_char(); // consume '@'

    match try!(self.input_reader.get_next_char()) {
      Some('b') => {
        self.get_base_directive()
      },
      Some('p') => {
        self.get_prefix_directive()
      },
      None | Some(_) => Err(Error::new(ErrorType::InvalidReaderInput,
                             "Invalid input while trying to parse base or prefix definition."))
    }
  }

  /// Parses the base directive.
  fn get_base_directive(&mut self) -> Result<Token> {
    let _ = self.input_reader.get_until(|c| c == '<');  // consume '@base'

    match try!(self.get_uri()) {
      Token::Uri(base_uri) => {
        Ok(Token::BaseDirective(base_uri))
      },
      _ => Err(Error::new(ErrorType::InvalidReaderInput,
                          "Invalid URI for Turtle base directive."))
    }
  }

  /// Parses the prefix directive.
  fn get_prefix_directive(&mut self) -> Result<Token> {
    let _ = self.input_reader.get_until(|c| c == ' ');  // consume '@prefix'

    let name = try!(self.input_reader.get_until_discard_leading_spaces(|c| c == ':'));

    let _ = self.input_reader.get_until(|c| c == '<');  // consume characters until URI begin

    match try!(self.get_uri()) {
      Token::Uri(prefix_uri) => {
        Ok(Token::PrefixDirective(name, prefix_uri))
      },
      _ => Err(Error::new(ErrorType::InvalidReaderInput,
                          "Invalid URI for Turtle prefix directive."))
    }
  }

  /// Parses the comment from the input and returns it as token.
  fn get_comment(&mut self) -> Result<Token> {
    self.consume_next_char();    // consume '#'

    match self.input_reader.get_until_discard_leading_spaces(|c| c == '\n' || c == '\r') {
      Ok(str) => {
        self.consume_next_char();  // consume comment delimiter
        Ok(Token::Comment(str))
      },
      Err(err) => {
        match err.error_type() {
          &ErrorType::EndOfInput(ref str) => Ok(Token::Comment(str.clone())),
          _ => Err(Error::new(ErrorType::InvalidReaderInput,
                              "Invalid input for Turtle lexer while parsing comment."))
        }
      }
    }
  }

  /// Parses the language specification from the input and returns it as token.
  fn get_language_specification(&mut self) -> Result<String> {
    match self.input_reader.get_until(|c| c == '\n' || c == '\r' || c == ' ' || c == '.') {
      Ok(str) => Ok(str),
      Err(err) => {
        match err.error_type() {
          &ErrorType::EndOfInput(ref str) => Ok(str.clone()),
          _ => Err(Error::new(ErrorType::InvalidReaderInput,
                              "Invalid input for Turtle lexer while parsing language specification."))
        }
      }
    }
  }

  /// Parses a literal from the input and returns it as token.
  fn get_literal(&mut self) -> Result<Token> {
    self.consume_next_char();  // consume '"'
    let literal = try!(self.input_reader.get_until(|c| c == '"'));
    self.consume_next_char(); // consume '"'

    match try!(self.input_reader.peek_next_char()) {
      Some('@') => {
        self.consume_next_char(); // consume '@'
        let language = try!(self.get_language_specification());
        Ok(Token::LiteralWithLanguageSpecification(literal, language))
      },
      Some('^') => {
        self.consume_next_char(); // consume '^'
        self.consume_next_char(); // consume '^'

        match try!(self.input_reader.peek_next_char()) {
          Some('<') => {    // data type is an URI (NTriples allows only URI data types)
            match try!(self.get_uri()) {
              Token::Uri(datatype_uri) => {
                Ok(Token::LiteralWithUrlDatatype(literal, datatype_uri))
              },
              _ => Err(Error::new(ErrorType::InvalidReaderInput,
                                  "Invalid data type URI for Turtle literal."))
            }
          },
          Some(c) => {
            match try!(self.get_qname()) {
              Token::QName(prefix, path) => Ok(Token::LiteralWithQNameDatatype(literal, prefix, path)),
              _ => Err(Error::new(ErrorType::InvalidReaderInput, "Invalid Turtle input for parsing QName data type."))
            }
          },
          None => Err(Error::new(ErrorType::InvalidReaderInput, "Invalid Turtle input."))
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
    let str = try!(self.input_reader.get_until(|c| c == '>'));
    self.consume_next_char();    // consume '>'
    Ok(Token::Uri(str))
  }

  /// Parses a blank node ID from the input and returns it as token.
  fn get_blank_node(&mut self) -> Result<Token> {
    self.consume_next_char();    // consume '_'

    // get colon after under score
    match try!(self.input_reader.get_next_char()) {
      Some(':') => { }
      Some(c) => return Err(Error::new(ErrorType::InvalidReaderInput,
                                       "Invalid character while parsing Turtle blank node: ".to_string() + &c.to_string())),
      None => return Err(Error::new(ErrorType::InvalidReaderInput,
                                    "Error while parsing Turtle blank node."))
    }

    match self.input_reader.get_until(|c| c == '\n' || c == '\r' || c == ' ' || c == '.') {
      Ok(str) => Ok(Token::BlankNode(str)),
      Err(err) => {
        match err.error_type() {
          &ErrorType::EndOfInput(ref str) => Ok(Token::BlankNode(str.clone())),
          _ => Err(Error::new(ErrorType::InvalidReaderInput,
                              "Invalid input for Turtle lexer while parsing blank node."))
        }
      }
    }
  }

  /// Parses a QName.
  fn get_qname(&mut self) -> Result<Token> {
    let prefix = try!(self.input_reader.get_until(|c| c == ':'));
    self.consume_next_char();    // consume ':'

    match self.input_reader.get_until(|c| c == '\n' || c == '\r' || c == ' ' || c == '.') {
      Ok(str) => Ok(Token::QName(prefix, str)),
      Err(err) => {
        match err.error_type() {
          &ErrorType::EndOfInput(ref str) => Ok(Token::QName(prefix, str.clone())),
          _ => Err(Error::new(ErrorType::InvalidReaderInput,
                              "Invalid input for Turtle lexer while parsing QName."))
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use reader::lexer::rdf_lexer::RdfLexer;
  use reader::lexer::turtle_lexer::TurtleLexer;
  use reader::lexer::token::Token;

  #[test]
  fn parse_base_directive() {
    let input = "@base <http://example.org/> .".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::BaseDirective("http://example.org/".to_string()));
    assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
  }

  #[test]
  fn parse_prefix_directive() {
    let input = "@prefix foaf: <http://xmlns.com/foaf/0.1/> .".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::PrefixDirective("foaf".to_string(), "http://xmlns.com/foaf/0.1/".to_string()));
    assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
  }

  #[test]
  fn parse_comment() {
    let input = "# Hello World!\n# Foo".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::Comment("Hello World!".to_string()));
    assert_eq!(lexer.get_next_token().unwrap(), Token::Comment("Foo".to_string()));
  }

  #[test]
  fn parse_literal() {
    let input = "\"a\"".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::Literal("a".to_string()));
  }

  #[test]
  fn parse_uri() {
    let input = "<example.org/a>".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::Uri("example.org/a".to_string()));
  }

  #[test]
  fn parse_literal_with_language_specification() {
    let input = "\"a\"@abc".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::LiteralWithLanguageSpecification("a".to_string(), "abc".to_string()));
  }

  #[test]
  fn parse_blank_node() {
    let input = "_:auto".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::BlankNode("auto".to_string()));
  }

  #[test]
  fn parse_qname() {
    let input = "abc:def:ghij".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::QName("abc".to_string(), "def:ghij".to_string()));
  }

  #[test]
  fn parse_literal_with_data_type() {
    let input = "\"a\"^^<example.org/abc>".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::LiteralWithUrlDatatype("a".to_string(), "example.org/abc".to_string()));
  }

  #[test]
  fn parse_literal_with_qname_data_type() {
    let input = "\"a\"^^ex:abc:asdf".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::LiteralWithQNameDatatype("a".to_string(), "ex".to_string(), "abc:asdf".to_string()));
  }

  #[test]
  fn parse_triple_delimiter() {
    let input = ".   \"a\"   .".as_bytes();

    let mut lexer = TurtleLexer::new(input);

    assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
    assert_eq!(lexer.get_next_token().unwrap(), Token::Literal("a".to_string()));
    assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
  }
}
