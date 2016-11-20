use Result;
use reader::rdf_parser::RdfParser;
use graph::Graph;
use error::{Error, ErrorType};
use triple::Triple;
use reader::lexer::turtle_lexer::TurtleLexer;
use reader::lexer::rdf_lexer::RdfLexer;
use node::Node;
use reader::lexer::token::Token;
use std::io::Read;
use uri::Uri;
use std::io::Cursor;
use namespace::Namespace;

/// RDF parser to generate an RDF graph from Turtle syntax.
pub struct TurtleParser<R: Read> {
  lexer: TurtleLexer<R>
}

impl<R: Read> RdfParser for TurtleParser<R> {
  /// Generates an RDF graph from a string containing Turtle syntax.
  ///
  /// Returns an error in case invalid Turtle syntax is provided.
  ///
  /// # Example
  ///
  /// ```
  /// use rdf_rs::reader::turtle_parser::TurtleParser;
  /// use rdf_rs::reader::rdf_parser::RdfParser;
  ///
  /// let input = "<http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://xmlns.com/foaf/0.1/maker> _:art .
  ///              _:art <http://xmlns.com/foaf/0.1/name> \"Art Barstow\" .";
  ///
  /// let mut reader = TurtleParser::from_string(input.to_string());
  ///
  /// match reader.decode() {
  ///   Ok(graph) => assert_eq!(graph.count(), 2),
  ///   Err(_) => assert!(false)
  /// }
  /// ```
  fn decode(&mut self) -> Result<Graph> {
    let mut graph = Graph::new(None);

    loop {
      match self.lexer.peek_next_token() {
        Ok(Token::Comment(_)) => {
          let _ = self.lexer.get_next_token();
          continue
        },
        Ok(Token::EndOfInput) => return Ok(graph),
        Ok(Token::BaseDirective(_)) => {
          let base_uri = try!(self.read_base_directive());
          graph.set_base_uri(&base_uri);
        },
        Ok(Token::PrefixDirective(_, _)) => {
          let namespace = try!(self.read_prefix_directive());
          graph.add_namespace(&namespace);
        },
        Ok(Token::Uri(_)) | Ok(Token::BlankNode(_)) | Ok(Token::QName(_, _)) => {
          let triples = try!(self.read_triples(&graph));
          graph.add_triples(&triples);
        },
        Err(err) => {
          match err.error_type() {
            &ErrorType::EndOfInput(_) => return Ok(graph),
            _ => return Err(Error::new(ErrorType::InvalidReaderInput,
                                       "Error while parsing Turtle syntax."))
          }
        }
        Ok(_) => {
          return Err(Error::new(ErrorType::InvalidToken,
                                "Invalid token while parsing Turtle syntax."))
        }
      }
    }
  }
}

impl TurtleParser<Cursor<Vec<u8>>> {
  /// Constructor of `TurtleParser` from input string.
  pub fn from_string<S>(input: S) -> TurtleParser<Cursor<Vec<u8>>> where S: Into<String> {
    TurtleParser::from_reader(Cursor::new(input.into().into_bytes()))
  }
}


impl<R: Read> TurtleParser<R> {
  /// Constructor of `TurtleParser` from input reader.
  pub fn from_reader(input: R) -> TurtleParser<R> {
    TurtleParser {
      lexer: TurtleLexer::new(input)
    }
  }

  /// Parses prefix directives and returns the created namespace.
  fn read_base_directive(&mut self) -> Result<Uri> {
    match try!(self.lexer.get_next_token()) {
      Token::BaseDirective(uri) => {
        match try!(self.lexer.get_next_token()) {
          Token::TripleDelimiter => Ok(Uri::new(uri)),
          _ => Err(Error::new(ErrorType::InvalidReaderInput,
                              "Turtle base directive does not end with '.'"))
        }
      },
      _ => Err(Error::new(ErrorType::InvalidReaderInput,
                          "Invalid input for Turtle base directive."))
    }
  }

  /// Parses prefix directives and returns the created namespace.
  fn read_prefix_directive(&mut self) -> Result<Namespace> {
    match try!(self.lexer.get_next_token()) {
      Token::PrefixDirective(prefix, uri) => {
        match try!(self.lexer.get_next_token()) {
          Token::TripleDelimiter => Ok(Namespace::new(prefix, Uri::new(uri))),
          _ => Err(Error::new(ErrorType::InvalidReaderInput,
                              "Turtle prefix directive does not end with '.'"))
        }
      },
      _ => Err(Error::new(ErrorType::InvalidReaderInput,
                          "Invalid input for Turtle prefix."))
    }
  }

  /// Creates a triple from the parsed tokens.
  fn read_triples(&mut self, graph: &Graph) -> Result<Vec<Triple>> {
    let mut triples: Vec<Triple> = Vec::new();

    let subject = try!(self.read_subject(&graph));
    let (predicate, object) = try!(self.read_predicate_with_object(graph));

    triples.push(Triple::new(&subject, &predicate, &object));

    loop {
      match self.lexer.get_next_token() {
        Ok(Token::TripleDelimiter) => break,
        Ok(Token::PredicateListDelimiter) => {
          let (predicate, object) = try!(self.read_predicate_with_object(graph));
          triples.push(Triple::new(&subject, &predicate, &object));
        },
        Ok(Token::ObjectListDelimiter) => {
          let object = try!(self.read_object(graph));
          triples.push(Triple::new(&subject, &predicate, &object));
        },
        _ => return Err(Error::new(ErrorType::InvalidReaderInput,
                                   "Invalid token while parsing Turtle triples."))
      }
    }

    Ok(triples)
  }

