use error::{Error, ErrorType};
use reader::input_reader::{InputReader, InputReaderHelper};
use reader::lexer::n_triples_lexer::TokensFromNTriples;
use reader::lexer::rdf_lexer::RdfLexer;
use reader::lexer::rdf_lexer::TokensFromRdf;
use reader::lexer::token::Token;
use specs::turtle_specs::TurtleSpecs;
use specs::xml_specs::XmlDataTypes;
use std::io::Read;
use Result;

/// Produces tokens from Turtle syntax input.
pub struct TurtleLexer<R: Read> {
    input_reader: InputReader<R>,
    peeked_token: Option<Token>,
}

/// Contains all implemented rules for creating tokens from Turtle syntax.
pub trait TokensFromTurtle<R: Read>: TokensFromNTriples<R> {
    /// Parses the base or prefix definition.
    fn get_base_or_prefix(input_reader: &mut InputReader<R>) -> Result<Token> {
        match input_reader.peek_next_char()? {
            Some('b') | Some('B') => Self::get_base_directive(input_reader),
            Some('p') | Some('P') => Self::get_prefix_directive(input_reader),
            None | Some(_) => Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid input while trying to parse base or prefix definition.",
            )),
        }
    }

    /// Parses the base directive.
    fn get_base_directive(input_reader: &mut InputReader<R>) -> Result<Token> {
        let base_directive = input_reader.peek_next_k_chars(5)?;

        if base_directive.to_string().to_lowercase() != "base " {
            return Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid URI for base directive.",
            ));
        }

        let _ = input_reader.get_until(|c| c == '<'); // consume 'base'

        match Self::get_uri(input_reader)? {
            Token::Uri(base_uri) => Ok(Token::BaseDirective(base_uri)),
            _ => Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid URI for base directive.",
            )),
        }
    }

    /// Parses the prefix directive.
    fn get_prefix_directive(input_reader: &mut InputReader<R>) -> Result<Token> {
        let prefix_directive = input_reader.peek_next_k_chars(7)?;

        if prefix_directive.to_string().to_lowercase() != "prefix " {
            return Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid URI for base directive.",
            ));
        }

        let _ = input_reader.get_until(|c| c == ' '); // consume 'prefix'

        // get prefix name including ':'
        let mut name = input_reader
            .get_until_discard_leading_spaces(|c| c == ':')?
            .to_string();
        name.push(':');

        let _ = input_reader.get_until(|c| c == '<'); // consume characters until URI begin

        match Self::get_uri(input_reader)? {
            Token::Uri(prefix_uri) => Ok(Token::PrefixDirective(name, prefix_uri)),
            _ => Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid URI for prefix directive.",
            )),
        }
    }

    /// Parses integer, decimals and doubles.
    fn get_numeric(input_reader: &mut InputReader<R>) -> Result<Token> {
        let numeric =
            input_reader.get_until_discard_leading_spaces(InputReaderHelper::node_delimiter)?;

        // check if delimiter was '.' and if it is part of a decimal or if it is a delimiter
        if input_reader.get_next_char()? == Some('.') {
            let mut complete_numeric = numeric.clone();
            match input_reader.peek_until(InputReaderHelper::node_delimiter) {
                Ok(mut input_chars) => {
                    complete_numeric.push(Some('.'));
                    complete_numeric.append(&mut input_chars);

                    if TurtleSpecs::is_double_literal(&complete_numeric.to_string()) {
                        let _ = input_reader
                            .get_until_discard_leading_spaces(InputReaderHelper::node_delimiter)?; // consume
                        return Ok(Token::LiteralWithUrlDatatype(
                            complete_numeric.to_string(),
                            XmlDataTypes::Double.to_string(),
                        ));
                    }
                }
                _ => {}
            }
        }

        if TurtleSpecs::is_integer_literal(&numeric.to_string()) {
            return Ok(Token::LiteralWithUrlDatatype(
                numeric.to_string(),
                XmlDataTypes::Integer.to_string(),
            ));
        } else if TurtleSpecs::is_double_literal(&numeric.to_string()) {
            return Ok(Token::LiteralWithUrlDatatype(
                numeric.to_string(),
                XmlDataTypes::Double.to_string(),
            ));
        } else {
            return Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid input for numeric literal.",
            ));
        }
    }

    /// Parses a boolean value and returns it as token.
    fn get_boolean_literal(input_reader: &mut InputReader<R>) -> Result<Token> {
        let boolean =
            input_reader.peek_until_discard_leading_spaces(InputReaderHelper::node_delimiter)?;

        if TurtleSpecs::is_boolean_literal(&boolean.to_string()) {
            return Ok(Token::LiteralWithUrlDatatype(
                boolean.to_string(),
                XmlDataTypes::Boolean.to_string(),
            ));
        } else {
            return Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid input for boolean.",
            ));
        }
    }

    /// Parses the 'a' keyword.
    fn get_a_keyword(input_reader: &mut InputReader<R>) -> Result<Token> {
        let a =
            input_reader.peek_until_discard_leading_spaces(InputReaderHelper::node_delimiter)?;

        if a.len() == 1 && a[0] == Some('a') {
            return Ok(Token::KeywordA);
        } else {
            return Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid input for keyword 'a'.",
            ));
        }
    }

    /// Parses a literal from the input and returns it as token.
    /// Parses a literal from the input and returns it as token.
    fn get_literal(input_reader: &mut InputReader<R>) -> Result<Token> {
        let literal_delimiter = input_reader.get_next_char()?;
        let mut is_multiline = false;

        let potential_literal_quotes = input_reader.peek_next_k_chars(2)?;

        // check if the literal is multiline
        if potential_literal_quotes[0] == literal_delimiter
            && potential_literal_quotes[1] == literal_delimiter
        {
            is_multiline = true;
            let _ = input_reader.get_next_k_chars(2); // consume
        }

        let mut found_literal_end = false;
        let mut literal = "".to_string();

        while !found_literal_end {
            literal.push_str(
                &input_reader
                    .get_until(|c| c == literal_delimiter.unwrap())?
                    .to_string(),
            );

            if is_multiline {
                // if not escaped check if the literal is complete
                let potential_literal_delimiters = input_reader.peek_next_k_chars(2)?.to_vec();

                if potential_literal_delimiters[0] == literal_delimiter
                    && potential_literal_delimiters[1] == literal_delimiter
                {
                    Self::consume_next_char(input_reader);
                    Self::consume_next_char(input_reader);

                    found_literal_end = true;
                } else {
                    literal.push_str(&input_reader.get_next_k_chars(1)?.to_string());
                }
            } else {
                found_literal_end = true;
            }
        }

        Self::consume_next_char(input_reader); // consume '"'

        match input_reader.peek_next_char()? {
            Some('@') => {
                Self::consume_next_char(input_reader); // consume '@'
                let language = Self::get_language_specification(input_reader)?;
                Ok(Token::LiteralWithLanguageSpecification(literal, language))
            }
            Some('^') => {
                Self::consume_next_char(input_reader); // consume '^'
                Self::consume_next_char(input_reader); // consume '^'

                match input_reader.peek_next_char()? {
                    Some('<') => {
                        // data type is an URI (NTriples allows only URI data types)
                        match Self::get_uri(input_reader)? {
                            Token::Uri(datatype_uri) => {
                                Ok(Token::LiteralWithUrlDatatype(literal, datatype_uri))
                            }
                            _ => Err(Error::new(
                                ErrorType::InvalidReaderInput,
                                "Invalid data type URI for Turtle literal.",
                            )),
                        }
                    }
                    Some(_) => match Self::get_qname(input_reader)? {
                        Token::QName(prefix, path) => {
                            Ok(Token::LiteralWithQNameDatatype(literal, prefix, path))
                        }
                        _ => Err(Error::new(
                            ErrorType::InvalidReaderInput,
                            "Invalid Turtle input for parsing QName data type.",
                        )),
                    },
                    None => Err(Error::new(
                        ErrorType::InvalidReaderInput,
                        "Invalid Turtle input.",
                    )),
                }
            }
            _ => Ok(Token::Literal(literal)),
        }
    }

    /// Parses a QName.
    fn get_qname(input_reader: &mut InputReader<R>) -> Result<Token> {
        let mut prefix = input_reader.get_until(|c| c == ':')?.to_string();
        prefix.push(':'); // ':' is part of prefix name
        Self::consume_next_char(input_reader); // consume ':'

        match input_reader.get_until(InputReaderHelper::node_delimiter) {
            Ok(chars) => Ok(Token::QName(prefix, chars.to_string())),
            Err(err) => match err.error_type() {
                &ErrorType::EndOfInput(ref chars) => Ok(Token::QName(prefix, chars.to_string())),
                _ => Err(Error::new(
                    ErrorType::InvalidReaderInput,
                    "Invalid input for Turtle lexer while parsing QName.",
                )),
            },
        }
    }
}

