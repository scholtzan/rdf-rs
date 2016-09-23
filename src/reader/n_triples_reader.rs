use Result;
use reader::rdf_reader::RdfReader;
use graph::Graph;
use error::Error;
use triple::Triple;
use reader::lexer::n_triples_lexer::NTriplesLexer;
use reader::lexer::rdf_lexer::RdfLexer;
use node::Node;
use std::io::Read;

/// RDF reader to generate an RDF graph from N-Triples syntax.
pub struct NTriplesReader<R: Read> {
  lexer: NTriplesLexer<R>
}


impl<R: Read> RdfReader for NTriplesReader<R> {
  /// Generates an RDF graph from a string containing N-Triples syntax.
  ///
  /// Returns in error in case invalid N-Triples syntax is provided.
  ///
  fn read_from_string(&self, input_str: &String) -> Result<Graph> {
    // todo
    Ok(Graph::new())
  }
}

impl<R: Read> NTriplesReader<R> {
  /// Constructor of `NTriplesReader`.
  pub fn new(input: R) -> NTriplesReader<R> {
    NTriplesReader {
      lexer: NTriplesLexer::new(input)
    }
  }


  fn read_triple() -> Result<Triple> {
    // todo
    Err(Error::InvalidReaderInput)
  }


  fn read_subject() -> Result<Node> {
    // todo
    Err(Error::InvalidReaderInput)
  }


  fn read_predicate() -> Result<Node> {
    // todo
    Err(Error::InvalidReaderInput)
  }

  fn read_object() -> Result<Node> {
    // todo
    Err(Error::InvalidReaderInput)
  }
}