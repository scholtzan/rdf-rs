use Result;
use reader::rdf_reader::RdfReader;
use graph::Graph;
use error::Error;
use triple::Triple;
use reader::lexer::n_triples_lexer::NTriplesLexer;
use reader::lexer::rdf_lexer::RdfLexer;
use node::Node;
use reader::lexer::token::Token;
use std::io::Read;
use uri::Uri;
use std::io::Cursor;

/// RDF reader to generate an RDF graph from N-Triples syntax.
pub struct NTriplesReader<R: Read> {
  lexer: NTriplesLexer<R>
}


impl<R: Read> RdfReader for NTriplesReader<R> {
  /// Generates an RDF graph from a string containing N-Triples syntax.
  ///
  /// Returns in error in case invalid N-Triples syntax is provided.
  ///
  /// # Example
  ///
  /// ```
  /// use rdf_rs::reader::n_triples_reader::NTriplesReader;
  /// use rdf_rs::reader::rdf_reader::RdfReader;
  ///
  /// let input = "<http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://xmlns.com/foaf/0.1/maker> _:art .
  ///              _:art <http://xmlns.com/foaf/0.1/name> \"Art Barstow\" .";
  ///
  /// let mut reader = NTriplesReader::from_string(input.to_string());
  ///
  /// match reader.decode() {
  ///   Ok(graph) => assert_eq!(graph.count(), 2),
  ///   Err(_) => assert!(false)
  /// }
  /// ```
  fn decode(&mut self) -> Result<Graph> {
    let mut graph = Graph::new();

    // todo: parse namespaces

    loop {
      match self.lexer.peek_next_token() {
        Ok(Token::Comment(_)) => {
          self.lexer.get_next_token();
          continue
        },
        Ok(Token::EndOfInput) => return Ok(graph),
        Ok(_) => {},
        Err(err) => return Err(err)
      }

      match self.read_triple() {
        Ok(triple) => graph.add_triple(&triple),
        Err(Error::EndOfInput(_)) => return Ok(graph),
        Err(err) => return Err(err)
      }
    }
  }
}

impl NTriplesReader<Cursor<Vec<u8>>> {
  /// Constructor of `NTriplesReader` from input string.
  pub fn from_string<S>(input: S) -> NTriplesReader<Cursor<Vec<u8>>> where S: Into<String> {
    NTriplesReader::from_reader(Cursor::new(input.into().into_bytes()))
  }
}


impl<R: Read> NTriplesReader<R> {
  /// Constructor of `NTriplesReader` from input reader.
  pub fn from_reader(input: R) -> NTriplesReader<R> {
    NTriplesReader {
      lexer: NTriplesLexer::new(input)
    }
  }

  /// Creates a triple from the parsed tokens.
  fn read_triple(&mut self) -> Result<Triple> {
    let subject = match self.read_subject() {
      Ok(s) => s,
      Err(err) => return Err(err)
    };

    let predicate = match self.read_predicate() {
      Ok(p) => p,
      Err(err) => return Err(err)
    };

    let object = match self.read_object() {
      Ok(o) => o,
      Err(err) => return Err(err)
    };

    match self.lexer.get_next_token() {
      Ok(Token::TripleDelimiter) => {},
      _ => return Err(Error::InvalidReaderInput)
    }

    Ok(Triple::new(subject, predicate, object))
  }

  /// Get the next token and check if it is a valid subject and create a new subject node.
  fn read_subject(&mut self) -> Result<Node> {
    match self.lexer.get_next_token() {
      Ok(Token::BlankNode(id)) => Ok(Node::BlankNode { id: id }),
      Ok(Token::Uri(uri)) => Ok(Node::UriNode { uri: Uri::new(uri) }),
      _ => Err(Error::InvalidToken)
    }
  }

  /// Get the next token and check if it is a valid predicate and create a new predicate node.
  fn read_predicate(&mut self) -> Result<Node> {
    match self.lexer.get_next_token() {
      Ok(Token::Uri(uri)) => Ok(Node::UriNode { uri: Uri::new(uri) }),
      _ => Err(Error::InvalidToken)
    }
  }

  /// Get the next token and check if it is a valid object and create a new object node.
  fn read_object(&mut self) -> Result<Node> {
    match self.lexer.get_next_token() {
      Ok(Token::BlankNode(id)) => Ok(Node::BlankNode { id: id }),
      Ok(Token::Uri(uri)) => Ok(Node::UriNode { uri: Uri::new(uri) }),
      Ok(Token::Literal(literal)) => {
        match self.lexer.peek_next_token() {
          Ok(Token::LanguageSpecification(lang)) => {
            self.lexer.get_next_token();
            Ok(Node::LiteralNode { literal: literal, prefix: None, data_type: None, language: Some(lang) })
          },
          Ok(Token::DataTypeStart) => {
            self.lexer.get_next_token();
            match self.lexer.get_next_token() {
              Ok(Token::Uri(uri)) =>
                Ok(Node::LiteralNode { literal: literal, prefix: None, data_type: Some(Uri::new(uri)), language: None }),
              _ => Err(Error::InvalidToken)
            }
          },
          _ => Ok(Node::LiteralNode { literal: literal, prefix: None, data_type: None, language: None }),
        }
      },
      _ => Err(Error::InvalidToken)
    }
  }
}


#[cfg(test)]
mod tests {
  use reader::n_triples_reader::NTriplesReader;
  use reader::rdf_reader::RdfReader;

  #[test]
  fn read_n_triples_from_string() {
    let input = "<http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Document> .
                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://purl.org/dc/terms/title> \"N-Triples\"@en-US .
                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://xmlns.com/foaf/0.1/maker> _:art .
                 _:art <http://xmlns.com/foaf/0.1/name> \"Art Barstow\" .";

    let mut reader = NTriplesReader::from_string(input.to_string());

    match reader.decode() {
      Ok(graph) => assert_eq!(graph.count(), 4),
      Err(_) => assert!(false)
    }
  }
}




