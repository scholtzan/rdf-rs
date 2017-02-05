use Result;
use error::{Error, ErrorType};
use reader::lexer::rdf_lexer::RdfLexer;
use reader::lexer::sparql_lexer::SparqlLexer;
use std::io::Read;
use std::io::Cursor;
use sparql::query::{SparqlQuery, SparqlQueryType};
use reader::lexer::token::Token;
use specs::sparql_specs::SparqlKeyword;


/// SPARQL parser to generate a `SparqlQuery` from SPARQL syntax.
pub struct SparqlParser<R: Read> {
  lexer: SparqlLexer<R>
}

impl SparqlParser<Cursor<Vec<u8>>> {
  /// Constructor of `SparqlParser` from input string.
  ///
  /// # Examples
  ///
  /// ```
  /// use rdf_rs::reader::sparql_parser::SparqlParser;
  ///
  /// let input = "SELECT ?test";
  ///
  /// let reader = SparqlParser::from_string(input.to_string());
  /// ```
  pub fn from_string<S>(input: S) -> SparqlParser<Cursor<Vec<u8>>> where S: Into<String> {
    SparqlParser::from_reader(Cursor::new(input.into().into_bytes()))
  }
}

impl<R: Read> SparqlParser<R> {
  /// Constructor of `SparqlParser` from input reader.
  ///
  /// # Examples
  ///
  /// ```
  /// use rdf_rs::reader::sparql_parser::SparqlParser;
  ///
  /// let input = "SELECT ?test";
  ///
  /// let reader = SparqlParser::from_reader(input.as_bytes());
  /// ```
  pub fn from_reader(input: R) -> SparqlParser<R> {
    SparqlParser {
      lexer: SparqlLexer::new(input)
    }
  }

  /// Transforms a SPARQL query string into a `SparqlQuery` object.
  ///
  /// Returns an error if invalid SPARQL is provided.
  ///
  /// # Examples
  ///
  /// todo
  ///
  /// # Failures
  ///
  /// - Invalid input that does not conform with SPARQL standard.
  ///
  pub fn decode(&mut self) -> Result<SparqlQuery> {
    let mut query = SparqlQuery::new(SparqlQueryType::Select);

    loop {
      match self.lexer.peek_next_token() {
        Ok(Token::Comment(_)) => {
          let _ = self.lexer.get_next_token();
          continue
        },
        Ok(Token::EndOfInput) => return Ok(query),
        Err(err) => {
          match err.error_type() {
            &ErrorType::EndOfInput(_) => return Ok(query),
            _ => return Err(Error::new(ErrorType::InvalidReaderInput,
                                       "Error while parsing SPARQL syntax."))
          }
        },
        Ok(_) => {
          return Err(Error::new(ErrorType::InvalidToken,
                                "Invalid token while parsing SPARQL syntax."))
        }
      }
    }

    Ok(query)
  }
}


#[cfg(test)]
mod tests {
  use uri::Uri;
  use sparql::query::*;
  use reader::sparql_parser::SparqlParser;

  fn sparql_query_type_from_string() {
    let input = "SELECT ?title";
    let mut reader = SparqlParser::from_string(input.to_string());

    match reader.decode() {
      Ok(SparqlQuery) => {
        assert!(true)
      }
      Err(e) => {
        println!("Err {}", e.to_string());
        assert!(false)
      }
    }
  }
}