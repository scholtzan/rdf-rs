use Result;
use reader::rdf_reader::RdfReader;
use graph::Graph;
use error::Error;
use triple::Triple;
use reader::lexer::turtle_lexer::TurtleLexer;
use reader::lexer::rdf_lexer::RdfLexer;
use node::Node;
use reader::lexer::token::Token;
use std::io::Read;
use uri::Uri;
use std::io::Cursor;
use namespace::Namespace;

/// RDF reader to generate an RDF graph from Turtle syntax.
pub struct TurtleReader<R: Read> {
  lexer: TurtleLexer<R>
}

impl<R: Read> RdfReader for TurtleReader<R> {
  /// Generates an RDF graph from a string containing Turtle syntax.
  ///
  /// Returns in error in case invalid Turtle syntax is provided.
  ///
  /// # Example
  ///
  /// todo
  ///
  fn decode(&mut self) -> Result<Graph> {
    let mut graph = Graph::new(None);

    loop {
      match self.lexer.peek_next_token() {
        Ok(Token::Comment(_)) => {
          let _ = self.lexer.get_next_token();
          continue
        },
        Ok(Token::EndOfInput) => return Ok(graph),
        Ok(Token::BaseDirective) => {
          let uri = try!(self.read_base_directive());
          graph.set_base_uri(&uri);
        },
        Ok(Token::PrefixDirective) => {
          let ns = try!(self.read_prefix_directive());
          graph.add_namespace(&ns);
        },
        Ok(Token::Uri(_)) | Ok(Token::BlankNode(_)) | Ok(Token::QName(_, _)) => {
          let triples = try!(self.read_triples());
          graph.add_triples(&triples);
        },
        Err(Error::EndOfInput(_)) => return Ok(graph),
        Ok(_) => return Err(Error::InvalidToken),
        Err(err) => return Err(err)
      }
    }
  }
}

impl TurtleReader<Cursor<Vec<u8>>> {
  /// Constructor of `TurtleReader` from input string.
  pub fn from_string<S>(input: S) -> TurtleReader<Cursor<Vec<u8>>> where S: Into<String> {
    TurtleReader::from_reader(Cursor::new(input.into().into_bytes()))
  }
}


impl<R: Read> TurtleReader<R> {
  /// Constructor of `TurtleReader` from input reader.
  pub fn from_reader(input: R) -> TurtleReader<R> {
    TurtleReader {
      lexer: TurtleLexer::new(input)
    }
  }

  /// Reads the base directive and returns the base URI.
  fn read_base_directive(&mut self) -> Result<Uri> {
    match self.lexer.get_next_token() {
      Ok(Token::Uri(uri)) => Ok(Uri::new(uri)),
      Ok(_) => Err(Error::InvalidToken),
      Err(err) => Err(err)
    }
  }

  /// Reads prefixes and creates namespaces.
  fn read_prefix_directive(&mut self) -> Result<Namespace> {
    let prefix = match self.lexer.get_next_token() {
      Ok(Token::Prefix(p)) => p,
      Ok(_) => return Err(Error::InvalidToken),
      Err(err) => return Err(err)
    };

    let uri = match self.lexer.get_next_token() {
      Ok(Token::Uri(uri)) => Uri::new(uri),
      Ok(_) => return Err(Error::InvalidToken),
      Err(err) => return Err(err)
    };

    Ok(Namespace::new(prefix, uri))
  }

  /// Creates a triple from the parsed tokens.
  fn read_triples(&mut self) -> Result<Vec<Triple>> {
    let mut triples: Vec<Triple> = Vec::new();

    let subject = match self.read_subject() {
      Ok(s) => s,
      Err(err) => return Err(err)
    };

    let (predicate, object) = try!(self.read_predicate_with_object());

    triples.push(Triple::new(&subject, &predicate, &object));

    loop {
      match self.lexer.get_next_token() {
        Ok(Token::TripleDelimiter) => break,
        Ok(Token::PredicateListDelimiter) => {
          let (predicate, object) = try!(self.read_predicate_with_object());
          triples.push(Triple::new(&subject, &predicate, &object));
        },
        Ok(Token::ObjectListDelimiter) => {
          let object = try!(self.read_object());
          triples.push(Triple::new(&subject, &predicate, &object));
        },
        _ => return Err(Error::InvalidReaderInput)
      }
    }

    Ok(triples)
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
  fn read_predicate_with_object(&mut self) -> Result<(Node, Node)> {
    // read the predicate
    let predicate = match self.lexer.get_next_token() {
      Ok(Token::Uri(uri)) => Node::UriNode { uri: Uri::new(uri) },
      _ => return Err(Error::InvalidToken)
    };

    // read the object
    let object = try!(self.read_object());

    Ok((predicate, object))
  }

  /// Get the next token and check if it is a valid object and create a new object node.
  fn read_object(&mut self) -> Result<Node> {
    match self.lexer.get_next_token() {
      Ok(Token::BlankNode(id)) => Ok(Node::BlankNode { id: id }),
      Ok(Token::Uri(uri)) => Ok(Node::UriNode { uri: Uri::new(uri) }),
      Ok(Token::Literal(literal)) => {
        match self.lexer.peek_next_token() {
//          Ok(Token::LanguageSpecification(lang)) => {   todo
//            let _ = self.lexer.get_next_token();
//            Ok(Node::LiteralNode { literal: literal, data_type: None, language: Some(lang) })
//          },
//          Ok(Token::DataTypeStart) => {
//            let _ = self.lexer.get_next_token();
//            match self.lexer.get_next_token() {
//              Ok(Token::Uri(uri)) =>
//                Ok(Node::LiteralNode { literal: literal, data_type: Some(Uri::new(uri)), language: None }),
//              _ => Err(Error::InvalidToken)
//            }
//          },
          _ => Ok(Node::LiteralNode { literal: literal, data_type: None, language: None }),
        }
      },
      _ => Err(Error::InvalidToken)
    }
  }
}