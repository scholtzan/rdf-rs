use Result;
use error::{Error, ErrorType};
use reader::lexer::rdf_lexer::RdfLexer;
use reader::lexer::sparql_lexer::SparqlLexer;
use std::io::Read;
use std::io::Cursor;


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
}