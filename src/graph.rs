use uri::Uri;
use triple::TripleStore;
use namespace::*;
use node::{Node};


pub struct Graph {
  base_uri: Uri,
  triples: TripleStore,
  namespaces: NamespaceStore
}

impl Graph {
  pub fn is_empty(&self) -> bool {
    self.triples.is_empty()
  }


  pub fn create_literal_node(&self, literal: String, namespace: &Namespace) -> Node {
    Node::LiteralNode {
      literal: literal,
      prefix: namespace.prefix().clone()
    }
  }
}