impl<R: Read> TokensFromRdf<R> for TurtleLexer<R> {}
impl<R: Read> TokensFromNTriples<R> for TurtleLexer<R> {}
impl<R: Read> TokensFromTurtle<R> for TurtleLexer<R> {}

impl<R: Read> RdfLexer<R> for TurtleLexer<R> {
    /// Constructor for `TurtleLexer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::reader::lexer::rdf_lexer::RdfLexer;
    /// use rdf::reader::lexer::turtle_lexer::TurtleLexer;
    ///
    /// let input = "<example.org/a>".as_bytes();
    ///
    /// TurtleLexer::new(input);
    /// ```
    fn new(input: R) -> TurtleLexer<R> {
        TurtleLexer {
            input_reader: InputReader::new(input),
            peeked_token: None,
        }
    }

    /// Determines the next token from the input and consumes the read input characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::reader::lexer::rdf_lexer::RdfLexer;
    /// use rdf::reader::lexer::turtle_lexer::TurtleLexer;
    /// use rdf::reader::lexer::token::Token;
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
    ///
    /// # Failures
    ///
    /// - Input that does not conform to the Turtle syntax standard.
    ///
    fn get_next_token(&mut self) -> Result<Token> {
        // first read peeked characters
        match self.peeked_token.clone() {
            Some(token) => {
                self.peeked_token = None;
                return Ok(token);
            }
            None => {}
        }

        match self.input_reader.peek_next_char_discard_leading_spaces()? {
            Some('#') => return TurtleLexer::get_comment(&mut self.input_reader),
            Some('@') => {
                TurtleLexer::consume_next_char(&mut self.input_reader); // consume '@'
                return TurtleLexer::get_base_or_prefix(&mut self.input_reader);
            }
            Some('"') | Some('\'') => {
                return <TurtleLexer<R> as TokensFromTurtle<R>>::get_literal(&mut self.input_reader)
            }
            Some('<') => return TurtleLexer::get_uri(&mut self.input_reader),
            Some('_') => return TurtleLexer::get_blank_node(&mut self.input_reader),
            Some('.') => {
                // try to parse a decimal, if there is an error then it is a triple delimiter
                return TurtleLexer::get_numeric(&mut self.input_reader)
                    .or_else(|_| Ok(Token::TripleDelimiter));
            }
            Some(',') => {
                TurtleLexer::consume_next_char(&mut self.input_reader); // consume ','
                return Ok(Token::ObjectListDelimiter);
            }
            Some(';') => {
                TurtleLexer::consume_next_char(&mut self.input_reader); // consume ';'
                return Ok(Token::PredicateListDelimiter);
            }
            Some('(') => {
                TurtleLexer::consume_next_char(&mut self.input_reader); // consume '('
                return Ok(Token::CollectionStart);
            }
            Some(')') => {
                TurtleLexer::consume_next_char(&mut self.input_reader); // consume ')'
                return Ok(Token::CollectionEnd);
            }
            Some('[') => {
                TurtleLexer::consume_next_char(&mut self.input_reader); // consume '['
                return Ok(Token::UnlabeledBlankNodeStart);
            }
            Some(']') => {
                TurtleLexer::consume_next_char(&mut self.input_reader); // consume ']'
                return Ok(Token::UnlabeledBlankNodeEnd);
            }
            Some('P') | Some('B') => {
                // try parsing PREFIX or BASE
                match TurtleLexer::get_base_or_prefix(&mut self.input_reader) {
                    Ok(token) => return Ok(token),
                    _ => {} // continue, because it could still be a QName
                }
            }
            Some('t') | Some('f') => {
                // try parsing 'true' or 'false'
                match TurtleLexer::get_boolean_literal(&mut self.input_reader) {
                    Ok(token) => return Ok(token),
                    _ => {} // continue, because it could still be a QName
                }
            }
            Some('a') => {
                // try parsing the 'a' keyword
                match TurtleLexer::get_a_keyword(&mut self.input_reader) {
                    Ok(token) => return Ok(token),
                    _ => {} // continue, because it could still be a QName
                }
            }
            Some('+') | Some('-') => return TurtleLexer::get_numeric(&mut self.input_reader),
            Some(c) if InputReaderHelper::digit(c) => {
                return TurtleLexer::get_numeric(&mut self.input_reader)
            }
            Some(_) => {}
            None => return Ok(Token::EndOfInput),
        }

        TurtleLexer::get_qname(&mut self.input_reader)
    }

