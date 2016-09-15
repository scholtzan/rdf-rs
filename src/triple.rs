use node::{Node};
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(PartialEq)]
pub enum TripleSegment {
  Subject,
  Predicate,
  Object
}


/// Triple representation.
#[derive(Clone)]
pub struct Triple {
  subject: Node,
  predicate: Node,
  object: Node
}


impl Triple {
  pub fn new(subject: Node, predicate: Node, object: Node) -> Triple {
    Triple {
      subject: subject.clone(),
      predicate: predicate.clone(),
      object: object.clone()
    }
  }


  pub fn subject(&self) -> &Node {
    &self.subject
  }

  pub fn predicate(&self) -> &Node {
    &self.predicate
  }

  pub fn object(&self) -> &Node {
    &self.object
  }
}




/// Storage for triples.
pub struct TripleStore {
  triples: Vec<Triple>
}


impl TripleStore {
  /// Constructs a new triple store.
  pub fn new() -> TripleStore {
    TripleStore {
      triples: Vec::new()
    }
  }

  /// Returns the number of triples that are stored.
  pub fn count(&self) -> usize {
    self.triples.len()
  }

  /// Checks if the triple store is empty.
  pub fn is_empty(&self) -> bool {
    self.count() == 0
  }

  /// Adds a new triple to the store.
  pub fn add_triple(&mut self, triple: Triple) {
    self.triples.push(triple.clone());
  }

  pub fn into_vec(self) -> Vec<Triple> {
    self.triples
  }

  pub fn iter(&self) -> Iter<Triple> {
    self.triples.iter()
  }
}


impl IntoIterator for TripleStore {
  type Item = Triple;
  type IntoIter = IntoIter<Triple>;

  fn into_iter(self) -> Self::IntoIter {
    self.into_iter()
  }
}



#[cfg(test)]
mod tests {
  use node::*;
  use triple::*;
  use std::collections::LinkedList;

  #[test]
  fn empty_triple_store() {
    let store = TripleStore::new();

    assert!(store.is_empty());
  }

  #[test]
  fn count_triples_in_triple_store() {
    let mut store = TripleStore::new();

    let subject = Node::LiteralNode { literal: "abcd".to_string(), prefix: "saf".to_string(), nodeType: LiteralNodeType::PlainLiteral };
    let predicate = Node::LiteralNode { literal: "d".to_string(), prefix: "asdf".to_string(), nodeType: LiteralNodeType::PlainLiteral };
    let object = Node::LiteralNode { literal: "s".to_string(), prefix: "asdf".to_string(), nodeType: LiteralNodeType::PlainLiteral };

    let trip = Triple::new(subject, predicate, object);

    store.add_triple(trip);

    assert_eq!(store.count(), 1);
  }
}
