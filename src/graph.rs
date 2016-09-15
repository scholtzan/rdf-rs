use uri::Uri;
use triple::*;
use namespace::*;
use node::*;
use std::slice::Iter;


pub struct Graph {
  base_uri: Uri,
  triples: TripleStore,
  namespaces: NamespaceStore
}

impl Graph {
  pub fn is_empty(&self) -> bool {
    self.triples.is_empty()
  }


  // todo: with language and/or data type and optional namespace
  pub fn create_literal_node(&self, literal: String, namespace: &Namespace) -> Node {
    Node::LiteralNode {
      literal: literal,
      prefix: Some(namespace.prefix().clone()),
      data_type: None,
      language: None
    }
  }

  pub fn triples_iter(&self) -> Iter<Triple> {
    self.triples.iter()
  }
}