use uri::Uri;
use triple::*;
use namespace::*;
use node::*;
use std::slice::Iter;


/// Representation of an RDF graph.
pub struct Graph {
  /// Base URI of the RDF graph.
  base_uri: Uri,

  /// All triples of the RDF graph.
  triples: TripleStore,

  /// All namespaces associated to the RDF graph.
  namespaces: NamespaceStore
}


impl Graph {
  /// Returns `true` if the graph does not contains any triples.
  pub fn is_empty(&self) -> bool {
    self.triples.is_empty()
  }

  // todo: with language and/or data type and optional namespace
  /// Returns a literal node of the specified namespace.
  pub fn create_literal_node(&self, literal: String, namespace: &Namespace) -> Node {
    Node::LiteralNode {
      literal: literal,
      prefix: Some(namespace.prefix().clone()),
      data_type: None,
      language: None
    }
  }

  /// Returns an iterator over the triples of the graph.
  pub fn triples_iter(&self) -> Iter<Triple> {
    self.triples.iter()
  }
}