  /// Get the next token and check if it is a valid subject and create a new subject node.
  fn read_subject(&mut self, graph: &Graph) -> Result<Node> {
    match try!(self.lexer.get_next_token()) {
      Token::BlankNode(id) => Ok(Node::BlankNode { id: id }),
      Token::QName(prefix, path) => {
        let mut uri = try!(graph.get_namespace_uri_by_prefix(prefix)).to_owned();
        uri.append_resource_path(path.replace(":", "/"));   // adjust the QName path to URI path
        Ok(Node::UriNode { uri: uri })
      }
      Token::Uri(uri) => Ok(Node::UriNode { uri: Uri::new(uri) }),
      _ => Err(Error::new(ErrorType::InvalidToken,
                          "Invalid token for Turtle subject."))
    }
  }

  /// Get the next token and check if it is a valid predicate and create a new predicate node.
  fn read_predicate_with_object(&mut self, graph: &Graph) -> Result<(Node, Node)> {
    // read the predicate
    let predicate = match try!(self.lexer.get_next_token()) {
      Token::Uri(uri) => Node::UriNode { uri: Uri::new(uri) },
      Token::QName(prefix, path) => {
        let mut uri = try!(graph.get_namespace_uri_by_prefix(prefix)).to_owned();
        uri.append_resource_path(path.replace(":", "/"));   // adjust the QName path to URI path
        Node::UriNode { uri: uri }
      },
      _ => return Err(Error::new(ErrorType::InvalidToken, "Invalid token for Turtle predicate."))
    };

    // read the object
    let object = try!(self.read_object(graph));

    Ok((predicate, object))
  }

  /// Get the next token and check if it is a valid object and create a new object node.
  fn read_object(&mut self, graph: &Graph) -> Result<Node> {
    match try!(self.lexer.get_next_token()) {
      Token::BlankNode(id) => Ok(Node::BlankNode { id: id }),
      Token::Uri(uri) => Ok(Node::UriNode { uri: Uri::new(uri) }),
      Token::QName(prefix, path) => {
        let mut uri = try!(graph.get_namespace_uri_by_prefix(prefix)).to_owned();
        uri.append_resource_path(path.replace(":", "/"));   // adjust the QName path to URI path
        Ok(Node::UriNode { uri: uri })
      },
      Token::LiteralWithLanguageSpecification(literal, lang) =>
        Ok(Node::LiteralNode { literal: literal, data_type: None, language: Some(lang) }),
      Token::LiteralWithUrlDatatype(literal, datatype) =>
        Ok(Node::LiteralNode { literal: literal, data_type: Some(Uri::new(datatype)), language: None }),
      Token::Literal(literal) =>
        Ok(Node::LiteralNode { literal: literal, data_type: None, language: None }),
      _ => Err(Error::new(ErrorType::InvalidToken, "Invalid token for Turtle object."))
    }
  }
}


#[cfg(test)]
mod tests {
  use reader::turtle_parser::TurtleParser;
  use reader::rdf_parser::RdfParser;
  use uri::Uri;

  #[test]
  fn read_n_triples_as_turtle_from_string() {
    let input = "<http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Document> .
                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://purl.org/dc/terms/title> \"N-Triples\"@en-US .
                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://xmlns.com/foaf/0.1/maker> _:art .
                 _:art <http://xmlns.com/foaf/0.1/name> \"Art Barstow\" .";

    let mut reader = TurtleParser::from_string(input.to_string());

    match reader.decode() {
      Ok(graph) => assert_eq!(graph.count(), 4),
      Err(e) => {
        println!("Err {}", e.to_string());
        assert!(false)
      }
    }
  }


  #[test]
  fn read_uncompressed_turtle_from_string() {
    let input = "@base <http://example.org/> .
                 @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
                 @prefix foaf: <http://xmlns.com/foaf/0.1/> .

                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> rdf:type foaf:Document .
                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://purl.org/dc/terms/title> \"N-Triples\"@en-US .
                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> foaf:maker _:art .
                 _:art foaf:name \"Art Barstow\" .";

    let mut reader = TurtleParser::from_string(input.to_string());

    match reader.decode() {
      Ok(graph) => {
        assert_eq!(graph.count(), 4);
        assert_eq!(graph.namespaces().len(), 2);
        assert_eq!(graph.base_uri(), &Some(Uri::new("http://example.org/".to_string())))
      },
      Err(e) => {
        println!("Err {}", e.to_string());
        assert!(false)
      }
    }
  }


  #[test]
  fn read_compressed_turtle_from_string() {
    let input = "@base <http://example.org/> .
                 @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
                 @prefix foaf: <http://xmlns.com/foaf/0.1/> .

                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> rdf:type foaf:Document ;
                                                               <http://purl.org/dc/terms/title> \"N-Triples\"@en-US ;
                                                               foaf:maker _:art .

                 _:art foaf:name \"Art Barstow\" ,
                                 \"Art Барстоу\" ,
                                 \"아트 바스트\" .";

    let mut reader = TurtleParser::from_string(input.to_string());

    match reader.decode() {
      Ok(graph) => {
        assert_eq!(graph.count(), 6);
        assert_eq!(graph.namespaces().len(), 2);
        assert_eq!(graph.base_uri(), &Some(Uri::new("http://example.org/".to_string())))
      },
      Err(e) => {
        println!("Err {}", e.to_string());
        assert!(false)
      }
    }
  }


  #[test]
  fn read_turtle_with_empty_prefix_from_string() {
    let input = "@prefix : <http://example/> .
                 :subject :predicate :object .";

    let mut reader = TurtleParser::from_string(input.to_string());

    match reader.decode() {
      Ok(graph) => assert_eq!(graph.count(), 1),
      Err(e) => {
        println!("Err {}", e.to_string());
        assert!(false)
      }
    }
  }

}