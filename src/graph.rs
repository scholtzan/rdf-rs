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


  pub fn create_literal_node(&self, literal: String, namespace: &Namespace, nodeType: LiteralNodeType) -> Node {
    Node::LiteralNode {
      literal: literal,
      prefix: namespace.prefix().clone(),
      nodeType: nodeType
    }
  }

  pub fn triples_iter(&self) -> Iter<Triple> {
    self.triples.iter()
  }
}