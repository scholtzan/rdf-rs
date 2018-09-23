use error::{Error, ErrorType};
use reader::input_reader::{InputReader, InputReaderHelper};
use reader::lexer::n_triples_lexer::TokensFromNTriples;
use reader::lexer::rdf_lexer::{RdfLexer, TokensFromRdf};
use reader::lexer::token::Token;
use reader::lexer::turtle_lexer::TokensFromTurtle;
use specs::sparql_specs::SparqlKeyword;
use std::io::Read;
use Result;

/// Produces tokens from SPARQL input.
pub struct SparqlLexer<R: Read> {
    input_reader: InputReader<R>,
    peeked_token: Option<Token>,
}

impl<R: Read> RdfLexer<R> for SparqlLexer<R> {
    /// Constructor for `SparqlLexer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::reader::lexer::rdf_lexer::RdfLexer;
    /// use rdf::reader::lexer::sparql_lexer::SparqlLexer;
    ///
    /// let input = "SELECT ?name".as_bytes();
    ///
    /// SparqlLexer::new(input);
    /// ```
    fn new(input: R) -> SparqlLexer<R> {
        SparqlLexer {
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
    /// use rdf::reader::lexer::sparql_lexer::SparqlLexer;
    /// use rdf::reader::lexer::token::Token;
    ///
    /// let input = "SELECT".as_bytes();
    ///
    /// let mut lexer = SparqlLexer::new(input);
    /// ```
    ///
    /// # Failures
    ///
    /// - Input that does not conform to the SPARQL syntax standard.
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

        // todo
        match self.input_reader.peek_next_char_discard_leading_spaces()? {
            Some('#') => return SparqlLexer::get_comment(&mut self.input_reader),
            Some('P') | Some('B') => {
                // try parsing PREFIX or BASE
                match <SparqlLexer<R> as TokensFromTurtle<R>>::get_base_or_prefix(
                    &mut self.input_reader,
                ) {
                    Ok(token) => return Ok(token),
                    _ => {} // continue, because it could still be a QName
                }
            }
            Some('"') | Some('\'') => {
                return <SparqlLexer<R> as TokensFromTurtle<R>>::get_literal(&mut self.input_reader)
            }
            Some('<') => return SparqlLexer::get_uri(&mut self.input_reader),
            Some('_') => return SparqlLexer::get_blank_node(&mut self.input_reader),
            Some('.') => {
                // try to parse a decimal, if there is an error then it is a triple delimiter
                return SparqlLexer::get_numeric(&mut self.input_reader)
                    .or_else(|_| Ok(Token::TripleDelimiter));
            }
            Some('[') => {
                SparqlLexer::consume_next_char(&mut self.input_reader); // consume '['
                return Ok(Token::UnlabeledBlankNodeStart);
            }
            Some(']') => {
                SparqlLexer::consume_next_char(&mut self.input_reader); // consume ']'
                return Ok(Token::UnlabeledBlankNodeEnd);
            }
            Some('{') => {
                SparqlLexer::consume_next_char(&mut self.input_reader); // consume '{'
                return Ok(Token::GroupStart);
            }
            Some('}') => {
                SparqlLexer::consume_next_char(&mut self.input_reader); // consume '}'
                return Ok(Token::GroupEnd);
            }
            Some(',') => {
                SparqlLexer::consume_next_char(&mut self.input_reader); // consume ','
                return Ok(Token::ObjectListDelimiter);
            }
            Some(';') => {
                SparqlLexer::consume_next_char(&mut self.input_reader); // consume ';'
                return Ok(Token::PredicateListDelimiter);
            }
            Some('*') => {
                SparqlLexer::consume_next_char(&mut self.input_reader); // consume '*'
                return Ok(Token::Asterisk);
            }
            Some('?') | Some('$') => {
                SparqlLexer::consume_next_char(&mut self.input_reader); // consume either '?' or '$'
                return SparqlLexer::get_variable(&mut self.input_reader);
            }
            Some('+') | Some('-') => return SparqlLexer::get_numeric(&mut self.input_reader),
            Some(c) if InputReaderHelper::digit(c) => {
                return SparqlLexer::get_numeric(&mut self.input_reader)
            }
            Some(_) => {}
            None => return Ok(Token::EndOfInput),
        }

        SparqlLexer::get_qname_or_keyword(&mut self.input_reader)
    }

    /// Determines the next token without consuming the input.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::reader::lexer::rdf_lexer::RdfLexer;
    /// use rdf::reader::lexer::sparql_lexer::SparqlLexer;
    /// use rdf::reader::lexer::token::Token;
    ///
    /// let input = "SELECT".as_bytes();
    ///
    /// let mut lexer = SparqlLexer::new(input);
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

/// Contains all implemented rules for creating tokens from SPARQL syntax.
pub trait TokensFromSparql<R: Read>: TokensFromTurtle<R> {
    /// Parses the base or prefix definition.
    fn get_base_or_prefix(input_reader: &mut InputReader<R>) -> Result<Token> {
        match input_reader.peek_next_char()? {
            Some('B') => Self::get_base_directive(input_reader),
            Some('P') => Self::get_prefix_directive(input_reader),
            None | Some(_) => Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid input while trying to parse base or prefix definition.",
            )),
        }
    }