    /// Determines the next token without consuming the input.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::reader::lexer::rdf_lexer::RdfLexer;
    /// use rdf::reader::lexer::turtle_lexer::TurtleLexer;
    /// use rdf::reader::lexer::token::Token;
    ///
    /// let input = "_:auto <example.org/b> \"test\" .".as_bytes();
    ///
    /// let mut lexer = TurtleLexer::new(input);
    ///
    /// assert_eq!(lexer.peek_next_token().unwrap(), Token::BlankNode("auto".to_string()));
    /// assert_eq!(lexer.peek_next_token().unwrap(), Token::BlankNode("auto".to_string()));
    /// assert_eq!(lexer.get_next_token().unwrap(), Token::BlankNode("auto".to_string()));
    /// assert_eq!(lexer.get_next_token().unwrap(), Token::Uri("example.org/b".to_string()));
    /// ```
    ///
    ///  # Failures
    ///
    /// - End of input reached.
    /// - Invalid input that does not conform with NTriples standard.
    ///
    fn peek_next_token(&mut self) -> Result<Token> {
        match self.peeked_token.clone() {
            Some(token) => Ok(token),
            None => match self.get_next_token() {
                Ok(next) => {
                    self.peeked_token = Some(next.clone());
                    return Ok(next);
                }
                Err(err) => return Err(err),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use reader::lexer::rdf_lexer::RdfLexer;
    use reader::lexer::token::Token;
    use reader::lexer::turtle_lexer::TurtleLexer;
    use specs::xml_specs::XmlDataTypes;

    #[test]
    fn parse_base_directive() {
        let input = "@base <http://example.org/> .".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::BaseDirective("http://example.org/".to_string())
        );
        assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
    }

    #[test]
    fn parse_sparql_base_directive() {
        let input = "BASE <http://example.org/> .".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::BaseDirective("http://example.org/".to_string())
        );
        assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
    }

    #[test]
    fn parse_prefix_directive() {
        let input = "@prefix foaf: <http://xmlns.com/foaf/0.1/> .".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::PrefixDirective(
                "foaf:".to_string(),
                "http://xmlns.com/foaf/0.1/".to_string()
            )
        );
        assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
    }