    /// Checks if the next word is a SPARQL keyword otherwise handles it as a QName.
    fn get_qname_or_keyword(input_reader: &mut InputReader<R>) -> Result<Token> {
        let input = input_reader.get_until(|c| c == ' ')?;

        match input.to_string().parse::<SparqlKeyword>()? {
            SparqlKeyword::Select => return Ok(Token::Select),
            SparqlKeyword::Where => return Ok(Token::Where),
            SparqlKeyword::Distinct => return Ok(Token::Distinct),
            SparqlKeyword::Reduced => return Ok(Token::Reduced),
            SparqlKeyword::Construct => return Ok(Token::Construct),
            SparqlKeyword::Describe => return Ok(Token::Describe),
            SparqlKeyword::Ask => return Ok(Token::Ask),
            SparqlKeyword::From => return Ok(Token::From),
            SparqlKeyword::Named => return Ok(Token::Named),
            SparqlKeyword::Order => return Ok(Token::Order),
            SparqlKeyword::By => return Ok(Token::By),
            SparqlKeyword::Asc => return Ok(Token::Asc),
            SparqlKeyword::Desc => return Ok(Token::Desc),
            SparqlKeyword::Offset => return Ok(Token::Offset),
            SparqlKeyword::Optional => return Ok(Token::Optional),
            SparqlKeyword::Filter => return Ok(Token::Filter),
            SparqlKeyword::Graph => return Ok(Token::Graph),
            SparqlKeyword::Union => return Ok(Token::Union),
            SparqlKeyword::Regex => return Ok(Token::Regex),
            _ => {}
        }

        Self::get_qname(input_reader)
    }

    /// Parses a SPARQL variable.
    fn get_variable(input_reader: &mut InputReader<R>) -> Result<Token> {
        let variable_name =
            input_reader.get_until_discard_leading_spaces(InputReaderHelper::node_delimiter)?;

        Ok(Token::SparqlVariable(variable_name.to_string()))
    }
}

impl<R: Read> TokensFromRdf<R> for SparqlLexer<R> {}
impl<R: Read> TokensFromNTriples<R> for SparqlLexer<R> {}
impl<R: Read> TokensFromTurtle<R> for SparqlLexer<R> {}
impl<R: Read> TokensFromSparql<R> for SparqlLexer<R> {}

#[cfg(test)]
mod tests {
    use reader::lexer::rdf_lexer::RdfLexer;
    use reader::lexer::sparql_lexer::SparqlLexer;
    use reader::lexer::token::Token;

    #[test]
    fn parse_variable() {
        let input = "?var1 $var2 ".as_bytes();

        let mut lexer = SparqlLexer::new(input);

        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::SparqlVariable("var1".to_string())
        );
        assert_eq!(
            lexer.get_next_token().unwrap(),
            Token::SparqlVariable("var2".to_string())
        );
    }
}