    #[test]
    fn parse_sparql_prefix_directive() {
        let input = "PREFIX foaf: <http://xmlns.com/foaf/0.1/> .".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::PrefixDirective(
                "foaf:".to_string(),
                "http://xmlns.com/foaf/0.1/".to_string()
            )
        );
        assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
    }

    #[test]
    fn parse_comment() {
        let input = "# Hello World!\n# Foo".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::Comment("Hello World!".to_string())
        );
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::Comment("Foo".to_string())
        );
    }

    #[test]
    fn parse_literal() {
        let input = "\"a\"".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::Literal("a".to_string())
        );
    }

    #[test]
    fn parse_uri() {
        let input = "<example.org/a>".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::Uri("example.org/a".to_string())
        );
    }

    #[test]
    fn parse_literal_with_language_specification() {
        let input = "\"a\"@abc".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::LiteralWithLanguageSpecification("a".to_string(), "abc".to_string())
        );
    }

    #[test]
    fn parse_blank_node() {
        let input = ". _:auto .".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::BlankNode("auto".to_string())
        );
    }

    #[test]
    fn parse_qname() {
        let input = " abc:def:ghij  gggg:gggg   abc:dd .".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::QName("abc:".to_string(), "def:ghij".to_string())
        );
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::QName("gggg:".to_string(), "gggg".to_string())
        );
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::QName("abc:".to_string(), "dd".to_string())
        );
    }

    #[test]
    fn parse_literal_with_data_type() {
        let input = "\"a\"^^<example.org/abc>".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::LiteralWithUrlDatatype("a".to_string(), "example.org/abc".to_string())
        );
    }

    #[test]
    fn parse_literal_with_qname_data_type() {
        let input = "\"a\"^^ex:abc:asdf".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::LiteralWithQNameDatatype(
                "a".to_string(),
                "ex:".to_string(),
                "abc:asdf".to_string()
            )
        );
    }

    #[test]
    fn parse_triple_delimiter() {
        let input = ". \"a\"   . ".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::Literal("a".to_string())
        );
        assert_eq!(lexer.get_next_token().unwrap(), Token::TripleDelimiter);
    }

    #[test]
    fn parse_multiline_literal_delimiter() {
        let input = "'''don't do \"this\"\''''".as_bytes();

        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::Literal("don't do \"this\"".to_string())
        );
    }

    #[test]
    fn parse_numeric_literals() {
        let input = "4 1.2 -5.123 -.123 .123 5e10 .".as_bytes();
        let mut lexer = TurtleLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::LiteralWithUrlDatatype("4".to_string(), XmlDataTypes::Integer.to_string())
        );
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::LiteralWithUrlDatatype("1.2".to_string(), XmlDataTypes::Double.to_string())
        );
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::LiteralWithUrlDatatype("-5.123".to_string(), XmlDataTypes::Double.to_string())
        );
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::LiteralWithUrlDatatype("-.123".to_string(), XmlDataTypes::Double.to_string())
        );
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::LiteralWithUrlDatatype(".123".to_string(), XmlDataTypes::Double.to_string())
        );
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::LiteralWithUrlDatatype("5e10".to_string(), XmlDataTypes::Double.to_string())
        );
    }
}
